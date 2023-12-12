// TODO: Correctly represent unset values in pattern, trig, track.

// pub mod plock_seq;
pub mod parameter_lock;
pub mod track;
pub mod types;

use self::{
    parameter_lock::ParameterLockPool,
    types::{Speed, TimeMode},
};
use crate::{
    error::{ParameterError, RytmError, SysexConversionError},
    impl_sysex_compatible,
    object::pattern::track::Track,
    sysex::{SysexCompatible, SysexMeta, SysexType, PATTERN_SYSEX_SIZE},
    util::{from_s_u16_t, to_s_u16_t_union_b},
};
use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::{ar_pattern_raw_to_syx, ar_pattern_t, ar_pattern_track_t, ar_sysex_meta_t};
use std::{cell::RefCell, rc::Rc};
pub use track::{
    trig::{types::*, Trig},
    types::*,
};

impl_sysex_compatible!(
    Pattern,
    ar_pattern_t,
    ar_pattern_raw_to_syx,
    SysexType::Pattern,
    PATTERN_SYSEX_SIZE
);

/// # Pattern
///
/// This structure represents a pattern in the analog rytm.
///
/// It does not map identically to the structure in the firmware.
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct Pattern {
    #[derivative(Debug = "ignore")]
    sysex_meta: SysexMeta,
    /// Index of this pattern.
    ///
    /// Range `0..=127` or 0 for the pattern at work buffer.
    index: usize,
    /// Version of the pattern structure.
    version: u32,
    /// Tracks
    ///
    /// 13 tracks of analog rytm.
    ///
    /// Fx track is the 13th track.
    tracks: [Track; 13],

    #[derivative(Debug = "ignore")]
    pub parameter_lock_pool: Rc<RefCell<ParameterLockPool>>,

    /// Master Length
    ///
    /// Range `1..=1024`
    ///
    /// - `1` = `Infinite`
    /// - `2` = `2`
    ///
    /// and onwards.
    master_length: u16,
    /// Master Change
    ///
    /// Range `1..=1024`
    ///
    /// - `1` = `OFF`
    /// - `2` = `2`
    ///
    /// and onwards.
    master_change: u16,
    /// Pattern Kit Number
    ///
    /// Range `0..=127`
    kit_number: u8,
    /// Pattern Swing Amount
    ///
    /// Range `0..=30`
    ///
    /// - `0` = 50%
    /// - `30` = 80%
    swing_amount: u8,
    /// Pattern Time Mode
    ///
    /// - Normal = `0`
    /// - Advanced = `1`
    time_mode: TimeMode,
    /// Pattern Speed
    ///
    /// - 1x
    /// - 2x
    /// - 3/2x
    /// - 3/4x
    /// - 1/2x
    /// - 1/4x
    /// - 1/8x
    speed: Speed,
    /// Pattern Global Quantize
    ///
    /// Range `0..=127`
    global_quantize: u8,
    /// Pattern BPM
    ///
    /// Range `30.0..=300.0`
    bpm: f32,

    #[derivative(Debug = "ignore")]
    /// Always 0x01, probably a marker for the end of pattern.
    pub(crate) _unknown_0x332c: u8,
}

impl From<&Pattern> for ar_pattern_t {
    fn from(pattern: &Pattern) -> Self {
        let mut tracks: [ar_pattern_track_t; 13] = [ar_pattern_track_t::default(); 13];

        for (i, track) in pattern.tracks.iter().enumerate() {
            tracks[i] = track.into();
        }

        let mut header = [0; 4];
        header[0] = (pattern.version >> 24) as u8;
        header[1] = (pattern.version >> 16) as u8;
        header[2] = (pattern.version >> 8) as u8;
        header[3] = pattern.version as u8;

        let bpm = (pattern.bpm * 120.0) as u16;
        Self {
            magic: header,
            tracks,
            plock_seqs: pattern.parameter_lock_pool.borrow().as_raw(),
            master_length: to_s_u16_t_union_b(pattern.master_length),
            master_chg_msb: (pattern.master_change >> 8) as u8,
            master_chg_lsb: pattern.master_change as u8,
            kit_number: pattern.kit_number,
            swing_amount: pattern.swing_amount,
            time_mode: pattern.time_mode.into(),
            master_speed: pattern.speed.into(),
            global_quantize: pattern.global_quantize,
            bpm_msb: (bpm >> 8) as u8,
            bpm_lsb: bpm as u8,
            __unknown332C: pattern._unknown_0x332c,
        }
    }
}

