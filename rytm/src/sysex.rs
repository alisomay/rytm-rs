pub mod types;

use crate::error::{RytmError, SysexConversionError};
use rytm_sys::{ar_global_t, ar_kit_t, ar_pattern_t, ar_settings_t, ar_sound_t};
pub use types::*;

/// Pattern sysex response size for FW 1.70.
pub const PATTERN_SYSEX_SIZE: usize = 14988;
/// Kit sysex response size for FW 1.70.
pub const KIT_SYSEX_SIZE: usize = 2998;
/// Sound sysex response size for FW 1.70.
pub const SOUND_SYSEX_SIZE: usize = 201;
/// Settings sysex response size for FW 1.70.
pub const SETTINGS_SYSEX_SIZE: usize = 2401;
/// Global sysex response size for FW 1.70.
pub const GLOBAL_SYSEX_SIZE: usize = 107;
/// Song sysex response size for FW 1.70.
pub const SONG_SYSEX_SIZE: usize = 1506;

// TODO: Debug this carefully..
const SYSEX_MESSAGE_TYPE_BYTE_INDEX: usize = 7;

pub const PATTERN_RAW_SIZE: usize = std::mem::size_of::<ar_pattern_t>();
pub const KIT_RAW_SIZE: usize = std::mem::size_of::<ar_kit_t>();
pub const SOUND_RAW_SIZE: usize = std::mem::size_of::<ar_sound_t>();
pub const SETTINGS_RAW_SIZE: usize = std::mem::size_of::<ar_settings_t>();
pub const GLOBAL_RAW_SIZE: usize = std::mem::size_of::<ar_global_t>();

pub trait SysexCompatible {
    fn r#type(&self) -> SysexType;
    fn as_sysex_message(&self) -> Result<Vec<u8>, RytmError>;
}

#[macro_export]
macro_rules! impl_sysex_compatible {
    ($object_type:ty, $object_raw_type:ty, $object_encoder_function:ident, $object_sysex_type:expr, $object_sysex_size:expr) => {
        impl SysexCompatible for $object_type {
            fn as_sysex_message(&self) -> Result<Vec<u8>, RytmError> {
                let (sysex_meta, raw_object) = self.as_raw_parts();

                let raw_size = std::mem::size_of::<$object_raw_type>();
                let mut raw_buffer: Vec<u8> = Vec::with_capacity(raw_size);

                unsafe {
                    let raw: *const u8 = &raw_object as *const $object_raw_type as *const u8;
                    for i in 0..raw_size {
                        raw_buffer.push(*raw.add(i));
                    }
                }

                let mut encoded_buffer_length: u32 = 0;
                let mut encoded_buf = vec![0; $object_sysex_size];

                let mut meta = sysex_meta.into();
                let meta_ptr = &mut meta as *mut ar_sysex_meta_t;

                unsafe {
                    let return_code = $object_encoder_function(
                        encoded_buf.as_mut_ptr(),
                        raw_buffer.as_ptr(),
                        std::mem::size_of::<$object_raw_type>() as u32,
                        &mut encoded_buffer_length as *mut u32,
                        meta_ptr,
                    ) as u8;

                    if return_code != 0 {
                        return Err(SysexConversionError::from(return_code).into());
                    }

                    Ok(encoded_buf)
                }
            }

            fn r#type(&self) -> SysexType {
                $object_sysex_type
            }
        }
    };
}

/// This function assumes that the response is a valid sysex response.
///
/// It should be used in a context where this case is true and validity check is not necessary.
pub fn decode_sysex_response_to_raw(response: &[u8]) -> Result<(Vec<u8>, SysexMeta), RytmError> {
    let response_type: SysexType = response[SYSEX_MESSAGE_TYPE_BYTE_INDEX].try_into()?;
    dbg!(response_type);
    let (expected_response_size, expected_raw_size) = match response_type {
        SysexType::Pattern => (PATTERN_SYSEX_SIZE, PATTERN_RAW_SIZE),
        SysexType::Kit => (KIT_SYSEX_SIZE, KIT_RAW_SIZE),
        SysexType::Sound => (SOUND_SYSEX_SIZE, SOUND_RAW_SIZE),
        SysexType::Settings => (SETTINGS_SYSEX_SIZE, SETTINGS_RAW_SIZE),
        SysexType::Global => (GLOBAL_SYSEX_SIZE, GLOBAL_RAW_SIZE),
        // Song raw size is guessed for now.
        SysexType::Song => (SONG_SYSEX_SIZE, 1024 * 16),
    };

    if response.len() != expected_response_size {
        return Err(
            SysexConversionError::InvalidSize(expected_response_size, response.len()).into(),
        );
    }

    // Make a default meta struct to fill.
    let meta = SysexMeta::default();
    let mut meta: rytm_sys::ar_sysex_meta_t = meta.into();
    let meta_p = &mut meta as *mut rytm_sys::ar_sysex_meta_t;

    // The response buffer.
    let mut src_buf = response.as_ptr();
    let src_buf_p = &mut src_buf as *mut *const u8;
    let mut src_buf_size = response.len() as u32;
    let src_buf_size_p = &mut src_buf_size as *mut u32;

    // Will be calculated by the first call to ar_sysex_to_raw.
    let dst_buf_size = 0; // Big enough for the largest sysex message probably.
    let dest_buf_size_p = dst_buf_size as *mut u32;

    // The destination buffer, raw buffer.
    let mut dst_buf = vec![0_u8; expected_raw_size];
    let dst_buf_p = dst_buf.as_mut_slice().as_mut_ptr();

    unsafe {
        let return_code = rytm_sys::ar_sysex_to_raw(
            dst_buf_p,
            src_buf_p,
            src_buf_size_p,
            dest_buf_size_p,
            meta_p,
        ) as u8;

        if return_code != 0 {
            return Err(SysexConversionError::from(return_code).into());
        }
    }

    Ok((dst_buf, SysexMeta::from(&meta)))
}
