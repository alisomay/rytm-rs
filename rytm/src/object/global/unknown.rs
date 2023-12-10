use rytm_sys::ar_global_t;

/// Unknown fields for global
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalUnknown {
    pub __unknown0x09_0x0a: [u8; 2],
    pub __unknown0x31: u8,
    pub __unknown0x36_0x45: [u8; 16], /* @?0x36..0x45  */
    pub __unknown0x50_0x4f: [u8; 6],  /* @?0x50..0x4F  */
}

impl Default for GlobalUnknown {
    fn default() -> Self {
        Self {
            /* @?0x09..0x0A  Currently reads  0x40, 0x00 */
            __unknown0x09_0x0a: [0x40, 0x00],
            /* ?@0x31        */
            __unknown0x31: 0,
            // All zeros. It is suspicious since it is exactly 16 bytes long, maybe related to midi channels?
            __unknown0x36_0x45: [0; 16],
            // All zeros.
            __unknown0x50_0x4f: [0; 6],
        }
    }
}

impl From<&ar_global_t> for GlobalUnknown {
    fn from(raw_global: &ar_global_t) -> Self {
        Self {
            __unknown0x09_0x0a: raw_global.__unknown0x09_0x0A,
            __unknown0x31: raw_global.__unknown0x31,
            __unknown0x36_0x45: raw_global.__unknown0x36_0x45,
            __unknown0x50_0x4f: raw_global.__unknown0x50_0x4F,
        }
    }
}

impl GlobalUnknown {
    pub(crate) fn apply_to_raw_global(&self, raw_global: &mut ar_global_t) {
        raw_global.__unknown0x09_0x0A = self.__unknown0x09_0x0a;
        raw_global.__unknown0x31 = self.__unknown0x31;
        raw_global.__unknown0x36_0x45 = self.__unknown0x36_0x45;
        raw_global.__unknown0x50_0x4F = self.__unknown0x50_0x4f;
    }
}
