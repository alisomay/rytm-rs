use rytm_sys::ar_settings_t;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

/// Unknown fields for settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SettingsUnknown {
    /// Always the duplicate of selected_track
    pub __selected_track_duplicate: u8,
    /// Always 0x00
    pub __unknown_0x000b: u8,
    /// @0x000E..0x0014 All zeros.
    pub __unknown0x000e_0x0014: [u8; 7],
    ///  @0x0017..0x0019 All zeros.
    pub __unknown0x0017_0x0019: [u8; 3],
    #[serde(with = "BigArray")]
    /// The response continues with the repeating 16 byte pattern of 0xFF_FF_FF_FF 0x00_00_00_00 0x00_00_00_00 0x00_00_00_00
    /// The repeating pattern repeats 128 times. Total length of 2048 bytes.
    pub __unknown0x001f: [u8; 16 * 128],
    /// @0x081F Always 0x01
    pub __unknown0x081f: u8,
    /// @0x0821..0x0826 All zeros.
    pub __unknown0x0821_0x0826: [u8; 6],
}

impl Default for SettingsUnknown {
    fn default() -> Self {
        Self {
            __selected_track_duplicate: 0,
            __unknown_0x000b: 0,
            __unknown0x000e_0x0014: [0; 7],
            __unknown0x0017_0x0019: [0; 3],
            __unknown0x001f: [0; 16 * 128],
            __unknown0x081f: 0,
            __unknown0x0821_0x0826: [0; 6],
        }
    }
}

impl From<&ar_settings_t> for SettingsUnknown {
    fn from(raw_settings: &ar_settings_t) -> Self {
        Self {
            __selected_track_duplicate: raw_settings._selected_track_duplicate,
            __unknown_0x000b: raw_settings.__unknown_0x000B,
            __unknown0x000e_0x0014: raw_settings.__unknown0x000E_0x0014,
            __unknown0x0017_0x0019: raw_settings.__unknown0x0017_0x0019,
            __unknown0x001f: raw_settings.__unknown0x001F,
            __unknown0x081f: raw_settings.__unknown0x081F,
            __unknown0x0821_0x0826: raw_settings.__unknown0x0821_0x0826,
        }
    }
}

impl SettingsUnknown {
    pub fn apply_to_raw_settings(&self, raw_settings: &mut ar_settings_t) {
        raw_settings._selected_track_duplicate = self.__selected_track_duplicate;
        raw_settings.__unknown_0x000B = self.__unknown_0x000b;
        raw_settings.__unknown0x000E_0x0014 = self.__unknown0x000e_0x0014;
        raw_settings.__unknown0x0017_0x0019 = self.__unknown0x0017_0x0019;
        raw_settings.__unknown0x001F = self.__unknown0x001f;
        raw_settings.__unknown0x081F = self.__unknown0x081f;
        raw_settings.__unknown0x0821_0x0826 = self.__unknown0x0821_0x0826;
    }
}
