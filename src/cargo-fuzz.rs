use std::env;
use std::process;


fn main() {
    env::set_var("AFL_DEFER_FORKSRV", "1");
    let args = env::args().skip(1).collect::<Vec<_>>();
    let output = process::Command::new("afl-fuzz")
                          .args(&args)
                          .stdin(process::Stdio::inherit())
                          .stderr(process::Stdio::inherit())
                          .stdout(process::Stdio::inherit())
                          .output()
                          .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
}
