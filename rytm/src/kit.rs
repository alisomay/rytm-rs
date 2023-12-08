pub mod comp;
pub mod delay;
pub mod dist;
pub mod lfo;
pub mod reverb;
pub mod types;
pub(crate) mod unknown;

use self::{
    comp::FxCompressor, delay::FxDelay, dist::FxDistortion, lfo::FxLfo, reverb::FxReverb,
    unknown::KitUnknown,
};
use crate::{
    error::{ParameterError, RytmError, SysexConversionError},
    impl_sysex_compatible,
    object::ObjectName,
    sysex::{SysexCompatible, SysexMeta, SysexType, KIT_SYSEX_SIZE},
    util::to_s_u16_t_union_b_from_u8_as_msb,
    Sound,
};
use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::{ar_kit_raw_to_syx, ar_kit_t, ar_sysex_meta_t};

impl_sysex_compatible!(
    Kit,
    ar_kit_t,
    ar_kit_raw_to_syx,
    SysexType::Kit,
    KIT_SYSEX_SIZE
);

#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Kit {
    #[derivative(Debug = "ignore")]
    sysex_meta: SysexMeta,
    /// Version of the kit structure.
    version: u32,

    index: usize,

    /// Name of the kit.
    name: ObjectName,

    track_levels: [u8; 12],

    sounds: [Sound; 12],

    fx_delay: FxDelay,
    fx_distortion: FxDistortion,
    fx_reverb: FxReverb,
    fx_compressor: FxCompressor,
    fx_lfo: FxLfo,

    // TODO:
    #[derivative(Debug = "ignore")]
    // @attention Will be ignored for now.
    pub(crate) perf_ctl: [u8; 48 * 4], /* @0x0842..0x0901 */
    #[derivative(Debug = "ignore")]
    // @attention Will be ignored for now.
    pub(crate) scene_ctl: [u8; 48 * 4], /* @0x0917..0x09D6 */
    // 0..=11 device 0..=11
    #[derivative(Debug = "ignore")]
    pub(crate) current_scene_id: u8, /* @0x09D8 (0..11) */

    #[derivative(Debug = "ignore")]
    pub(crate) __unknown: KitUnknown,
}

impl From<&Kit> for ar_kit_t {
    fn from(kit: &Kit) -> Self {
        let mut raw_kit = ar_kit_t {
            name: kit.name.copy_inner(),
            perf_ctl: kit.perf_ctl,
            scene_ctl: kit.scene_ctl,
            current_scene_id: kit.current_scene_id,
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

        raw_kit
    }
}

impl Kit {
    pub(crate) fn as_raw_parts(&self) -> (SysexMeta, ar_kit_t) {
        (self.sysex_meta, self.into())
    }

    pub fn try_from_raw(sysex_meta: SysexMeta, raw_kit: &ar_kit_t) -> Result<Self, RytmError> {
        let version = ((raw_kit.__unknown_arr1[0] as u32) << 24)
            | ((raw_kit.__unknown_arr1[1] as u32) << 16)
            | ((raw_kit.__unknown_arr1[2] as u32) << 8)
            | (raw_kit.__unknown_arr1[3] as u32);

        let kit_number = if sysex_meta.is_targeting_work_buffer() {
            // TODO: Double check
            0
        } else {
            sysex_meta.obj_nr as usize
        };

        let name = ObjectName::from_u8_array(raw_kit.name);

        let mut sounds = [Sound::work_buffer_default(); 12];
        for (i, sound) in raw_kit.tracks.iter().enumerate() {
            sounds[i] = Sound::try_from_raw(sysex_meta, sound, Some((kit_number, i)))?;
        }

        let mut track_levels = [0; 12];
        for (i, track_level) in raw_kit.track_levels.iter().enumerate() {
            // Only the high byte is used for the levels.
            track_levels[i] = unsafe { track_level.b.hi };
        }

        Ok(Self {
            index: kit_number,
            sysex_meta,
            version,

            name,

            track_levels,
            sounds,

            fx_delay: raw_kit.try_into()?,
            fx_distortion: raw_kit.try_into()?,
            fx_reverb: raw_kit.try_into()?,
            fx_compressor: raw_kit.try_into()?,
            fx_lfo: raw_kit.try_into()?,

            perf_ctl: raw_kit.perf_ctl,
            scene_ctl: raw_kit.scene_ctl,
            current_scene_id: raw_kit.current_scene_id,
            __unknown: raw_kit.into(),
        })
    }

    #[parameter_range(range = "kit_index:0..=127")]
    pub fn try_default(kit_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            index: kit_index,
            sysex_meta: SysexMeta::try_default_for_kit(kit_index, None)?,
            version: 6,

            name: ObjectName::from_u8_array([0_u8; 15]),

            track_levels: [100; 12],

            // TODO: Currently relevant indexes are omitted.
            // This array is not valid, it is temporary.
            sounds: [Sound::work_buffer_default(); 12],

            fx_delay: FxDelay::default(),
            fx_distortion: FxDistortion::default(),
            fx_reverb: FxReverb::default(),
            fx_compressor: FxCompressor::default(),
            fx_lfo: FxLfo::default(),

            perf_ctl: [0; 48 * 4],
            scene_ctl: [0; 48 * 4],
            current_scene_id: 0,
            __unknown: KitUnknown::default(),
        })
    }

    pub fn work_buffer_default() -> Self {
        Self {
            index: 0,
            sysex_meta: SysexMeta::default_for_kit_in_work_buffer(None),
            version: 6,

            name: ObjectName::from_u8_array([0_u8; 15]),

            track_levels: [100; 12],

            // TODO: Currently relevant indexes are omitted.
            // This array is not valid, it is temporary.
            sounds: [Sound::work_buffer_default(); 12],

            fx_delay: FxDelay::default(),
            fx_distortion: FxDistortion::default(),
            fx_reverb: FxReverb::default(),
            fx_compressor: FxCompressor::default(),
            fx_lfo: FxLfo::default(),

            perf_ctl: [0; 48 * 4],
            scene_ctl: [0; 48 * 4],
            current_scene_id: 0,
            __unknown: KitUnknown::default(),
        }
    }

    /// Returns the sounds assigned to the kit in the order of the tracks.
    pub fn sounds(&self) -> &[Sound; 12] {
        &self.sounds
    }

    /// Returns the sounds assigned to the kit in the order of the tracks mutably.
    pub fn sounds_mut(&mut self) -> &mut [Sound; 12] {
        &mut self.sounds
    }

    #[parameter_range(range = "track_index:0..=11", range = "level:0..=127")]
    pub fn set_level_of_a_track(
        &mut self,
        track_index: usize,
        level: usize,
    ) -> Result<(), RytmError> {
        self.track_levels[track_index] = level as u8;
        Ok(())
    }

    #[parameter_range(range = "level:0..=127")]
    pub fn set_level_of_all_tracks(&mut self, level: usize) -> Result<(), RytmError> {
        for track_level in self.track_levels.iter_mut() {
            *track_level = level as u8;
        }
        Ok(())
    }

    /// Sets the level of a range of tracks.
    ///
    /// Maximum range `0..=11`
    pub fn set_level_of_a_range_of_tracks(
        &mut self,
        range: std::ops::Range<usize>,
        level: usize,
    ) -> Result<(), RytmError> {
        if range.end > 11 {
            return Err(RytmError::Parameter(ParameterError::Range {
                value: format!("{:?}", range),
                parameter_name: "range".to_string(),
            }));
        }

        for track_index in range {
            self.set_level_of_a_track(track_index, level)?;
        }

        Ok(())
    }
}
