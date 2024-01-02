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
