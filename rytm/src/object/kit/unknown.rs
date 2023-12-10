use rytm_sys::ar_kit_t;

/// Unknown fields for kit
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KitUnknown {
    pub __pad_name: u8,             /* @0x000C */
    pub __unknown_09d9: [u8; 0x4],  /* @0x07C6..0x07C9 */
    pub __unknown_0902: [u8; 0x15], /* @0x0902..0x0916 */
    pub __unknown_09d7: u8,         /* @0x09D7 (scene_id MSB?) */

    pub __unknown_arr2: [u8; 0x4], /* @0x09D9..0x09DD */
    pub __unknown_arr6: [u8; 35],  /* @0x0A0F..0x0A31 */

    pub __unused_pad1: u8,  /* @0x07CB   */
    pub __unused_pad2: u8,  /* @0x07CD   */
    pub __unused_pad3: u8,  /* @0x07CF   */
    pub __unused_pad4: u8,  /* @0x07D1   */
    pub __unused_pad5: u8,  /* @0x07D3   */
    pub __unused_pad6: u8,  /* @0x07D5   */
    pub __unused_pad7: u8,  /* @0x07D7   */
    pub __unused_pad8: u8,  /* @0x07D9   */
    pub __unused_pad9: u8,  /* @0x07DB ? */
    pub __unused_pad11: u8, /* @0x07DD ? */
    pub __unused_pad12: u8, /* @0x07DF   */
    pub __unused_pad13: u8, /* @0x07E1   */
    pub __unused_pad14: u8, /* @0x07E3   */
    pub __unused_pad15: u8, /* @0x07E5   */
    pub __unused_pad16: u8, /* @0x07E7   */
    pub __unused_pad17: u8, /* @0x07E9   */
    pub __unused_pad18: u8, /* @0x07EB   */
    pub __unused_pad19: u8, /* @0x07ED   */
    pub __unused_pad20: u8, /* @0x07EF   */
    pub __unused_pad21: u8, /* @0x07F1   */
    pub __unknown_fx_1: u8, /* @0x07F2   */
    pub __unknown_fx_2: u8, /* @0x07F3   */
    pub __unused_pad22: u8, /* @0x07F5   */
    pub __unused_pad23: u8, /* @0x07F7   */
    pub __unused_pad24: u8, /* @0x07F9   */
    pub __unused_pad25: u8, /* @0x07FB   */
    pub __unused_pad26: u8, /* @0x07FD   */
    pub __unused_pad27: u8, /* @0x07FF   */
    pub __unused_pad28: u8, /* @0x0801   */
    pub __unused_pad29: u8, /* @0x0803   */
    pub __unused_pad30: u8, /* @0x0805   */
    pub __unused_pad31: u8, /* @0x0807   */
    pub __unused_pad32: u8, /* @0x0809   */
    pub __unused_pad33: u8, /* @0x080B   */
    pub __unused_pad34: u8, /* @0x080D   */
    pub __unused_pad35: u8, /* @0x080F   */
    pub __unused_pad36: u8, /* @0x0811   */
}

impl Default for KitUnknown {
    fn default() -> Self {
        Self {
            __pad_name: 0,
            __unknown_09d9: [0; 0x4],
            __unknown_0902: [0; 0x15],
            __unknown_09d7: 0,

            __unknown_arr2: [0; 0x4],
            __unknown_arr6: [0; 35],

            __unused_pad1: 0,
            __unused_pad2: 0,
            __unused_pad3: 0,
            __unused_pad4: 0,
            __unused_pad5: 0,
            __unused_pad6: 0,
            __unused_pad7: 0,
            __unused_pad8: 0,
            __unused_pad9: 0,
            __unused_pad11: 0,
            __unused_pad12: 0,
            __unused_pad13: 0,
            __unused_pad14: 0,
            __unused_pad15: 0,
            __unused_pad16: 0,
            __unused_pad17: 0,
            __unused_pad18: 0,
            __unused_pad19: 0,
            __unused_pad20: 0,
            __unused_pad21: 0,
            __unknown_fx_1: 0,
            __unknown_fx_2: 0,
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
            __unused_pad33: 0,
            __unused_pad34: 0,
            __unused_pad35: 0,
            __unused_pad36: 0,
        }
    }
}

