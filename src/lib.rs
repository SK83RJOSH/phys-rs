#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(stdsimd))]
#![cfg_attr(target_arch = "spirv", feature(repr_simd))]

pub mod math;
