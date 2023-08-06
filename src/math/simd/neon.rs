use super::*;

#[repr(C)]
union UnionCast_u8x16 {
    pub a: [u8; 16],
    pub v: uint8x16_t,
}

pub(crate) unsafe fn neon_shuffle<const I0: i32, const I1: i32, const I2: i32, const I3: i32>(
    value: float32x4_t,
) -> float32x4_t {
    let mov = vmovq_n_f32(vgetq_lane_f32(value, I0));
    let set = vsetq_lane_f32(vgetq_lane_f32(value, I1), mov, 1);
    let set = vsetq_lane_f32(vgetq_lane_f32(value, I2), set, 2);
    vsetq_lane_f32(vgetq_lane_f32(value, I3), set, 3)
}

const SHUFFLE_1200: UnionCast_u8x16 = UnionCast_u8x16 {
    a: [
        0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0x0, 0x1, 0x2, 0x3, 0x0, 0x1, 0x2, 0x3,
    ],
};

#[inline(always)]
pub(crate) unsafe fn neon_shuffle_1200(value: float32x4_t) -> float32x4_t {
    vreinterpretq_f32_u8(vqtbl1q_u8(vreinterpretq_u8_f32(value), SHUFFLE_1200.v))
}
