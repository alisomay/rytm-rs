pub(crate) mod defaults;
pub mod error;
pub mod object;
pub mod prelude;
pub mod query;
pub(crate) mod sysex;

// TODO: pub(crate) before wrapping up.
pub mod util;

pub use sysex::{AnySysexType, SysexCompatible, SysexType};

use self::error::RytmError;
use crate::error::ParameterError;
use error::SysexConversionError;
use object::{
    global::Global,
    kit::Kit,
    pattern::Pattern,
    settings::Settings,
    sound::{Sound, SoundType},
};

use defaults::*;
use rytm_sys::{ar_global_t, ar_kit_t, ar_pattern_t, ar_settings_t, ar_sound_t};
use sysex::decode_sysex_response_to_raw;

/// [`RytmProject`] represents the state of the analog rytm.
///
/// It contains all structures scoped to an Analog Rytm MKII FW 1.70 project.
#[derive(Clone, Debug)]
pub struct RytmProject {
    work_buffer: RytmProjectWorkBuffer,
    patterns: Vec<Pattern>,
    pool_sounds: [Sound; POOL_SOUND_MAX_COUNT],
    kits: Vec<Kit>,
    globals: [Global; GLOBAL_MAX_COUNT],
    // TODO: Songs (16)
    settings: Settings,
}

impl Default for RytmProject {
    fn default() -> Self {
        let mut patterns = Vec::with_capacity(PATTERN_MAX_COUNT);
        for i in 0..PATTERN_MAX_COUNT {
            patterns.push(Pattern::try_default(i).unwrap());
        }

        let mut kits = Vec::with_capacity(KIT_MAX_COUNT);
        for i in 0..KIT_MAX_COUNT {
            kits.push(Kit::try_default(i).unwrap());
        }

        Self {
            work_buffer: RytmProjectWorkBuffer::default(),
            patterns,
            pool_sounds: default_pool_sounds(),
            kits,
            globals: default_globals(),
            settings: Settings::default(),
        }
    }
}

