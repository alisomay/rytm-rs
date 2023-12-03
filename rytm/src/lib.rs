pub mod error;
pub mod pattern;
pub mod query;
pub mod sound;
pub(crate) mod sysex;
pub(crate) mod util;

use pattern::Pattern;
use sound::{Sound, SoundType};

use crate::error::ParameterError;
use rytm_rs_macro::parameter_range;
use rytm_sys::{ar_pattern_t, ar_sound_t};
use sysex::{decode_sysex_response_to_raw, SysexCompatible};

use self::error::RytmError;

/// Rytm is the main struct that holds all the patterns.
#[derive(Clone, Debug)]
pub struct Rytm {
    patterns: Vec<Pattern>,
    work_buffer_pattern: Pattern,
    pool_sounds: Vec<Sound>,
    work_buffer_sounds: Vec<Sound>,
    // Kits
    // Global
    // Settings
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

        Self {
            patterns,
            work_buffer_pattern: Pattern::work_buffer_default(),
            pool_sounds,
            work_buffer_sounds,
        }
    }
}

impl Rytm {
    #[parameter_range(range = "pattern_index:0..=127")]
    pub fn update_pattern_from_sysex_response(
        &mut self,
        response: &[u8],
        pattern_index: usize,
    ) -> Result<(), RytmError> {
        let (mut raw, meta) = decode_sysex_response_to_raw(response)?;

        unsafe {
            let raw_pattern: &ar_pattern_t = &*(raw.as_mut_ptr() as *const ar_pattern_t);
            self.patterns[pattern_index] =
                Pattern::try_from_raw(meta.obj_nr as usize, meta, raw_pattern)?;
            Ok(())
        }
    }

    pub fn update_sound_from_sysex_response(
        &mut self,
        response: &[u8],
        sound_index: usize,
    ) -> Result<(), RytmError> {
        let (mut raw, meta) = decode_sysex_response_to_raw(response)?;

        unsafe {
            let raw_sound: &ar_sound_t = &*(raw.as_mut_ptr() as *const ar_sound_t);

            let sound = Sound::try_from_raw(meta, raw_sound, None)?;

            match sound.sound_type() {
                SoundType::Pool => {
                    self.pool_sounds[sound_index] = sound;
                }
                SoundType::WorkBuffer => {
                    self.work_buffer_sounds[sound_index] = sound;
                }
                SoundType::KitQuery => {
                    unreachable!("Then it is not a sound query. Handle properly?")
                }
            }

            Ok(())
        }
    }

    #[parameter_range(range = "pattern_index:0..=127")]
    pub fn encode_pattern_as_sysex_message(
        &self,
        pattern_index: usize,
    ) -> Result<Vec<u8>, RytmError> {
        self.patterns[pattern_index].as_sysex_message()
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
}
