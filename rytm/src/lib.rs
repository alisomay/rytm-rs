pub mod error;
pub mod global;
pub mod kit;
pub mod object;
pub mod pattern;
pub mod query;
pub mod settings;
pub mod sound;

pub(crate) mod sysex;
pub(crate) mod util;

use global::Global;
use kit::Kit;
use pattern::Pattern;
use settings::Settings;
use sound::{Sound, SoundType};

use crate::error::ParameterError;
use rytm_rs_macro::parameter_range;
use rytm_sys::{ar_global_t, ar_kit_t, ar_pattern_t, ar_settings_t, ar_sound_t};
use sysex::{decode_sysex_response_to_raw, SysexCompatible, SysexType};

use self::error::RytmError;

/// Rytm is the main struct that holds all the patterns.
#[derive(Clone, Debug)]
pub struct Rytm {
    patterns: Vec<Pattern>,
    work_buffer_pattern: Pattern,
    pool_sounds: Vec<Sound>,
    work_buffer_sounds: Vec<Sound>,
    kits: Vec<Kit>,
    work_buffer_kit: Kit,
    globals: Vec<Global>,
    work_buffer_global: Global,
    settings: Settings,
    // Songs
}

impl Default for Rytm {
    fn default() -> Self {
        let mut patterns = vec![];
        for i in 0..127 {
            patterns.push(Pattern::try_default(i).unwrap());
        }

        let mut pool_sounds = vec![];
        for i in 0..127 {
            pool_sounds.push(Sound::try_default(i).unwrap());
        }

        let work_buffer_sounds = vec![Sound::work_buffer_default()];

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
    pub fn update_from_sysex_response(&mut self, response: &[u8]) -> Result<(), RytmError> {
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
                self.patterns[meta.obj_nr as usize] = pattern;
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

                match sound.sound_type() {
                    SoundType::Pool => {
                        self.pool_sounds[meta.obj_nr as usize] = sound;
                    }
                    SoundType::WorkBuffer => {
                        self.work_buffer_sounds[meta.obj_nr as usize] = sound;
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

                self.globals[meta.obj_nr as usize] = global;
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

    #[parameter_range(range = "pattern_index:0..=127")]
    pub fn encode_pattern_as_sysex_message(
        &self,
        pattern_index: usize,
    ) -> Result<Vec<u8>, RytmError> {
        self.patterns[pattern_index].as_sysex_message()
    }

    pub fn encode_work_buffer_pattern_as_sysex_message(&self) -> Result<Vec<u8>, RytmError> {
        self.work_buffer_pattern.as_sysex_message()
    }

    pub fn patterns(&self) -> &[Pattern] {
        &self.patterns
    }

    pub fn patterns_mut(&mut self) -> &mut [Pattern] {
        &mut self.patterns
    }

    pub fn work_buffer_pattern(&self) -> &Pattern {
        &self.work_buffer_pattern
    }

    pub fn work_buffer_pattern_mut(&mut self) -> &mut Pattern {
        &mut self.work_buffer_pattern
    }

    pub fn kits(&self) -> &[Kit] {
        &self.kits
    }

    pub fn kits_mut(&mut self) -> &mut [Kit] {
        &mut self.kits
    }

    pub fn work_buffer_kit(&self) -> &Kit {
        &self.work_buffer_kit
    }

    pub fn work_buffer_kit_mut(&mut self) -> &mut Kit {
        &mut self.work_buffer_kit
    }

    pub fn pool_sounds(&self) -> &[Sound] {
        &self.pool_sounds
    }

    pub fn pool_sounds_mut(&mut self) -> &mut [Sound] {
        &mut self.pool_sounds
    }

    pub fn work_buffer_sounds(&self) -> &[Sound] {
        &self.work_buffer_sounds
    }

    pub fn work_buffer_sounds_mut(&mut self) -> &mut [Sound] {
        &mut self.work_buffer_sounds
    }

    pub fn globals(&self) -> &[Global] {
        &self.globals
    }

    pub fn globals_mut(&mut self) -> &mut [Global] {
        &mut self.globals
    }

    pub fn work_buffer_global(&self) -> &Global {
        &self.work_buffer_global
    }

    pub fn work_buffer_global_mut(&mut self) -> &mut Global {
        &mut self.work_buffer_global
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }
}
