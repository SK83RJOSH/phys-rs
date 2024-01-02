#![allow(unused_imports, dead_code)]

mod math {
    #[inline(always)]
    pub(crate) fn abs(f: f32) -> f32 {
        libm::fabsf(f)
    }

    #[inline(always)]
    pub(crate) fn acos(f: f32) -> f32 {
        libm::acosf(f)
    }

    #[inline(always)]
    pub(crate) fn asin(f: f32) -> f32 {
        libm::asinf(f)
    }

    #[inline(always)]
    pub(crate) fn atan2(f: f32, other: f32) -> f32 {
        libm::atan2f(f, other)
    }

    #[inline(always)]
    pub(crate) fn sin(f: f32) -> f32 {
        libm::sinf(f)
    }

    #[inline(always)]
    pub(crate) fn sin_cos(f: f32) -> (f32, f32) {
        libm::sincosf(f)
    }

    #[inline(always)]
    pub(crate) fn tan(f: f32) -> f32 {
        libm::tanf(f)
    }

    #[inline(always)]
    pub(crate) fn sqrt(f: f32) -> f32 {
        libm::sqrtf(f)
    }

    #[inline(always)]
    pub(crate) fn copysign(f: f32, sign: f32) -> f32 {
        libm::copysignf(f, sign)
    }

    #[inline(always)]
    pub(crate) fn signum(f: f32) -> f32 {
        if f.is_nan() {
            f32::NAN
        } else {
            copysign(1.0, f)
        }
    }

    #[inline(always)]
    pub(crate) fn round(f: f32) -> f32 {
        libm::roundf(f)
    }

    #[inline(always)]
    pub(crate) fn trunc(f: f32) -> f32 {
        libm::truncf(f)
    }

    #[inline(always)]
    pub(crate) fn ceil(f: f32) -> f32 {
        libm::ceilf(f)
    }

    #[inline(always)]
    pub(crate) fn floor(f: f32) -> f32 {
        libm::floorf(f)
    }

    #[inline(always)]
    pub(crate) fn exp(f: f32) -> f32 {
        libm::expf(f)
    }

    #[inline(always)]
    pub(crate) fn powf(f: f32, n: f32) -> f32 {
        libm::powf(f, n)
    }

    #[inline(always)]
    pub(crate) fn mul_add(a: f32, b: f32, c: f32) -> f32 {
        libm::fmaf(a, b, c)
    }

    #[inline(always)]
    pub fn div_euclid(a: f32, b: f32) -> f32 {
        let q = libm::truncf(a / b);
        if a % b < 0.0 {
            return if b > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }

    #[inline(always)]
    pub fn rem_euclid(a: f32, b: f32) -> f32 {
        let r = a % b;
        if r < 0.0 {
            r + abs(b)
        } else {
            r
        }
    }
}

pub(crate) use math::*;

pub(crate) trait F32Ext {
    fn abs(&self) -> f32;
    fn acos(&self) -> f32;
    fn asin(&self) -> f32;
    fn atan2(&self, other: f32) -> f32;
    fn sin(&self) -> f32;
    fn sin_cos(&self) -> (f32, f32);
    fn tan(&self) -> f32;
    fn sqrt(&self) -> f32;
    fn copysign(&self, sign: f32) -> f32;
    fn signum(&self) -> f32;
    fn round(&self) -> f32;
    fn trunc(&self) -> f32;
    fn ceil(&self) -> f32;
    fn floor(&self) -> f32;
    fn exp(&self) -> f32;
    fn powf(&self, n: f32) -> f32;
    fn mul_add(&self, a: f32, b: f32) -> f32;
    fn div_euclid(&self, other: f32) -> f32;
    fn rem_euclid(&self, other: f32) -> f32;
}

impl F32Ext for f32 {
    #[inline(always)]
    fn abs(&self) -> f32 {
        abs(*self)
    }

    #[inline(always)]
    fn acos(&self) -> f32 {
        acos(*self)
    }

    #[inline(always)]
    fn asin(&self) -> f32 {
        asin(*self)
    }

    #[inline(always)]
    fn atan2(&self, other: f32) -> f32 {
        atan2(*self, other)
    }

    #[inline(always)]
    fn sin(&self) -> f32 {
        sin(*self)
    }

    #[inline(always)]
    fn sin_cos(&self) -> (f32, f32) {
        sin_cos(*self)
    }

    #[inline(always)]
    fn tan(&self) -> f32 {
        tan(*self)
    }

    #[inline(always)]
    fn sqrt(&self) -> f32 {
        sqrt(*self)
    }

    #[inline(always)]
    fn copysign(&self, sign: f32) -> f32 {
        copysign(*self, sign)
    }

    #[inline(always)]
    fn signum(&self) -> f32 {
        signum(*self)
    }

    #[inline(always)]
    fn round(&self) -> f32 {
        round(*self)
    }

    #[inline(always)]
    fn trunc(&self) -> f32 {
        trunc(*self)
    }

    #[inline(always)]
    fn ceil(&self) -> f32 {
        ceil(*self)
    }

    #[inline(always)]
    fn floor(&self) -> f32 {
        floor(*self)
    }

    #[inline(always)]
    fn exp(&self) -> f32 {
        exp(*self)
    }

    #[inline(always)]
    fn powf(&self, n: Self) -> f32 {
        powf(*self, n)
    }

    #[inline(always)]
    fn mul_add(&self, a: f32, b: f32) -> f32 {
        mul_add(*self, a, b)
    }

    #[inline(always)]
    fn div_euclid(&self, other: f32) -> f32 {
        div_euclid(*self, other)
    }

    #[inline(always)]
    fn rem_euclid(&self, other: f32) -> f32 {
        rem_euclid(*self, other)
    }
}
