pub(crate) mod de;
/// All structures related to machines and their parameters.
pub mod machine;
/// Holds the page settings of the sound. Like `[AMP]`, `[FLT]`, `[LFO]`, `[SAMP]` on the device.
pub mod page;
/// Holds the structures which represent the settings of the sound.
pub mod settings;
/// Types which are relevant to sounds.
pub mod types;
pub(crate) mod unknown;

use self::{
    machine::MachineParameters,
    page::{Amplitude, Filter, Lfo, Sample},
    settings::SoundSettings,
    types::MachineType,
    unknown::SoundUnknown,
};
use super::pattern::plock::ParameterLockPool;
use crate::{
    error::{RytmError, SysexConversionError},
    impl_sysex_compatible,
    object::types::ObjectName,
    sysex::{SysexCompatible, SysexMeta, SysexType, SOUND_SYSEX_SIZE},
    util::arc_mutex_owner,
    ParameterError,
};
use crate::{util::assemble_u32_from_u8_array, AnySysexType};
use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::{ar_sound_raw_to_syx, ar_sound_t, ar_sysex_meta_t};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// An enum to understand where the sound is coming from.
///
/// The sound can be a pool sound, the work buffer or as a part of a kit.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SoundType {
    Pool,
    #[default]
    WorkBuffer,
    KitQuery,
}

impl_sysex_compatible!(
    Sound,
    ar_sound_t,
    ar_sound_raw_to_syx,
    SysexType::Sound,
    SOUND_SYSEX_SIZE
);

/// Represents a sound in the analog rytm.
///
/// This structure does not map identically to the relevant structure in the firmware.
#[derive(Derivative, Clone, Serialize)]
#[derivative(Debug)]
pub struct Sound {
    #[derivative(Debug = "ignore")]
    sysex_meta: SysexMeta,
    /// Version of the sound structure.
    version: u32,

    /// Index of the sound.
    ///
    /// This can mean various things depending on the context
    ///
    /// - If this sound is retrieved from the sound pool, this is the index of the sound in the pool.
    /// - If this sound is retrieved from a track from the work buffer or a kit query, this is the index of the track.
    index: usize,
    /// Index of the sound if it was retrieved from the sound pool.
    pool_index: Option<usize>,
    /// Kit number if this sound is retrieved from a kit query
    kit_number: Option<usize>,
    /// Index of the sound if it was retrieved from a track from the work buffer.
    assigned_track: Option<usize>,

    /// Name of the sound.
    name: ObjectName,

    machine_parameters: MachineParameters,

    sample: Sample,
    filter: Filter,
    amplitude: Amplitude,
    lfo: Lfo,
    settings: SoundSettings,

    accent_level: u8,
    // TODO: Understand what this corresponds to.
    // Currently not implementing it.
    def_note: u8,

    #[derivative(Debug = "ignore")]
    __unknown: SoundUnknown,

    #[derivative(Debug = "ignore")]
    #[serde(serialize_with = "arc_mutex_owner::opt_serialize")]
    pub(crate) parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,
}

impl From<&Sound> for ar_sound_t {
    fn from(sound: &Sound) -> Self {
        let mut raw_sound = Self {
            name: sound.name.copy_inner(),
            accent_level: sound.accent_level,
            def_note: sound.def_note,
            ..Default::default()
        };

        sound.sample.apply_to_raw_sound(&mut raw_sound);
        sound.filter.apply_to_raw_sound(&mut raw_sound);
        sound.amplitude.apply_to_raw_sound(&mut raw_sound);
        sound.lfo.apply_to_raw_sound(&mut raw_sound);
        sound.settings.apply_to_raw_sound(&mut raw_sound);
        sound.machine_parameters.apply_to_raw_sound(&mut raw_sound);

        sound.__unknown.apply_to_raw_sound(&mut raw_sound);

        raw_sound
    }
}

