#![allow(unused)]

use rytm_rs::{
    error::{ConversionError, ParameterError, RytmError, SysexConversionError},
    object::pattern::MicroTime,
    object::sound::types::Machine,
};
use rytm_rs_macro::parameter_range;
use rytm_sys::{
    ar_sysex_id_t_AR_TYPE_GLOBAL, ar_sysex_id_t_AR_TYPE_KIT, ar_sysex_id_t_AR_TYPE_PATTERN,
    ar_sysex_id_t_AR_TYPE_SETTINGS, ar_sysex_id_t_AR_TYPE_SONG, ar_sysex_id_t_AR_TYPE_SOUND,
    ar_sysex_meta_t, s_u16_t, s_u16_t__bindgen_ty_1,
};

/// Pattern sysex response size for FW 1.70.
pub(crate) const PATTERN_SYSEX_SIZE: usize = 2998;
/// Kit sysex response size for FW 1.70.
pub(crate) const KIT_SYSEX_SIZE: usize = 14988;
/// Sound sysex response size for FW 1.70.
pub(crate) const SOUND_SYSEX_SIZE: usize = 201;
/// Settings sysex response size for FW 1.70.
pub(crate) const SETTINGS_SYSEX_SIZE: usize = 2401;
/// Global sysex response size for FW 1.70.
pub(crate) const GLOBAL_SYSEX_SIZE: usize = 107;
/// Song sysex response size for FW 1.70.
pub(crate) const SONG_SYSEX_SIZE: usize = 1506;

