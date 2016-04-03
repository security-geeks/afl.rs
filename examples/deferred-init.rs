// Copyright 2015 Keegan McAllister.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// See `LICENSE` in this repository.

// Fuzz me with AFL_DEFER_FORKSRV=1 for a huge speedup.

#![feature(plugin)]

#![plugin(afl_plugin)]

extern crate afl;

use std::io::{self, Read};
use std::thread;

fn main() {
    println!("An eternity in...");
    thread::sleep_ms(500);

    afl::fuzz_run_str(|input: &str| {
        println!("the blink of an eye.");
        if input.starts_with("x") {
            println!("going...");
            if input.starts_with("xy") {
                println!("going...");
                if input.starts_with("xyz") {
                    println!("gone!");
                    unsafe {
                        let x: *mut usize = 0 as *mut usize;
                        *x = 0xBEEF;
                    }
                }
            }
        }
    });
}
