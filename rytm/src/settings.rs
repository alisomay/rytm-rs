use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_settings_t;

use crate::error::{ParameterError, RytmError};
use crate::sysex::SysexMeta;

// /*
//  *
//  ** Settings structure
//  *
//  *    0x0827(2087) bytes (v5/FW1.70)
//  *
//  */
// typedef struct {
//    sU8 version[4];                       /* @0x0000..0x0003  32 bit version number (0x00, 0x00, 0x00, 0x03) */
//    sU8 bpm_msb;                          /* @0x0004          multiplied by 120. (used when BPM mode=PRJ)                                      */
//    sU8 bpm_lsb;                          /* @0x0005                                                                                           */
//    sU8 selected_track;                   /* @0x0006          range=0..11                                                                      */
//    sU8 _selected_track_duplicate;        /* @0x0007          duplicate of the selected_track, I don't know why it exists.                     */
//    sU8 selected_trig_or_parameter_menu;  /* @0x0008          range=0..5, 0=TRIG, 1=SRC, 2=SMPL, 3=FLTR, 4=AMP, 5=LFO                          */
//    sU8 selected_fx_menu;                 /* @0x0009          range=0..5, 0=TODO, 1=Delay, 2=Reverb, 3=Dist, 4=Comp, 5=LFO                     */
//    sU8 selected_page;                    /* @0x000A          range=0..3, 0=Page 1, 1=Page 2, 2=Page 3, 3=Page 4, only when manually selected. */
//    sU8 __unknown_0x000B;                 /* ?@0x000B         Reads 0x00 */
//    sU8 track_mute_msb;                   /* ?@0x000C         (semantics not decoded yet) */
//    sU8 track_mute_lsb;                   /* ?@0x000D         (semantics not decoded yet) */
//    sU8 __unknown0x000E_0x0014[7];        /* ?@0x000E..0x0014 All zeros. */
//    sU8 selected_mode;                    /* @0x0015          range=0..2, 0=NORMAL, 1=CHAIN, 2=SONG */
//    /* I'd expect 0..3 for the range of this parameter but it stops at 2. It might be a bug in elektron. */
//    sU8 selected_pattern_transition_mode; /* @0x0016          range=0..2, 0=Sequential, 1=Direct Start, 2=Direct Jump or Temp Jump */
//    sU8 __unknown0x0017_0x0019[3];        /* ?@0x0017..0x0019 All zeros. */
//    sU8 fixed_velocity_enable;            /* @0x001A          0=OFF, 1=ON */
//    sU8 fixed_velocity_amount;            /* @0x001B          range=0..127 */
//    sU8 sample_recorder_src;              /* @0x001C range=0..14, 0=AUD L+R, 1=AUD L, 2=AUD R, 3=BD, 4=SD, 5=RS/CP, 6=BT, 7=LT, */
//                                          /*               8=MT/HT, 9=CH/OH, 10=CY/CB, 11=MAIN, 12=USB L, 13=USB R, 14=USB L+R, */
//    sU8 sample_recorder_thr;              /* @0x001D          range=0..127  Threshold                                           */
//    sU8 sample_recorder_monitor;          /* @0x001E          range=0..1, 0=OFF, 1=ON                                           */
//    /* The response continues with the repeating 16 byte pattern of 0xFF_FF_FF_FF 0x00_00_00_00 0x00_00_00_00 0x00_00_00_00 */
//    /* The repeating pattern repeats 128 times. Total length of 2048 bytes. */
//    sU8 __unknown0x001F[16 * 128];        /* ?@0x001F..0x081E repeating_pattern? */
//    sU8 __unknown0x081F;                  /* ?@0x081F         Always 0x01 */
//    sU8 sample_recorder_rlen;             /* @0x0820          range=0..8  Recording length */
//    sU8 __unknown0x0821_0x0826[6];        /* ?@0x0821..0x0826 All zeros. */
// } ar_settings_t;

