pub(crate) mod deref;

#[cfg(all(no_simd, no_std, libm))]
pub(crate) mod libm;

pub(crate) mod float2;
pub use float2::{float2, Float2};

pub(crate) mod float3;
pub use float3::{float3, Float3};

pub(crate) mod float4;
pub use float4::{float4, Float4};

pub(crate) mod quat;
pub use quat::{quat, Quat};

#[cfg(any(not(no_simd), not(any(arm_neon, x86_sse, wasm_simd128))))]
pub(crate) mod simd;

pub(crate) mod vec3;
pub use vec3::{vec3, Vec3};

pub(crate) mod vec4;
pub use vec4::{vec4, Vec4};
