use std::env;
use std::io::{self, Write};
use std::process::Command;

extern crate afl_sys;


fn main() {
    env::set_var("AFL_DEFER_FORKSRV", "1");
    let _ = afl_sys::fuzz::afl_fuzz_env();
}
