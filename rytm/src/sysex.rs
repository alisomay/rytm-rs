//! Internal module for sysex related operations.

// All casts in this file are intended or safe within the context of this library.
//
// One can change `allow` to `warn` to review them if necessary.
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

pub mod types;

use crate::error::{RytmError, SysexConversionError};
use rytm_sys::{ar_global_t, ar_kit_t, ar_pattern_t, ar_settings_t, ar_sound_t};
use std::ptr::addr_of_mut;
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

const SYSEX_MESSAGE_TYPE_BYTE_INDEX: usize = 6;

pub const PATTERN_RAW_SIZE: usize = std::mem::size_of::<ar_pattern_t>();
pub const KIT_RAW_SIZE: usize = std::mem::size_of::<ar_kit_t>();
pub const SOUND_RAW_SIZE: usize = std::mem::size_of::<ar_sound_t>();
pub const SETTINGS_RAW_SIZE: usize = std::mem::size_of::<ar_settings_t>();
pub const GLOBAL_RAW_SIZE: usize = std::mem::size_of::<ar_global_t>();

/// Meta type for sysex messages.
///
/// Can represent known and unknown sysex types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnySysexType {
    Known(SysexType),
    Unknown(u8),
}

impl From<SysexType> for AnySysexType {
    fn from(t: SysexType) -> Self {
        Self::Known(t)
    }
}

impl From<u8> for AnySysexType {
    fn from(t: u8) -> Self {
        if let Ok(t) = SysexType::try_from_dump_id(t) {
            return Self::from(t);
        }
        if let Ok(t) = SysexType::try_from(t) {
            return Self::from(t);
        }
        Self::Unknown(t)
    }
}

impl From<AnySysexType> for u8 {
    fn from(t: AnySysexType) -> Self {
        match t {
            AnySysexType::Known(t) => t.into(),
            AnySysexType::Unknown(t) => t,
        }
    }
}

/// A trait which is implemented by all objects which can be converted to sysex messages including queries and rytm project structures.
pub trait SysexCompatible {
    /// Returns the sysex type of the object.
    fn sysex_type(&self) -> AnySysexType;

    /// Serializes the object to a sysex message.
    ///
    /// # Errors
    ///
    /// May return a [`SysexConversionError`](crate::error::SysexConversionError) if the conversion fails.
    fn as_sysex(&self) -> Result<Vec<u8>, RytmError>;
}

// Helper macro to implement the SysexCompatible trait for a given object.
#[macro_export]
macro_rules! impl_sysex_compatible {
    ($object_type:ty, $object_raw_type:ty, $object_encoder_function:ident, $object_sysex_type:expr, $object_sysex_size:expr) => {
        impl SysexCompatible for $object_type {
            fn as_sysex(&self) -> Result<Vec<u8>, RytmError> {
                let (sysex_meta, raw_object) = self.as_raw_parts();

                let raw_size = std::mem::size_of::<$object_raw_type>();
                let mut raw_buffer: Vec<u8> = Vec::with_capacity(raw_size);

                unsafe {
                    let raw: *const u8 = (&raw_object as *const $object_raw_type).cast::<u8>();
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

            fn sysex_type(&self) -> AnySysexType {
                $object_sysex_type.into()
            }
        }
    };
}

/// This function assumes that the response is a valid sysex response.
///
/// It should be used in a context where this case is true and validity check is not necessary.
pub fn decode_sysex_response_to_raw(response: &[u8]) -> Result<(Vec<u8>, SysexMeta), RytmError> {
    if response.get(SYSEX_MESSAGE_TYPE_BYTE_INDEX).is_none() {
        // Message is too short, rytm sometimes sends incomplete sysex messages especially in the initial parts of the transmission.
        // One can check for this error and ignore it.
        return Err(SysexConversionError::ShortRead.into());
    }
    let response_type = SysexType::try_from_dump_id(response[SYSEX_MESSAGE_TYPE_BYTE_INDEX])?;
    let (expected_response_size, expected_raw_size) = match response_type {
        SysexType::Pattern => (PATTERN_SYSEX_SIZE, PATTERN_RAW_SIZE),
        SysexType::Kit => (KIT_SYSEX_SIZE, KIT_RAW_SIZE),
        SysexType::Sound => (SOUND_SYSEX_SIZE, SOUND_RAW_SIZE),
        SysexType::Settings => (SETTINGS_SYSEX_SIZE, SETTINGS_RAW_SIZE),
        SysexType::Global => (GLOBAL_SYSEX_SIZE, GLOBAL_RAW_SIZE),
        // Song raw size is guessed for now.
        SysexType::Song => (SONG_SYSEX_SIZE, 1024 * 16),
    };

    // Check for completeness.
    if response.len() != expected_response_size {
        return Err(
            SysexConversionError::InvalidSize(expected_response_size, response.len()).into(),
        );
    }

    // Make a default meta struct to fill.
    let meta = SysexMeta::default();
    let mut meta: rytm_sys::ar_sysex_meta_t = meta.into();

    // The response buffer.
    let mut src_buf = response.as_ptr();

    // u32 is big enough for any possible buffer in this context.
    #[allow(clippy::cast_possible_truncation)]
    let mut src_buf_size = response.len() as u32;

    // Will be calculated by the first call to ar_sysex_to_raw.
    let mut dst_buf_size = 0u32;

    // The destination buffer, raw buffer.
    let mut dst_buf = vec![0_u8; expected_raw_size];

    unsafe {
        // The count of return error codes from `rytm-sys` is far below 255.
        #[allow(clippy::cast_possible_truncation)]
        let return_code = rytm_sys::ar_sysex_to_raw(
            dst_buf.as_mut_slice().as_mut_ptr(),
            addr_of_mut!(src_buf),
            addr_of_mut!(src_buf_size),
            addr_of_mut!(dst_buf_size),
            addr_of_mut!(meta),
        ) as u8;

        if return_code != 0 {
            return Err(SysexConversionError::from(return_code).into());
        }
    }

    Ok((dst_buf, SysexMeta::from(&meta)))
}
