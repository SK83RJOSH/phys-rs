#[cfg(not(spirv))]
use core::fmt;
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::math::simd::*;
use crate::math::{float3::Float3, vec4::Vec4};

/// Creates a 3-dimensional vector.
#[inline(always)]
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

/// A 4-dimensional vector. SIMD vector types are used for storage.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3(pub(crate) VectorType);

impl Vec3 {
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
    pub const X: Self = Self::new(1.0, 0.0, 0.0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);

    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);

    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);

    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    /// The negative unit axes.
    pub const NEG_AXES: [Self; 3] = [Self::NEG_X, Self::NEG_Y, Self::NEG_Z];

    /// Creates a new vector.
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        unsafe { VectorUnionCast { a: [x, y, z, z] }.v }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: f32) -> Self {
        unsafe { VectorUnionCast { a: [v; 4] }.v }
    }

    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        #[cfg(x86_sse)]
        return Self(unsafe { _mm_min_ps(self.0, rhs.0) });
        #[cfg(arm_neon)]
        return Self(unsafe { vminq_f32(self.0, rhs.0) });
        #[cfg(wasm_simd128)]
        return Self(f32x4_pmin(self.0, rhs.0));
        #[cfg(not(any(x86_sse, arm_neon, wasm_simd128)))]
        return Self::new(self.x.min(rhs.x), self.y.min(rhs.y), self.z.min(rhs.z));
    }

    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        #[cfg(x86_sse)]
        return Self(unsafe { _mm_max_ps(self.0, rhs.0) });
        #[cfg(arm_neon)]
        return Self(unsafe { vmaxq_f32(self.0, rhs.0) });
        #[cfg(wasm_simd128)]
        return Self(f32x4_pmax(self.0, rhs.0));
        #[cfg(not(any(x86_sse, arm_neon, wasm_simd128)))]
        return Self::new(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z));
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    #[inline]
    pub fn abs(self) -> Self {
        #[cfg(x86_sse)]
        return Self(unsafe { _mm_max_ps(_mm_sub_ps(_mm_setzero_ps(), self.0), self.0) });
        #[cfg(arm_neon)]
        return Self(unsafe { vabsq_f32(self.0) });
        #[cfg(not(any(x86_sse, arm_neon)))]
        return Self::new(self.x.abs(), self.y.abs(), self.z.abs());
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        #[cfg(all(x86_sse, not(x86_sse4_1)))]
        return unsafe { sse_dot3_f32(self.0, rhs.0) };
        #[cfg(x86_sse4_1)]
        return unsafe { _mm_cvtss_f32(_mm_dp_ps(self.0, rhs.0, 0x7f)) };
        #[cfg(arm_neon)]
        return unsafe { vaddvq_f32(vsetq_lane_f32(0.0, vmulq_f32(self.0, rhs.0), 3)) };
        #[cfg(not(any(x86_sse, x86_sse4_1, arm_neon)))]
        return (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z);
    }

    #[inline]
    pub fn dot_into_vec3(self, rhs: Self) -> Self {
        #[cfg(all(x86_sse, not(x86_sse4_1)))]
        return Self(unsafe { sse_dot3_m128(self.0, rhs.0) });
        #[cfg(x86_sse4_1)]
        return Self(unsafe { _mm_dp_ps(self.0, rhs.0, 0x7f) });
        #[cfg(arm_neon)]
        return Self(unsafe {
            vdupq_n_f32(vaddvq_f32(vsetq_lane_f32(0.0, vmulq_f32(self.0, rhs.0), 3)))
        });
        #[cfg(not(any(x86_sse, x86_sse4_1, arm_neon)))]
        return Self::splat((self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z));
    }

    #[inline]
    pub fn dot_into_vec4(self, rhs: Self) -> Vec4 {
        #[cfg(all(x86_sse, not(x86_sse4_1)))]
        return Vec4(unsafe { sse_dot3_m128(self.0, rhs.0) });
        #[cfg(x86_sse4_1)]
        return Vec4(unsafe { _mm_dp_ps(self.0, rhs.0, 0x7f) });
        #[cfg(arm_neon)]
        return Vec4(unsafe {
            vdupq_n_f32(vaddvq_f32(vsetq_lane_f32(0.0, vmulq_f32(self.0, rhs.0), 3)))
        });
        #[cfg(not(any(x86_sse, x86_sse4_1, arm_neon)))]
        return Vec4::splat((self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z));
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        #[cfg(x86_sse)]
        return unsafe {
            const MASK: i32 = 0b00_00_10_01;
            let sub = _mm_sub_ps(
                _mm_mul_ps(_mm_shuffle_ps(rhs.0, rhs.0, MASK), self.0),
                _mm_mul_ps(_mm_shuffle_ps(self.0, self.0, MASK), rhs.0),
            );
            Self(_mm_shuffle_ps(sub, sub, MASK))
        };
        #[cfg(arm_neon)]
        return unsafe {
            let sub = vsubq_f32(
                vmulq_f32(neon_shuffle_1200(rhs.0), self.0),
                vmulq_f32(neon_shuffle_1200(self.0), rhs.0),
            );
            Self(neon_shuffle_1200(sub))
        };
        #[cfg(wasm_simd128)]
        return {
            let sub = f32x4_sub(
                f32x4_mul(i32x4_shuffle::<2, 0, 1, 1>(self.0, self.0), rhs.0),
                f32x4_mul(i32x4_shuffle::<2, 0, 1, 1>(rhs.0, rhs.0), self.0),
            );
            Self(i32x4_shuffle::<2, 0, 1, 1>(sub, sub))
        };
        #[cfg(not(any(x86_sse, arm_neon, wasm_simd128)))]
        return Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        );
    }
}

#[cfg(not(spirv))]
impl AsRef<[f32; 3]> for Vec3 {
    #[inline]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3 as *const [f32; 3]) }
    }
}

#[cfg(not(spirv))]
impl AsMut<[f32; 3]> for Vec3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3 as *mut [f32; 3]) }
    }
}

impl Deref for Vec3 {
    type Target = crate::math::deref::Vec3<f32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl DerefMut for Vec3 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self).cast() }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(spirv))]
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

#[cfg(not(spirv))]
impl fmt::Debug for Vec3 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Vec3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl From<Float3> for Vec3 {
    #[inline]
    fn from(t: Float3) -> Self {
        Self::new(t.x, t.y, t.z)
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(v: Vec3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}
