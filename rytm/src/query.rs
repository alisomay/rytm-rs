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
const RYTM_SYSEX_QUERY_SIZE: usize = 15;

use super::error::{RytmError, SysexConversionError};
use crate::sysex::SysexMeta;

pub trait ObjectQuery
where
    Self::SysexTypeExpression: Into<u8>,
{
    type SysexTypeExpression;

    fn r#type(&self) -> Self::SysexTypeExpression;
    fn device_id(&self) -> u8;
    fn obj_nr(&self) -> u16;

    fn as_sysex_meta(&self) -> SysexMeta {
        SysexMeta {
            container_version: 0x0101,
            dev_id: self.device_id(),
            obj_type: self.r#type().into(),
            obj_nr: self.obj_nr(),
            // Calculated in libanalogrytm, they're dummy values here in this state.
            chksum: 0,
            data_size: 0,
        }
    }

    fn is_targeting_work_buffer(&self) -> bool {
        self.obj_nr() >= 128
    }

    fn serialize_to_sysex(&self) -> Result<Vec<u8>, RytmError> {
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
