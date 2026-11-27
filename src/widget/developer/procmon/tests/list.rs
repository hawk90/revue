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
        // Pages up to usize::MAX: callers pass a huge page for "to the end"
        // (#1084)
        prop_oneof![0usize..15, Just(usize::MAX)].prop_map(Move::PageDown),
        prop_oneof![0usize..15, Just(usize::MAX)].prop_map(Move::PageUp),
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
                    model = model.saturating_add(k).min(last);
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

    /// Changing the filter or the sort after the list came in lists the same
    /// processes, in the same order, as setting them before it (#1084)
    #[test]
    fn settings_changed_after_the_list_apply_to_it_at_once(
        list in prop::collection::vec(arb_process(), 0..25),
        sort in prop::sample::select(SORTS.to_vec()),
        asc in any::<bool>(),
        filter in prop::sample::select(vec!["", "sh", "Bash", "build", "nothing"]),
    ) {
        let mut before = bare().sort_by(sort).ascending(asc);
        before.filter(filter);
        before.set_process_list(list.clone());

        // Listed first with the defaults (CPU, descending, no filter), then
        // filtered and sorted with the calls an app makes
        let mut after = bare();
        after.set_process_list(list);
        after.filter(filter);
        after.toggle_sort(sort);
        if after.sort_asc != asc {
            after.toggle_sort(sort);
        }
        prop_assert_eq!((after.sort, after.sort_asc), (sort, asc));
        let got: Vec<_> = after.processes.iter().map(key).collect();
        let want: Vec<_> = before.processes.iter().map(key).collect();
        prop_assert_eq!(got, want);
        prop_assert_eq!(
            after.selected_process().is_some(),
            after.process_count() > 0
        );
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
fn a_nan_cpu_does_not_panic_the_sort() {
    // #986
    let mut monitor = bare().sort_by(ProcessSort::Cpu);
    let list = (0..40)
        .map(|i| {
            let cpu = if i % 3 == 0 {
                f32::NAN
            } else {
                (i % 11) as f32
            };
            process(i, "p", cpu, 0, 0.0, "Run", "", "0")
        })
        .collect();
    monitor.set_process_list(list);
    assert_eq!(monitor.process_count(), 40);
}

#[test]
fn page_down_by_a_huge_page_stops_at_the_last_process() {
    let mut monitor = monitor_with(numbered(3));
    monitor.select_next();
    monitor.page_down(usize::MAX);
    assert_eq!(monitor.selected_process().map(|p| p.pid), Some(102));
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

/// Every refresh re-sorted the list and kept the selection's position, so a
/// different process slid under it (#975).
#[test]
fn the_selection_stays_on_its_process_when_a_refresh_reorders_the_list() {
    let mut monitor = bare(); // sorted by CPU, descending
    monitor.set_process_list(vec![
        process(1, "build", 90.0, 0, 0.0, "Run", "", "0"),
        process(2, "editor", 50.0, 0, 0.0, "Run", "", "0"),
        process(3, "shell", 10.0, 0, 0.0, "Run", "", "0"),
    ]);
    monitor.select_next();
    assert_eq!(monitor.selected_process().unwrap().name, "editor");
    // A second later the shell is busiest and the editor last: neither the
    // first row nor the editor's old row holds it
    monitor.set_process_list(vec![
        process(1, "build", 80.0, 0, 0.0, "Run", "", "0"),
        process(2, "editor", 70.0, 0, 0.0, "Run", "", "0"),
        process(3, "shell", 95.0, 0, 0.0, "Run", "", "0"),
    ]);
    assert_eq!(monitor.selected_process().unwrap().name, "editor");
    // When it exits, the process now in its place is selected
    monitor.set_process_list(vec![process(1, "build", 5.0, 0, 0.0, "Run", "", "0")]);
    assert_eq!(monitor.selected_process().unwrap().name, "build");
}

/// `filter`, `clear_filter` and `toggle_sort` changed only the settings; the
/// rows waited for the next refresh, and `clear_filter` could not bring the
/// hidden processes back before it (#1084)
#[test]
fn filter_and_clear_filter_apply_to_the_rows_at_once() {
    let mut monitor = monitor_with(vec![
        process(1, "init", 0.0, 0, 0.0, "Run", "", "0"),
        process(2, "sh", 0.0, 0, 0.0, "Run", "", "0"),
        process(3, "bash", 0.0, 0, 0.0, "Run", "", "0"),
        process(4, "vim", 0.0, 0, 0.0, "Run", "", "0"),
    ]);
    monitor.page_down(2); // bash
    monitor.filter("SH");
    assert_eq!(pids_on_screen(&draw(&monitor, 60, 8)), [2, 3]);
    assert_eq!(monitor.process_count(), 2);
    assert_eq!(monitor.selected_process().unwrap().name, "bash");

    monitor.filter("zz-no-such-process-zz");
    assert_eq!(monitor.process_count(), 0);
    assert!(monitor.selected_process().is_none());
    assert!(pids_on_screen(&draw(&monitor, 60, 8)).is_empty());

    monitor.clear_filter();
    let buffer = draw(&monitor, 60, 8);
    assert_eq!(pids_on_screen(&buffer), [1, 2, 3, 4]);
    // With nothing selected, the selection starts at the top
    assert_eq!(monitor.selected_process().unwrap().pid, 1);
    assert_eq!(highlighted_rows(&monitor, &buffer), [2]);

    // A selection that stays listed stays on its process
    monitor.page_down(3); // vim
    monitor.filter("i"); // init, vim
    assert_eq!(monitor.selected_process().unwrap().name, "vim");
    monitor.clear_filter();
    assert_eq!(monitor.selected_process().unwrap().name, "vim");
    let buffer = draw(&monitor, 60, 8);
    assert_eq!(pids_on_screen(&buffer), [1, 2, 3, 4]);
    assert_eq!(highlighted_rows(&monitor, &buffer), [5]);
}

#[test]
fn toggle_sort_reorders_the_rows_with_the_header() {
    let mut monitor = monitor_with(numbered(3)); // PID, ascending
    monitor.toggle_sort(ProcessSort::Pid);
    let buffer = draw(&monitor, 60, 6);
    assert!(super::row(&buffer, 1).starts_with("PID▼"));
    assert_eq!(pids_on_screen(&buffer), [102, 101, 100]);
    // The selection stays on its process, now the last row
    assert_eq!(monitor.selected_process().unwrap().pid, 100);
    assert_eq!(highlighted_rows(&monitor, &buffer), [4]);

    monitor.toggle_sort(ProcessSort::Pid);
    assert_eq!(pids_on_screen(&draw(&monitor, 60, 6)), [100, 101, 102]);
}

/// The builders apply to a list already held, as the calls do
#[test]
fn the_sort_and_view_builders_apply_to_the_list_held() {
    let list = vec![
        process(1, "init", 0.0, 0, 0.0, "Run", "", "0"),
        process(2, "mine", 0.0, 0, 0.0, "Run", "", "501"),
    ];
    let monitor = monitor_with(list.clone()).ascending(false);
    assert_eq!(pids_on_screen(&draw(&monitor, 60, 6)), [2, 1]);
    let monitor = monitor_with(list.clone())
        .sort_by(ProcessSort::Name)
        .ascending(false);
    assert_eq!(pids_on_screen(&draw(&monitor, 60, 6)), [2, 1]);

    let mut monitor = monitor_with(list);
    monitor.current_user = Some("501".to_string());
    let monitor = monitor.view(ProcessView::User);
    assert_eq!(pids_on_screen(&draw(&monitor, 60, 6)), [2]);
}
