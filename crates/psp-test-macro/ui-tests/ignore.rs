#![no_implicit_prelude]

extern crate psp_test_macro;

use psp_test_macro::test;

#[test]
#[ignore]
fn success_1() {}

#[test]
#[ignore = "test"]
fn success_2() {}

#[test]
#[ignore]
async fn async_success_1() {}

#[test]
#[ignore = "test"]
async fn async_success_2() {}

#[test]
#[ignore::error]
fn fail_1() {}

#[test]
#[ignore = 42]
fn fail_2() {}

#[test]
#[ignore[]]
fn fail_3() {}

#[test]
#[ignore(42)]
fn fail_4() {}

#[test]
#[ignore(test)]
fn fail_5() {}

#[test]
#[ignore("test")]
fn fail_6() {}

#[test]
#[ignore = "test"]
#[ignore = "test"]
fn fail_7() {}

fn main() {}
