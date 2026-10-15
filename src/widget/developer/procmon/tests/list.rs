//! Filtering, sorting and moving the selection, against plain models

use super::{bare, draw, highlighted_rows, monitor_with, numbered, pids_on_screen, process};
use crate::widget::developer::procmon::{ProcessInfo, ProcessSort, ProcessView};
use proptest::prelude::*;
use std::cmp::Ordering;

const SORTS: [ProcessSort; 5] = [
    ProcessSort::Pid,
    ProcessSort::Name,
    ProcessSort::Cpu,
    ProcessSort::Memory,
    ProcessSort::Status,
];

/// A process from a small pool of names, commands, users and statuses, so
/// that filters match some, users split the list and sort keys tie. CPU
/// values are finite: a NaN breaks the CPU sort (#986).
fn arb_process() -> impl Strategy<Value = ProcessInfo> {
    (
        0u32..50,
        prop::sample::select(vec!["sh", "SH", "bash", "Zsh", "rustc", "init", "a"]),
        prop::sample::select(vec![0.0f32, 30.0, 30.1, 80.0, 80.1, 5.5, 250.0]),
        0u64..5,
        prop::sample::select(vec!["Run", "Sleeping", "Idle", "Zombie"]),
        prop::sample::select(vec!["", "/bin/sh -c x", "cargo build", "BASH --login"]),
        prop::sample::select(vec!["0", "501"]),
    )
        .prop_map(|(pid, name, cpu, memory, status, cmd, user)| {
            process(pid, name, cpu, memory, 0.0, status, cmd, user)
        })
}

/// The order `sort` and `asc` ask for, as a comparison
fn ordering(sort: ProcessSort, asc: bool, a: &ProcessInfo, b: &ProcessInfo) -> Ordering {
    let ord = match sort {
        ProcessSort::Pid => a.pid.cmp(&b.pid),
        ProcessSort::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        ProcessSort::Cpu => a.cpu.total_cmp(&b.cpu),
        ProcessSort::Memory => a.memory.cmp(&b.memory),
        ProcessSort::Status => a.status.cmp(&b.status),
    };
    if asc {
        ord
    } else {
        ord.reverse()
    }
}

/// What the keys key one process: the fields the tests can tell apart
fn key(p: &ProcessInfo) -> (u32, String, u32, u64, String, String, String) {
    (
        p.pid,
        p.name.clone(),
        p.cpu.to_bits(),
        p.memory,
        p.status.clone(),
        p.cmd.clone(),
        p.user.clone(),
    )
}

#[derive(Clone, Debug)]
enum Move {
    Next,
    Prev,
    PageDown(usize),
    PageUp(usize),
}

fn arb_move() -> impl Strategy<Value = Move> {
    prop_oneof![
        Just(Move::Next),
        Just(Move::Prev),
        (0usize..15).prop_map(Move::PageDown),
        (0usize..15).prop_map(Move::PageUp),
    ]
}

