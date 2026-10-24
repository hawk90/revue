//! Write each target's seed corpus and libFuzzer dictionary:
//! `seed <corpus dir> <dict dir>` makes `<corpus dir>/<target>/` with every
//! fault-layer corpus entry up to 64 KiB and each of the target's tokens,
//! and `<dict dir>/<target>.dict` with the tokens.

use std::fs;
use std::path::PathBuf;

/// Larger entries would make every libFuzzer run slow (`-max_len` is set
/// to this in the workflow too)
const MAX_SEED: usize = 64 * 1024;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let corpus_dir = PathBuf::from(args.next().unwrap_or_else(|| "corpus".into()));
    let dict_dir = PathBuf::from(args.next().unwrap_or_else(|| "dicts".into()));
    fs::create_dir_all(&dict_dir)?;
    let corpus = revue_fuzz::corpus();
    for (name, _, tokens) in revue_fuzz::targets() {
        let dir = corpus_dir.join(name);
        fs::create_dir_all(&dir)?;
        for (i, (_, input)) in corpus.iter().enumerate() {
            if input.len() <= MAX_SEED {
                fs::write(dir.join(format!("corpus-{i:03}")), input)?;
            }
        }
        let mut dict = String::new();
        for (i, token) in tokens.iter().enumerate() {
            fs::write(dir.join(format!("token-{i:03}")), token)?;
            dict.push_str(&format!("\"{}\"\n", escape(token)));
        }
        fs::write(dict_dir.join(format!("{name}.dict")), dict)?;
    }
    Ok(())
}

/// A token as a libFuzzer dictionary string: `\` and `"` escaped, bytes
/// outside printable ASCII as `\xNN`
fn escape(token: &str) -> String {
    let mut out = String::new();
    for b in token.bytes() {
        match b {
            b'\\' => out.push_str("\\\\"),
            b'"' => out.push_str("\\\""),
            0x20..=0x7e => out.push(b as char),
            _ => out.push_str(&format!("\\x{b:02X}")),
        }
    }
    out
}
