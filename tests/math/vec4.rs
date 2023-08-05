use phys_rs::math::{vec4, Vec4};

#[test]
fn test_new() {
    let v0 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!((1.0, 2.0, 3.0, 4.0), v0.into());
    let v1 = Vec4::from((1.0, 2.0, 3.0, 4.0));
    assert_eq!((1.0, 2.0, 3.0, 4.0), v1.into());
    let v2 = Vec4::from([1.0, 2.0, 3.0, 4.0]);
    assert_eq!((1.0, 2.0, 3.0, 4.0), v2.into());
}

#[test]
fn test_splat() {
    const V0: Vec4 = Vec4::splat(1.0);
    assert_eq!([1.0; 4], *V0.as_ref());
    const V1: Vec4 = Vec4::splat(0.5);
    assert_eq!([0.5; 4], *V1.as_ref());
}

#[test]
fn test_const() {
    const V0: Vec4 = Vec4::splat(1.0);
    assert_eq!([1.0; 4], *V0.as_ref());
    const V1: Vec4 = vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!((1.0, 2.0, 3.0, 4.0), V1.into());
}

#[test]
fn test_consts() {
    assert_eq!([0.0; 4], *Vec4::ZERO.as_ref());
    assert_eq!([1.0; 4], *Vec4::ONE.as_ref());
    assert_eq!([-1.0; 4], *Vec4::NEG_ONE.as_ref());
    assert_eq!([f32::INFINITY; 4], *Vec4::INFINITY.as_ref());
    assert_eq!([f32::NEG_INFINITY; 4], *Vec4::NEG_INFINITY.as_ref());
    assert_eq!((1.0, 0.0, 0.0, 0.0), Vec4::X.into());
    assert_eq!((-1.0, 0.0, 0.0, 0.0), Vec4::NEG_X.into());
    assert_eq!((0.0, 1.0, 0.0, 0.0), Vec4::Y.into());
    assert_eq!((0.0, -1.0, 0.0, 0.0), Vec4::NEG_Y.into());
    assert_eq!((0.0, 0.0, 1.0, 0.0), Vec4::Z.into());
    assert_eq!((0.0, 0.0, -1.0, 0.0), Vec4::NEG_Z.into());
    assert_eq!((0.0, 0.0, 0.0, 1.0), Vec4::W.into());
    assert_eq!((0.0, 0.0, 0.0, -1.0), Vec4::NEG_W.into());
    assert_eq!([f32::MIN; 4], *Vec4::MIN.as_ref());
    assert_eq!([f32::MAX; 4], *Vec4::MAX.as_ref());
}

#[test]
#[allow(clippy::uninlined_format_args)]
fn test_fmt() {
    const V0: Vec4 = vec4(1.0, 2.0, 3.0, 4.0);
    assert_eq!(
        format!("{:?}", V0),
        format!(
            "{}({:?}, {:?}, {:?}, {:?})",
            stringify!(Vec4),
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
            stringify!(Vec4),
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
    let mut a = Vec4::ZERO;
    a.x = 1.0;
    a.y = 2.0;
    a.z = 3.0;
    a.w = 4.0;
    assert_eq!(1.0, a.x);
    assert_eq!(2.0, a.y);
    assert_eq!(3.0, a.z);
    assert_eq!(4.0, a.w);
    assert_eq!((1.0, 2.0, 3.0, 4.0), a.into());

    let mut a = Vec4::ZERO;
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
    assert_eq!(0.0, Vec4::ZERO[4]);
}

#[test]
fn test_dot() {
    let x = vec4(1.0, 0.0, 0.0, 0.0);
    let y = vec4(0.0, 1.0, 0.0, 0.0);
    assert_eq!(1.0, x.dot(x));
    assert_eq!(0.0, x.dot(y));
    assert_eq!(
        [14.0; 4],
        *vec4(0.0, 4.0, 6.0, 8.0)
            .dot_into_vec4(vec4(3.0, 2.0, 1.0, 0.0))
            .as_ref()
    );
    assert_eq!(
        [14.0; 4],
        *vec4(0.0, 4.0, 6.0, 8.0)
            .dot_into_vec4(vec4(3.0, 2.0, 1.0, 0.0))
            .as_ref()
    );
}

#[test]
fn test_min_max() {
    let a = vec4(3.0, 5.0, 1.0, 8.0);
    let b = vec4(4.0, 2.0, 6.0, 3.0);
    assert_eq!((3.0, 2.0, 1.0, 3.0), a.min(b).into());
    assert_eq!((3.0, 2.0, 1.0, 3.0), b.min(a).into());
    assert_eq!((4.0, 5.0, 6.0, 8.0), a.max(b).into());
    assert_eq!((4.0, 5.0, 6.0, 8.0), b.max(a).into());
}

#[test]
fn test_clamp() {
    let min = vec4(1.0, 3.0, 3.0, 1.0);
    let max = vec4(6.0, 8.0, 8.0, 6.0);
    assert_eq!(
        (1.0, 3.0, 3.0, 1.0),
        vec4(0.0, 0.0, 0.0, 0.0).clamp(min, max).into()
    );
    assert_eq!(
        (2.0, 3.0, 3.0, 2.0),
        vec4(2.0, 2.0, 2.0, 2.0).clamp(min, max).into()
    );
    assert_eq!(
        (4.0, 5.0, 5.0, 4.0),
        vec4(4.0, 5.0, 5.0, 4.0).clamp(min, max).into()
    );
    assert_eq!(
        (6.0, 6.0, 6.0, 6.0),
        vec4(6.0, 6.0, 6.0, 6.0).clamp(min, max).into()
    );
    assert_eq!(
        (6.0, 7.0, 7.0, 6.0),
        vec4(7.0, 7.0, 7.0, 7.0).clamp(min, max).into()
    );
    assert_eq!(
        (6.0, 8.0, 8.0, 6.0),
        vec4(9.0, 9.0, 9.0, 9.0).clamp(min, max).into()
    );
}
