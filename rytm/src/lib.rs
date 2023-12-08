pub mod error;
pub mod object;
pub mod query;
pub(crate) mod sysex;
pub mod util;

use self::error::RytmError;
use crate::error::ParameterError;
use object::{
    global::Global,
    kit::Kit,
    pattern::Pattern,
    settings::Settings,
    sound::{Sound, SoundType},
};
use rytm_rs_macro::parameter_range;
use rytm_sys::{ar_global_t, ar_kit_t, ar_pattern_t, ar_settings_t, ar_sound_t};
use sysex::{decode_sysex_response_to_raw, SysexCompatible, SysexType};

/// Rytm is the main struct that holds structures for projects.
#[derive(Clone, Debug)]
pub struct Rytm {
    patterns: Vec<Pattern>,
    pool_sounds: Vec<Sound>,
    kits: Vec<Kit>,
    globals: Vec<Global>,
    // TODO: Songs?
    work_buffer_pattern: Pattern,
    work_buffer_sounds: Vec<Sound>,
    work_buffer_kit: Kit,
    work_buffer_global: Global,
    // TODO: Work buffer songs?
    settings: Settings,
}

impl Default for Rytm {
    fn default() -> Self {
        // TODO: Arrays or vectors?
        let mut patterns = vec![];
        for i in 0..127 {
            patterns.push(Pattern::try_default(i).unwrap());
        }

        let mut pool_sounds = vec![];
        for i in 0..127 {
            pool_sounds.push(Sound::try_default(i).unwrap());
        }

        let mut work_buffer_sounds = vec![];
        for _ in 0..=11 {
            work_buffer_sounds.push(Sound::work_buffer_default());
        }

        let mut kits = vec![];
        for i in 0..127 {
            kits.push(Kit::try_default(i).unwrap());
        }

        let work_buffer_kit = Kit::work_buffer_default();

        let mut globals = vec![];
        for i in 0..3 {
            globals.push(Global::try_default(i).unwrap());
        }

        let work_buffer_global = Global::work_buffer_default();

        Self {
            patterns,
            work_buffer_pattern: Pattern::work_buffer_default(),
            pool_sounds,
            work_buffer_sounds,
            kits,
            work_buffer_kit,
            globals,
            work_buffer_global,
            settings: Settings::default(),
        }
    }
}

impl Rytm {
    /// Updates the Rytm struct from a sysex response.
    ///
    /// All decoding is done in Rytm struct, so this is the only method that needs to be called to update the struct when a sysex response is received.
    ///
    /// # Important
    ///
    /// This method will act as a no-op if the given slice does not contain a valid sysex message.
    ///
    /// The check is performed by checking the first and last byte of the slice.
    ///
    /// This behaviour is preferred to returning an error, as it is expected to be called in a midi callback which receives all midi messages, not just sysex messages.
    pub fn update_from_sysex_response(&mut self, response: &[u8]) -> Result<(), RytmError> {
        // Ignore non-sysex messages.
        if !(response[0] == 0xF0 && response[response.len() - 1] == 0xF7) {
            return Ok(());
        }

        // Invalid sysex messages for Rytm will return an error.
        let (mut raw, meta) = decode_sysex_response_to_raw(response)?;

        match meta.object_type()? {
            SysexType::Pattern => {
                let raw_pattern: &ar_pattern_t =
                    unsafe { &*(raw.as_mut_ptr() as *const ar_pattern_t) };

                let pattern = Pattern::try_from_raw(meta, raw_pattern)?;

                if meta.is_targeting_work_buffer() {
                    self.work_buffer_pattern = pattern;
                    return Ok(());
                }

                let index = (meta.obj_nr & 0b0111_1111) as usize;
                self.patterns[index] = pattern;
                Ok(())
            }

            SysexType::Kit => {
                let raw_kit: &ar_kit_t = unsafe { &*(raw.as_mut_ptr() as *const ar_kit_t) };
                let kit = Kit::try_from_raw(meta, raw_kit)?;

                if meta.is_targeting_work_buffer() {
                    self.work_buffer_kit = kit;
                    return Ok(());
                }

                self.kits[meta.obj_nr as usize] = kit;
                Ok(())
            }

            SysexType::Sound => {
                let raw_sound: &ar_sound_t = unsafe { &*(raw.as_mut_ptr() as *const ar_sound_t) };
                let sound = Sound::try_from_raw(meta, raw_sound, None)?;

                let index = (meta.obj_nr & 0b0111_1111) as usize;
                match sound.sound_type() {
                    SoundType::Pool => {
                        self.pool_sounds[index] = sound;
                    }
                    SoundType::WorkBuffer => {
                        self.work_buffer_sounds[index] = sound;
                    }
                    SoundType::KitQuery => {
                        todo!("Then it is not a sound query. Handle properly?")
                        // TODO: Return error..
                    }
                };

                Ok(())
            }

            SysexType::Global => {
                let raw_global: &ar_global_t =
                    unsafe { &*(raw.as_mut_ptr() as *const ar_global_t) };
                let global = Global::try_from_raw(meta, raw_global)?;

                if meta.is_targeting_work_buffer() {
                    self.work_buffer_global = global;
                    return Ok(());
                }

                let index = (meta.obj_nr & 0b0000_0011) as usize;
                self.globals[index] = global;
                Ok(())
            }

            SysexType::Settings => {
                let raw_settings: &ar_settings_t =
                    unsafe { &*(raw.as_mut_ptr() as *const ar_settings_t) };
                let settings = Settings::try_from_raw(meta, raw_settings)?;
                self.settings = settings;
                Ok(())
            }

            SysexType::Song => {
                unimplemented!("TODO: Song")
                //TODO: Return error..
            }
        }
    }

