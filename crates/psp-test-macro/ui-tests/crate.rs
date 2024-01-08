#![no_implicit_prelude]

extern crate psp_test_macro;

use psp_test_macro::test;

pub mod psp {
    pub extern crate test as test;
}

#[test(crate = ::test)]
fn success_1() {}

#[test(crate = crate::psp::test)]
fn success_2() {}

#[test(crate = foo)]
fn failure_1() {}

fn main() {}
