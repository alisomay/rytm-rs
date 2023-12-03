use rytm_sys::{s_u16_t, s_u16_t__bindgen_ty_1};

use crate::error::ConversionError;
use crate::pattern::MicroTime;

#[allow(unused)]
pub fn to_s_u16_t_union_a(value: u16) -> s_u16_t {
    let msb = (value >> 8) as u8;
    let lsb = (value & 0xFF) as u8;
    s_u16_t { a: [msb, lsb] }
}

#[allow(unused)]
pub fn to_s_u16_t_union_v(value: u16) -> s_u16_t {
    s_u16_t { v: value }
}

pub fn to_s_u16_t_union_b(value: u16) -> s_u16_t {
    let msb = (value >> 8) as u8;
    let lsb = (value & 0xFF) as u8;
    s_u16_t {
        b: s_u16_t__bindgen_ty_1 { hi: msb, lo: lsb },
    }
}

pub unsafe fn from_s_u16_t(value: &s_u16_t) -> u16 {
    let msb = value.b.hi as u16;
    let lsb = value.b.lo as u16;
    (msb << 8) | lsb
}

pub(crate) fn decode_micro_timing_byte(
    micro_timing_value: i8,
) -> Result<MicroTime, ConversionError> {
    match micro_timing_value {
        -92 => Ok(MicroTime::N23B384),
        -88 => Ok(MicroTime::N11B192),
        -84 => Ok(MicroTime::N7B128),
        -80 => Ok(MicroTime::N5B96),
        -76 => Ok(MicroTime::N19B384),
        -72 => Ok(MicroTime::N3B64),
        -68 => Ok(MicroTime::N17B384),
        -64 => Ok(MicroTime::N1B24),
        -60 => Ok(MicroTime::N5B128),
        -56 => Ok(MicroTime::N7B192),
        -52 => Ok(MicroTime::N13B384),
        -48 => Ok(MicroTime::N32nd),
        -44 => Ok(MicroTime::N11B384),
        -40 => Ok(MicroTime::N5B192),
        -36 => Ok(MicroTime::N3B128),
        -32 => Ok(MicroTime::N1B48),
        -28 => Ok(MicroTime::N7B384),
        -24 => Ok(MicroTime::N64th),
        -20 => Ok(MicroTime::N5B384),
        -16 => Ok(MicroTime::N1B96),
        -12 => Ok(MicroTime::N1B128),
        -8 => Ok(MicroTime::N1B192),
        -4 => Ok(MicroTime::N1B384),
        0 => Ok(MicroTime::OnGrid),
        4 => Ok(MicroTime::P1B384),
        8 => Ok(MicroTime::P1B192),
        12 => Ok(MicroTime::P1B128),
        16 => Ok(MicroTime::P1B96),
        20 => Ok(MicroTime::P5B384),
        24 => Ok(MicroTime::P64th),
        28 => Ok(MicroTime::P7B384),
        32 => Ok(MicroTime::P1B48),
        36 => Ok(MicroTime::P3B128),
        40 => Ok(MicroTime::P5B192),
        44 => Ok(MicroTime::P11B384),
        48 => Ok(MicroTime::P32nd),
        52 => Ok(MicroTime::P13B384),
        56 => Ok(MicroTime::P7B192),
        60 => Ok(MicroTime::P5B128),
        64 => Ok(MicroTime::P1B24),
        68 => Ok(MicroTime::P17B384),
        72 => Ok(MicroTime::P3B64),
        76 => Ok(MicroTime::P19B384),
        80 => Ok(MicroTime::P5B96),
        84 => Ok(MicroTime::P7B128),
        88 => Ok(MicroTime::P11B192),
        92 => Ok(MicroTime::P23B384),
        _ => Err(ConversionError::Range {
            value: micro_timing_value.to_string(),
            type_name: "MicroTime".into(),
        }),
    }
}

pub(crate) fn encode_micro_timing_byte(micro_timing: &MicroTime) -> i8 {
    match micro_timing {
        MicroTime::N23B384 => -92,
        MicroTime::N11B192 => -88,
        MicroTime::N7B128 => -84,
        MicroTime::N5B96 => -80,
        MicroTime::N19B384 => -76,
        MicroTime::N3B64 => -72,
        MicroTime::N17B384 => -68,
        MicroTime::N1B24 => -64,
        MicroTime::N5B128 => -60,
        MicroTime::N7B192 => -56,
        MicroTime::N13B384 => -52,
        MicroTime::N32nd => -48,
        MicroTime::N11B384 => -44,
        MicroTime::N5B192 => -40,
        MicroTime::N3B128 => -36,
        MicroTime::N1B48 => -32,
        MicroTime::N7B384 => -28,
        MicroTime::N64th => -24,
        MicroTime::N5B384 => -20,
        MicroTime::N1B96 => -16,
        MicroTime::N1B128 => -12,
        MicroTime::N1B192 => -8,
        MicroTime::N1B384 => -4,
        MicroTime::OnGrid => 0,
        MicroTime::P1B384 => 4,
        MicroTime::P1B192 => 8,
        MicroTime::P1B128 => 12,
        MicroTime::P1B96 => 16,
        MicroTime::P5B384 => 20,
        MicroTime::P64th => 24,
        MicroTime::P7B384 => 28,
        MicroTime::P1B48 => 32,
        MicroTime::P3B128 => 36,
        MicroTime::P5B192 => 40,
        MicroTime::P11B384 => 44,
        MicroTime::P32nd => 48,
        MicroTime::P13B384 => 52,
        MicroTime::P7B192 => 56,
        MicroTime::P5B128 => 60,
        MicroTime::P1B24 => 64,
        MicroTime::P17B384 => 68,
        MicroTime::P3B64 => 72,
        MicroTime::P19B384 => 76,
        MicroTime::P5B96 => 80,
        MicroTime::P7B128 => 84,
        MicroTime::P11B192 => 88,
        MicroTime::P23B384 => 92,
    }
}