    /// Encodes the chosen pattern as a sysex message.
    #[parameter_range(range = "pattern_index:0..=127")]
    pub fn encode_pattern_as_sysex_message(
        &self,
        pattern_index: usize,
    ) -> Result<Vec<u8>, RytmError> {
        self.patterns[pattern_index].as_sysex_message()
    }

    // Encode kit as a sysex message.
    #[parameter_range(range = "kit_index:0..=127")]
    pub fn encode_kit_as_sysex_message(&self, kit_index: usize) -> Result<Vec<u8>, RytmError> {
        self.kits[kit_index].as_sysex_message()
    }

    // Encode sound as a sysex message.
    #[parameter_range(range = "sound_index:0..=127")]
    pub fn encode_sound_as_sysex_message(&self, sound_index: usize) -> Result<Vec<u8>, RytmError> {
        self.pool_sounds[sound_index].as_sysex_message()
    }

    // Encode global as a sysex message.
    #[parameter_range(range = "global_slot:0..=3")]
    pub fn encode_global_as_sysex_message(&self, global_slot: usize) -> Result<Vec<u8>, RytmError> {
        self.globals[global_slot].as_sysex_message()
    }

    // Encode settings as a sysex message.
    pub fn encode_settings_as_sysex_message(&self) -> Result<Vec<u8>, RytmError> {
        self.settings.as_sysex_message()
    }

    /// Encode work buffer pattern as a sysex message.
    pub fn encode_work_buffer_pattern_as_sysex_message(&self) -> Result<Vec<u8>, RytmError> {
        self.work_buffer_pattern.as_sysex_message()
    }

    /// Encode work buffer kit as a sysex message.
    pub fn encode_work_buffer_kit_as_sysex_message(&self) -> Result<Vec<u8>, RytmError> {
        self.work_buffer_kit.as_sysex_message()
    }

    /// Encode work buffer sound as a sysex message.
    #[parameter_range(range = "track_index:0..=11")]
    pub fn encode_work_buffer_sound_as_sysex_message(
        &self,
        track_index: usize,
    ) -> Result<Vec<u8>, RytmError> {
        self.work_buffer_sounds[track_index].as_sysex_message()
    }

    /// Encode work buffer global as a sysex message.
    pub fn encode_work_buffer_global_as_sysex_message(&self) -> Result<Vec<u8>, RytmError> {
        self.work_buffer_global.as_sysex_message()
    }

    /// Get all patterns.
    ///
    /// Total of 128 patterns.
    pub fn patterns(&self) -> &[Pattern] {
        &self.patterns
    }

    /// Get all kits.
    ///
    /// Total of 128 kits.
    pub fn kits(&self) -> &[Kit] {
        &self.kits
    }

    /// Get all sounds in the pool.
    ///
    /// Total of 128 sounds.
    pub fn pool_sounds(&self) -> &[Sound] {
        &self.pool_sounds
    }

    /// Get all global slots.
    ///
    /// Total of 4 global slots.
    pub fn globals(&self) -> &[Global] {
        &self.globals
    }

    /// Get the settings.
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Get all patterns mutably.
    ///
    /// Total of 128 patterns.
    pub fn patterns_mut(&mut self) -> &mut [Pattern] {
        &mut self.patterns
    }

    /// Get all kits mutably.
    ///
    /// Total of 128 kits.
    pub fn kits_mut(&mut self) -> &mut [Kit] {
        &mut self.kits
    }

    /// Get all sounds in the pool mutably.
    ///
    /// Total of 128 sounds.
    pub fn pool_sounds_mut(&mut self) -> &mut [Sound] {
        &mut self.pool_sounds
    }

    /// Get all global slots mutably.
    ///
    /// Total of 4 global slots.
    pub fn globals_mut(&mut self) -> &mut [Global] {
        &mut self.globals
    }

    /// Get the settings mutably.
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    // Work buffer

    /// Get the pattern in the work buffer.
    pub fn work_buffer_pattern(&self) -> &Pattern {
        &self.work_buffer_pattern
    }

    /// Get the kit in the work buffer.
    pub fn work_buffer_kit(&self) -> &Kit {
        &self.work_buffer_kit
    }

    /// Get the sounds in the work buffer.
    ///
    /// Total of 12 sounds for 12 tracks.
    pub fn work_buffer_sounds(&self) -> &[Sound] {
        &self.work_buffer_sounds
    }

    /// Get the global in the work buffer.
    pub fn work_buffer_global(&self) -> &Global {
        &self.work_buffer_global
    }

    /// Get the pattern in the work buffer mutably.
    pub fn work_buffer_pattern_mut(&mut self) -> &mut Pattern {
        &mut self.work_buffer_pattern
    }

    /// Get the kit in the work buffer mutably.
    pub fn work_buffer_kit_mut(&mut self) -> &mut Kit {
        &mut self.work_buffer_kit
    }

    /// Get the sounds in the work buffer mutably.
    ///
    /// Total of 12 sounds for 12 tracks.
    pub fn work_buffer_sounds_mut(&mut self) -> &mut [Sound] {
        &mut self.work_buffer_sounds
    }

    /// Get the global in the work buffer mutably.
    pub fn work_buffer_global_mut(&mut self) -> &mut Global {
        &mut self.work_buffer_global
    }
}
