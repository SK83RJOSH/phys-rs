#![no_std]
#![no_main]
#![feature(used_with_arg)]

extern crate alloc;

include!("../../tests/mod.rs");

psp::module!("psp_tests", 1, 1);

fn psp_main() {
    psp_test::run_tests();
    unsafe {
        psp::sys::sceKernelExitGame();
    }
}
