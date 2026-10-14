//! highlight_code: a file's spans spread over its lines

use proptest::prelude::*;
use revue::widget::syntax::{HighlightSpan, Language};
use revue::widget::TreeSitterHighlighter;

use super::{d, described, numbered};

fn code_spans(language: Language, code: &str) -> Vec<Vec<HighlightSpan>> {
    TreeSitterHighlighter::with_theme(language, numbered()).highlight_code(code)
}

#[test]
fn empty_code_has_no_lines() {
    assert!(code_spans(Language::Rust, "").is_empty());
    assert!(code_spans(Language::None, "").is_empty());
}

#[test]
fn one_list_per_line_as_str_lines_counts_them() {
    // A final newline does not start a line; an empty line in between does
    for (code, n) in [("x", 1), ("x\n", 1), ("x\n\n", 2), ("\n", 1), ("a\n\nb", 3)] {
        assert_eq!(code_spans(Language::Rust, code).len(), n, "{code:?}");
        assert_eq!(code_spans(Language::None, code).len(), n, "{code:?}");
    }
}

#[test]
fn a_comment_across_lines_is_cut_at_each_line_end() {
    let code = "/* a\nbb\nc */ x";
    let lines: Vec<&str> = code.lines().collect();
    let spans = code_spans(Language::JavaScript, code);
    assert_eq!(described(lines[0], &spans[0]), [d("/* a", 6, "I")]);
    assert_eq!(described(lines[1], &spans[1]), [d("bb", 6, "I")]);
    // The last line: from its start to the comment's end, then `x`
    assert_eq!(described(lines[2], &spans[2])[0], d("c */", 6, "I"));
}

#[test]
fn an_empty_line_inside_a_comment_has_no_span() {
    let code = "/* a\n\nb */";
    let spans = code_spans(Language::JavaScript, code);
    assert_eq!(spans.len(), 3);
    assert!(spans[1].is_empty());
    assert_eq!((spans[2][0].start, spans[2][0].end), (0, 4));
}

/// The plain model: the whole file's spans (as `highlight_line` gives them
/// for the file in one piece), each cut to every line it touches - the
/// newline itself belongs to no line - with offsets from the line's start
fn split_model(code: &str, whole: &[HighlightSpan]) -> Vec<Vec<(usize, usize, u8, bool, bool)>> {
    let mut starts = vec![0];
    starts.extend(code.match_indices('\n').map(|(i, _)| i + 1));
    let n = code.lines().count();
    (0..n)
        .map(|i| {
            let start = starts[i];
            let end = starts.get(i + 1).map_or(code.len(), |s| s - 1);
            let mut out: Vec<_> = whole
                .iter()
                .filter_map(|s| {
                    let (a, b) = (s.start.max(start), s.end.min(end));
                    (a < b).then(|| (a - start, b - start, s.fg.r, s.bold, s.italic))
                })
                .collect();
            out.sort_by_key(|t| t.0);
            out
        })
        .collect()
}

fn token() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "fn ",
        "let ",
        "x",
        " = ",
        "1",
        "42",
        "\"s\"",
        "\"a\\nb\"",
        "'c'",
        "// c",
        "# c",
        "/* a\nb */",
        "/* \n\n */",
        "\"\"\"doc\nstring\"\"\"",
        "(",
        ")",
        "{",
        "}",
        ";",
        ", ",
        "\n",
        "\n",
        "\n\n",
        "    ",
        "def ",
        "const ",
        "true",
        "None",
        "x.y",
        "[1, 2]",
    ])
}

proptest! {
    /// Any code spreads its spans over its lines exactly as the model cuts
    /// the whole file's spans; every span is non-empty, inside its line,
    /// and the spans of a line are sorted and do not overlap. (ASCII only,
    /// LF only: offsets are bytes, and a CR is counted into the line, both
    /// filed from this audit.)
    #[test]
    fn code_spans_are_the_file_spans_cut_at_line_ends(
        language in prop::sample::select(vec![
            Language::Rust, Language::Python, Language::JavaScript, Language::Go,
            Language::Json, Language::Shell, Language::Css,
        ]),
        tokens in prop::collection::vec(token(), 0..30),
    ) {
        let code: String = tokens.concat();
        let mut hl = TreeSitterHighlighter::with_theme(language, numbered());
        let whole = hl.highlight_line(&code);
        let lines = hl.highlight_code(&code);
        let got: Vec<Vec<_>> = lines
            .iter()
            .map(|l| l.iter().map(|s| (s.start, s.end, s.fg.r, s.bold, s.italic)).collect())
            .collect();
        prop_assert_eq!(&got, &split_model(&code, &whole));
        for (spans, line) in lines.iter().zip(code.lines()) {
            let mut end = 0;
            for s in spans {
                prop_assert!(s.start < s.end && s.end <= line.len(), "{:?} in {:?}", s, line);
                prop_assert!(s.start >= end, "overlap at {:?} in {:?}", s, line);
                end = s.end;
            }
        }
    }
}
