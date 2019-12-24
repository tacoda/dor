extern crate docopt;
extern crate serde;

use docopt::Docopt;
use serde::Deserialize;

// Write the Docopt usage string.
const USAGE: &'static str = "
Usage: cp [-a] <source> <dest>
       cp [-a] <source>... <dir>

Options:
    -a, --archive  Copy everything.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_source: Vec<String>,
    arg_dest: String,
    arg_dir: String,
    flag_archive: bool,
}

fn s(x: &str) -> String { x.to_string() }

fn main() {
    let argv = || vec!["cp", "-a", "file1", "file2", "dest/"];
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(argv().into_iter()).deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);

    assert!(args.flag_archive);
    assert_eq!(args.arg_source, vec![s("file1"), s("file2")]);
    assert_eq!(args.arg_dir, s("dest/"));
    assert_eq!(args.arg_dest, s(""));
}
