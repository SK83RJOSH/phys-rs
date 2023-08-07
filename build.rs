extern crate cfg_aliases;
use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        simd: { not(feature = "no-simd") },
        aarch32: { target_arch = "arm" },
        aarch64: { target_arch = "aarch64" },
        arm: { any(aarch32, aarch64) },
        arm_neon: { all(simd, arm, target_feature = "neon", any(aarch64, feature = "nightly")) },
        spirv: { target_arch = "spirv" },
        wasm: { any(target_arch = "wasm32", target_arch = "wasm64") },
        wasm_simd128: { all(simd, wasm, target_feature = "simd128") },
        x86: { any(target_arch = "x86", target_arch = "x86_64") },
        x86_sse: { all(simd, x86, target_feature = "sse") },
        x86_sse2: { all(simd, x86, target_feature = "sse2") },
        x86_sse3: { all(simd, x86, target_feature = "sse3") },
        x86_sse4: { all(simd, x86, target_feature = "sse4") },
        x86_sse4_1: { all(simd, x86, target_feature = "sse4.1") },
        x86_sse4_2: { all(simd, x86, target_feature = "sse4.2") },
    }
}
