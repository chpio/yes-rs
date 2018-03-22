use std::io::{self, Write};
use std::env::args;
use std::iter::once;

const BUF_LEN: usize = 8 * 1024;

fn main() {
    let s: String = if let Some(arg) = args().skip(1).next() {
        once(arg.as_str())
            .chain(once("\n"))
            .cycle()
            .take(BUF_LEN / (arg.len() + 1))
            .collect()
    } else {
        once("y\n").cycle().take(BUF_LEN / 2).collect()
    };

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    while stdout.write_all(s.as_bytes()).is_ok() {}
}
