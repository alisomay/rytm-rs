// TODO:
// - Not understood kit offsets is it relevant?

pub mod machine;
pub mod page;
pub mod settings;
pub mod types;
pub(crate) mod unknown;

use self::{
    machine::MachineParameters,
    page::{Amplitude, Filter, Lfo, Sample},
    settings::SoundSettings,
    unknown::SoundUnknown,
};
use super::pattern::parameter_lock::ParameterLockPool;
use crate::{
    error::{RytmError, SysexConversionError},
    impl_sysex_compatible,
    object::types::ObjectName,
    sysex::{SysexCompatible, SysexMeta, SysexType, SOUND_SYSEX_SIZE},
    ParameterError,
};
use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::{ar_sound_raw_to_syx, ar_sound_t, ar_sysex_meta_t};
use std::{cell::RefCell, rc::Rc};

// Internal type to understand where the sound comes from.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum SoundType {
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

#[derive(Derivative, Clone)]
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
    pub parameter_lock_pool: Option<Rc<RefCell<ParameterLockPool>>>,
}

impl From<&Sound> for ar_sound_t {
    fn from(sound: &Sound) -> Self {
        // TODO: Synth parameters omitted. Don't forget to implement them.

        let mut raw_sound = rytm_sys::ar_sound_t {
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
    /// Links a pattern's parameter lock pool to this sound.
    ///
    /// This way one can set parameter locks for trigs for the machine assigned to this sound.
    pub fn link_parameter_lock_pool(
        &mut self,
        parameter_lock_pool: Rc<RefCell<ParameterLockPool>>,
    ) {
        self.parameter_lock_pool = Some(parameter_lock_pool);
        let parameter_lock_pool_ref = Rc::clone(self.parameter_lock_pool.as_ref().unwrap());
        self.machine_parameters
            .link_parameter_lock_pool(parameter_lock_pool_ref);
    }

    pub(crate) fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_sound: &ar_sound_t,
        kit_number_and_assigned_track: Option<(usize, usize)>,
    ) -> Result<Self, RytmError> {
        let version = ((raw_sound.__unknown_arr1[4] as u32) << 24)
            | ((raw_sound.__unknown_arr1[5] as u32) << 16)
            | ((raw_sound.__unknown_arr1[6] as u32) << 8)
            | (raw_sound.__unknown_arr1[7] as u32);

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
                    // TODO: Maybe better handle all these.
                    todo!("Error here, this is not a sound query. Kit queries should provide the kit number and assigned track.")
                }
            }
            _ => unreachable!(" TODO: This is not a sound or kit query handle error."),
        }

        Ok(Self {
            index,
            pool_index,
            kit_number,
            assigned_track,
            sysex_meta,
            version,

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

    pub(crate) fn sound_type(&self) -> SoundType {
        if self.is_pool_sound() {
            SoundType::Pool
        } else if self.is_work_buffer_sound() {
            SoundType::WorkBuffer
        } else {
            SoundType::KitQuery
        }
    }

    /// Returns if the sound is coming from the sound pool.
    pub fn is_pool_sound(&self) -> bool {
        self.pool_index.is_some()
    }

    /// Returns if the sound is coming from the work buffer and assigned to a track.
    pub fn is_work_buffer_sound(&self) -> bool {
        self.assigned_track().is_some() && self.kit_number.is_none()
    }

    /// Returns if the sound is coming from a kit query and loaded as a part of a kit.
    pub fn is_part_of_a_kit_query(&self) -> bool {
        self.kit_number.is_some()
    }

    /// Sets the name of the sound.
    ///
    /// The name must be ASCII and have a length of 15 characters or less.
    pub fn set_name(&mut self, name: &str) -> Result<(), RytmError> {
        if !name.is_ascii() || name.len() > 15 {
            return Err(ParameterError::Compatibility {
                value: name.to_string(),
                parameter_name: "Name".to_string(),
                reason: Some(
                    "Name must be ASCII and have a length of 15 characters or less.".to_owned(),
                ),
            }
            .into());
        }

        self.name = ObjectName::from_u8_array(name.as_bytes().try_into().unwrap());
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
    pub fn assigned_track(&self) -> Option<usize> {
        self.assigned_track
    }

    /// Returns the accent level of the sound.
    ///
    /// Range: `0..=127`
    pub fn accent_level(&self) -> usize {
        self.accent_level as usize
    }

    /// Returns the name of the sound.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the sample page parameters of the sound.
    pub fn sample(&self) -> &Sample {
        &self.sample
    }

    /// Returns the filter page parameters of the sound.
    pub fn filter(&self) -> &Filter {
        &self.filter
    }

    /// Returns the amplitude page parameters of the sound.
    pub fn amplitude(&self) -> &Amplitude {
        &self.amplitude
    }

    /// Returns the LFO page parameters of the sound.
    pub fn lfo(&self) -> &Lfo {
        &self.lfo
    }

    /// Returns sound settings of the sound.
    pub fn settings(&self) -> &SoundSettings {
        &self.settings
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

    /// Returns the machine parameters of the sound.
    pub fn machine_parameters(&self) -> &MachineParameters {
        &self.machine_parameters
    }

    /// Returns the machine parameters of the sound mutably.
    pub fn machine_parameters_mut(&mut self) -> &mut MachineParameters {
        &mut self.machine_parameters
    }

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
            name: ObjectName::try_from(format!("POOL_SOUND {}", sound_index))?,
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
            name: ObjectName::try_from(format!("KIT_SOUND {}", track_index))?,
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

    #[parameter_range(range = "track_index:0..=11")]
    pub fn try_work_buffer_default(track_index: usize) -> Result<Self, RytmError> {
        let index = track_index | 0b1000_0000;
        Ok(Self {
            sysex_meta: SysexMeta::default_for_sound_in_work_buffer(None),
            index,
            pool_index: None,
            kit_number: None,
            assigned_track: Some(track_index),

            version: 4,
            name: ObjectName::try_from(format!("SOUND {}", track_index))?,
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
}
