// All casts in this file are intended or safe within the context of this library.
//
// One can change `allow` to `warn` to review them if necessary.
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

// TODO: Check if we can get info about if this kit is assigned to a pattern.
// TODO: Add control mod in parts once the pr merge to libanalogrytm is done.

/// Holds the structure to represent compressor fx parameters.
pub mod comp;
/// Holds the structure to represent delay fx parameters.
pub mod delay;
/// Holds the structure to represent distortion fx parameters.
pub mod dist;
/// Holds the structure to represent lfo fx parameters.
pub mod lfo;
/// Holds the structure to represent retrig settings scoped to a track.
pub mod retrig;
/// Holds the structure to represent reverb fx parameters.
pub mod reverb;
/// Holds types relevant to the kit object.
pub mod types;
pub(crate) mod unknown;

use std::sync::{Arc, Mutex};

use self::types::ControlInModTarget;
use self::{
    comp::FxCompressor, delay::FxDelay, dist::FxDistortion, lfo::FxLfo, reverb::FxReverb,
    unknown::KitUnknown,
};
use crate::defaults::{default_perf_ctl_array, default_scene_ctl_array};
use crate::util::{assemble_u32_from_u8_array, break_u32_into_u8_array};
use crate::AnySysexType;
use crate::{
    error::{ParameterError, RytmError, SysexConversionError},
    impl_sysex_compatible,
    object::types::ObjectName,
    sysex::{SysexCompatible, SysexMeta, SysexType, KIT_SYSEX_SIZE},
    util::to_s_u16_t_union_b_from_u8_as_msb,
    Sound,
};
use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::{ar_kit_raw_to_syx, ar_kit_t, ar_sysex_meta_t};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::pattern::plock::ParameterLockPool;

impl_sysex_compatible!(
    Kit,
    ar_kit_t,
    ar_kit_raw_to_syx,
    SysexType::Kit,
    KIT_SYSEX_SIZE
);

/// Represents a kit in the analog rytm.
///
/// It does not map identically to the structure in the firmware.
#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct Kit {
    #[derivative(Debug = "ignore")]
    sysex_meta: SysexMeta,
    /// Version of the kit structure.
    version: u32,
    pub(crate) index: usize,

    /// Name of the kit.
    name: ObjectName,

    // 13th is the fx track.
    #[derivative(Debug = "ignore")]
    track_levels: [u8; 13],
    #[derivative(Debug = "ignore")]
    track_retrig_settings: [retrig::TrackRetrigMenu; 13],
    #[derivative(Debug = "ignore")]
    sounds: [Sound; 12],

    fx_delay: FxDelay,
    fx_distortion: FxDistortion,
    fx_reverb: FxReverb,
    fx_compressor: FxCompressor,
    fx_lfo: FxLfo,

    control_in_1_mod_target_1: ControlInModTarget,
    control_in_1_mod_target_2: ControlInModTarget,
    control_in_1_mod_target_3: ControlInModTarget,
    control_in_1_mod_target_4: ControlInModTarget,

    control_in_2_mod_target_1: ControlInModTarget,
    control_in_2_mod_target_2: ControlInModTarget,
    control_in_2_mod_target_3: ControlInModTarget,
    control_in_2_mod_target_4: ControlInModTarget,

    control_in_1_mod_amt_1: i8,
    control_in_1_mod_amt_2: i8,
    control_in_1_mod_amt_3: i8,
    control_in_1_mod_amt_4: i8,

    control_in_2_mod_amt_1: i8,
    control_in_2_mod_amt_2: i8,
    control_in_2_mod_amt_3: i8,
    control_in_2_mod_amt_4: i8,

    // Currently these are out of my interest.
    // Maybe in the feature we can add support for these.
    //
    // ---- TODO: ----
    #[derivative(Debug = "ignore")]
    #[serde(with = "BigArray")]
    pub(crate) perf_ctl: [u8; 48 * 4], /* @0x0842..0x0901 */
    #[derivative(Debug = "ignore")]
    #[serde(with = "BigArray")]
    pub(crate) scene_ctl: [u8; 48 * 4], /* @0x0917..0x09D6 */
    // 0..=11 device 0..=11
    #[derivative(Debug = "ignore")]
    pub(crate) current_scene_id: u8, /* @0x09D8 (0..11) */
    // ----------------
    //
    #[derivative(Debug = "ignore")]
    pub(crate) __unknown: KitUnknown,
}

