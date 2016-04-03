use std::env;
use std::process::Command;


fn main() {
    env::set_var("AFL_DEFER_FORKSRV", "1");
    let args = env::args().skip(1).collect::<Vec<_>>();
    let output = Command::new("afl-fuzz")
                          .args(&args)
                          .status()
                          .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
}