impl From<&ar_kit_t> for KitUnknown {
    fn from(raw_kit: &ar_kit_t) -> Self {
        Self {
            __pad_name: raw_kit.__pad_name,

            __unknown_arr2: raw_kit.__unknown_arr2,
            __unknown_0902: raw_kit.__unknown_0902,
            __unknown_09d7: raw_kit.__unknown_09D7,
            __unknown_09d9: raw_kit.__unknown_09D9,
            __unknown_arr6: raw_kit.__unknown_arr6,

            __unused_pad1: raw_kit.__unused_pad1,
            __unused_pad2: raw_kit.__unused_pad2,
            __unused_pad3: raw_kit.__unused_pad3,
            __unused_pad4: raw_kit.__unused_pad4,
            __unused_pad5: raw_kit.__unused_pad5,
            __unused_pad6: raw_kit.__unused_pad6,
            __unused_pad7: raw_kit.__unused_pad7,
            __unused_pad8: raw_kit.__unused_pad8,
            __unused_pad9: raw_kit.__unused_pad9,
            __unused_pad11: raw_kit.__unused_pad11,
            __unused_pad12: raw_kit.__unused_pad12,
            __unused_pad13: raw_kit.__unused_pad13,
            __unused_pad14: raw_kit.__unused_pad14,
            __unused_pad15: raw_kit.__unused_pad15,
            __unused_pad16: raw_kit.__unused_pad16,
            __unused_pad17: raw_kit.__unused_pad17,
            __unused_pad18: raw_kit.__unused_pad18,
            __unused_pad19: raw_kit.__unused_pad19,
            __unused_pad20: raw_kit.__unused_pad20,
            __unused_pad21: raw_kit.__unused_pad21,
            __unknown_fx_1: raw_kit.__unknown_fx_1,
            __unknown_fx_2: raw_kit.__unknown_fx_2,
            __unused_pad22: raw_kit.__unused_pad22,
            __unused_pad23: raw_kit.__unused_pad23,
            __unused_pad24: raw_kit.__unused_pad24,
            __unused_pad25: raw_kit.__unused_pad25,
            __unused_pad26: raw_kit.__unused_pad26,
            __unused_pad27: raw_kit.__unused_pad27,
            __unused_pad28: raw_kit.__unused_pad28,
            __unused_pad29: raw_kit.__unused_pad29,
            __unused_pad30: raw_kit.__unused_pad30,
            __unused_pad31: raw_kit.__unused_pad31,
            __unused_pad32: raw_kit.__unused_pad32,
            __unused_pad33: raw_kit.__unused_pad33,
            __unused_pad34: raw_kit.__unused_pad34,
            __unused_pad35: raw_kit.__unused_pad35,
            __unused_pad36: raw_kit.__unused_pad36,
        }
    }
}

impl KitUnknown {
    pub fn apply_to_raw_kit(&self, raw_kit: &mut ar_kit_t) {
        raw_kit.__pad_name = self.__pad_name;
        raw_kit.__unknown_arr2 = self.__unknown_arr2;

        raw_kit.__unknown_0902 = self.__unknown_0902;
        raw_kit.__unknown_09D7 = self.__unknown_09d7;
        raw_kit.__unknown_09D9 = self.__unknown_09d9;

        raw_kit.__unknown_arr6 = self.__unknown_arr6;
        raw_kit.__unused_pad1 = self.__unused_pad1;
        raw_kit.__unused_pad2 = self.__unused_pad2;
        raw_kit.__unused_pad3 = self.__unused_pad3;
        raw_kit.__unused_pad4 = self.__unused_pad4;
        raw_kit.__unused_pad5 = self.__unused_pad5;
        raw_kit.__unused_pad6 = self.__unused_pad6;
        raw_kit.__unused_pad7 = self.__unused_pad7;
        raw_kit.__unused_pad8 = self.__unused_pad8;
        raw_kit.__unused_pad9 = self.__unused_pad9;
        raw_kit.__unused_pad11 = self.__unused_pad11;
        raw_kit.__unused_pad12 = self.__unused_pad12;
        raw_kit.__unused_pad13 = self.__unused_pad13;
        raw_kit.__unused_pad14 = self.__unused_pad14;
        raw_kit.__unused_pad15 = self.__unused_pad15;
        raw_kit.__unused_pad16 = self.__unused_pad16;
        raw_kit.__unused_pad17 = self.__unused_pad17;
        raw_kit.__unused_pad18 = self.__unused_pad18;
        raw_kit.__unused_pad19 = self.__unused_pad19;
        raw_kit.__unused_pad20 = self.__unused_pad20;
        raw_kit.__unused_pad21 = self.__unused_pad21;
        raw_kit.__unknown_fx_1 = self.__unknown_fx_1;
        raw_kit.__unknown_fx_2 = self.__unknown_fx_2;
        raw_kit.__unused_pad22 = self.__unused_pad22;
        raw_kit.__unused_pad23 = self.__unused_pad23;
        raw_kit.__unused_pad24 = self.__unused_pad24;
        raw_kit.__unused_pad25 = self.__unused_pad25;
        raw_kit.__unused_pad26 = self.__unused_pad26;
        raw_kit.__unused_pad27 = self.__unused_pad27;
        raw_kit.__unused_pad28 = self.__unused_pad28;
        raw_kit.__unused_pad29 = self.__unused_pad29;
        raw_kit.__unused_pad30 = self.__unused_pad30;
        raw_kit.__unused_pad31 = self.__unused_pad31;
        raw_kit.__unused_pad32 = self.__unused_pad32;
        raw_kit.__unused_pad33 = self.__unused_pad33;
        raw_kit.__unused_pad34 = self.__unused_pad34;
        raw_kit.__unused_pad35 = self.__unused_pad35;
        raw_kit.__unused_pad36 = self.__unused_pad36;
    }
}
