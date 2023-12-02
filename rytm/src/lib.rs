pub mod error;
pub mod pattern;
pub mod query;
pub(crate) mod sysex;
pub(crate) mod util;

use pattern::Pattern;

use crate::error::ParameterError;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_pattern_t;
use sysex::SysexCompatible;

use self::error::RytmError;

/// Rytm is the main struct that holds all the patterns.
#[derive(Clone, Debug)]
pub struct Rytm {
    patterns: Vec<Pattern>,
    pattern_at_work_buffer: Pattern,
    // Kits
    // Sounds
    // Global
    // Settings
}

impl Default for Rytm {
    fn default() -> Self {
        let mut patterns = vec![];
        for i in 0..127 {
            patterns.push(Pattern::try_default(i).unwrap());
        }
        Self {
            patterns,
            pattern_at_work_buffer: Pattern::work_buffer_default(),
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
        let (mut raw, meta) = util::decode_sysex_response_to_raw(response)?;

        unsafe {
            let raw_pattern: &ar_pattern_t = &*(raw.as_mut_ptr() as *const ar_pattern_t);

            self.patterns[pattern_index] =
                Pattern::try_from_raw(meta.obj_nr as usize, meta, raw_pattern)?;

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
}
