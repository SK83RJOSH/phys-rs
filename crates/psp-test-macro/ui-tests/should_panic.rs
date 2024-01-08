#![no_implicit_prelude]

extern crate psp_test_macro;

use psp_test_macro::test;

#[test]
#[should_panic]
fn success_1() {}

#[test]
#[should_panic = "test"]
fn success_2() {}

#[test]
#[should_panic(expected = "test")]
fn success_3() {}

#[test]
#[should_panic]
async fn async_success_1() {}

#[test]
#[should_panic = "test"]
async fn async_success_2() {}

#[test]
#[should_panic(expected = "test")]
async fn async_success_3() {}

#[test]
#[should_panic::error]
fn fail_1() {}

#[test]
#[should_panic = 42]
fn fail_2() {}

#[test]
#[should_panic[]]
fn fail_3() {}

#[test]
#[should_panic(42)]
fn fail_4() {}

#[test]
#[should_panic(test)]
fn fail_5() {}

#[test]
#[should_panic(expected)]
fn fail_6() {}

#[test]
#[should_panic(expected::error)]
fn fail_7() {}

#[test]
#[should_panic(expected =)]
fn fail_8() {}

#[test]
#[should_panic(expected = 5)]
fn fail_9() {}

#[test]
#[should_panic = "test"]
#[should_panic = "test"]
fn fail_10() {}

fn main() {}
