#[cfg(target_os = "psp")]
use alloc::format;
use core::assert_eq;
use core::convert::{AsRef, From, Into};

#[cfg(target_os = "psp")]
pub(crate) use psp_test::test;
#[cfg(target_family = "wasm")]
pub(crate) use wasm_bindgen_test::wasm_bindgen_test as test;

use phys_rs::math::{quat, Quat};

#[test]
fn test_new() {
    let v0 = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
    assert_eq!((1.0, 2.0, 3.0, 4.0), v0.into());
    let v1 = Quat::from((1.0, 2.0, 3.0, 4.0));
    assert_eq!((1.0, 2.0, 3.0, 4.0), v1.into());
    let v2 = Quat::from([1.0, 2.0, 3.0, 4.0]);
    assert_eq!((1.0, 2.0, 3.0, 4.0), v2.into());
}

#[test]
fn test_const() {
    const V1: Quat = quat(1.0, 2.0, 3.0, 4.0);
    assert_eq!((1.0, 2.0, 3.0, 4.0), V1.into());
}

#[test]
fn test_consts() {
    assert_eq!([0.0; 4], *Quat::ZERO.as_ref());
    assert_eq!((0.0, 0.0, 0.0, 1.0), Quat::IDENTITY.into());
}

#[test]
#[allow(clippy::uninlined_format_args)]
fn test_fmt() {
    const V0: Quat = quat(1.0, 2.0, 3.0, 4.0);
    assert_eq!(
        format!("{:?}", V0),
        format!(
            "{}({:?}, {:?}, {:?}, {:?})",
            stringify!(Quat),
            V0.x,
            V0.y,
            V0.z,
            V0.w
        )
    );
    assert_eq!(
        format!("{:#?}", V0),
        format!(
            "{}(\n    {:#?},\n    {:#?},\n    {:#?},\n    {:#?},\n)",
            stringify!(Quat),
            V0.x,
            V0.y,
            V0.z,
            V0.w
        )
    );
    assert_eq!(format!("{}", V0), "[1, 2, 3, 4]");
}

#[test]
fn test_accessors() {
    let mut a = Quat::ZERO;
    a.x = 1.0;
    a.y = 2.0;
    a.z = 3.0;
    a.w = 4.0;
    assert_eq!(1.0, a.x);
    assert_eq!(2.0, a.y);
    assert_eq!(3.0, a.z);
    assert_eq!(4.0, a.w);
    assert_eq!((1.0, 2.0, 3.0, 4.0), a.into());

    let mut a = Quat::ZERO;
    a[0] = 1.0;
    a[1] = 2.0;
    a[2] = 3.0;
    a[3] = 4.0;
    assert_eq!(1.0, a[0]);
    assert_eq!(2.0, a[1]);
    assert_eq!(3.0, a[2]);
    assert_eq!(4.0, a[3]);
    assert_eq!((1.0, 2.0, 3.0, 4.0), a.into());
}

#[test]
#[should_panic]
fn test_invalid_accessors() {
    assert_eq!(1.0, Quat::ZERO[4]);
}
