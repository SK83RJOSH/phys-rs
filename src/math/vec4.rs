#[cfg(not(spirv))]
use core::fmt;
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::math::simd::*;
use crate::math::{float4::Float4, vec3::Vec3};

/// Creates a 4-dimensional vector.
#[inline(always)]
pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}

/// A 4-dimensional vector. SIMD vector types are used for storage.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec4(pub(crate) VectorType);

impl Vec4 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All `f32::MIN`.
    pub const MIN: Self = Self::splat(f32::MIN);

    /// All `f32::MAX`.
    pub const MAX: Self = Self::splat(f32::MAX);

    /// All `f32::NAN`.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// All `f32::INFINITY`.
    pub const INFINITY: Self = Self::splat(f32::INFINITY);

    /// All `f32::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);

    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);

    /// A unit vector pointing along the positive W axis.
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// The unit axes.
    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0, 0.0);

    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0, 0.0);

    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0, 0.0);

    /// A unit vector pointing along the negative W axis.
    pub const NEG_W: Self = Self::new(0.0, 0.0, 0.0, -1.0);

    /// The negative unit axes.
    pub const NEG_AXES: [Self; 4] = [Self::NEG_X, Self::NEG_Y, Self::NEG_Z, Self::NEG_W];

    /// Creates a new vector.
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        unsafe { VectorUnionCast { a: [x, y, z, w] }.v }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: f32) -> Self {
        unsafe { VectorUnionCast { a: [v; 4] }.v }
    }

    #[inline]
    pub const fn from_array(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    #[inline]
    pub const fn to_array(&self) -> [f32; 4] {
        unsafe { *(self as *const Vec4 as *const [f32; 4]) }
    }

    #[inline]
    pub const fn from_slice(a: &[f32]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    #[inline]
    pub fn write_to_slice(self, slice: &mut [f32]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
        slice[4] = self.z;
    }

    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        #[cfg(x86_sse)]
        return Self(unsafe { _mm_min_ps(self.0, rhs.0) });
        #[cfg(arm_neon)]
        return Self(unsafe { vminq_f32(self.0, rhs.0) });
        #[cfg(not(any(x86_sse, arm_neon)))]
        return Self::new(
            self.x.min(rhs.x),
            self.y.min(rhs.y),
            self.z.min(rhs.z),
            self.w.min(rhs.w),
        );
    }

    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        #[cfg(x86_sse)]
        return Self(unsafe { _mm_max_ps(self.0, rhs.0) });
        #[cfg(arm_neon)]
        return Self(unsafe { vmaxq_f32(self.0, rhs.0) });
        #[cfg(not(any(x86_sse, arm_neon)))]
        return Self::new(
            self.x.max(rhs.x),
            self.y.max(rhs.y),
            self.z.max(rhs.z),
            self.w.max(rhs.w),
        );
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        #[cfg(all(x86_sse, not(x86_sse4_1)))]
        return unsafe { sse_dot3_f32(self.0, rhs.0) };
        #[cfg(x86_sse4_1)]
        return unsafe { _mm_cvtss_f32(_mm_dp_ps(self.0, rhs.0, 0x7f)) };
        #[cfg(arm_neon)]
        return unsafe { neon_add(vsetq_lane_f32(0.0, vmulq_f32(self.0, rhs.0), 3)) };
        #[cfg(not(any(x86_sse, x86_sse4_1, arm_neon)))]
        return (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z);
    }

    #[inline]
    pub fn dot_into_vec3(self, rhs: Self) -> Vec3 {
        #[cfg(all(x86_sse, not(x86_sse4_1)))]
        return Vec3(unsafe { sse_dot3_m128(self.0, rhs.0) });
        #[cfg(x86_sse4_1)]
        return Vec3(unsafe { _mm_dp_ps(self.0, rhs.0, 0x7f) });
        #[cfg(arm_neon)]
        return Vec3(unsafe {
            vdupq_n_f32(neon_add(vsetq_lane_f32(0.0, vmulq_f32(self.0, rhs.0), 3)))
        });
        #[cfg(not(any(x86_sse, x86_sse4_1, arm_neon)))]
        return Vec3::splat((self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z));
    }

    #[inline]
    pub fn dot_into_vec4(self, rhs: Self) -> Self {
        #[cfg(all(x86_sse, not(x86_sse4_1)))]
        return Self(unsafe { sse_dot3_m128(self.0, rhs.0) });
        #[cfg(x86_sse4_1)]
        return Self(unsafe { _mm_dp_ps(self.0, rhs.0, 0x7f) });
        #[cfg(all(arm_neon, aarch64))]
        return Self(unsafe {
            vdupq_n_f32(neon_add(vsetq_lane_f32(0.0, vmulq_f32(self.0, rhs.0), 3)))
        });
        #[cfg(not(any(x86_sse, x86_sse4_1, all(arm_neon, aarch64))))]
        return Self::splat((self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z));
    }

    #[inline]
    pub fn min_element(self) -> f32 {
        #[cfg(x86_sse)]
        unsafe {
            let min = _mm_min_ps(self.0, _mm_shuffle_ps(self.0, self.0, 0b00_00_11_10));
            _mm_cvtss_f32(_mm_min_ps(min, _mm_shuffle_ps(min, min, 0b00_00_00_01)))
        }
        #[cfg(all(arm_neon, aarch64))]
        return unsafe {
            let min = vminq_f32(self.0, neon_shuffle::<2, 3, 0, 0>(self.0));
            vgetq_lane_f32::<0>(vminq_f32(min, neon_shuffle::<1, 0, 0, 0>(min)))
        };
        #[cfg(wasm_simd128)]
        return {
            let min = f32x4_pmin(self.0, i32x4_shuffle::<2, 3, 0, 0>(self.0, self.0));
            f32x4_extract_lane::<0>(f32x4_pmin(min, i32x4_shuffle::<1, 0, 0, 0>(min, min)))
        };
        #[cfg(not(any(x86_sse, all(arm_neon, aarch64), wasm_simd128)))]
        return self.x.min(self.y).min(self.z).min(self.w);
    }

    #[inline]
    pub fn max_element(self) -> f32 {
        #[cfg(x86_sse)]
        unsafe {
            let max = _mm_max_ps(self.0, _mm_shuffle_ps(self.0, self.0, 0b00_00_11_10));
            _mm_cvtss_f32(_mm_max_ps(max, _mm_shuffle_ps(max, max, 0b00_00_00_01)))
        }
        #[cfg(all(arm_neon, aarch64))]
        return unsafe {
            let max = vmaxq_f32(self.0, neon_shuffle::<2, 3, 0, 0>(self.0));
            vgetq_lane_f32::<0>(vmaxq_f32(max, neon_shuffle::<1, 0, 0, 0>(max)))
        };
        #[cfg(wasm_simd128)]
        return {
            let max = f32x4_pmax(self.0, i32x4_shuffle::<2, 3, 0, 0>(self.0, self.0));
            f32x4_extract_lane::<0>(f32x4_pmax(max, i32x4_shuffle::<1, 0, 0, 0>(max, max)))
        };
        #[cfg(not(any(x86_sse, all(arm_neon, aarch64), wasm_simd128)))]
        return self.x.max(self.y).max(self.z).max(self.w);
    }
}

#[cfg(not(spirv))]
impl AsRef<[f32; 4]> for Vec4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Vec4 as *const [f32; 4]) }
    }
}

#[cfg(not(spirv))]
impl AsMut<[f32; 4]> for Vec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Vec4 as *mut [f32; 4]) }
    }
}

impl Deref for Vec4 {
    type Target = crate::math::deref::Vec4<f32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl DerefMut for Vec4 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self).cast() }
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(spirv))]
impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

#[cfg(not(spirv))]
impl fmt::Debug for Vec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Vec4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl From<Float4> for Vec4 {
    #[inline]
    fn from(v: Float4) -> Self {
        Self::new(v.x, v.y, v.z, v.w)
    }
}

impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline]
    fn from(v: Vec4) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: Vec4) -> Self {
        (v.x, v.y, v.z, v.w)
    }
}
