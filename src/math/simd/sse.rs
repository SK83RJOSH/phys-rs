#![allow(dead_code)]

use super::*;

#[inline(always)]
unsafe fn sse_dot3(lhs: __m128, rhs: __m128) -> __m128 {
    let mul = _mm_mul_ps(lhs, rhs);
    _mm_add_ss(
        _mm_add_ss(mul, _mm_shuffle_ps(mul, mul, 0b00_00_00_01)),
        _mm_shuffle_ps(mul, mul, 0b00_00_00_10),
    )
}

#[inline(always)]
pub(crate) unsafe fn sse_dot3_f32(lhs: __m128, rhs: __m128) -> f32 {
    _mm_cvtss_f32(sse_dot3(lhs, rhs))
}

#[inline(always)]
pub(crate) unsafe fn sse_dot3_m128(lhs: __m128, rhs: __m128) -> __m128 {
    let dot = sse_dot3(lhs, rhs);
    _mm_shuffle_ps(dot, dot, 0b00_00_00_00)
}

#[inline(always)]
unsafe fn sse_dot4(lhs: __m128, rhs: __m128) -> __m128 {
    let mul = _mm_mul_ps(lhs, rhs);
    let add = _mm_add_ps(mul, _mm_shuffle_ps(mul, mul, 0b00_00_11_10));
    _mm_add_ps(add, _mm_shuffle_ps(add, add, 0b00_00_00_01))
}

#[inline(always)]
pub(crate) unsafe fn sse_dot4_f32(lhs: __m128, rhs: __m128) -> f32 {
    _mm_cvtss_f32(sse_dot4(lhs, rhs))
}

#[inline(always)]
pub(crate) unsafe fn sse_dot4_m128(lhs: __m128, rhs: __m128) -> __m128 {
    let dot = sse_dot4(lhs, rhs);
    _mm_shuffle_ps(dot, dot, 0b00_00_00_00)
}