impl RytmProject {
    /// Updates the Rytm struct from a sysex response.
    ///
    /// All encoding/decoding is done in [`RytmProject`], so this is the only method that needs to be called to update the struct when a sysex response is received.
    ///
    /// # Important
    ///
    /// This method will act as a no-op if the given slice does not contain a valid sysex message.
    /// The check is performed by checking the first and last byte of the slice.
    /// This behaviour is preferred to returning an error, as it is expected to be called in a midi callback which receives not just sysex messages.
    ///
    /// # Errors
    ///
    /// This method will return an error
    ///
    /// - If the sysex message is invalid in the context of Rytm
    /// - If the sysex message is valid, but the object type is not supported or implemented yet. Example: [`crate::error::RytmError::SysexConversionError::Unimplemented`] variant.
    /// - If the sysex message is incomplete, this sometimes happens in the initial parts of the transmission and is a behaviour of Rytm.
    /// You may check for the error [`crate::error::RytmError::SysexConversionError::ShortRead`] and ignore it.
    /// - If the sysex message is valid, but the size of the expected object does not match the size of the received object.
    /// This may happen if the firmware version of Rytm is different than the one this library supports which is currently FW 1.70 only.
    /// Never happened to me in practice but a cut transmission may also cause this in theory.
    /// - All other  [`crate::error::RytmError::SysexConversionError`] variants are possible which are inherited from [libanalogrytm](https://github.com/bsp2/libanalogrytm).
    pub fn update_from_sysex_response(&mut self, response: &[u8]) -> Result<(), RytmError> {
        if response.len() < 2 {
            return Err(SysexConversionError::ShortRead.into());
        }

        // Ignore non-sysex messages.
        if !(response[0] == 0xF0 && response[response.len() - 1] == 0xF7) {
            return Ok(());
        }

        // Raw buffer and metadata about the sysex message.
        let (mut raw, meta) = decode_sysex_response_to_raw(response)?;

        // `& 0b0111_1111` is to get rid of the bit that indicates if the object is in the work buffer or not.
        // `- 0x80` is to get the actual index of the object in the work buffer. Example, there could be 12 sounds in the work buffer.
        match meta.object_type()? {
            SysexType::Pattern => {
                let raw_pattern: &ar_pattern_t =
                    unsafe { &*(raw.as_mut_ptr() as *const ar_pattern_t) };
                let pattern = Pattern::try_from_raw(meta, raw_pattern)?;

                if meta.is_targeting_work_buffer() {
                    self.work_buffer.pattern = pattern;
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
                    self.work_buffer.kit = kit;
                    return Ok(());
                }

                self.kits[(meta.obj_nr & 0b0111_1111) as usize] = kit;
                Ok(())
            }

            SysexType::Sound => {
                let raw_sound: &ar_sound_t = unsafe { &*(raw.as_mut_ptr() as *const ar_sound_t) };
                let sound = Sound::try_from_raw(meta, raw_sound, None)?;

                match sound.sound_type() {
                    SoundType::Pool => {
                        let index = (meta.obj_nr & 0b0111_1111) as usize;
                        self.pool_sounds[index] = sound;
                    }
                    SoundType::WorkBuffer => {
                        let index = (meta.obj_nr - 0x80) as usize;
                        self.work_buffer.sounds[index] = sound;
                    }
                    SoundType::KitQuery => {
                        unreachable!("A response from a sound query can not contain a sound which is part of a kit.")
                    }
                };

                Ok(())
            }

            SysexType::Global => {
                let raw_global: &ar_global_t =
                    unsafe { &*(raw.as_mut_ptr() as *const ar_global_t) };
                let global = Global::try_from_raw(meta, raw_global)?;

                if meta.is_targeting_work_buffer() {
                    self.work_buffer.global = global;
                    return Ok(());
                }

                let index = (meta.obj_nr & 0b0111_1111) as usize;
                self.globals[index] = global;
                Ok(())
            }

            // There is only a single settings object for a project.
            SysexType::Settings => {
                let raw_settings: &ar_settings_t =
                    unsafe { &*(raw.as_mut_ptr() as *const ar_settings_t) };
                let settings = Settings::try_from_raw(meta, raw_settings)?;
                self.settings = settings;
                Ok(())
            }

            // TODO: Implement Song
            SysexType::Song => Err(SysexConversionError::Unimplemented("Song".to_owned()).into()),
        }
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

    /// Get the work buffer structures.
    pub fn work_buffer(&self) -> &RytmProjectWorkBuffer {
        &self.work_buffer
    }

    /// Get the work buffer structures mutably.
    pub fn work_buffer_mut(&mut self) -> &mut RytmProjectWorkBuffer {
        &mut self.work_buffer
    }
}

/// [`RytmProject`] represents the state of the analog rytm work buffer.
///
/// It contains all structures scoped to an Analog Rytm MKII FW 1.70 project work buffer.
#[derive(Clone, Debug)]
pub struct RytmProjectWorkBuffer {
    pattern: Pattern,
    kit: Kit,
    sounds: [Sound; TRACK_MAX_COUNT],
    global: Global,
    // TODO: Work buffer song
}

impl Default for RytmProjectWorkBuffer {
    fn default() -> Self {
        Self {
            pattern: Pattern::work_buffer_default(),
            kit: Kit::work_buffer_default(),
            sounds: default_work_buffer_sounds(),
            global: Global::work_buffer_default(),
        }
    }
}

impl RytmProjectWorkBuffer {
    /// Get the pattern in the work buffer.
    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// Get the kit in the work buffer.
    pub fn kit(&self) -> &Kit {
        &self.kit
    }

    /// Get the sounds in the work buffer.
    ///
    /// Total of 12 sounds for 12 tracks.
    pub fn sounds(&self) -> &[Sound] {
        &self.sounds
    }

    /// Get the global in the work buffer.
    pub fn global(&self) -> &Global {
        &self.global
    }

    /// Get the pattern in the work buffer mutably.
    pub fn pattern_mut(&mut self) -> &mut Pattern {
        &mut self.pattern
    }

    /// Get the kit in the work buffer mutably.
    pub fn kit_mut(&mut self) -> &mut Kit {
        &mut self.kit
    }

    /// Get the sounds in the work buffer mutably.
    ///
    /// Total of 12 sounds for 12 tracks.
    pub fn sounds_mut(&mut self) -> &mut [Sound] {
        &mut self.sounds
    }

    /// Get the global in the work buffer mutably.
    pub fn global_mut(&mut self) -> &mut Global {
        &mut self.global
    }
}
