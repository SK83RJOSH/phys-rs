#![allow(unused_imports, dead_code)]

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
pub(crate) union VectorUnionCast<C: Copy, T: Copy> {
    pub a: [C; 4],
    pub v: T,
}

#[cfg(arm_neon)]
pub type U8x16 = uint8x16_t;
#[cfg(not(arm_neon))]
pub type U8x16 = VectorType;

#[repr(C)]
pub(crate) union UnionCast_U8x16 {
    pub a: [u8; 16],
    pub v: U8x16,
}

impl UnionCast_U8x16 {
    #[inline(always)]
    pub const fn cast(a: [u8; 16]) -> U8x16 {
        unsafe { Self { a }.v }
    }
}

#[cfg(arm_neon)]
pub type U32x4 = uint32x4_t;
#[cfg(not(arm_neon))]
pub type U32x4 = VectorType;

#[repr(C)]
pub(crate) union UnionCast_U32x4 {
    pub a: [u32; 4],
    pub v: U32x4,
}

impl UnionCast_U32x4 {
    #[inline(always)]
    pub const fn cast(a: [u32; 4]) -> U32x4 {
        unsafe { Self { a }.v }
    }
}

pub(crate) const U32X4_FLIP_X: U32x4 = UnionCast_U32x4::cast([(1 << 31), 0, 0, 0]);
pub(crate) const U32X4_FLIP_Y: U32x4 = UnionCast_U32x4::cast([0, (1 << 31), 0, 0]);
pub(crate) const U32X4_FLIP_Z: U32x4 = UnionCast_U32x4::cast([0, 0, (1 << 31), 0]);
pub(crate) const U32X4_FLIP_W: U32x4 = UnionCast_U32x4::cast([0, 0, 0, (1 << 31)]);
pub(crate) const U32X4_MASK_X: U32x4 = UnionCast_U32x4::cast([!0, 0, 0, 0]);
pub(crate) const U32X4_MASK_Y: U32x4 = UnionCast_U32x4::cast([0, !0, 0, 0]);
pub(crate) const U32X4_MASK_Z: U32x4 = UnionCast_U32x4::cast([0, 0, !0, 0]);
pub(crate) const U32X4_MASK_W: U32x4 = UnionCast_U32x4::cast([0, 0, 0, !0]);
pub(crate) const U32X4_MASK_XY: U32x4 = UnionCast_U32x4::cast([!0, !0, 0, 0]);
pub(crate) const U32X4_MASK_XYZ: U32x4 = UnionCast_U32x4::cast([!0, !0, !0, 0]);
pub(crate) const U32X4_MASK_XYZW: U32x4 = UnionCast_U32x4::cast([!0, !0, !0, !0]);
