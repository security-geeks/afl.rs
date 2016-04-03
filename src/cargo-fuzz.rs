use std::env;
use std::io::{self, Write};
use std::process::Command;


fn main() {
    env::set_var("AFL_DEFER_FORKSRV", "1");
    let args = env::args().skip(1).collect::<Vec<_>>();
    let status = Command::new("afl-fuzz").args(&args).status();
    if let Err(e) = status {
        writeln!(io::stderr(), "Could not run 'afl-fuzz': {}", e).unwrap();
    }
}
