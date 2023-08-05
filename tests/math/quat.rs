use phys_rs::math::{quat, Quat};

#[cfg_attr(not(target_family = "wasm"), test)]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
fn test_new() {
    let v0 = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
    assert_eq!((1.0, 2.0, 3.0, 4.0), v0.into());
    let v1 = Quat::from((1.0, 2.0, 3.0, 4.0));
    assert_eq!((1.0, 2.0, 3.0, 4.0), v1.into());
    let v2 = Quat::from([1.0, 2.0, 3.0, 4.0]);
    assert_eq!((1.0, 2.0, 3.0, 4.0), v2.into());
}

#[cfg_attr(not(target_family = "wasm"), test)]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
fn test_const() {
    const V1: Quat = quat(1.0, 2.0, 3.0, 4.0);
    assert_eq!((1.0, 2.0, 3.0, 4.0), V1.into());
}

#[cfg_attr(not(target_family = "wasm"), test)]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
fn test_consts() {
    assert_eq!([0.0; 4], *Quat::ZERO.as_ref());
    assert_eq!((0.0, 0.0, 0.0, 1.0), Quat::IDENTITY.into());
}

#[allow(clippy::uninlined_format_args)]
#[cfg_attr(not(target_family = "wasm"), test)]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
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

#[cfg_attr(not(target_family = "wasm"), test)]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
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

#[should_panic]
#[cfg_attr(not(target_family = "wasm"), test)]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
fn test_invalid_accessors() {
    assert_eq!(1.0, Quat::ZERO[4]);
}
