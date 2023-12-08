use rytm_sys::ar_sound_t;

/// Unknown fields for sound
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoundUnknown {
    pub __unknown_arr1: [u8; 12], /* @0x0000 reads BE EF BA CE 00 00 00 04 00 00 00 00 */
    pub __pad_name: u8,           /* @0x000C */
    pub __unknown_006f: [u8; 0xd], /* @0x006f..0x007B   */
    pub __unknown_007e: [u8; 16], /* @0x007E..0x008D */
    pub __unknown_009e: [u8; 4],  /* @0x009E..0x00A1 */

    pub __unused_pad9: u8,  /* @0x002d (lsb, always 0) */
    pub __unused_pad10: u8, /* @0x002f (lsb, always 0) */
    pub __unused_pad11: u8, /* @0x0031 (lsb, always 0) */
    pub __unused_pad12: u8, /* @0x0033 (lsb, always 0) */
    pub __unused_pad15: u8, /* @0x0039 */
    pub __unused_pad16: u8, /* @0x003b (lsb, always 0) */
    pub __unused_pad17: u8, /* @0x003d (lsb, always 0) */
    pub __unused_pad18: u8, /* @0x003f (lsb, always 0) */
    pub __unused_pad19: u8, /* @0x0041 (lsb, always 0) */
    pub __unused_pad20: u8, /* @0x0043 (lsb, always 0) */
    pub __unused_pad21: u8, /* @0x0045 (lsb, always 0) */
    pub __unused_pad22: u8, /* @0x0047 (lsb, always 0) */
    pub __unused_pad23: u8, /* @0x0049 (lsb, always 0) */
    pub __unused_pad24: u8, /* @0x004b (lsb, always 0) */
    pub __unused_pad25: u8, /* @0x004d (lsb, always 0) */
    pub __unused_pad26: u8, /* @0x004f (lsb, always 0) */
    pub __unused_pad27: u8, /* @0x0051 (lsb, always 0) */
    pub __unused_pad28: u8, /* @0x0053 (lsb, always 0) */
    pub __unused_pad29: u8, /* @0x0055 (lsb, always 0) */
    pub __unused_pad30: u8, /* @0x0057 (lsb, always 0) */
    pub __unused_pad31: u8, /* @0x0059 (lsb, always 0) */
    pub __unused_pad32: u8, /* @0x005b (lsb, always 0) */
    pub __unused005d: u8,   /* @0x005d (lsb, always 0)         */
    pub __unused_pad33: u8, /* @0x005f (lsb, always 0) */
    pub __unused_pad34: u8, /* @0x0061 (lsb, always 0) */
    pub __unused_pad35: u8, /* @0x0063 (lsb, always 0) */
    pub __unused_pad36: u8, /* @0x0065 (lsb, always 0) */
    pub __unused_pad37: u8, /* @0x0067 (lsb, always 0) */
    pub __unused_pad38: u8, /* @0x0069 (lsb, always 0) */
    pub __unused_pad39: u8, /* @0x006b (lsb, always 0) */
}

impl Default for SoundUnknown {
    fn default() -> Self {
        Self {
            __unknown_arr1: [
                0xBE, 0xEF, 0xBA, 0xCE, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
            ],
            __pad_name: 0,
            __unknown_006f: [0; 0xd],
            __unknown_007e: [0; 16],
            __unknown_009e: [0; 4],
            __unused_pad9: 0,
            __unused_pad10: 0,
            __unused_pad11: 0,
            __unused_pad12: 0,
            __unused_pad15: 0,
            __unused_pad16: 0,
            __unused_pad17: 0,
            __unused_pad18: 0,
            __unused_pad19: 0,
            __unused_pad20: 0,
            __unused_pad21: 0,
            __unused_pad22: 0,
            __unused_pad23: 0,
            __unused_pad24: 0,
            __unused_pad25: 0,
            __unused_pad26: 0,
            __unused_pad27: 0,
            __unused_pad28: 0,
            __unused_pad29: 0,
            __unused_pad30: 0,
            __unused_pad31: 0,
            __unused_pad32: 0,
            __unused005d: 0,
            __unused_pad33: 0,
            __unused_pad34: 0,
            __unused_pad35: 0,
            __unused_pad36: 0,
            __unused_pad37: 0,
            __unused_pad38: 0,
            __unused_pad39: 0,
        }
    }
}

