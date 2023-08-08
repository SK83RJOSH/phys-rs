use super::*;

#[inline(always)]
#[cfg(arm64_neon)]
const fn shuffle_mask<const I0: u8, const I1: u8, const I2: u8, const I3: u8>() -> uint8x16_t {
    UnionCast_U8x16::cast([
        (I0 * 4),
        (I0 * 4) + 1,
        (I0 * 4) + 2,
        (I0 * 4) + 3,
        (I1 * 4),
        (I1 * 4) + 1,
        (I1 * 4) + 2,
        (I1 * 4) + 3,
        (I2 * 4),
        (I2 * 4) + 1,
        (I2 * 4) + 2,
        (I2 * 4) + 3,
        (I3 * 4),
        (I3 * 4) + 1,
        (I3 * 4) + 2,
        (I3 * 4) + 3,
    ])
}

#[inline(always)]
#[cfg(arm64_neon)]
pub(crate) unsafe fn neon_shuffle<const I0: u8, const I1: u8, const I2: u8, const I3: u8>(
    value: float32x4_t,
) -> float32x4_t {
    vreinterpretq_f32_u8(vqtbl1q_u8(
        vreinterpretq_u8_f32(value),
        shuffle_mask::<I0, I1, I2, I3>(),
    ))
}

#[inline(always)]
#[cfg(arm64_neon)]
pub(crate) unsafe fn neon_shuffle_1200(value: float32x4_t) -> float32x4_t {
    const SHUFFLE: uint8x16_t = UnionCast_U8x16::cast([
        0x4, 0x5, 0x6, 0x7, // 1
        0x8, 0x9, 0xa, 0xb, // 2
        0x0, 0x1, 0x2, 0x3, // 0
        0x0, 0x1, 0x2, 0x3, // 0
    ]);
    vreinterpretq_f32_u8(vqtbl1q_u8(vreinterpretq_u8_f32(value), SHUFFLE))
}

#[inline(always)]
#[cfg(all(arm_neon, not(arm64_neon)))]
pub(crate) unsafe fn vaddvq_f32(value: float32x4_t) -> f32 {
    let add = vadd_f32(vget_low_f32(value), vget_high_f32(value));
    vget_lane_f32::<0>(vpadd_f32(add, add))
}
