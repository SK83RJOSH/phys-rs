#![cfg_attr(no_std, no_std)]
#![cfg_attr(nightly, feature(stdsimd))]
#![cfg_attr(spirv, feature(repr_simd))]
#![cfg_attr(psp_vfpu, feature(asm_experimental_arch))]

pub mod math;