proptest! {
    /// A new list keeps the processes the view and the filter let through
    /// (the filter ignores case and also looks at the command), in the sort
    /// order; processes that tie keep the order they came in.
    #[test]
    fn a_list_is_filtered_and_sorted(
        list in prop::collection::vec(arb_process(), 0..25),
        sort in prop::sample::select(SORTS.to_vec()),
        asc in any::<bool>(),
        user_view in any::<bool>(),
        current_user in prop::option::of(prop::sample::select(vec!["0", "501"])),
        filter in prop::sample::select(vec!["", "sh", "SH", "Bash", "build", "nothing"]),
        down in 0usize..30,
    ) {
        let view = if user_view { ProcessView::User } else { ProcessView::All };
        let mut monitor = bare().sort_by(sort).ascending(asc).view(view);
        monitor.current_user = current_user.map(str::to_string);
        monitor.set_process_list(numbered(30));
        monitor.page_down(down);
        monitor.filter(filter);
        monitor.set_process_list(list.clone());

        let needle = filter.to_lowercase();
        let kept: Vec<(usize, &ProcessInfo)> = list
            .iter()
            .enumerate()
            .filter(|(_, p)| match (user_view, current_user) {
                (true, Some(user)) => p.user == user,
                _ => true,
            })
            .filter(|(_, p)| {
                p.name.to_lowercase().contains(&needle) || p.cmd.to_lowercase().contains(&needle)
            })
            .collect();
        prop_assert_eq!(monitor.process_count(), kept.len());

        // Same processes, sorted, ties in their incoming order
        let mut want = kept.clone();
        want.sort_by(|(i, a), (j, b)| ordering(sort, asc, a, b).then(i.cmp(j)));
        let got: Vec<_> = monitor.processes.iter().map(key).collect();
        let want: Vec<_> = want.iter().map(|(_, p)| key(p)).collect();
        prop_assert_eq!(got, want);

        // `filter` started the selection over; it stays on the list. Where it
        // lands after a list change is #975 (kept as a position).
        prop_assert_eq!(monitor.selected_process().is_some(), !kept.is_empty());
    }

    /// Moves keep the selection on the list, as a plain index model says,
    /// and the selected process is drawn highlighted wherever it is.
    #[test]
    fn moves_follow_an_index_model(
        n in 0usize..40,
        moves in prop::collection::vec(arb_move(), 0..30),
        height in 5u16..20,
    ) {
        let mut monitor = monitor_with(numbered(n));
        let last = n.saturating_sub(1);
        let mut model = 0usize;
        for m in &moves {
            match *m {
                Move::Next => {
                    monitor.select_next();
                    model = (model + 1).min(last);
                }
                Move::Prev => {
                    monitor.select_prev();
                    model = model.saturating_sub(1);
                }
                Move::PageDown(k) => {
                    monitor.page_down(k);
                    model = (model + k).min(last);
                }
                Move::PageUp(k) => {
                    monitor.page_up(k);
                    model = model.saturating_sub(k);
                }
            }
            prop_assert_eq!(
                monitor.selected_process().map(|p| p.pid),
                (n > 0).then_some(100 + model as u32)
            );
        }

        // Rows show consecutive processes and the selection is among them.
        // Which page is shown after moving back up is #973.
        let buffer = draw(&monitor, 60, height);
        let pids = pids_on_screen(&buffer);
        prop_assert_eq!(pids.len(), n.min(height as usize - 2));
        prop_assert!(pids.windows(2).all(|w| w[1] == w[0] + 1));
        if n > 0 {
            let y = pids.iter().position(|&pid| pid == 100 + model as u32);
            prop_assert!(y.is_some(), "selection {model} not in {pids:?}");
            prop_assert_eq!(highlighted_rows(&monitor, &buffer), vec![2 + y.unwrap() as u16]);
        } else {
            prop_assert!(highlighted_rows(&monitor, &buffer).is_empty());
        }
    }

    /// Choosing the sort column again flips the direction; another column
    /// sorts descending. The header's arrow follows.
    #[test]
    fn toggle_sort_follows_a_model(
        start in prop::sample::select(SORTS.to_vec()),
        start_asc in any::<bool>(),
        toggles in prop::collection::vec(prop::sample::select(SORTS.to_vec()), 0..12),
    ) {
        let mut monitor = bare().sort_by(start).ascending(start_asc);
        let (mut sort, mut asc) = (start, start_asc);
        for column in toggles {
            monitor.toggle_sort(column);
            if column == sort {
                asc = !asc;
            } else {
                sort = column;
                asc = false;
            }
            prop_assert_eq!((monitor.sort, monitor.sort_asc), (sort, asc));
        }
        let header = super::row(&draw(&monitor, 70, 5), 1);
        let arrow = if asc { "▲" } else { "▼" };
        let name = match sort {
            ProcessSort::Pid => "PID",
            ProcessSort::Name => "NAME",
            ProcessSort::Cpu => "CPU%",
            ProcessSort::Memory => "MEM",
            ProcessSort::Status => "STATUS",
        };
        prop_assert!(header.contains(&format!("{name}{arrow}")), "{header}");
        let arrows = header.matches(arrow).count();
        prop_assert_eq!(arrows, if sort == ProcessSort::Memory { 2 } else { 1 });
    }
}

#[test]
fn moves_on_an_empty_list_select_nothing() {
    let mut monitor = monitor_with(Vec::new());
    monitor.select_next();
    monitor.page_down(5);
    monitor.select_prev();
    monitor.page_up(5);
    assert!(monitor.selected_process().is_none());
    assert_eq!(monitor.process_count(), 0);
}

#[test]
fn the_user_view_keeps_everything_when_the_user_is_unknown() {
    let list = vec![
        process(1, "init", 0.0, 0, 0.0, "Run", "", "0"),
        process(2, "mine", 0.0, 0, 0.0, "Run", "", "501"),
    ];
    let mut monitor = bare().view(ProcessView::User);
    monitor.set_process_list(list);
    assert_eq!(monitor.process_count(), 2);
}
