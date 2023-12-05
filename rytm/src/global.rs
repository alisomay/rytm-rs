pub mod menu;
pub mod types;

use derivative::Derivative;
use menu::*;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_global_t;

use crate::error::{ParameterError, RytmError};
use crate::sysex::SysexMeta;

/// # Settings
///
/// This structure represents a global slot in the analog rytm.
///
/// It does not map identically to the relevant structure in the firmware.
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Global {
    index: usize,
    sysex_meta: SysexMeta,
    /// Version of the kit structure.
    version: u32,

    metronome_settings: MetronomeSettings,
    midi_config: MidiConfig,
    sequencer_config: SequencerConfig,
    routing: Routing,

    #[derivative(Debug = "ignore")]
    pub(crate) __unknown0x09_0x0a: [u8; 2], /* @?0x09..0x0A  Currently reads  0x40, 0x00 */
    #[derivative(Debug = "ignore")]
    pub(crate) __unknown0x31: u8, /* ?@0x31        */
    #[derivative(Debug = "ignore")]
    // All zeros. It is suspicious since it is exactly 16 bytes long, maybe related to midi channels?
    pub(crate) __unknown0x36_0x45: [u8; 16], /* @?0x36..0x45  */
    #[derivative(Debug = "ignore")]
    // All zeros.
    pub(crate) __unknown0x50_0x4f: [u8; 6], /* @?0x50..0x4F  */
}

impl From<&Global> for ar_global_t {
    fn from(global: &Global) -> Self {
        let mut raw_global = ar_global_t::default();

        raw_global.version[0] = ((global.version >> 24) & 0xFF) as u8;
        raw_global.version[1] = ((global.version >> 16) & 0xFF) as u8;
        raw_global.version[2] = ((global.version >> 8) & 0xFF) as u8;
        raw_global.version[3] = (global.version & 0xFF) as u8;

        global
            .metronome_settings
            .apply_to_raw_global(&mut raw_global);
        global.midi_config.apply_to_raw_global(&mut raw_global);
        global.sequencer_config.apply_to_raw_global(&mut raw_global);
        global.routing.apply_to_raw_global(&mut raw_global);

        raw_global.__unknown0x09_0x0A = global.__unknown0x09_0x0a;
        raw_global.__unknown0x31 = global.__unknown0x31;
        raw_global.__unknown0x36_0x45 = global.__unknown0x36_0x45;
        raw_global.__unknown0x50_0x4F = global.__unknown0x50_0x4f;

        raw_global
    }
}

impl Global {
    pub fn to_raw_parts(&self) -> (SysexMeta, ar_global_t) {
        (self.sysex_meta, self.into())
    }

    pub fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_global: &ar_global_t,
    ) -> Result<Self, RytmError> {
        let version = ((raw_global.version[0] as u32) << 24)
            | ((raw_global.version[1] as u32) << 16)
            | ((raw_global.version[2] as u32) << 8)
            | (raw_global.version[3] as u32);

        let slot_number = if sysex_meta.is_targeting_work_buffer() {
            // TODO: Double check
            0
        } else {
            sysex_meta.obj_nr as usize
        };

        Ok(Self {
            index: slot_number,
            sysex_meta,
            version,

            metronome_settings: raw_global.try_into()?,
            midi_config: raw_global.try_into()?,
            sequencer_config: raw_global.try_into()?,
            routing: raw_global.try_into()?,

            __unknown0x09_0x0a: raw_global.__unknown0x09_0x0A,
            __unknown0x31: raw_global.__unknown0x31,
            __unknown0x36_0x45: raw_global.__unknown0x36_0x45,
            __unknown0x50_0x4f: raw_global.__unknown0x50_0x4F,
        })
    }

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

            __unknown0x09_0x0a: [0; 2],
            __unknown0x31: 0,
            __unknown0x36_0x45: [0; 16],
            __unknown0x50_0x4f: [0; 6],
        })
    }

    pub fn work_buffer_default() -> Self {
        Self {
            index: 0,
            sysex_meta: SysexMeta::default_for_global_in_work_buffer(None),
            version: 2,

            metronome_settings: MetronomeSettings::default(),
            midi_config: MidiConfig::default(),
            sequencer_config: SequencerConfig::default(),
            routing: Routing::default(),

            __unknown0x09_0x0a: [0; 2],
            __unknown0x31: 0,
            __unknown0x36_0x45: [0; 16],
            __unknown0x50_0x4f: [0; 6],
        }
    }
}