impl Sound {
    // This can never fail.
    #[allow(clippy::missing_panics_doc)]
    /// Links a pattern's parameter lock pool to this sound.
    ///
    /// This way, one can set parameter locks for trigs for the machine assigned to this sound.
    ///
    /// # Errors
    ///
    /// Sound must be a track sound. This is necessary because the pattern's parameter lock pool
    /// belongs to a pattern but sounds are not. Sounds are received with different query compared to patterns.
    ///
    /// Calling this method on a pool sound will result in an error.
    pub fn link_parameter_lock_pool(
        &mut self,
        parameter_lock_pool: &Arc<Mutex<ParameterLockPool>>,
    ) -> Result<(), RytmError> {
        if self.is_pool_sound() {
            return Err(ParameterError::Compatibility {
                value: "ParameterLockPool".into(),
                parameter_name: "parameter_lock_pool".into(),
                reason: Some("The sound you're trying to link the parameter lock pool is a pool sound. Pool sounds cannot have parameter locks.".into()),
            }
            .into());
        }
        self.parameter_lock_pool = Some(Arc::clone(parameter_lock_pool));
        let parameter_lock_pool_ref = Arc::clone(self.parameter_lock_pool.as_ref().unwrap());
        self.machine_parameters
            .link_parameter_lock_pool(parameter_lock_pool_ref);
        Ok(())
    }

