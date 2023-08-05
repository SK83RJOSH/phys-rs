use phys_rs::math::{vec3, Vec3};

#[test]
fn test_new() {
    let v0 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!((1.0, 2.0, 3.0), v0.into());
    let v1 = Vec3::from((1.0, 2.0, 3.0));
    assert_eq!((1.0, 2.0, 3.0), v1.into());
    let v2 = Vec3::from([1.0, 2.0, 3.0]);
    assert_eq!((1.0, 2.0, 3.0), v2.into());
}

#[test]
fn test_splat() {
    const V0: Vec3 = Vec3::splat(1.0);
    assert_eq!([1.0; 3], *V0.as_ref());
    const V1: Vec3 = Vec3::splat(0.5);
    assert_eq!([0.5; 3], *V1.as_ref());
}

#[test]
fn test_const() {
    const V0: Vec3 = Vec3::splat(1.0);
    assert_eq!([1.0; 3], *V0.as_ref());
    const V1: Vec3 = vec3(1.0, 2.0, 3.0);
    assert_eq!((1.0, 2.0, 3.0), V1.into());
}

#[test]
fn test_consts() {
    assert_eq!([0.0; 3], *Vec3::ZERO.as_ref());
    assert_eq!([1.0; 3], *Vec3::ONE.as_ref());
    assert_eq!([-1.0; 3], *Vec3::NEG_ONE.as_ref());
    assert_eq!([f32::INFINITY; 3], *Vec3::INFINITY.as_ref());
    assert_eq!([f32::NEG_INFINITY; 3], *Vec3::NEG_INFINITY.as_ref());
    assert_eq!((1.0, 0.0, 0.0), Vec3::X.into());
    assert_eq!((-1.0, 0.0, 0.0), Vec3::NEG_X.into());
    assert_eq!((0.0, 1.0, 0.0), Vec3::Y.into());
    assert_eq!((0.0, -1.0, 0.0), Vec3::NEG_Y.into());
    assert_eq!((0.0, 0.0, 1.0), Vec3::Z.into());
    assert_eq!((0.0, 0.0, -1.0), Vec3::NEG_Z.into());
    assert_eq!([f32::MIN; 3], *Vec3::MIN.as_ref());
    assert_eq!([f32::MAX; 3], *Vec3::MAX.as_ref());
}

#[test]
#[allow(clippy::uninlined_format_args)]
fn test_fmt() {
    const V0: Vec3 = vec3(1.0, 2.0, 3.0);
    assert_eq!(
        format!("{:?}", V0),
        format!("{}({:?}, {:?}, {:?})", stringify!(Vec3), V0.x, V0.y, V0.z)
    );
    assert_eq!(
        format!("{:#?}", V0),
        format!(
            "{}(\n    {:#?},\n    {:#?},\n    {:#?},\n)",
            stringify!(Vec3),
            V0.x,
            V0.y,
            V0.z
        )
    );
    assert_eq!(format!("{}", V0), "[1, 2, 3]");
}

#[test]
fn test_accessors() {
    let mut a = Vec3::ZERO;
    a.x = 1.0;
    a.y = 2.0;
    a.z = 3.0;
    assert_eq!(1.0, a.x);
    assert_eq!(2.0, a.y);
    assert_eq!(3.0, a.z);
    assert_eq!((1.0, 2.0, 3.0), a.into());

    let mut a = Vec3::ZERO;
    a[0] = 1.0;
    a[1] = 2.0;
    a[2] = 3.0;
    assert_eq!(1.0, a[0]);
    assert_eq!(2.0, a[1]);
    assert_eq!(3.0, a[2]);
    assert_eq!((1.0, 2.0, 3.0), a.into());
}

#[test]
#[should_panic]
fn test_invalid_accessors() {
    assert_eq!(0.0, Vec3::ZERO[3]);
}

#[test]
fn test_dot() {
    let x = vec3(1.0, 0.0, 0.0);
    let y = vec3(0.0, 1.0, 0.0);
    assert_eq!(1.0, x.dot(x));
    assert_eq!(0.0, x.dot(y));
    assert_eq!(
        [14.0; 3],
        *vec3(0.0, 4.0, 6.0)
            .dot_into_vec3(vec3(3.0, 2.0, 1.0))
            .as_ref()
    );
    assert_eq!(
        [14.0; 4],
        *vec3(0.0, 4.0, 6.0)
            .dot_into_vec4(vec3(3.0, 2.0, 1.0))
            .as_ref()
    );
}

#[test]
fn test_cross() {
    const X: Vec3 = Vec3::X;
    const Y: Vec3 = Vec3::Y;
    const Z: Vec3 = Vec3::Z;
    assert_eq!(*Y.as_ref(), *Z.cross(X).as_ref());
    assert_eq!(*Z.as_ref(), *X.cross(Y).as_ref());
}

#[test]
fn test_min_max() {
    let a = vec3(3.0, 5.0, 1.0);
    let b = vec3(4.0, 2.0, 6.0);
    assert_eq!((3.0, 2.0, 1.0), a.min(b).into());
    assert_eq!((3.0, 2.0, 1.0), b.min(a).into());
    assert_eq!((4.0, 5.0, 6.0), a.max(b).into());
    assert_eq!((4.0, 5.0, 6.0), b.max(a).into());
}

#[test]
fn test_clamp() {
    let min = vec3(1.0, 3.0, 3.0);
    let max = vec3(6.0, 8.0, 8.0);
    assert_eq!((1.0, 3.0, 3.0), vec3(0.0, 0.0, 0.0).clamp(min, max).into());
    assert_eq!((2.0, 3.0, 3.0), vec3(2.0, 2.0, 2.0).clamp(min, max).into());
    assert_eq!((4.0, 5.0, 5.0), vec3(4.0, 5.0, 5.0).clamp(min, max).into());
    assert_eq!((6.0, 6.0, 6.0), vec3(6.0, 6.0, 6.0).clamp(min, max).into());
    assert_eq!((6.0, 7.0, 7.0), vec3(7.0, 7.0, 7.0).clamp(min, max).into());
    assert_eq!((6.0, 8.0, 8.0), vec3(9.0, 9.0, 9.0).clamp(min, max).into());
}
