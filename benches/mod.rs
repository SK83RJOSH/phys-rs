use criterion::{criterion_group, criterion_main};

mod math;
use math::*;

criterion_group!(
    benches,
    vec3_dot,
    vec3_dot_into_vec3,
    vec3_dot_into_vec4,
    vec3_cross,
    vec3_min,
    vec3_max,
    vec3_clamp,
    vec3_min_element,
    vec3_max_element,
);
criterion_main!(benches);
