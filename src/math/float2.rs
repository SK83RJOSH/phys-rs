#[cfg(not(spirv))]
use core::fmt;
use core::{f32, ops::*};

/// Creates a 2-dimensional vector.
#[inline(always)]
pub const fn float2(x: f32, y: f32) -> Float2 {
    Float2::new(x, y)
}

/// A 2-dimensional vector, used for storage.
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    #[inline(always)]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Index<usize> for Float2 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Float2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(spirv))]
impl fmt::Display for Float2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[cfg(not(spirv))]
impl fmt::Debug for Float2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Float2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<[f32; 2]> for Float2 {
    #[inline]
    fn from(a: [f32; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<Float2> for [f32; 2] {
    #[inline]
    fn from(v: Float2) -> Self {
        [v.x, v.y]
    }
}

impl From<(f32, f32)> for Float2 {
    #[inline]
    fn from(t: (f32, f32)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<Float2> for (f32, f32) {
    #[inline]
    fn from(v: Float2) -> Self {
        (v.x, v.y)
    }
}