impl From<&ar_sound_t> for SoundUnknown {
    fn from(raw_sound: &ar_sound_t) -> Self {
        Self {
            __unknown_arr1: raw_sound.__unknown_arr1,
            __pad_name: raw_sound.__pad_name,
            __unknown_006f: raw_sound.__unknown_006F,
            __unknown_009e: raw_sound.__unknown_009E,
            __unknown_007e: raw_sound.__unknown_007E,
            __unused_pad9: raw_sound.__unused_pad9,
            __unused_pad10: raw_sound.__unused_pad10,
            __unused_pad11: raw_sound.__unused_pad11,
            __unused_pad12: raw_sound.__unused_pad12,
            __unused_pad15: raw_sound.__unused_pad15,
            __unused_pad16: raw_sound.__unused_pad16,
            __unused_pad17: raw_sound.__unused_pad17,
            __unused_pad18: raw_sound.__unused_pad18,
            __unused_pad19: raw_sound.__unused_pad19,
            __unused_pad20: raw_sound.__unused_pad20,
            __unused_pad21: raw_sound.__unused_pad21,
            __unused_pad22: raw_sound.__unused_pad22,
            __unused_pad23: raw_sound.__unused_pad23,
            __unused_pad24: raw_sound.__unused_pad24,
            __unused_pad25: raw_sound.__unused_pad25,
            __unused_pad26: raw_sound.__unused_pad26,
            __unused_pad27: raw_sound.__unused_pad27,
            __unused_pad28: raw_sound.__unused_pad28,
            __unused_pad29: raw_sound.__unused_pad29,
            __unused_pad30: raw_sound.__unused_pad30,
            __unused_pad31: raw_sound.__unused_pad31,
            __unused_pad32: raw_sound.__unused_pad32,
            __unused005d: raw_sound.__unused005D,
            __unused_pad33: raw_sound.__unused_pad33,
            __unused_pad34: raw_sound.__unused_pad34,
            __unused_pad35: raw_sound.__unused_pad35,
            __unused_pad36: raw_sound.__unused_pad36,
            __unused_pad37: raw_sound.__unused_pad37,
            __unused_pad38: raw_sound.__unused_pad38,
            __unused_pad39: raw_sound.__unused_pad39,
        }
    }
}

impl SoundUnknown {
    pub fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        raw_sound.__unknown_arr1 = self.__unknown_arr1;
        raw_sound.__pad_name = self.__pad_name;
        raw_sound.__unknown_006F = self.__unknown_006f;
        raw_sound.__unknown_009E = self.__unknown_009e;
        raw_sound.__unknown_007E = self.__unknown_007e;
        raw_sound.__unused_pad9 = self.__unused_pad9;
        raw_sound.__unused_pad10 = self.__unused_pad10;
        raw_sound.__unused_pad11 = self.__unused_pad11;
        raw_sound.__unused_pad12 = self.__unused_pad12;
        raw_sound.__unused_pad15 = self.__unused_pad15;
        raw_sound.__unused_pad16 = self.__unused_pad16;
        raw_sound.__unused_pad17 = self.__unused_pad17;
        raw_sound.__unused_pad18 = self.__unused_pad18;
        raw_sound.__unused_pad19 = self.__unused_pad19;
        raw_sound.__unused_pad20 = self.__unused_pad20;
        raw_sound.__unused_pad21 = self.__unused_pad21;
        raw_sound.__unused_pad22 = self.__unused_pad22;
        raw_sound.__unused_pad23 = self.__unused_pad23;
        raw_sound.__unused_pad24 = self.__unused_pad24;
        raw_sound.__unused_pad25 = self.__unused_pad25;
        raw_sound.__unused_pad26 = self.__unused_pad26;
        raw_sound.__unused_pad27 = self.__unused_pad27;
        raw_sound.__unused_pad28 = self.__unused_pad28;
        raw_sound.__unused_pad29 = self.__unused_pad29;
        raw_sound.__unused_pad30 = self.__unused_pad30;
        raw_sound.__unused_pad31 = self.__unused_pad31;
        raw_sound.__unused_pad32 = self.__unused_pad32;
        raw_sound.__unused005D = self.__unused005d;
        raw_sound.__unused_pad33 = self.__unused_pad33;
        raw_sound.__unused_pad34 = self.__unused_pad34;
        raw_sound.__unused_pad35 = self.__unused_pad35;
        raw_sound.__unused_pad36 = self.__unused_pad36;
        raw_sound.__unused_pad37 = self.__unused_pad37;
        raw_sound.__unused_pad38 = self.__unused_pad38;
        raw_sound.__unused_pad39 = self.__unused_pad39;
    }
}