impl From<&Kit> for ar_kit_t {
    fn from(kit: &Kit) -> Self {
        let mut raw_kit = Self {
            // Version
            __unknown_arr1: break_u32_into_u8_array(kit.version),
            name: kit.name.copy_inner(),
            perf_ctl: kit.perf_ctl,
            scene_ctl: kit.scene_ctl,
            current_scene_id: kit.current_scene_id,

            ctrl_in_mod_1_target_1: kit.control_in_1_mod_target_1.into(),
            ctrl_in_mod_1_target_2: kit.control_in_1_mod_target_2.into(),
            ctrl_in_mod_1_target_3: kit.control_in_1_mod_target_3.into(),
            ctrl_in_mod_1_target_4: kit.control_in_1_mod_target_4.into(),

            ctrl_in_mod_2_target_1: kit.control_in_2_mod_target_1.into(),
            ctrl_in_mod_2__target_2: kit.control_in_2_mod_target_2.into(),
            ctrl_in_mod_2_target_3: kit.control_in_2_mod_target_3.into(),
            ctrl_in_mod_2_target_4: kit.control_in_2_mod_target_4.into(),

            ctrl_in_mod_1_amt_1: kit.control_in_1_mod_amt_1 as u8,
            ctrl_in_mod_1_amt_2: kit.control_in_1_mod_amt_2 as u8,
            ctrl_in_mod_1_amt_3: kit.control_in_1_mod_amt_3 as u8,
            ctrl_in_mod_1_amt_4: kit.control_in_1_mod_amt_4 as u8,

            ctrl_in_mod_2_amt_1: kit.control_in_2_mod_amt_1 as u8,
            ctrl_in_mod_2_amt_2: kit.control_in_2_mod_amt_2 as u8,
            ctrl_in_mod_2_amt_3: kit.control_in_2_mod_amt_3 as u8,
            ctrl_in_mod_2_amt_4: kit.control_in_2_mod_amt_4 as u8,

            ..Default::default()
        };

        for (i, sound) in kit.sounds.iter().enumerate() {
            raw_kit.tracks[i] = sound.into();
        }

        for (i, track_level) in kit.track_levels.iter().enumerate() {
            // Only the high byte is used for the levels.
            raw_kit.track_levels[i] = to_s_u16_t_union_b_from_u8_as_msb(*track_level);
        }

        kit.fx_delay.apply_to_raw_kit(&mut raw_kit);
        kit.fx_distortion.apply_to_raw_kit(&mut raw_kit);
        kit.fx_reverb.apply_to_raw_kit(&mut raw_kit);
        kit.fx_compressor.apply_to_raw_kit(&mut raw_kit);
        kit.fx_lfo.apply_to_raw_kit(&mut raw_kit);

        for retrig_settings in &kit.track_retrig_settings {
            retrig_settings.apply_to_raw_kit(&mut raw_kit);
        }

        kit.__unknown.apply_to_raw_kit(&mut raw_kit);

        raw_kit
    }
}

