pub mod menu;
pub mod types;
pub(crate) mod unknown;

use self::unknown::GlobalUnknown;
use crate::{
    error::{ParameterError, RytmError, SysexConversionError},
    impl_sysex_compatible,
    sysex::{SysexCompatible, SysexMeta, SysexType, GLOBAL_SYSEX_SIZE},
    util::{assemble_u32_from_u8_array_be, break_u32_into_u8_array_be},
    AnySysexType,
};
use derivative::Derivative;
use menu::*;
use rytm_rs_macro::parameter_range;
use rytm_sys::{ar_global_raw_to_syx, ar_global_t, ar_sysex_meta_t};
use serde::{Deserialize, Serialize};

impl_sysex_compatible!(
    Global,
    ar_global_t,
    ar_global_raw_to_syx,
    SysexType::Global,
    GLOBAL_SYSEX_SIZE
);

/// Represents a global in the analog rytm.
///
/// It does not map identically to the structure in the firmware.
///
/// Globals are global settings which you may found in the settings menu of the device.
#[derive(Derivative, Clone, Copy, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct Global {
    #[derivative(Debug = "ignore")]
    sysex_meta: SysexMeta,
    /// Version of the global structure.
    version: u32,

    index: usize,

    metronome_settings: MetronomeSettings,
    midi_config: MidiConfig,
    sequencer_config: SequencerConfig,
    routing: Routing,

    #[derivative(Debug = "ignore")]
    __unknown: GlobalUnknown,
}

impl From<&Global> for ar_global_t {
    fn from(global: &Global) -> Self {
        let mut raw_global = Self {
            version: break_u32_into_u8_array_be(global.version),
            ..Default::default()
        };

        global
            .metronome_settings
            .apply_to_raw_global(&mut raw_global);
        global.midi_config.apply_to_raw_global(&mut raw_global);
        global.sequencer_config.apply_to_raw_global(&mut raw_global);
        global.routing.apply_to_raw_global(&mut raw_global);

        global.__unknown.apply_to_raw_global(&mut raw_global);

        raw_global
    }
}

impl Global {
    pub(crate) fn as_raw_parts(&self) -> (SysexMeta, ar_global_t) {
        (self.sysex_meta, self.into())
    }

    pub(crate) fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_global: &ar_global_t,
    ) -> Result<Self, RytmError> {
        let slot_number = sysex_meta.get_normalized_object_index();

        Ok(Self {
            index: slot_number,
            sysex_meta,
            version: assemble_u32_from_u8_array_be(&raw_global.version),

            metronome_settings: raw_global.try_into()?,
            midi_config: raw_global.try_into()?,
            sequencer_config: raw_global.try_into()?,
            routing: raw_global.try_into()?,

            __unknown: raw_global.into(),
        })
    }

    /// Makes a new global complying to project defaults.
    ///
    /// Accepts a global slot index in the range of `0..=3`.
    #[parameter_range(range = "global_slot:0..=3")]
    pub fn try_default(global_slot: usize) -> Result<Self, RytmError> {
        Ok(Self {
            index: global_slot,
            sysex_meta: SysexMeta::try_default_for_global(global_slot, None)?,
            version: 2,

            metronome_settings: MetronomeSettings::default(),
            midi_config: MidiConfig::default(),
            sequencer_config: SequencerConfig::default(),
            routing: Routing::default(),

            __unknown: GlobalUnknown::default(),
        })
    }

    /// Makes a new global in the work buffer complying to project defaults as if it comes from the work buffer.
    pub fn work_buffer_default() -> Self {
        Self {
            index: 0,
            sysex_meta: SysexMeta::default_for_global_in_work_buffer(None),
            version: 2,

            metronome_settings: MetronomeSettings::default(),
            midi_config: MidiConfig::default(),
            sequencer_config: SequencerConfig::default(),
            routing: Routing::default(),

            __unknown: GlobalUnknown::default(),
        }
    }

    /// Returns the version of the global structure.
    pub const fn structure_version(&self) -> u32 {
        self.version
    }

    /// Checks if this global is the global at work buffer.
    pub const fn is_work_buffer(&self) -> bool {
        self.sysex_meta.is_targeting_work_buffer()
    }

    /// Returns the global slot index.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the metronome settings.
    pub const fn metronome_settings(&self) -> &MetronomeSettings {
        &self.metronome_settings
    }

    /// Returns the metronome settings mutably.
    pub fn metronome_settings_mut(&mut self) -> &mut MetronomeSettings {
        &mut self.metronome_settings
    }

    /// Returns the midi config.
    pub const fn midi_config(&self) -> &MidiConfig {
        &self.midi_config
    }

    /// Returns the midi config mutably.
    pub fn midi_config_mut(&mut self) -> &mut MidiConfig {
        &mut self.midi_config
    }

    /// Returns the sequencer config.
    pub const fn sequencer_config(&self) -> &SequencerConfig {
        &self.sequencer_config
    }

    /// Returns the sequencer config mutably.
    pub fn sequencer_config_mut(&mut self) -> &mut SequencerConfig {
        &mut self.sequencer_config
    }

    /// Returns the routing.
    pub const fn routing(&self) -> &Routing {
        &self.routing
    }

    /// Returns the routing mutably.
    pub fn routing_mut(&mut self) -> &mut Routing {
        &mut self.routing
    }
}
