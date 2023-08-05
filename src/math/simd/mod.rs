#![allow(unused_imports)]

#[cfg(all(target_arch = "aarch64", arm_neon))]
pub(crate) use core::arch::aarch64::*;
#[cfg(all(target_arch = "arm", arm_neon))]
pub(crate) use core::arch::arm::*;
#[cfg(all(target_arch = "wasm32", wasm_simd128))]
pub(crate) use core::arch::wasm32::*;
#[cfg(all(target_arch = "wasm64", wasm_simd128))]
pub(crate) use core::arch::wasm64::*;
#[cfg(all(target_arch = "x86", x86_sse))]
pub(crate) use core::arch::x86::*;
#[cfg(all(target_arch = "x86_64", x86_sse))]
pub(crate) use core::arch::x86_64::*;

#[cfg(arm_neon)]
pub(crate) use neon::*;
#[cfg(x86_sse)]
pub(crate) use sse::*;
#[cfg(wasm_simd128)]
pub(crate) use wasm::*;

#[cfg(arm_neon)]
pub(crate) mod neon;
#[cfg(x86_sse)]
pub(crate) mod sse;
#[cfg(wasm_simd128)]
pub(crate) mod wasm;

#[cfg(arm_neon)]
pub type VectorType = float32x4_t;
#[cfg(x86_sse)]
pub type VectorType = __m128;
#[cfg(wasm_simd128)]
pub type VectorType = v128;
#[cfg(not(any(arm_neon, x86_sse, wasm_simd128)))]
pub type VectorType = [f32; 4];

#[repr(C)]
pub(crate) union VectorUnionCast<T: Copy> {
    pub a: [f32; 4],
    pub v: T,
}