impl Kit {
    pub(crate) fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_kit: &ar_kit_t,
    ) -> Result<Self, RytmError> {
        let kit_number = sysex_meta.get_normalized_object_index();

        let name = ObjectName::from_u8_array(raw_kit.name);

        let mut sounds = [
            Sound::try_kit_default(0, kit_number, sysex_meta)?,
            Sound::try_kit_default(1, kit_number, sysex_meta)?,
            Sound::try_kit_default(2, kit_number, sysex_meta)?,
            Sound::try_kit_default(3, kit_number, sysex_meta)?,
            Sound::try_kit_default(4, kit_number, sysex_meta)?,
            Sound::try_kit_default(5, kit_number, sysex_meta)?,
            Sound::try_kit_default(6, kit_number, sysex_meta)?,
            Sound::try_kit_default(7, kit_number, sysex_meta)?,
            Sound::try_kit_default(8, kit_number, sysex_meta)?,
            Sound::try_kit_default(9, kit_number, sysex_meta)?,
            Sound::try_kit_default(10, kit_number, sysex_meta)?,
            Sound::try_kit_default(11, kit_number, sysex_meta)?,
        ];

        for (i, sound) in raw_kit.tracks.iter().enumerate() {
            sounds[i] = Sound::try_from_raw(sysex_meta, sound, Some((kit_number, i)))?;
        }

        let mut track_levels = [0; 13];
        for (i, track_level) in raw_kit.track_levels.iter().enumerate() {
            // Only the high byte is used for the levels.
            track_levels[i] = unsafe { track_level.b.hi };
        }

        Ok(Self {
            index: kit_number,
            sysex_meta,
            version: assemble_u32_from_u8_array(&raw_kit.__unknown_arr1),

            name,

            track_levels,
            track_retrig_settings: retrig::TrackRetrigMenu::get_default_for_13_tracks(),
            sounds,

            fx_delay: raw_kit.try_into()?,
            fx_distortion: raw_kit.try_into()?,
            fx_reverb: raw_kit.try_into()?,
            fx_compressor: raw_kit.try_into()?,
            fx_lfo: raw_kit.try_into()?,

            perf_ctl: raw_kit.perf_ctl,
            scene_ctl: raw_kit.scene_ctl,
            current_scene_id: raw_kit.current_scene_id,

            control_in_1_mod_target_1: raw_kit.ctrl_in_mod_1_target_1.try_into()?,
            control_in_1_mod_target_2: raw_kit.ctrl_in_mod_1_target_2.try_into()?,
            control_in_1_mod_target_3: raw_kit.ctrl_in_mod_1_target_3.try_into()?,
            control_in_1_mod_target_4: raw_kit.ctrl_in_mod_1_target_4.try_into()?,

            control_in_2_mod_target_1: raw_kit.ctrl_in_mod_2_target_1.try_into()?,
            control_in_2_mod_target_2: raw_kit.ctrl_in_mod_2__target_2.try_into()?,
            control_in_2_mod_target_3: raw_kit.ctrl_in_mod_2_target_3.try_into()?,
            control_in_2_mod_target_4: raw_kit.ctrl_in_mod_2_target_4.try_into()?,

            control_in_1_mod_amt_1: raw_kit.ctrl_in_mod_1_amt_1 as i8,
            control_in_1_mod_amt_2: raw_kit.ctrl_in_mod_1_amt_2 as i8,
            control_in_1_mod_amt_3: raw_kit.ctrl_in_mod_1_amt_3 as i8,
            control_in_1_mod_amt_4: raw_kit.ctrl_in_mod_1_amt_4 as i8,

            control_in_2_mod_amt_1: raw_kit.ctrl_in_mod_2_amt_1 as i8,
            control_in_2_mod_amt_2: raw_kit.ctrl_in_mod_2_amt_2 as i8,
            control_in_2_mod_amt_3: raw_kit.ctrl_in_mod_2_amt_3 as i8,
            control_in_2_mod_amt_4: raw_kit.ctrl_in_mod_2_amt_4 as i8,

            __unknown: raw_kit.into(),
        })
    }

    pub(crate) fn as_raw_parts(&self) -> (SysexMeta, ar_kit_t) {
        (self.sysex_meta, self.into())
    }

    /// Makes a new kit with the given index complying to project defaults.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "kit_index:0..=127")]
    pub fn try_default(kit_index: usize) -> Result<Self, RytmError> {
        //    sU8 ctrl_in_mod_1_amt_1;    /* @0x0A12 (-128..127) */
        //    sU8 ctrl_in_mod_1_target_1; /* @0x0A14 See sound.h, same as AR_SOUND_MOD_DEST_XXX, AR_SOUND_MOD_DEST_SYN_X variants can not be used. */
        //    sU8 ctrl_in_mod_1_amt_2;    /* @0x0A15 (-128..127) */
        //    sU8 ctrl_in_mod_1_target_2; /* @0x0A17 See sound.h, same as AR_SOUND_MOD_DEST_XXX, AR_SOUND_MOD_DEST_SYN_X variants can not be used. */
        //    sU8 ctrl_in_mod_1_amt_3;    /* @0x0A18 (-128..127) */
        //    sU8 ctrl_in_mod_1_target_3; /* @0x0A1A See sound.h, same as AR_SOUND_MOD_DEST_XXX, AR_SOUND_MOD_DEST_SYN_X variants can not be used. */
        //    sU8 ctrl_in_mod_1_amt_4;    /* @0x0A1B (-128..127) */
        //    sU8 ctrl_in_mod_1_target_4; /* @0x0A1D See sound.h, same as AR_SOUND_MOD_DEST_XXX, AR_SOUND_MOD_DEST_SYN_X variants can not be used. */
        //    sU8 ctrl_in_mod_2_amt_1;     /* @0x0A22 (-128..127) */
        //    sU8 ctrl_in_mod_2_target_1;  /* @0x0A24 See sound.h, same as AR_SOUND_MOD_DEST_XXX, AR_SOUND_MOD_DEST_SYN_X variants can not be used. */
        //    sU8 ctrl_in_mod_2_amt_2;     /* @0x0A25 (-128..127) */
        //    sU8 ctrl_in_mod_2__target_2; /* @0x0A27 See sound.h, same as AR_SOUND_MOD_DEST_XXX, AR_SOUND_MOD_DEST_SYN_X variants can not be used. */
        //    sU8 ctrl_in_mod_2_amt_3;     /* @0x0A28 (-128..127) */
        //    sU8 ctrl_in_mod_2_target_3;  /* @0x0A2A See sound.h, same as AR_SOUND_MOD_DEST_XXX, AR_SOUND_MOD_DEST_SYN_X variants can not be used. */
        //    sU8 ctrl_in_mod_2_amt_4;     /* @0x0A2B (-128..127) */
        //    sU8 ctrl_in_mod_2_target_4;  /* @0x0A2D See sound.h, same as AR_SOUND_MOD_DEST_XXX, AR_SOUND_MOD_DEST_SYN_X variants can not be used. */
        let meta = SysexMeta::try_default_for_kit(kit_index, None)?;
        Ok(Self {
            index: kit_index,
            sysex_meta: meta,
            version: 6,

            name: format!("KIT {kit_index}").try_into()?,

            track_levels: [100; 13],
            track_retrig_settings: retrig::TrackRetrigMenu::get_default_for_13_tracks(),

            sounds: [
                Sound::try_kit_default(0, kit_index, meta)?,
                Sound::try_kit_default(1, kit_index, meta)?,
                Sound::try_kit_default(2, kit_index, meta)?,
                Sound::try_kit_default(3, kit_index, meta)?,
                Sound::try_kit_default(4, kit_index, meta)?,
                Sound::try_kit_default(5, kit_index, meta)?,
                Sound::try_kit_default(6, kit_index, meta)?,
                Sound::try_kit_default(7, kit_index, meta)?,
                Sound::try_kit_default(8, kit_index, meta)?,
                Sound::try_kit_default(9, kit_index, meta)?,
                Sound::try_kit_default(10, kit_index, meta)?,
                Sound::try_kit_default(11, kit_index, meta)?,
            ],

            fx_delay: FxDelay::default(),
            fx_distortion: FxDistortion::default(),
            fx_reverb: FxReverb::default(),
            fx_compressor: FxCompressor::default(),
            fx_lfo: FxLfo::default(),

            perf_ctl: default_perf_ctl_array(),
            scene_ctl: default_scene_ctl_array(),
            current_scene_id: 0,

            control_in_1_mod_target_1: ControlInModTarget::default(),
            control_in_1_mod_target_2: ControlInModTarget::default(),
            control_in_1_mod_target_3: ControlInModTarget::default(),
            control_in_1_mod_target_4: ControlInModTarget::default(),

            control_in_2_mod_target_1: ControlInModTarget::default(),
            control_in_2_mod_target_2: ControlInModTarget::default(),
            control_in_2_mod_target_3: ControlInModTarget::default(),
            control_in_2_mod_target_4: ControlInModTarget::default(),

            control_in_1_mod_amt_1: 0,
            control_in_1_mod_amt_2: 0,
            control_in_1_mod_amt_3: 0,
            control_in_1_mod_amt_4: 0,

            control_in_2_mod_amt_1: 0,
            control_in_2_mod_amt_2: 0,
            control_in_2_mod_amt_3: 0,
            control_in_2_mod_amt_4: 0,

            __unknown: KitUnknown::default(),
        })
    }

    /// Makes a new kit in the work buffer complying to project defaults as if it comes from the work buffer.
    #[allow(clippy::missing_panics_doc)]
    pub fn work_buffer_default() -> Self {
        Self {
            index: 0,
            sysex_meta: SysexMeta::default_for_kit_in_work_buffer(None),
            version: 6,

            name: "WB_KIT".try_into().unwrap(),

            track_levels: [100; 13],
            track_retrig_settings: retrig::TrackRetrigMenu::get_default_for_13_tracks(),

            // TODO: I don't know if we choose wb defaults or kit defaults for sounds here..
            sounds: [
                Sound::try_work_buffer_default(0).unwrap(),
                Sound::try_work_buffer_default(1).unwrap(),
                Sound::try_work_buffer_default(2).unwrap(),
                Sound::try_work_buffer_default(3).unwrap(),
                Sound::try_work_buffer_default(4).unwrap(),
                Sound::try_work_buffer_default(5).unwrap(),
                Sound::try_work_buffer_default(6).unwrap(),
                Sound::try_work_buffer_default(7).unwrap(),
                Sound::try_work_buffer_default(8).unwrap(),
                Sound::try_work_buffer_default(9).unwrap(),
                Sound::try_work_buffer_default(10).unwrap(),
                Sound::try_work_buffer_default(11).unwrap(),
            ],

            fx_delay: FxDelay::default(),
            fx_distortion: FxDistortion::default(),
            fx_reverb: FxReverb::default(),
            fx_compressor: FxCompressor::default(),
            fx_lfo: FxLfo::default(),

            perf_ctl: default_perf_ctl_array(),
            scene_ctl: default_scene_ctl_array(),
            current_scene_id: 0,

            control_in_1_mod_target_1: ControlInModTarget::default(),
            control_in_1_mod_target_2: ControlInModTarget::default(),
            control_in_1_mod_target_3: ControlInModTarget::default(),
            control_in_1_mod_target_4: ControlInModTarget::default(),

            control_in_2_mod_target_1: ControlInModTarget::default(),
            control_in_2_mod_target_2: ControlInModTarget::default(),
            control_in_2_mod_target_3: ControlInModTarget::default(),
            control_in_2_mod_target_4: ControlInModTarget::default(),

            control_in_1_mod_amt_1: 0,
            control_in_1_mod_amt_2: 0,
            control_in_1_mod_amt_3: 0,
            control_in_1_mod_amt_4: 0,

            control_in_2_mod_amt_1: 0,
            control_in_2_mod_amt_2: 0,
            control_in_2_mod_amt_3: 0,
            control_in_2_mod_amt_4: 0,

            __unknown: KitUnknown::default(),
        }
    }

    /// Sets the name of the kit.
    ///
    /// # Errors
    ///
    /// The name must be ASCII and have a length of 15 characters or less. Other cases will result in an error.
    pub fn set_name(&mut self, name: &str) -> Result<(), RytmError> {
        self.name = name.try_into()?;
        Ok(())
    }

    /// Returns the name of the kit.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the sounds assigned to the kit in the order of the tracks.
    pub const fn sounds(&self) -> &[Sound; 12] {
        &self.sounds
    }

    /// Returns the sounds assigned to the kit in the order of the tracks mutably.
    pub fn sounds_mut(&mut self) -> &mut [Sound; 12] {
        &mut self.sounds
    }

    /// Sets the level of a track.
    ///
    /// Range `0..=12`
    ///
    /// 12th track is the fx track.
    #[parameter_range(range = "track_index:0..=12", range = "level:0..=127")]
    pub fn set_track_level(&mut self, track_index: usize, level: usize) -> Result<(), RytmError> {
        self.track_levels[track_index] = level as u8;
        Ok(())
    }

    /// Sets the level of all tracks including the Fx track.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "level:0..=127")]
    pub fn set_all_track_levels(&mut self, level: usize) -> Result<(), RytmError> {
        for track_level in &mut self.track_levels {
            *track_level = level as u8;
        }
        Ok(())
    }

    /// Sets the level of a range of tracks.
    ///
    /// 12th track is the fx track.
    ///
    /// Maximum range `0..=12`
    ///
    /// Level range `0..=127`
    #[parameter_range(range = "level:0..=127")]
    pub fn set_a_range_of_track_levels(
        &mut self,
        range: std::ops::Range<usize>,
        level: usize,
    ) -> Result<(), RytmError> {
        if range.end > 12 {
            return Err(RytmError::Parameter(ParameterError::Range {
                value: format!("{range:?}"),
                parameter_name: "range".to_string(),
            }));
        }

        for track_index in range {
            self.set_track_level(track_index, level)?;
        }

        Ok(())
    }

    /// Gets the level of a track.
    ///
    /// Range `0..=12`
    #[parameter_range(range = "track_index:0..=12")]
    pub fn track_level(&self, track_index: usize) -> Result<usize, RytmError> {
        Ok(self.track_levels[track_index] as usize)
    }

    /// Gets the level of all tracks including the Fx track.
    ///
    /// Range `0..=127`
    pub fn track_levels(&self) -> Vec<usize> {
        self.track_levels
            .iter()
            .map(|&l| l as usize)
            .collect::<Vec<_>>()
    }

    /// Gets the level of a range of tracks.
    ///
    /// 12th track is the fx track.
    ///
    /// Maximum range `0..=12`
    ///
    /// Level range `0..=127`
    ///
    /// # Errors
    ///
    /// Returns an error if the range is out of bounds.
    pub fn range_of_track_levels(
        &self,
        range: std::ops::Range<usize>,
    ) -> Result<Vec<usize>, RytmError> {
        let mut levels = Vec::new();
        for track_index in range {
            levels.push(self.track_level(track_index)?);
        }
        Ok(levels)
    }

    /// Returns the version of the kit structure.
    pub const fn structure_version(&self) -> u32 {
        self.version
    }

    /// Gets the retrig menu of a track
    ///
    /// 12th track is the fx track.
    ///
    /// Range `0..=12`
    #[parameter_range(range = "track_index:0..=12")]
    pub fn track_retrig_settings(
        &self,
        track_index: usize,
    ) -> Result<&retrig::TrackRetrigMenu, RytmError> {
        Ok(&self.track_retrig_settings[track_index])
    }

    /// Gets the retrig menu of a track mutably
    ///
    /// 12th track is the fx track.
    ///
    /// Range `0..=12`
    #[parameter_range(range = "track_index:0..=12")]
    pub fn track_retrig_settings_mut(
        &mut self,
        track_index: usize,
    ) -> Result<&mut retrig::TrackRetrigMenu, RytmError> {
        Ok(&mut self.track_retrig_settings[track_index])
    }

    /// Gets the fx delay parameters.
    pub const fn fx_delay(&self) -> &FxDelay {
        &self.fx_delay
    }

    /// Gets the fx delay parameters mutably.
    pub fn fx_delay_mut(&mut self) -> &mut FxDelay {
        &mut self.fx_delay
    }

    /// Gets the fx distortion parameters.
    pub const fn fx_distortion(&self) -> &FxDistortion {
        &self.fx_distortion
    }

    /// Gets the fx distortion parameters mutably.
    pub fn fx_distortion_mut(&mut self) -> &mut FxDistortion {
        &mut self.fx_distortion
    }

    /// Gets the fx reverb parameters.
    pub const fn fx_reverb(&self) -> &FxReverb {
        &self.fx_reverb
    }

    /// Gets the fx reverb parameters mutably.
    pub fn fx_reverb_mut(&mut self) -> &mut FxReverb {
        &mut self.fx_reverb
    }

    /// Gets the fx compressor parameters.
    pub const fn fx_compressor(&self) -> &FxCompressor {
        &self.fx_compressor
    }

    /// Gets the fx compressor parameters mutably.
    pub fn fx_compressor_mut(&mut self) -> &mut FxCompressor {
        &mut self.fx_compressor
    }

    /// Gets the fx lfo parameters.
    pub const fn fx_lfo(&self) -> &FxLfo {
        &self.fx_lfo
    }

    /// Gets the fx lfo parameters mutably.
    pub fn fx_lfo_mut(&mut self) -> &mut FxLfo {
        &mut self.fx_lfo
    }

    /// Returns the index of the kit.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Sets the control in 1 mod target 1
    pub fn set_control_in_1_mod_target_1(&mut self, control_in_1_mod_target_1: ControlInModTarget) {
        self.control_in_1_mod_target_1 = control_in_1_mod_target_1;
    }

    /// Sets the control in 1 mod target 2
    pub fn set_control_in_1_mod_target_2(&mut self, control_in_1_mod_target_2: ControlInModTarget) {
        self.control_in_1_mod_target_2 = control_in_1_mod_target_2;
    }

    /// Sets the control in 1 mod target 3
    pub fn set_control_in_1_mod_target_3(&mut self, control_in_1_mod_target_3: ControlInModTarget) {
        self.control_in_1_mod_target_3 = control_in_1_mod_target_3;
    }

    /// Sets the control in 1 mod target 4
    pub fn set_control_in_1_mod_target_4(&mut self, control_in_1_mod_target_4: ControlInModTarget) {
        self.control_in_1_mod_target_4 = control_in_1_mod_target_4;
    }

    /// Sets the control in 2 mod target 1
    pub fn set_control_in_2_mod_target_1(&mut self, control_in_2_mod_target_1: ControlInModTarget) {
        self.control_in_2_mod_target_1 = control_in_2_mod_target_1;
    }

    /// Sets the control in 2 mod target 2
    pub fn set_control_in_2_mod_target_2(&mut self, control_in_2_mod_target_2: ControlInModTarget) {
        self.control_in_2_mod_target_2 = control_in_2_mod_target_2;
    }

    /// Sets the control in 2 mod target 3
    pub fn set_control_in_2_mod_target_3(&mut self, control_in_2_mod_target_3: ControlInModTarget) {
        self.control_in_2_mod_target_3 = control_in_2_mod_target_3;
    }

    /// Sets the control in 2 mod target 4
    pub fn set_control_in_2_mod_target_4(&mut self, control_in_2_mod_target_4: ControlInModTarget) {
        self.control_in_2_mod_target_4 = control_in_2_mod_target_4;
    }

    /// Sets the control in 1 mod amt 1
    ///
    /// Range: `-128..=127`
    #[parameter_range(range = "control_in_1_mod_amt_1:-128..=127")]
    pub fn set_control_in_1_mod_amt_1(
        &mut self,
        control_in_1_mod_amt_1: isize,
    ) -> Result<(), RytmError> {
        self.control_in_1_mod_amt_1 = control_in_1_mod_amt_1 as i8;
        Ok(())
    }

    /// Sets the control in 1 mod amt 2
    ///
    /// Range: `-128..=127`
    #[parameter_range(range = "control_in_1_mod_amt_2:-128..=127")]
    pub fn set_control_in_1_mod_amt_2(
        &mut self,
        control_in_1_mod_amt_2: isize,
    ) -> Result<(), RytmError> {
        self.control_in_1_mod_amt_2 = control_in_1_mod_amt_2 as i8;
        Ok(())
    }

    /// Sets the control in 1 mod amt 3
    ///
    /// Range: `-128..=127`
    #[parameter_range(range = "control_in_1_mod_amt_3:-128..=127")]
    pub fn set_control_in_1_mod_amt_3(
        &mut self,
        control_in_1_mod_amt_3: isize,
    ) -> Result<(), RytmError> {
        self.control_in_1_mod_amt_3 = control_in_1_mod_amt_3 as i8;
        Ok(())
    }

    /// Sets the control in 1 mod amt 4
    ///
    /// Range: `-128..=127`
    #[parameter_range(range = "control_in_1_mod_amt_4:-128..=127")]
    pub fn set_control_in_1_mod_amt_4(
        &mut self,
        control_in_1_mod_amt_4: isize,
    ) -> Result<(), RytmError> {
        self.control_in_1_mod_amt_4 = control_in_1_mod_amt_4 as i8;
        Ok(())
    }

    /// Sets the control in 2 mod amt 1
    ///
    /// Range: `-128..=127`
    #[parameter_range(range = "control_in_2_mod_amt_1:-128..=127")]
    pub fn set_control_in_2_mod_amt_1(
        &mut self,
        control_in_2_mod_amt_1: isize,
    ) -> Result<(), RytmError> {
        self.control_in_2_mod_amt_1 = control_in_2_mod_amt_1 as i8;
        Ok(())
    }

    /// Sets the control in 2 mod amt 2
    ///
    /// Range: `-128..=127`
    #[parameter_range(range = "control_in_2_mod_amt_2:-128..=127")]
    pub fn set_control_in_2_mod_amt_2(
        &mut self,
        control_in_2_mod_amt_2: isize,
    ) -> Result<(), RytmError> {
        self.control_in_2_mod_amt_2 = control_in_2_mod_amt_2 as i8;
        Ok(())
    }

    /// Sets the control in 2 mod amt 3
    ///
    /// Range: `-128..=127`
    #[parameter_range(range = "control_in_2_mod_amt_3:-128..=127")]
    pub fn set_control_in_2_mod_amt_3(
        &mut self,
        control_in_2_mod_amt_3: isize,
    ) -> Result<(), RytmError> {
        self.control_in_2_mod_amt_3 = control_in_2_mod_amt_3 as i8;
        Ok(())
    }

    /// Sets the control in 2 mod amt 4
    ///
    /// Range: `-128..=127`
    #[parameter_range(range = "control_in_2_mod_amt_4:-128..=127")]
    pub fn set_control_in_2_mod_amt_4(
        &mut self,
        control_in_2_mod_amt_4: isize,
    ) -> Result<(), RytmError> {
        self.control_in_2_mod_amt_4 = control_in_2_mod_amt_4 as i8;
        Ok(())
    }

    /// Gets the control in 1 mod target 1
    pub const fn control_in_1_mod_target_1(&self) -> ControlInModTarget {
        self.control_in_1_mod_target_1
    }

    /// Gets the control in 1 mod target 2
    pub const fn control_in_1_mod_target_2(&self) -> ControlInModTarget {
        self.control_in_1_mod_target_2
    }

    /// Gets the control in 1 mod target 3
    pub const fn control_in_1_mod_target_3(&self) -> ControlInModTarget {
        self.control_in_1_mod_target_3
    }

    /// Gets the control in 1 mod target 4
    pub const fn control_in_1_mod_target_4(&self) -> ControlInModTarget {
        self.control_in_1_mod_target_4
    }

    /// Gets the control in 2 mod target 1
    pub const fn control_in_2_mod_target_1(&self) -> ControlInModTarget {
        self.control_in_2_mod_target_1
    }

    /// Gets the control in 2 mod target 2
    pub const fn control_in_2_mod_target_2(&self) -> ControlInModTarget {
        self.control_in_2_mod_target_2
    }

    /// Gets the control in 2 mod target 3
    pub const fn control_in_2_mod_target_3(&self) -> ControlInModTarget {
        self.control_in_2_mod_target_3
    }

    /// Gets the control in 2 mod target 4
    pub const fn control_in_2_mod_target_4(&self) -> ControlInModTarget {
        self.control_in_2_mod_target_4
    }

    /// Gets the control in 1 mod amt 1
    ///
    /// Range: `-128..=127`
    pub const fn control_in_1_mod_amt_1(&self) -> isize {
        self.control_in_1_mod_amt_1 as isize
    }

    /// Gets the control in 1 mod amt 2
    ///
    /// Range: `-128..=127`
    pub const fn control_in_1_mod_amt_2(&self) -> isize {
        self.control_in_1_mod_amt_2 as isize
    }

    /// Gets the control in 1 mod amt 3
    ///
    /// Range: `-128..=127`
    pub const fn control_in_1_mod_amt_3(&self) -> isize {
        self.control_in_1_mod_amt_3 as isize
    }

    /// Gets the control in 1 mod amt 4
    ///
    /// Range: `-128..=127`
    pub const fn control_in_1_mod_amt_4(&self) -> isize {
        self.control_in_1_mod_amt_4 as isize
    }

    /// Gets the control in 2 mod amt 1
    ///
    /// Range: `-128..=127`
    pub const fn control_in_2_mod_amt_1(&self) -> isize {
        self.control_in_2_mod_amt_1 as isize
    }

    /// Gets the control in 2 mod amt 2
    ///
    /// Range: `-128..=127`
    pub const fn control_in_2_mod_amt_2(&self) -> isize {
        self.control_in_2_mod_amt_2 as isize
    }

    /// Gets the control in 2 mod amt 3
    ///
    /// Range: `-128..=127`
    pub const fn control_in_2_mod_amt_3(&self) -> isize {
        self.control_in_2_mod_amt_3 as isize
    }

    /// Gets the control in 2 mod amt 4
    ///
    /// Range: `-128..=127`
    pub const fn control_in_2_mod_amt_4(&self) -> isize {
        self.control_in_2_mod_amt_4 as isize
    }

    // TODO: Comment
    pub fn link_parameter_lock_pool(
        &mut self,
        parameter_lock_pool: &Arc<Mutex<ParameterLockPool>>,
    ) {
        for sound in self.sounds_mut() {
            sound.link_parameter_lock_pool(parameter_lock_pool).unwrap();
        }
    }
}