    // The panics in this function should be basically unreachable when this function is used correctly.
    pub(crate) fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_sound: &ar_sound_t,
        kit_number_and_assigned_track: Option<(usize, usize)>,
    ) -> Result<Self, RytmError> {
        let mut index: usize = 0;
        let mut assigned_track = None;
        let mut kit_number = None;
        let mut pool_index = None;

        match sysex_meta.object_type()? {
            SysexType::Sound => {
                if sysex_meta.is_targeting_work_buffer() {
                    index = (sysex_meta.obj_nr & 0b0111_1111) as usize;
                    assigned_track = Some(index);
                }

                if let Some((kit_n, assigned_t)) = kit_number_and_assigned_track {
                    index = assigned_t;
                    assigned_track = Some(assigned_t);
                    kit_number = Some(kit_n);
                }

                if kit_number_and_assigned_track.is_none() && !sysex_meta.is_targeting_work_buffer()
                {
                    index = (sysex_meta.obj_nr & 0b0111_1111) as usize;
                    pool_index = Some(index);
                }
            }
            SysexType::Kit => {
                // When this sound is part of a kit query...
                if let Some((kit_n, assigned_t)) = kit_number_and_assigned_track {
                    index = assigned_t;
                    assigned_track = Some(assigned_t);
                    kit_number = Some(kit_n);
                } else {
                    panic!("This is not a sound query. Kit queries should provide the kit number and assigned track.")
                }
            }
            _ => panic!(" This is not a sound or kit query."),
        }

        Ok(Self {
            index,
            pool_index,
            kit_number,
            assigned_track,
            sysex_meta,
            version: assemble_u32_from_u8_array(&raw_sound.__unknown_arr1[4..=7]),

            name: ObjectName::from_u8_array(raw_sound.name),

            sample: raw_sound.try_into()?,
            filter: raw_sound.try_into()?,
            amplitude: raw_sound.try_into()?,
            lfo: raw_sound.try_into()?,
            settings: raw_sound.try_into()?,
            machine_parameters: MachineParameters::try_from_raw_sound(raw_sound, assigned_track)?,

            accent_level: raw_sound.accent_level,
            def_note: raw_sound.def_note,

            __unknown: raw_sound.into(),

            parameter_lock_pool: None,
        })
    }

    pub(crate) fn as_raw_parts(&self) -> (SysexMeta, ar_sound_t) {
        (self.sysex_meta, self.into())
    }

    /// Returns the type of the sound.
    pub const fn sound_type(&self) -> SoundType {
        if self.is_pool_sound() {
            SoundType::Pool
        } else if self.is_work_buffer_sound() {
            SoundType::WorkBuffer
        } else {
            SoundType::KitQuery
        }
    }

    /// Returns if the sound is coming from the sound pool.
    pub const fn is_pool_sound(&self) -> bool {
        self.pool_index.is_some()
    }

    /// Returns if the sound is coming from the work buffer and assigned to a track.
    pub const fn is_work_buffer_sound(&self) -> bool {
        self.assigned_track().is_some() && self.kit_number.is_none()
    }

    /// Returns if the sound is coming from a kit query and loaded as a part of a kit.
    pub const fn is_part_of_a_kit_query(&self) -> bool {
        self.kit_number.is_some()
    }

    /// Sets the name of the sound.
    ///
    /// # Errors
    ///
    /// The name must be ASCII and have a length of 15 characters or less. Other cases will result in an error.
    pub fn set_name(&mut self, name: &str) -> Result<(), RytmError> {
        self.name = name.try_into()?;
        Ok(())
    }

    /// Sets the accent level of the sound.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "accent_level:0..=127")]
    pub fn set_accent_level(&mut self, accent_level: u8) -> Result<(), RytmError> {
        self.accent_level = accent_level;
        Ok(())
    }

    /// Returns the assigned track if this is a track sound.
    ///
    /// Returns `None` if this is not a track sound.
    ///
    /// Range: `0..=11`
    pub const fn assigned_track(&self) -> Option<usize> {
        self.assigned_track
    }

    /// Returns the kit number if this sound is a part of a kit.
    ///
    /// Returns `None` if this is not a kit sound.
    ///
    /// Range: `0..=127`
    pub const fn kit_number(&self) -> Option<usize> {
        self.kit_number
    }

    /// Returns the kit number if this sound is a part of a kit.
    ///
    /// Returns `None` if this is not a kit sound.
    ///
    /// Range: `0..=127`
    pub const fn pool_index(&self) -> Option<usize> {
        self.pool_index
    }

    /// Returns the accent level of the sound.
    ///
    /// Range: `0..=127`
    pub const fn accent_level(&self) -> usize {
        self.accent_level as usize
    }

    /// Returns the name of the sound.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the sample page parameters of the sound.
    pub const fn sample(&self) -> &Sample {
        &self.sample
    }

    /// Returns the filter page parameters of the sound.
    pub const fn filter(&self) -> &Filter {
        &self.filter
    }

    /// Returns the amplitude page parameters of the sound.
    pub const fn amplitude(&self) -> &Amplitude {
        &self.amplitude
    }

    /// Returns the LFO page parameters of the sound.
    pub const fn lfo(&self) -> &Lfo {
        &self.lfo
    }

    /// Returns sound settings of the sound.
    pub const fn settings(&self) -> &SoundSettings {
        &self.settings
    }

    /// Returns the machine parameters of the sound.
    pub const fn machine_parameters(&self) -> &MachineParameters {
        &self.machine_parameters
    }

    /// Returns the sample page parameters of the sound mutably.
    pub fn sample_mut(&mut self) -> &mut Sample {
        &mut self.sample
    }

    /// Returns the filter page parameters of the sound mutably.
    pub fn filter_mut(&mut self) -> &mut Filter {
        &mut self.filter
    }

    /// Returns the amplitude page parameters of the sound mutably.
    pub fn amplitude_mut(&mut self) -> &mut Amplitude {
        &mut self.amplitude
    }

    /// Returns the LFO page parameters of the sound mutably.
    pub fn lfo_mut(&mut self) -> &mut Lfo {
        &mut self.lfo
    }

    /// Returns sound settings of the sound mutably.
    pub fn settings_mut(&mut self) -> &mut SoundSettings {
        &mut self.settings
    }

    /// Returns the machine parameters of the sound mutably.
    pub fn machine_parameters_mut(&mut self) -> &mut MachineParameters {
        &mut self.machine_parameters
    }

    /// Returns the version of the sound structure.
    pub const fn structure_version(&self) -> u32 {
        self.version
    }

    /// Makes a new pool sound with the given index complying to project defaults.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "sound_index:0..=127")]
    pub fn try_default(sound_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            sysex_meta: SysexMeta::try_default_for_sound(sound_index, None)?,
            index: sound_index,
            pool_index: Some(sound_index),
            kit_number: None,
            assigned_track: None,

            version: 4,
            // TODO: Decide default name.
            name: ObjectName::try_from(format!("POOL_SOUND {sound_index}"))?,
            accent_level: 32,

            sample: Sample::default(),
            filter: Filter::default(),
            amplitude: Amplitude::default(),
            lfo: Lfo::default(),
            settings: SoundSettings::default(),
            machine_parameters: MachineParameters::default(),

            // Don't know what this is still..
            def_note: 0,

            __unknown: SoundUnknown::default(),

            parameter_lock_pool: None,
        })
    }

    /// Makes a new sound with the given index complying to project defaults as if it comes from a part of a kit.
    ///
    /// Track index range: `0..=11`
    /// Kit index range: `0..=127`
    #[parameter_range(range = "track_index:0..=11", range = "kit_index:0..=127")]
    pub fn try_kit_default(
        track_index: usize,
        kit_index: usize,
        sysex_meta: SysexMeta,
    ) -> Result<Self, RytmError> {
        // TODO: Do we need a work buffer | here?
        let index = track_index;
        Ok(Self {
            sysex_meta,
            index,
            pool_index: None,
            kit_number: Some(kit_index),
            assigned_track: Some(track_index),

            version: 4,
            name: ObjectName::try_from(format!("KIT_SOUND {track_index}"))?,
            accent_level: 32,

            sample: Sample::default(),
            filter: Filter::default(),
            amplitude: Amplitude::default(),
            lfo: Lfo::default(),
            settings: SoundSettings::try_default_for_track(track_index)?,
            machine_parameters: MachineParameters::try_default_for_track(track_index)?,

            // Don't know what this is still..
            def_note: 0,

            __unknown: SoundUnknown::default(),

            parameter_lock_pool: None,
        })
    }

    /// Makes a new sound with the given index complying to project defaults as if it comes from the work buffer.
    ///
    /// Track index range: `0..=11`
    #[parameter_range(range = "track_index:0..=11")]
    pub fn try_work_buffer_default(track_index: usize) -> Result<Self, RytmError> {
        // Continue indexing from 128 since this is in work buffer.
        let index = track_index | 0b1000_0000;
        Ok(Self {
            sysex_meta: SysexMeta::default_for_sound_in_work_buffer(track_index, None),
            index,
            pool_index: None,
            kit_number: None,
            assigned_track: Some(track_index),

            version: 4,
            name: ObjectName::try_from(format!("SOUND {track_index}"))?,
            accent_level: 32,

            sample: Sample::default(),
            filter: Filter::default(),
            amplitude: Amplitude::default(),
            lfo: Lfo::default(),
            settings: SoundSettings::try_default_for_track(track_index)?,
            machine_parameters: MachineParameters::try_default_for_track(track_index)?,

            // Don't know what this is still..
            def_note: 0,

            __unknown: SoundUnknown::default(),

            parameter_lock_pool: None,
        })
    }

    /// Sets the machine type of the sound.
    ///
    /// # Errors
    ///
    /// Not every machine type could be set for every sound if they're assigned to a track.
    ///
    /// For the sounds which are assigned to a track, the machine type must be compatible with the track or an error will be returned.
    ///
    /// For pool sounds this function will always succeed.
    pub fn set_machine_type(&mut self, machine_type: MachineType) -> Result<(), RytmError> {
        if let Some(assigned_track) = self.assigned_track() {
            if !crate::util::is_machine_compatible_for_track(assigned_track, machine_type) {
                return Err(ParameterError::Compatibility {
                    value: machine_type.to_string(),
                    parameter_name: "Machine".to_string(),
                    reason: Some(format!(
                        "Given machine {} is not compatible for track {}",
                        machine_type, self.index
                    )),
                }
                .into());
            }
        }

        self.settings_mut().machine_type = machine_type;
        self.machine_parameters = machine_type.into();
        Ok(())
    }
}