const SYSEX_MESSAGE_TYPE_BYTE_INDEX: usize = 7;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SysexType {
    Pattern,
    Kit,
    Sound,
    Song,
    #[default]
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

#[allow(non_upper_case_globals)]
impl TryFrom<u8> for SysexType {
    type Error = ConversionError;
    fn try_from(sysex_type: u8) -> Result<Self, Self::Error> {
        match sysex_type as u32 {
            ar_sysex_id_t_AR_TYPE_PATTERN => Ok(SysexType::Pattern),
            ar_sysex_id_t_AR_TYPE_KIT => Ok(SysexType::Kit),
            ar_sysex_id_t_AR_TYPE_SOUND => Ok(SysexType::Sound),
            ar_sysex_id_t_AR_TYPE_SONG => Ok(SysexType::Song),
            ar_sysex_id_t_AR_TYPE_SETTINGS => Ok(SysexType::Settings),
            ar_sysex_id_t_AR_TYPE_GLOBAL => Ok(SysexType::Global),
            _ => Err(ConversionError::Range {
                value: sysex_type.to_string(),
                type_name: "SysexType".into(),
            }),
        }
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
    const SYSEX_META_CONTAINER_VERSION: u16 = 0x0101;

    pub fn is_targeting_work_buffer(&self) -> bool {
        self.obj_nr >= 128
    }

    pub fn object_type(&self) -> Result<SysexType, RytmError> {
        let r#type: SysexType = self.obj_type.try_into()?;
        Ok(r#type)
    }

    pub fn default_for_settings(dev_id: Option<usize>) -> Self {
        Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Settings.into(),
            obj_nr: 0b0000_0000,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    #[parameter_range(range = "global_slot:0..=3")]
    pub fn try_default_for_global(
        global_slot: usize,
        dev_id: Option<usize>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Global.into(),
            obj_nr: global_slot as u16,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        })
    }

    pub fn default_for_global_in_work_buffer(dev_id: Option<usize>) -> Self {
        Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Global.into(),
            obj_nr: 0b1000_0000,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    #[parameter_range(range = "pattern_index:0..=127")]
    pub fn try_default_for_pattern(
        pattern_index: usize,
        dev_id: Option<usize>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Pattern.into(),
            obj_nr: pattern_index as u16,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        })
    }

    pub fn default_for_pattern_in_work_buffer(dev_id: Option<usize>) -> Self {
        Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Pattern.into(),
            obj_nr: 0b1000_0000,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    #[parameter_range(range = "kit_index:0..=127")]
    pub fn try_default_for_kit(kit_index: usize, dev_id: Option<usize>) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Kit.into(),
            obj_nr: kit_index as u16,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        })
    }

    pub fn default_for_kit_in_work_buffer(dev_id: Option<usize>) -> Self {
        Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Kit.into(),
            obj_nr: 0b1000_0000,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    #[parameter_range(range = "sound_index:0..=127")]
    pub fn try_default_for_sound(
        sound_index: usize,
        dev_id: Option<usize>,
    ) -> Result<Self, RytmError> {
        Ok(Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Sound.into(),
            obj_nr: sound_index as u16,
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        })
    }

    pub fn default_for_sound_in_work_buffer(dev_id: Option<usize>) -> Self {
        Self {
            container_version: SysexMeta::SYSEX_META_CONTAINER_VERSION,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Sound.into(),
            obj_nr: 0b1000_0000,
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

/// This function assumes that the response is a valid sysex response.
///
/// It should be used in a context where this case is true and validity check is not necessary.
pub fn decode_sysex_response_to_raw(response: &[u8]) -> Result<(Vec<u8>, SysexMeta), RytmError> {
    let response_type: SysexType = response[SYSEX_MESSAGE_TYPE_BYTE_INDEX].try_into()?;
    let response_size = match response_type {
        SysexType::Pattern => PATTERN_SYSEX_SIZE,
        SysexType::Kit => KIT_SYSEX_SIZE,
        SysexType::Sound => SOUND_SYSEX_SIZE,
        SysexType::Settings => SETTINGS_SYSEX_SIZE,
        SysexType::Global => GLOBAL_SYSEX_SIZE,
        SysexType::Song => SONG_SYSEX_SIZE,
    };

    // Make a default meta struct to fill.
    let meta = SysexMeta::default();
    let mut meta: rytm_sys::ar_sysex_meta_t = meta.into();
    let meta_p = &mut meta as *mut rytm_sys::ar_sysex_meta_t;

    // The response buffer.
    let mut src_buf = response.as_ptr();
    let src_buf_p = &mut src_buf as *mut *const u8;
    let mut src_buf_size = response.len() as u32;
    let src_buf_size_p = &mut src_buf_size as *mut u32;

    // Will be calculated by the first call to ar_sysex_to_raw.
    let dst_buf_size = 0; // Big enough for the largest sysex message probably.
    let dest_buf_size_p = dst_buf_size as *mut u32;

    // The destination buffer, raw buffer.
    let mut dst_buf = vec![0_u8; response_size];
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

pub fn to_s_u16_t_union_b_from_u8_as_msb(value: u8) -> s_u16_t {
    s_u16_t {
        b: s_u16_t__bindgen_ty_1 { hi: value, lo: 0 },
    }
}

pub fn to_s_u16_t_union_b_from_u8_as_lsb(value: u8) -> s_u16_t {
    s_u16_t {
        b: s_u16_t__bindgen_ty_1 { hi: 0, lo: value },
    }
}

pub unsafe fn from_s_u16_t(value: &s_u16_t) -> u16 {
    let msb = value.b.hi as u16;
    let lsb = value.b.lo as u16;
    (msb << 8) | lsb
}

pub fn decode_micro_timing_byte(micro_timing_value: i8) -> Result<MicroTime, ConversionError> {
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

pub fn encode_micro_timing_byte(micro_timing: &MicroTime) -> i8 {
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

/// Checks if the given machine is compatible for the given track.
pub fn is_machine_compatible_for_track(track_index: usize, machine: Machine) -> bool {
    let compatible_machines = unsafe { rytm_sys::ar_sound_compatible_machines };
    let compatible_machines_for_track = compatible_machines[track_index];

    let mut compatible_machines_for_track_size = 0;
    loop {
        unsafe {
            let return_id = rytm_sys::ar_sound_get_machine_id_by_track_and_list_idx(
                track_index as u32,
                compatible_machines_for_track_size,
            );
            if return_id == -1 {
                break;
            }
            compatible_machines_for_track_size += 1;
        }
    }

    let compatible_machines_for_track_slice = unsafe {
        std::slice::from_raw_parts(
            compatible_machines_for_track,
            compatible_machines_for_track_size as usize,
        )
    };

    compatible_machines_for_track_slice.contains(&((machine as u8) as i32))
}