impl Pattern {
    #[allow(clippy::too_many_arguments)]
    pub fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_pattern: &ar_pattern_t,
    ) -> Result<Self, RytmError> {
        let index = if sysex_meta.is_targeting_work_buffer() {
            0
        } else {
            sysex_meta.obj_nr as usize
        };

        let mut tracks: [Track; 13] = [
            Track::try_default(0).unwrap(),
            Track::try_default(1).unwrap(),
            Track::try_default(2).unwrap(),
            Track::try_default(3).unwrap(),
            Track::try_default(4).unwrap(),
            Track::try_default(5).unwrap(),
            Track::try_default(6).unwrap(),
            Track::try_default(7).unwrap(),
            Track::try_default(8).unwrap(),
            Track::try_default(9).unwrap(),
            Track::try_default(10).unwrap(),
            Track::try_default(11).unwrap(),
            // Fx Track
            Track::try_default(12).unwrap(),
        ];

        let parameter_lock_pool = Rc::new(RefCell::new(ParameterLockPool::from_raw(
            raw_pattern.plock_seqs,
        )));

        for (i, track) in raw_pattern.tracks.iter().enumerate() {
            let parameter_lock_pool_ref = Rc::clone(&parameter_lock_pool);
            tracks[i] = Track::try_from_raw(i, track, parameter_lock_pool_ref)?;
        }

        let version = ((raw_pattern.magic[0] as u32) << 24)
            | ((raw_pattern.magic[1] as u32) << 16)
            | ((raw_pattern.magic[2] as u32) << 8)
            | (raw_pattern.magic[3] as u32);

        let bpm = ((raw_pattern.bpm_msb as u16) << 8) | (raw_pattern.bpm_lsb as u16);
        let bpm = bpm as f32 / 120.0;

        let master_change =
            ((raw_pattern.master_chg_msb as u16) << 8) | (raw_pattern.master_chg_lsb as u16);

        Ok(Self {
            index,
            sysex_meta,
            version,
            tracks,
            parameter_lock_pool,
            master_length: unsafe { from_s_u16_t(&raw_pattern.master_length) },
            master_change,
            kit_number: raw_pattern.kit_number,
            swing_amount: raw_pattern.swing_amount,
            time_mode: raw_pattern.time_mode.try_into()?,
            speed: raw_pattern.master_speed.try_into()?,
            global_quantize: raw_pattern.global_quantize,
            bpm,
            _unknown_0x332c: raw_pattern.__unknown332C,
        })
    }

    pub(crate) fn as_raw_parts(&self) -> (SysexMeta, ar_pattern_t) {
        (self.sysex_meta, self.into())
    }

    #[parameter_range(range = "index:0..=127")]
    pub fn try_default(index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            sysex_meta: SysexMeta::try_default_for_pattern(index, None)?,
            index,
            version: 5,
            tracks: [
                Track::try_default(0).unwrap(),
                Track::try_default(1).unwrap(),
                Track::try_default(2).unwrap(),
                Track::try_default(3).unwrap(),
                Track::try_default(4).unwrap(),
                Track::try_default(5).unwrap(),
                Track::try_default(6).unwrap(),
                Track::try_default(7).unwrap(),
                Track::try_default(8).unwrap(),
                Track::try_default(9).unwrap(),
                Track::try_default(10).unwrap(),
                Track::try_default(11).unwrap(),
                // Fx Track
                Track::try_default(12).unwrap(),
            ],
            parameter_lock_pool: Rc::new(RefCell::new(ParameterLockPool::default())),
            master_length: 16,
            master_change: 1,
            kit_number: 0,
            swing_amount: 0,
            time_mode: TimeMode::Normal,
            speed: Speed::default(),
            global_quantize: 0,
            bpm: 120.0,
            _unknown_0x332c: 0x01,
        })
    }

    pub fn work_buffer_default() -> Self {
        Self {
            sysex_meta: SysexMeta::default_for_pattern_in_work_buffer(None),
            index: 0,
            version: 5,
            tracks: [
                Track::try_default(0).unwrap(),
                Track::try_default(1).unwrap(),
                Track::try_default(2).unwrap(),
                Track::try_default(3).unwrap(),
                Track::try_default(4).unwrap(),
                Track::try_default(5).unwrap(),
                Track::try_default(6).unwrap(),
                Track::try_default(7).unwrap(),
                Track::try_default(8).unwrap(),
                Track::try_default(9).unwrap(),
                Track::try_default(10).unwrap(),
                Track::try_default(11).unwrap(),
                // Fx Track
                Track::try_default(12).unwrap(),
            ],
            parameter_lock_pool: Rc::new(RefCell::new(ParameterLockPool::default())),
            master_length: 16,
            master_change: 1,
            kit_number: 0,
            swing_amount: 0,
            time_mode: TimeMode::Normal,
            speed: Speed::default(),
            global_quantize: 0,
            bpm: 120.0,
            _unknown_0x332c: 0x01,
        }
    }

    /// Returns a mutable reference to the tracks which this pattern contains.
    ///
    /// 13th element is the FX track.
    pub fn tracks_mut(&mut self) -> &mut [Track] {
        &mut self.tracks
    }

    /// Sets master length for the pattern.
    ///
    /// Range `1..=1024`
    ///
    /// - `1` = `Infinite`
    /// - `2` = `2`
    ///
    /// and onwards.
    #[parameter_range(range = "master_length:1..=1024")]
    pub fn set_master_length(&mut self, master_length: usize) -> Result<(), RytmError> {
        self.master_length = master_length as u16;
        Ok(())
    }

    /// Sets swing amount for the pattern.
    ///
    /// Range `50..=80`
    ///
    /// Range denotes percentage.
    #[parameter_range(range = "swing_amount:50..=80")]
    pub fn set_swing_amount(&mut self, swing_amount: usize) -> Result<(), RytmError> {
        // Internally, swing amount is stored as 0..=30
        self.swing_amount = (swing_amount - 50) as u8;
        Ok(())
    }

    /// Sets the speed for the pattern.
    ///
    /// Check [`Speed`] for options.
    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    /// Sets the global quantize for the pattern.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "global_quantize:0..=127")]
    pub fn set_global_quantize(&mut self, global_quantize: usize) -> Result<(), RytmError> {
        self.global_quantize = global_quantize as u8;
        Ok(())
    }

    /// Sets the kit number for the pattern.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "kit_number:0..=127")]
    pub fn set_kit_number(&mut self, kit_number: usize) -> Result<(), RytmError> {
        self.kit_number = kit_number as u8;
        Ok(())
    }

    /// Sets the time mode for the pattern.
    ///
    /// Check [`TimeMode`] for options.
    pub fn set_time_mode(&mut self, time_mode: TimeMode) {
        self.time_mode = time_mode;
    }

    /// Sets the master change for the pattern.
    ///
    /// Range `1..=1024`
    ///
    /// - `1` = `OFF`
    /// - `2` = `2`
    ///
    /// and onwards.
    #[parameter_range(range = "master_change:1..=1024")]
    pub fn set_master_change(&mut self, master_change: usize) -> Result<(), RytmError> {
        self.master_change = master_change as u16;
        Ok(())
    }

    /// Sets the BPM for the pattern.
    ///
    /// Range `30.0..=300.0`
    ///
    /// This is only effective when pattern level bpm is enabled.
    #[parameter_range(range = "bpm:30.0..=300.0")]
    pub fn set_bpm(&mut self, bpm: f32) -> Result<(), RytmError> {
        self.bpm = bpm;
        Ok(())
    }

    /// Returns a reference to the tracks which this pattern contains.
    ///
    /// 13th element is the FX track.
    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    /// Returns the master length for the pattern.
    ///
    /// Range `1..=1024`
    ///
    /// - `1` = `Infinite`
    /// - `2` = `2`
    ///
    /// and onwards.
    pub fn master_length(&self) -> usize {
        self.master_length as usize
    }

    /// Returns the swing amount for the pattern.
    ///
    /// Range `50..=80`
    ///
    /// Range denotes percentage.
    pub fn swing_amount(&self) -> usize {
        // Internally, swing amount is stored as 0..=30
        self.swing_amount as usize + 50
    }

    /// Returns the speed for the pattern.
    ///
    /// Check [`Speed`] for options.
    pub fn speed(&self) -> Speed {
        self.speed
    }

    /// Returns the global quantize for the pattern.
    ///
    /// Range `0..=127`
    pub fn global_quantize(&self) -> usize {
        self.global_quantize as usize
    }

    /// Returns the kit number for the pattern.
    ///
    /// Range `0..=127`
    pub fn kit_number(&self) -> usize {
        self.kit_number as usize
    }

    /// Returns the time mode for the pattern.
    ///
    /// Check [`TimeMode`] for options.
    pub fn time_mode(&self) -> TimeMode {
        self.time_mode
    }

    /// Returns the master change for the pattern.
    ///
    /// Range `1..=1024`
    ///
    /// - `1` = `OFF`
    /// - `2` = `2`
    ///
    /// and onwards.
    pub fn master_change(&self) -> usize {
        self.master_change as usize
    }

    /// Returns the BPM for the pattern.
    ///
    /// Range `30.0..=300.0`
    ///
    /// This is only effective when pattern level bpm is enabled.
    pub fn bpm(&self) -> f32 {
        self.bpm
    }

    /// Returns the index of the pattern.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Checks if this pattern is the pattern at work buffer.
    pub fn is_work_buffer_pattern(&self) -> bool {
        self.sysex_meta.is_targeting_work_buffer()
    }

    /// Returns the version of the pattern structure.
    pub fn structure_version(&self) -> u32 {
        self.version
    }
}
