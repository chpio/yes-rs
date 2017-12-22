use std::io::{self, Write};
use std::env::args;
use std::iter::once;

fn main() {
    let s: String = if let Some(arg) = args().skip(1).next() {
        once(arg.as_str())
            .chain(once("\n"))
            .cycle()
            .take(8192)
            .collect()
    } else {
        once("y\n").cycle().take(8192).collect()
    };

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    loop {
        if stdout.write_all(s.as_bytes()).is_err() {
            break;
        }
    }
}