#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Settings {
    sysex_meta: SysexMeta,
    /// Version of the kit structure.
    version: u32,

    bpm_msb: u8, /* @0x0004          multiplied by 120. (used when BPM mode=PRJ)                                      */
    bpm_lsb: u8, /* @0x0005                                                                                           */
    selected_track: u8, /* @0x0006          range=0..11                                                                      */
    _selected_track_duplicate: u8, /* @0x0007          duplicate of the selected_track, I don't know why it exists.                     */
    selected_trig_or_parameter_menu: u8, /* @0x0008          range=0..5, 0=TRIG, 1=SRC, 2=SMPL, 3=FLTR, 4=AMP, 5=LFO                          */
    selected_fx_menu: u8, /* @0x0009          range=0..5, 0=TODO, 1=Delay, 2=Reverb, 3=Dist, 4=Comp, 5=LFO                     */
    selected_page: u8, /* @0x000A          range=0..3, 0=Page 1, 1=Page 2, 2=Page 3, 3=Page 4, only when manually selected. */

    #[derivative(Debug = "ignore")]
    __unknown_0x000B: u8, /* ?@0x000B         Reads 0x00 */

    track_mute_msb: u8, /* ?@0x000C         (semantics not decoded yet) */
    track_mute_lsb: u8, /* ?@0x000D         (semantics not decoded yet) */

    #[derivative(Debug = "ignore")]
    __unknown0x000E_0x0014: [u8; 7], /* ?@0x000E..0x0014 All zeros. */

    selected_mode: u8, /* @0x0015          range=0..2, 0=NORMAL, 1=CHAIN, 2=SONG */
    // I'd expect 0..3 for the range of this parameter but it stops at 2. It might be a bug in elektron. */
    selected_pattern_transition_mode: u8, /* @0x0016          range=0..2, 0=Sequential, 1=Direct Start, 2=Direct Jump or Temp Jump */

    #[derivative(Debug = "ignore")]
    __unknown0x0017_0x0019: [u8; 3], /* ?@0x0017..0x0019 All zeros. */

    fixed_velocity_enable: u8, /* @0x001A          0=OFF, 1=ON */
    fixed_velocity_amount: u8, /* @0x001B          range=0..127 */
    sample_recorder_src: u8, /* @0x001C range=0..14, 0=AUD L+R, 1=AUD L, 2=AUD R, 3=BD, 4=SD, 5=RS/CP, 6=BT, 7=LT, */
    /*               8=MT/HT, 9=CH/OH, 10=CY/CB, 11=MAIN, 12=USB L, 13=USB R, 14=USB L+R, */
    sample_recorder_thr: u8, /* @0x001D          range=0..127  Threshold                                           */
    sample_recorder_monitor: u8, /* @0x001E          range=0..1, 0=OFF, 1=ON                                           */

    #[derivative(Debug = "ignore")]
    // The response continues with the repeating 16 byte pattern of 0xFF_FF_FF_FF 0x00_00_00_00 0x00_00_00_00 0x00_00_00_00 */
    // The repeating pattern repeats 128 times. Total length of 2048 bytes. */
    __unknown0x001F: [u8; 16 * 128], /* ?@0x001F..0x081E repeating_pattern? */
    #[derivative(Debug = "ignore")]
    __unknown0x081F: u8, /* ?@0x081F         Always 0x01 */

    sample_recorder_rlen: u8, /* @0x0820          range=0..8  Recording length */

    #[derivative(Debug = "ignore")]
    __unknown0x0821_0x0826: [u8; 6], /* ?@0x0821..0x0826 All zeros. */
}

impl From<&Settings> for ar_settings_t {
    fn from(kit: &Settings) -> Self {
        todo!("Conversion to ar_settings_t is not implemented yet.")
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sysex_meta: SysexMeta::default_for_settings(None),
            version: 3,
            bpm_msb: 0,
            bpm_lsb: 0,
            selected_track: 0,
            _selected_track_duplicate: 0,
            selected_trig_or_parameter_menu: 0,
            selected_fx_menu: 0,
            selected_page: 0,
            __unknown_0x000B: 0,
            track_mute_msb: 0,
            track_mute_lsb: 0,
            __unknown0x000E_0x0014: [0; 7],
            selected_mode: 0,
            selected_pattern_transition_mode: 0,
            __unknown0x0017_0x0019: [0; 3],
            fixed_velocity_enable: 0,
            fixed_velocity_amount: 0,
            sample_recorder_src: 0,
            sample_recorder_thr: 0,
            sample_recorder_monitor: 0,
            __unknown0x001F: [0; 16 * 128],
            __unknown0x081F: 0,
            sample_recorder_rlen: 0,
            __unknown0x0821_0x0826: [0; 6],
        }
    }
}

impl Settings {
    pub fn to_raw_parts(&self) -> (SysexMeta, ar_settings_t) {
        (self.sysex_meta, self.into())
    }

    pub fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_settings: &ar_settings_t,
    ) -> Result<Self, RytmError> {
        let version = ((raw_settings.version[0] as u32) << 24)
            | ((raw_settings.version[1] as u32) << 16)
            | ((raw_settings.version[2] as u32) << 8)
            | (raw_settings.version[3] as u32);

        Ok(Self {
            sysex_meta,
            version,
            bpm_msb: raw_settings.bpm_msb,
            bpm_lsb: raw_settings.bpm_lsb,
            selected_track: raw_settings.selected_track,
            _selected_track_duplicate: raw_settings._selected_track_duplicate,
            selected_trig_or_parameter_menu: raw_settings.selected_trig_or_parameter_menu,
            selected_fx_menu: raw_settings.selected_fx_menu,
            selected_page: raw_settings.selected_page,
            __unknown_0x000B: raw_settings.__unknown_0x000B,
            track_mute_msb: raw_settings.track_mute_msb,
            track_mute_lsb: raw_settings.track_mute_lsb,
            __unknown0x000E_0x0014: raw_settings.__unknown0x000E_0x0014,
            selected_mode: raw_settings.selected_mode,
            selected_pattern_transition_mode: raw_settings.selected_pattern_transition_mode,
            __unknown0x0017_0x0019: raw_settings.__unknown0x0017_0x0019,
            fixed_velocity_enable: raw_settings.fixed_velocity_enable,
            fixed_velocity_amount: raw_settings.fixed_velocity_amount,
            sample_recorder_src: raw_settings.sample_recorder_src,
            sample_recorder_thr: raw_settings.sample_recorder_thr,
            sample_recorder_monitor: raw_settings.sample_recorder_monitor,
            __unknown0x001F: raw_settings.__unknown0x001F,
            __unknown0x081F: raw_settings.__unknown0x081F,
            sample_recorder_rlen: raw_settings.sample_recorder_rlen,
            __unknown0x0821_0x0826: raw_settings.__unknown0x0821_0x0826,
        })
    }
}
