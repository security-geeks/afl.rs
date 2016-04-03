// Copyright 2015 Keegan McAllister.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// See `LICENSE` in this repository.

#![feature(core_intrinsics)]
#![feature(recover, std_panic)]

use std::intrinsics::abort;
use std::io::{self, Read};
use std::panic::{self, RecoverSafe};

#[cfg(not(test))]
#[link(name="afl-llvm-rt", kind="static")]
extern "C" {
    fn __afl_manual_init();
}

#[cfg(not(test))]
/// Initialize the afl runtime.
///
/// Only needed when the env var `AFL_DEFER_FORKSRV` is set.
///
/// It's undefined behavior to call this function from multiple
/// threads. You almost certainly need to call it before any
/// additional threads are created.
///
/// However, calling this more than once in a single-threaded
/// program, or calling it when not running with
/// `AFL_DEFER_FORKSRV` is safe and a no-op.
pub unsafe fn init() {
    __afl_manual_init();
}

pub fn fuzz_run_str<F>(some_closure: F)
    where F: FnOnce(&str) + RecoverSafe
{
    unsafe {
        __afl_manual_init();
    }

    let mut input = String::new();
    let result = io::stdin().read_to_string(&mut input);
    if result.is_err() {
        return;
    }

    let result = panic::recover(|| {
        some_closure(&input);
    });
    if result.is_err() {
        // TODO: add option to prevent this abort?
        unsafe {
            abort();
        }
    }
}
