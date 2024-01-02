#[cfg(not(spirv))]
use core::fmt;
use core::{f32, ops::*};

use crate::math::simd::*;
use crate::math::vec4::Vec4;

/// Creates a 4-dimensional vector.
#[inline(always)]
pub const fn quat(x: f32, y: f32, z: f32, w: f32) -> Quat {
    Quat::from_xyzw(x, y, z, w)
}

/// A quaternion representing an orientation. SIMD vector types are used for storage.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Quat(pub(crate) Vec4);

impl Quat {
    /// All zeroes.
    pub const ZERO: Self = Self::from_array([0.0; 4]);

    /// The identity quaternion. Corresponds to no rotation.
    pub const IDENTITY: Self = Self::from_xyzw(0.0, 0.0, 0.0, 1.0);

    /// All `f32::NAN`.
    pub const NAN: Self = Self::from_array([f32::NAN; 4]);

    /// Creates a new rotation quaternion.
    #[inline(always)]
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        unsafe { VectorUnionCast { a: [x, y, z, w] }.v }
    }

    /// Creates a new rotation quaternion from an array.
    #[inline]
    pub const fn from_array(a: [f32; 4]) -> Self {
        unsafe { VectorUnionCast { a }.v }
    }

    /// Creates a new rotation quaternion from a 4-dimensional vector.
    #[inline]
    pub const fn from_vec4(v: Vec4) -> Self {
        Self(v)
    }
}

#[cfg(not(spirv))]
impl AsRef<[f32; 4]> for Quat {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Quat as *const [f32; 4]) }
    }
}

#[cfg(not(spirv))]
impl AsMut<[f32; 4]> for Quat {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Quat as *mut [f32; 4]) }
    }
}

impl Deref for Quat {
    type Target = crate::math::deref::Vec4<f32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl DerefMut for Quat {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self).cast() }
    }
}

impl Index<usize> for Quat {
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

impl IndexMut<usize> for Quat {
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
impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

#[cfg(not(spirv))]
impl fmt::Debug for Quat {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Quat))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl From<Vec4> for Quat {
    #[inline]
    fn from(v: Vec4) -> Self {
        Self(v)
    }
}

impl From<[f32; 4]> for Quat {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Self::from_xyzw(a[0], a[1], a[2], a[3])
    }
}

impl From<Quat> for [f32; 4] {
    #[inline]
    fn from(v: Quat) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self::from_xyzw(t.0, t.1, t.2, t.3)
    }
}

impl From<Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: Quat) -> Self {
        (v.x, v.y, v.z, v.w)
    }
}
