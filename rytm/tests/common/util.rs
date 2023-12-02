use rytm_sys::{
    ar_pattern_request, ar_pattern_request_x, ar_sysex_meta_t, s_u16_t, s_u16_t__bindgen_ty_1,
};

const LARGE_SYSEX_SIZE: usize = 5572; // This is the maximum that worked for me (aka midir)

pub fn to_s_u16_t_union_a(value: u16) -> s_u16_t {
    let msb = (value >> 8) as u8;
    let lsb = (value & 0xFF) as u8;
    s_u16_t { a: [msb, lsb] }
}

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

// TODO: Reduce sizes.
pub fn make_pattern_request_with_default_device(pattern_index: u8) -> [u8; LARGE_SYSEX_SIZE] {
    let mut buffer = [0; LARGE_SYSEX_SIZE];
    unsafe {
        ar_pattern_request((&mut buffer[..]).as_mut_ptr(), 0, pattern_index);
    }
    buffer
}

pub fn make_pattern_request_with_default_device_x(pattern_index: u8) -> [u8; LARGE_SYSEX_SIZE] {
    let mut buffer = [0; LARGE_SYSEX_SIZE];
    unsafe {
        ar_pattern_request_x((&mut buffer[..]).as_mut_ptr(), 0, pattern_index);
    }
    buffer
}

pub fn make_pattern_request(pattern_index: u8, device_id: u8) -> [u8; LARGE_SYSEX_SIZE] {
    let mut buffer = [0; LARGE_SYSEX_SIZE];
    unsafe {
        ar_pattern_request((&mut buffer[..]).as_mut_ptr(), device_id, pattern_index);
    }
    buffer
}

pub fn make_pattern_request_x(pattern_index: u8, device_id: u8) -> [u8; LARGE_SYSEX_SIZE] {
    let mut buffer = [0; LARGE_SYSEX_SIZE];
    unsafe {
        ar_pattern_request_x((&mut buffer[..]).as_mut_ptr(), device_id, pattern_index);
    }
    buffer
}

pub fn clip_usize_in_range(value: usize, min: usize, max: usize) -> usize {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn clip_f64_in_range(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SysexType {
    Pattern,
    Kit,
    Sound,
    Song,
    Settings,
    Global,
}

use rytm_sys::{
    ar_sysex_id_t_AR_TYPE_GLOBAL, ar_sysex_id_t_AR_TYPE_KIT, ar_sysex_id_t_AR_TYPE_PATTERN,
    ar_sysex_id_t_AR_TYPE_SETTINGS, ar_sysex_id_t_AR_TYPE_SONG, ar_sysex_id_t_AR_TYPE_SOUND,
};

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

#[derive(Clone, Copy, Debug, Default)]
pub struct SysexMeta {
    pub container_version: u16,
    pub dev_id: u8,
    pub obj_type: u8,
    pub obj_nr: u16,
    pub chksum: u16,
    pub data_size: u16,
}

impl SysexMeta {
    pub fn default_for_pattern(object_index: usize, dev_id: Option<usize>) -> Self {
        let object_index = clip_usize_in_range(object_index, 0, 127);
        Self {
            container_version: 0x0101,
            dev_id: dev_id.unwrap_or(0) as u8,
            obj_type: SysexType::Pattern.into(),
            obj_nr: object_index as u16,
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
