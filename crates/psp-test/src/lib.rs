#![no_std]
#![feature(used_with_arg)]

extern crate alloc;

use alloc::format;
use core::{ffi::c_void, option::Option};
use psp::sys;

pub use linkme;
pub use psp_test_macro::test;

pub const OUTPUT_FILENAME: &str = "psp_output_file.log";

#[linkme::distributed_slice]
pub static TESTS: [fn(&mut PspTestContext)];

pub struct PspTestContext {
    fd: sys::SceUid,
}

fn psp_filename(filename: &str) -> *const u8 {
    format!("host0:/{}\0", filename).as_bytes().as_ptr()
}

fn get_test_output_file() -> sys::SceUid {
    unsafe {
        let fd = sys::sceIoOpen(
            psp_filename(OUTPUT_FILENAME),
            sys::IoOpenFlags::TRUNC | sys::IoOpenFlags::CREAT | sys::IoOpenFlags::RD_WR,
            0o777,
        );
        if fd.0 < 0 {
            panic!("Unable to open file \"{}\" for output!", OUTPUT_FILENAME);
        }
        fd
    }
}

fn close_psp_file(fd: sys::SceUid) {
    unsafe {
        sys::sceIoClose(fd);
    }
}

impl PspTestContext {
    pub fn new() -> Self {
        let fd = get_test_output_file();
        Self { fd }
    }

    fn log(&self, msg: &str) {
        unsafe {
            sys::sceIoWrite(
                self.fd,
                msg.as_bytes().as_ptr() as *const u8 as *const c_void,
                msg.len(),
            );
        }
    }

    pub fn execute(
        &mut self,
        name: &str,
        function: fn(),
        should_panic: Option<Option<&'static str>>,
        ignore: Option<Option<&'static str>>,
    ) {
        self.log(&format!("test {name} ... "));
        if !should_panic.is_some() && !ignore.is_some() {
            (function)();
            self.log("ok\n")
        } else {
            self.log("ignored\n")
        }
    }
}

struct PspTestRunner {
    context: PspTestContext,
}

impl PspTestRunner {
    pub fn new() -> Self {
        Self {
            context: PspTestContext::new(),
        }
    }

    pub fn run(&mut self) {
        let count = TESTS.len();
        let noun = if count == 1 { "test" } else { "tests" };
        let message = format!("running {count} {noun}\n");
        self.context.log(&message);
        for test in TESTS {
            (test)(&mut self.context);
        }
        close_psp_file(self.context.fd);
    }
}

pub fn run_tests() {
    PspTestRunner::new().run();
}
