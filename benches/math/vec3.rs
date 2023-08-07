use criterion::{black_box, Criterion};
use phys_rs::math::vec3;

pub fn vec3_dot(c: &mut Criterion) {
    let x = vec3(black_box(1.0), black_box(2.0), black_box(3.0));
    let y = vec3(black_box(3.0), black_box(2.0), black_box(1.0));
    c.bench_function("vec3_dot", |b| b.iter(|| black_box(x).dot(y)));
}

pub fn vec3_dot_into_vec3(c: &mut Criterion) {
    let x = vec3(black_box(1.0), black_box(2.0), black_box(3.0));
    let y = vec3(black_box(3.0), black_box(2.0), black_box(1.0));
    c.bench_function("vec3_dot_into_vec3", |b| {
        b.iter(|| black_box(x).dot_into_vec3(y))
    });
}

pub fn vec3_dot_into_vec4(c: &mut Criterion) {
    let x = vec3(black_box(1.0), black_box(2.0), black_box(3.0));
    let y = vec3(black_box(3.0), black_box(2.0), black_box(1.0));
    c.bench_function("vec3_dot_into_vec4", |b| {
        b.iter(|| black_box(x).dot_into_vec4(y))
    });
}

pub fn vec3_cross(c: &mut Criterion) {
    let x = vec3(black_box(1.0), black_box(2.0), black_box(3.0));
    let y = vec3(black_box(3.0), black_box(2.0), black_box(1.0));
    c.bench_function("vec3_cross", |b| b.iter(|| black_box(x).cross(y)));
}

pub fn vec3_min(c: &mut Criterion) {
    let x = vec3(black_box(1.0), black_box(2.0), black_box(3.0));
    let y = vec3(black_box(3.0), black_box(2.0), black_box(1.0));
    c.bench_function("vec3_min", |b| b.iter(|| black_box(x).min(y)));
}

pub fn vec3_max(c: &mut Criterion) {
    let x = vec3(black_box(1.0), black_box(2.0), black_box(3.0));
    let y = vec3(black_box(3.0), black_box(2.0), black_box(1.0));
    c.bench_function("vec3_max", |b| b.iter(|| black_box(x).max(y)));
}

pub fn vec3_clamp(c: &mut Criterion) {
    let x = vec3(black_box(1.0), black_box(2.0), black_box(3.0));
    let y = vec3(black_box(0.0), black_box(1.0), black_box(2.0));
    let z = vec3(black_box(2.0), black_box(3.0), black_box(4.0));
    c.bench_function("vec3_clamp", |b| b.iter(|| black_box(x).clamp(y, z)));
}

pub fn vec3_min_element(c: &mut Criterion) {
    let x = vec3(black_box(1.0), black_box(2.0), black_box(3.0));
    c.bench_function("vec3_min_element", |b| {
        b.iter(|| black_box(x).min_element())
    });
}

pub fn vec3_max_element(c: &mut Criterion) {
    let x = vec3(black_box(1.0), black_box(2.0), black_box(3.0));
    c.bench_function("vec3_max_element", |b| {
        b.iter(|| black_box(x).max_element())
    });
}
