mod global;
mod kit;
mod pattern;
mod raw;
mod settings;
mod song;
mod sound;

pub use global::GlobalQuery;
pub use kit::KitQuery;
pub use pattern::PatternQuery;
pub use raw::RawQuery;
pub use settings::SettingsQuery;
pub use song::SongQuery;
pub use sound::SoundQuery;

/// The size of the rytm sysex query in bytes.
///
/// `0xF0 0x00 0x20 0x3c 0x07 0x00 <id> 0x01 0x01 <nr> 0x00 0x00 0x00 0x05 0xF7`
const RYTM_SYSEX_QUERY_SIZE: usize = rytm_sys::AR_SYSEX_REQUEST_MSG_SZ as usize;

use super::error::{RytmError, SysexConversionError};
use crate::{
    sysex::{AnySysexType, SysexMeta},
    SysexCompatible,
};

/// A trait which is implemented by all structures which can be converted to rytm sysex query messages.
pub trait ObjectQuery {
    /// Returns the sysex type of the object.
    fn sysex_type(&self) -> AnySysexType;

    /// Returns the device id of the object.
    fn device_id(&self) -> u8;

    /// Returns the object number (index) of the object.
    fn obj_nr(&self) -> u16;

    /// Returns the sysex meta data for the object creating it.
    fn as_sysex_meta(&self) -> SysexMeta {
        SysexMeta {
            container_version: 0x0101,
            dev_id: self.device_id(),
            obj_type: self.sysex_type().into(),
            obj_nr: self.obj_nr(),
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    /// Returns the information if this query is targeting the work buffer.
    fn is_targeting_work_buffer(&self) -> bool {
        self.obj_nr() >= 128
    }
}

impl<T: ObjectQuery> SysexCompatible for T {
    fn sysex_type(&self) -> AnySysexType {
        self.sysex_type()
    }

    fn as_sysex(&self) -> Result<Vec<u8>, RytmError> {
        let mut buffer = vec![0; RYTM_SYSEX_QUERY_SIZE];
        let destination_buffer = buffer.as_mut_slice();
        let meta: rytm_sys::ar_sysex_meta_t = self.as_sysex_meta().into();

        // TODO: Write safety comment.
        unsafe {
            let return_code = rytm_sys::ar_sysex_request(
                destination_buffer.as_mut_ptr(),
                &meta as *const rytm_sys::ar_sysex_meta_t,
            ) as u8;

            if return_code != 0 {
                return Err(SysexConversionError::from(return_code).into());
            }
        }

        Ok(buffer)
    }
}
