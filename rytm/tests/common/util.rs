use rytm_rs_macro::parameter_range;
use rytm_sys::{
    ar_sysex_id_t_AR_TYPE_GLOBAL, ar_sysex_id_t_AR_TYPE_KIT, ar_sysex_id_t_AR_TYPE_PATTERN,
    ar_sysex_id_t_AR_TYPE_SETTINGS, ar_sysex_id_t_AR_TYPE_SONG, ar_sysex_id_t_AR_TYPE_SOUND,
};
use rytm_sys::{ar_sysex_meta_t, s_u16_t, s_u16_t__bindgen_ty_1};

use rytm_rs::error::ParameterError;
use rytm_rs::error::RytmError;
use rytm_rs::error::{ConversionError, SysexConversionError};
use rytm_rs::pattern::MicroTime;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SysexType {
    Pattern,
    Kit,
    Sound,
    Song,
    Settings,
    Global,
}

impl From<SysexType> for u8 {
    fn from(sysex_type: SysexType) -> Self {
        let sysex_type = match sysex_type {
            SysexType::Pattern => ar_sysex_id_t_AR_TYPE_PATTERN,
            SysexType::Kit => ar_sysex_id_t_AR_TYPE_KIT,
            SysexType::Sound => ar_sysex_id_t_AR_TYPE_SOUND,
            SysexType::Song => ar_sysex_id_t_AR_TYPE_SONG,
            SysexType::Settings => ar_sysex_id_t_AR_TYPE_SETTINGS,
            SysexType::Global => ar_sysex_id_t_AR_TYPE_GLOBAL,
        };

        sysex_type as u8
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SysexMeta {
    pub container_version: u16,
    pub dev_id: u8,
    pub obj_type: u8,
    pub obj_nr: u16,
    pub chksum: u16,
    pub data_size: u16,
}

impl SysexMeta {
    #[parameter_range(range = "object_index:0..=127")]
    pub fn try_default_for_pattern(
        object_index: usize,
        dev_id: Option<usize>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: 0x0101,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Pattern.into(),
            obj_nr: object_index as u16,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        })
    }

    pub fn default_for_pattern_in_work_buffer(dev_id: Option<usize>) -> Self {
        Self {
            container_version: 0x0101,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Pattern.into(),
            obj_nr: 0xFF,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    // TODO: Find the right range.
    #[parameter_range(range = "object_index:0..=127")]
    pub fn try_default_for_sound(
        object_index: usize,
        dev_id: Option<usize>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: 0x0101,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Sound.into(),
            obj_nr: object_index as u16,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        })
    }

    pub fn default_for_sound_in_work_buffer(dev_id: Option<usize>) -> Self {
        Self {
            container_version: 0x0101,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Sound.into(),
            // TODO: Double check this.
            obj_nr: 0xFF,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }
}

impl From<SysexMeta> for ar_sysex_meta_t {
    fn from(meta: SysexMeta) -> Self {
        ar_sysex_meta_t {
            container_version: to_s_u16_t_union_b(meta.container_version),
            dev_id: meta.dev_id,
            obj_type: meta.obj_type,
            obj_nr: meta.obj_nr,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: to_s_u16_t_union_b(meta.chksum),
            data_size: to_s_u16_t_union_b(meta.data_size),
        }
    }
}

impl From<&ar_sysex_meta_t> for SysexMeta {
    fn from(meta: &ar_sysex_meta_t) -> Self {
        SysexMeta {
            container_version: unsafe { from_s_u16_t(&meta.container_version) },
            dev_id: meta.dev_id,
            obj_type: meta.obj_type,
            obj_nr: meta.obj_nr,
            chksum: unsafe { from_s_u16_t(&meta.chksum) },
            data_size: unsafe { from_s_u16_t(&meta.data_size) },
        }
    }
}

pub fn decode_sysex_response_to_raw(response: &[u8]) -> Result<(Vec<u8>, SysexMeta), RytmError> {
    // This could be made smaller later, the largest reverse-engineered sysex response I've seen is 14988 bytes.
    // But for now I think it is a reasonable value.
    // It can be optimized if we know which response we're expecting if necessary.
    const LARGE_SYSEX_GUESSED_SIZE: usize = 4096 * 4;

    let meta = SysexMeta::default();
    let mut meta: rytm_sys::ar_sysex_meta_t = meta.into();
    let meta_p = &mut meta as *mut rytm_sys::ar_sysex_meta_t;

    let mut src_buf = response.as_ptr();
    let src_buf_p = &mut src_buf as *mut *const u8;
    let mut src_buf_size = response.len() as u32;
    let src_buf_size_p = &mut src_buf_size as *mut u32;

    // Will be calculated by the first call to ar_sysex_to_raw.
    let dst_buf_size = 0; // Big enough for the largest sysex message probably.
    let dest_buf_size_p = dst_buf_size as *mut u32;

    let mut dst_buf = vec![0_u8; LARGE_SYSEX_GUESSED_SIZE];
    let dst_buf_p = dst_buf.as_mut_slice().as_mut_ptr();

    unsafe {
        let return_code = rytm_sys::ar_sysex_to_raw(
            dst_buf_p,
            src_buf_p,
            src_buf_size_p,
            dest_buf_size_p,
            meta_p,
        ) as u8;

        if return_code != 0 {
            return Err(SysexConversionError::from(return_code).into());
        }
    }

    dst_buf.shrink_to_fit();

    Ok((dst_buf, SysexMeta::from(&meta)))
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
