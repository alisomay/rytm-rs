use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_kit_t;

use crate::error::{ParameterError, RytmError};
use crate::object::ObjectName;
use crate::sysex::SysexMeta;
use crate::Sound;

// /*
//  *
//  ** Kit v5 (FW1.70) structure
//  *
//  */
// typedef struct { /* 0x0A87 bytes in v1,
//                     0x0A57 bytes in v2,
//                     0x0A79 bytes in v3,
//                     0x0A7A bytes in v4(FW1.50..1.61B),
//                     0x0A32 bytes in v5(FW1.70)
//                  */
//    sU8 __unknown_arr1[0x4]: u8,    /* @0x0000 (reads 00 00 00 06 in FW1.70 -- version nr?) */
//    sU8 name[15];               /* @0x0004 */
//    sU8 __pad_name;             /* @0x0013  (ASCIIZ?) */
//    s_u16_t track_levels[12];   /* @0x0014..0x002b   (note) LSB (track_levels[i].b.hi) is unused (always 0x00) */
//    sU8 __unknown_arr1b[0x2];   /* @0x002c..0x002d */
//    ar_sound_t tracks[12];      /* @0x002E..0x07C5 (12*162=1944($798) bytes */
//    sU8 __unknown_arr2[0x4];    /* @0x07C6..0x07C9 */
//    /* FX-track parameters: */
//    sU8 fx_delay_time;          /* @0x07CA   */
//    sU8 __unused_pad1;          /* @0x07CB   */
//    sU8 fx_delay_pingpong;      /* @0x07CC  0=off, 1=on */
//    sU8 __unused_pad2;          /* @0x07CD   */
//    sU8 fx_delay_stereo_width;  /* @0x07CE  0x40=+0 */
//    sU8 __unused_pad3;          /* @0x07CF   */
//    sU8 fx_delay_feedback;      /* @0x07D0   */
//    sU8 __unused_pad4;          /* @0x07D1   */
//    sU8 fx_delay_hpf;           /* @0x07D2   */
//    sU8 __unused_pad5;          /* @0x07D3   */
//    sU8 fx_delay_lpf;           /* @0x07D4   */
//    sU8 __unused_pad6;          /* @0x07D5   */
//    sU8 fx_delay_reverb_send;   /* @0x07D6   */
//    sU8 __unused_pad7;          /* @0x07D7   */
//    sU8 fx_delay_volume;        /* @0x07D8   */
//    sU8 __unused_pad8;          /* @0x07D9   */
//    sU8 fx_dist_reverb_send;    /* @0x07DA ? */
//    sU8 __unused_pad9;          /* @0x07DB ? */
//    sU8 fx_dist_delay_pre_post; /* @0x07DC ? */
//    sU8 __unused_pad11;         /* @0x07DD ? */
//    sU8 fx_reverb_pre;          /* @0x07DE   */
//    sU8 __unused_pad12;         /* @0x07DF   */
//    sU8 fx_reverb_decay;        /* @0x07E0   */
//    sU8 __unused_pad13;         /* @0x07E1   */
//    sU8 fx_reverb_freq;         /* @0x07E2   */
//    sU8 __unused_pad14;         /* @0x07E3   */
//    sU8 fx_reverb_gain;         /* @0x07E4   */
//    sU8 __unused_pad15;         /* @0x07E5   */
//    sU8 fx_reverb_hpf;          /* @0x07E6   */
//    sU8 __unused_pad16;         /* @0x07E7   */
//    sU8 fx_reverb_lpf;          /* @0x07E8   */
//    sU8 __unused_pad17;         /* @0x07E9   */
//    sU8 fx_reverb_volume;       /* @0x07EA   */
//    sU8 __unused_pad18;         /* @0x07EB   */
//    sU8 fx_dist_reverb_pre_post;/* @0x07EC   */
//    sU8 __unused_pad19;         /* @0x07ED   */
//    sU8 fx_dist_amount;         /* @0x07EE   */
//    sU8 __unused_pad20;         /* @0x07EF   */
//    sU8 fx_dist_sym;            /* @0x07F0   */
//    sU8 __unused_pad21;         /* @0x07F1   */
//    sU8 __unknown_fx_1;         /* @0x07F2   */
//    sU8 __unknown_fx_2;         /* @0x07F3   */
//    sU8 fx_comp_threshold;      /* @0x07F4   */
//    sU8 __unused_pad22;         /* @0x07F5   */
//    sU8 fx_comp_attack;         /* @0x07F6   */
//    sU8 __unused_pad23;         /* @0x07F7   */
//    sU8 fx_comp_release;        /* @0x07F8   */
//    sU8 __unused_pad24;         /* @0x07F9   */
//    sU8 fx_comp_ratio;          /* @0x07FA   */
//    sU8 __unused_pad25;         /* @0x07FB   */
//    sU8 fx_comp_seq;            /* @0x07FC (0=off,1=lpf,2=hpf,3=hit) */
//    sU8 __unused_pad26;         /* @0x07FD   */
//    sU8 fx_comp_gain;           /* @0x07FE   */
//    sU8 __unused_pad27;         /* @0x07FF   */
//    sU8 fx_comp_mix;            /* @0x0800   */
//    sU8 __unused_pad28;         /* @0x0801   */
//    sU8 fx_comp_volume;         /* @0x0802   */
//    sU8 __unused_pad29;         /* @0x0803   */
//    sU8 fx_lfo_speed;           /* @0x0804   */
//    sU8 __unused_pad30;         /* @0x0805   */
//    sU8 fx_lfo_multiplier;      /* @0x0806   */
//    sU8 __unused_pad31;         /* @0x0807   */
//    sU8 fx_lfo_fade;            /* @0x0808   */
//    sU8 __unused_pad32;         /* @0x0809   */
//    sU8 fx_lfo_dest;            /* @0x080A (37/$25=off / "META:None") */
//    sU8 __unused_pad33;         /* @0x080B   */
//    sU8 fx_lfo_wave;            /* @0x080C   */
//    sU8 __unused_pad34;         /* @0x080D   */
//    sU8 fx_lfo_start_phase;     /* @0x080E   */
//    sU8 __unused_pad35;         /* @0x080F   */
//    sU8 fx_lfo_mode;            /* @0x0810   */
//    sU8 __unused_pad36;         /* @0x0811   */
//    sU8 fx_lfo_depth_msb;       /* @0x0812   */
//    sU8 fx_lfo_depth_lsb;       /* @0x0813   */
//    sU8 __unknown_arr3[0x2E];   /* @0x0814..0x0841 (all 0 in test kit) */
//    sU8 perf_ctl[48 * 4];       /* @0x0842..0x0901 */
//                      /* (old comment, offsets have changed by now)
//                         @0x08ba: perf1: (clear)
//                                    off=2234 (0x8ba) a=0x01 b=0x00   (note: pre OS1.31 debug output, perf_ctl are now at 0x88a)
//                                    off=2235 (0x8bb) a=0x00 b=0xff
//                                    off=2236 (0x8bc) a=0x08 b=0x00

//                                  perf1: (assign sample tune +1)
//                                    off=2234 (0x8ba) a=0x00 b=0x01  <-- signed delta value
//                                    off=2235 (0x8bb) a=0xff b=0x00  <-- target id msb ?
//                                    off=2236 (0x8bc) a=0x00 b=0x08  <-- target id (8=sample tune, 9=sample fine tune, ..)

//                                  perf1: (assign sample tune -1)
//                                    off=2234 (0x8ba) a=0x01 b=0xff

//                                  perf1: (assign sample fine tune +1)
//                                    off=2234 (0x8ba) a=0x00 b=0x01
//                                    off=2235 (0x8bb) a=0xff b=0x00
//                                    off=2236 (0x8bc) a=0x00 b=0x09 <--

//                                  perf1: (assign 2nd target sample tune +1)
//                                    off=2238 (0x8be) a=0x00 b=0x01
//                                    off=2239 (0x8bf) a=0xff b=0x00
//                                    off=2240 (0x8c0) a=0x00 b=0x08

//                                  perf1: (assign 3rd target sample br +1)
//                                    off=2242 (0x8c2) a=0x00 b=0x01
//                                    off=2243 (0x8c3) a=0xff b=0x00
//                                    off=2244 (0x8c4) a=0x00 b=0x0b

//                                  perf2: (assign sample tune +1)
//                                    off=2246 (0x8c6) a=0x00 b=0x01
//                                    off=2247 (0x8c7) a=0xff b=0x00
//                                    off=2248 (0x8c8) a=0x00 b=0x08
//                                    off=2249 (0x8c9) a=0x00 b=0x01  <-- perf id
//                      */
//    sU8 __unknown_arr4[0x15];   /* @0x0902..0x0916 */
//    sU8 scene_ctl[48 * 4];      /* @0x0917..0x09D6 */
//                      /* old comment, offsets have changed by now
//                                  scene1: (assign sample tune +1)
//                                    off=2447 (0x98f) a=0x00 b=0x41
//                                    off=2448 (0x990) a=0xff b=0x00
//                                    off=2449 (0x991) a=0x00 b=0x08
//                                    off=2640 (0xa50) a=0x02 b=0x00  <-- current scene id ???

//                                  scene1: (assign 2nd target sample fine tune +1)
//                                    off=2451 (0x993) a=0x00 b=0x41
//                                    off=2452 (0x994) a=0xff b=0x00
//                                    off=2453 (0x995) a=0x00 b=0x09

//                                  scene2: (assign sample tune +1)
//                                    off=2455 (0x997) a=0x00 b=0x41
//                                    off=2456 (0x998) a=0xff b=0x00
//                                    off=2457 (0x999) a=0x00 b=0x08
//                                    off=2458 (0x99a) a=0x00 b=0x01  <-- scene id
//                                    off=2640 (0xa50) a=0x00 b=0x01
//                      */
//    sU8 __unknown_pad37;        /* @0x09D7 (scene_id MSB?) */
//    sU8 current_scene_id;       /* @0x09D8 (0..11) */
//    /* (note) 54 unknown bytes not present in v1 kit data */
//    sU8 __unknown_arr5[54];     /* @0x09D9..0x0A0E */
//    sU8 __unknown_arr6[35];     /* @0x0A0F..0x0A31 */
// } ar_kit_t;

#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Kit {
    index: usize,
    sysex_meta: SysexMeta,
    /// Version of the kit structure.
    version: u32,
    name: ObjectName,

    #[derivative(Debug = "ignore")]
    __pad_name: u8, /* @0x000C */

    track_levels: [u8; 12], /* @0x0014..0x002b   (note) LSB (track_levels[i].b.hi) is unused (always 0x00) */

    #[derivative(Debug = "ignore")]
    __unknown_arr1b: [u8; 0x2], /* @0x002c..0x002d */

    sounds: [Sound; 12], /* @0x002E..0x07C5 (12*162=1944($798) bytes */

    #[derivative(Debug = "ignore")]
    __unknown_arr2: [u8; 0x4], /* @0x07C6..0x07C9 */

    fx_delay_time: u8,         /* @0x07CA   */
    fx_delay_pingpong: u8,     /* @0x07CC  0=off, 1=on */
    fx_delay_stereo_width: u8, /* @0x07CE  0x40=+0 */
    fx_delay_feedback: u8,     /* @0x07D0   */
    fx_delay_hpf: u8,          /* @0x07D2   */
    fx_delay_lpf: u8,          /* @0x07D4   */
    fx_delay_reverb_send: u8,  /* @0x07D6   */
    fx_delay_volume: u8,       /* @0x07D8   */

    fx_dist_reverb_send: u8,    /* @0x07DA ? */
    fx_dist_delay_pre_post: u8, /* @0x07DC ? */

    fx_reverb_pre: u8,    /* @0x07DE   */
    fx_reverb_decay: u8,  /* @0x07E0   */
    fx_reverb_freq: u8,   /* @0x07E2   */
    fx_reverb_gain: u8,   /* @0x07E4   */
    fx_reverb_hpf: u8,    /* @0x07E6   */
    fx_reverb_lpf: u8,    /* @0x07E8   */
    fx_reverb_volume: u8, /* @0x07EA   */

    fx_dist_reverb_pre_post: u8, /* @0x07EC   */
    fx_dist_amount: u8,          /* @0x07EE   */
    fx_dist_sym: u8,             /* @0x07F0   */

    fx_comp_threshold: u8, /* @0x07F4   */
    fx_comp_attack: u8,    /* @0x07F6   */
    fx_comp_release: u8,   /* @0x07F8   */
    fx_comp_ratio: u8,     /* @0x07FA   */
    fx_comp_seq: u8,       /* @0x07FC (0=off,1=lpf,2=hpf,3=hit) */
    fx_comp_gain: u8,      /* @0x07FE   */
    fx_comp_mix: u8,       /* @0x0800   */
    fx_comp_volume: u8,    /* @0x0802   */

    fx_lfo_speed: u8,       /* @0x0804   */
    fx_lfo_multiplier: u8,  /* @0x0806   */
    fx_lfo_fade: u8,        /* @0x0808   */
    fx_lfo_dest: u8,        /* @0x080A (37/$25=off / "META:None") */
    fx_lfo_wave: u8,        /* @0x080C   */
    fx_lfo_start_phase: u8, /* @0x080E   */
    fx_lfo_mode: u8,        /* @0x0810   */
    fx_lfo_depth_msb: u8,   /* @0x0812   */
    fx_lfo_depth_lsb: u8,   /* @0x0813   */

    // @attention Will be ignored for now.
    perf_ctl: [u8; 48 * 4], /* @0x0842..0x0901 */
    #[derivative(Debug = "ignore")]
    __unknown_arr4: [u8; 0x15], /* @0x0902..0x0916 */
    // @attention Will be ignored for now.
    scene_ctl: [u8; 48 * 4], /* @0x0917..0x09D6 */

    __unknown_pad37: u8,  /* @0x09D7 (scene_id MSB?) */
    current_scene_id: u8, /* @0x09D8 (0..11) */

    // (note) 54 unknown bytes not present in v1 kit data
    #[derivative(Debug = "ignore")]
    __unknown_arr5: [u8; 54], /* @0x09D9..0x0A0E */
    #[derivative(Debug = "ignore")]
    __unknown_arr6: [u8; 35], /* @0x0A0F..0x0A31 */

    #[derivative(Debug = "ignore")]
    __unused_pad1: u8, /* @0x07CB   */
    #[derivative(Debug = "ignore")]
    __unused_pad2: u8, /* @0x07CD   */
    #[derivative(Debug = "ignore")]
    __unused_pad3: u8, /* @0x07CF   */
    #[derivative(Debug = "ignore")]
    __unused_pad4: u8, /* @0x07D1   */
    #[derivative(Debug = "ignore")]
    __unused_pad5: u8, /* @0x07D3   */
    #[derivative(Debug = "ignore")]
    __unused_pad6: u8, /* @0x07D5   */
    #[derivative(Debug = "ignore")]
    __unused_pad7: u8, /* @0x07D7   */
    #[derivative(Debug = "ignore")]
    __unused_pad8: u8, /* @0x07D9   */
    #[derivative(Debug = "ignore")]
    __unused_pad9: u8, /* @0x07DB ? */
    #[derivative(Debug = "ignore")]
    __unused_pad11: u8, /* @0x07DD ? */
    #[derivative(Debug = "ignore")]
    __unused_pad12: u8, /* @0x07DF   */
    #[derivative(Debug = "ignore")]
    __unused_pad13: u8, /* @0x07E1   */
    #[derivative(Debug = "ignore")]
    __unused_pad14: u8, /* @0x07E3   */
    #[derivative(Debug = "ignore")]
    __unused_pad15: u8, /* @0x07E5   */
    #[derivative(Debug = "ignore")]
    __unused_pad16: u8, /* @0x07E7   */
    #[derivative(Debug = "ignore")]
    __unused_pad17: u8, /* @0x07E9   */
    #[derivative(Debug = "ignore")]
    __unused_pad18: u8, /* @0x07EB   */
    #[derivative(Debug = "ignore")]
    __unused_pad19: u8, /* @0x07ED   */
    #[derivative(Debug = "ignore")]
    __unused_pad20: u8, /* @0x07EF   */
    #[derivative(Debug = "ignore")]
    __unused_pad21: u8, /* @0x07F1   */

    __unknown_fx_1: u8, /* @0x07F2   */
    __unknown_fx_2: u8, /* @0x07F3   */

    #[derivative(Debug = "ignore")]
    __unused_pad22: u8, /* @0x07F5   */
    #[derivative(Debug = "ignore")]
    __unused_pad23: u8, /* @0x07F7   */
    #[derivative(Debug = "ignore")]
    __unused_pad24: u8, /* @0x07F9   */
    #[derivative(Debug = "ignore")]
    __unused_pad25: u8, /* @0x07FB   */
    #[derivative(Debug = "ignore")]
    __unused_pad26: u8, /* @0x07FD   */
    #[derivative(Debug = "ignore")]
    __unused_pad27: u8, /* @0x07FF   */
    #[derivative(Debug = "ignore")]
    __unused_pad28: u8, /* @0x0801   */
    #[derivative(Debug = "ignore")]
    __unused_pad29: u8, /* @0x0803   */
    #[derivative(Debug = "ignore")]
    __unused_pad30: u8, /* @0x0805   */
    #[derivative(Debug = "ignore")]
    __unused_pad31: u8, /* @0x0807   */
    #[derivative(Debug = "ignore")]
    __unused_pad32: u8, /* @0x0809   */
    #[derivative(Debug = "ignore")]
    __unused_pad33: u8, /* @0x080B   */
    #[derivative(Debug = "ignore")]
    __unused_pad34: u8, /* @0x080D   */
    #[derivative(Debug = "ignore")]
    __unused_pad35: u8, /* @0x080F   */
    #[derivative(Debug = "ignore")]
    __unused_pad36: u8, /* @0x0811   */
}

impl From<&Kit> for ar_kit_t {
    fn from(kit: &Kit) -> Self {
        todo!("Conversion to ar_kit_t is not implemented yet.")
    }
}

impl Kit {
    pub fn to_raw_parts(&self) -> (SysexMeta, ar_kit_t) {
        (self.sysex_meta, self.into())
    }

    pub fn try_from_raw(sysex_meta: SysexMeta, raw_kit: &ar_kit_t) -> Result<Self, RytmError> {
        let version = ((raw_kit.__unknown_arr1[0] as u32) << 24)
            | ((raw_kit.__unknown_arr1[1] as u32) << 16)
            | ((raw_kit.__unknown_arr1[2] as u32) << 8)
            | (raw_kit.__unknown_arr1[3] as u32);

        let kit_number = if sysex_meta.is_targeting_work_buffer() {
            // TODO: Double check
            0
        } else {
            sysex_meta.obj_nr as usize
        };

        let name = ObjectName::from_u8_array(raw_kit.name);

        let mut sounds = [Sound::work_buffer_default(); 12];
        for (i, sound) in raw_kit.tracks.iter().enumerate() {
            sounds[i] = Sound::try_from_raw(sysex_meta, sound, Some((kit_number, i)))?;
        }

        let mut track_levels = [0; 12];
        for (i, track_level) in raw_kit.track_levels.iter().enumerate() {
            track_levels[i] = unsafe { track_level.b.lo };
        }

        Ok(Self {
            index: kit_number,
            sysex_meta,
            version,

            name,
            __pad_name: raw_kit.__pad_name,

            track_levels,
            sounds,

            fx_delay_time: raw_kit.fx_delay_time,
            fx_delay_pingpong: raw_kit.fx_delay_pingpong,
            fx_delay_stereo_width: raw_kit.fx_delay_stereo_width,
            fx_delay_feedback: raw_kit.fx_delay_feedback,
            fx_delay_hpf: raw_kit.fx_delay_hpf,
            fx_delay_lpf: raw_kit.fx_delay_lpf,
            fx_delay_reverb_send: raw_kit.fx_delay_reverb_send,
            fx_delay_volume: raw_kit.fx_delay_volume,

            fx_dist_reverb_send: raw_kit.fx_dist_reverb_send,
            fx_dist_delay_pre_post: raw_kit.fx_dist_delay_pre_post,

            fx_reverb_pre: raw_kit.fx_reverb_pre,
            fx_reverb_decay: raw_kit.fx_reverb_decay,
            fx_reverb_freq: raw_kit.fx_reverb_freq,
            fx_reverb_gain: raw_kit.fx_reverb_gain,
            fx_reverb_hpf: raw_kit.fx_reverb_hpf,
            fx_reverb_lpf: raw_kit.fx_reverb_lpf,
            fx_reverb_volume: raw_kit.fx_reverb_volume,

            fx_dist_reverb_pre_post: raw_kit.fx_dist_reverb_pre_post,
            fx_dist_amount: raw_kit.fx_dist_amount,
            fx_dist_sym: raw_kit.fx_dist_sym,

            fx_comp_threshold: raw_kit.fx_comp_threshold,
            fx_comp_attack: raw_kit.fx_comp_attack,
            fx_comp_release: raw_kit.fx_comp_release,
            fx_comp_ratio: raw_kit.fx_comp_ratio,
            fx_comp_seq: raw_kit.fx_comp_seq,
            fx_comp_gain: raw_kit.fx_comp_gain,
            fx_comp_mix: raw_kit.fx_comp_mix,
            fx_comp_volume: raw_kit.fx_comp_volume,

            fx_lfo_speed: raw_kit.fx_lfo_speed,
            fx_lfo_multiplier: raw_kit.fx_lfo_multiplier,
            fx_lfo_fade: raw_kit.fx_lfo_fade,
            fx_lfo_dest: raw_kit.fx_lfo_dest,
            fx_lfo_wave: raw_kit.fx_lfo_wave,
            fx_lfo_start_phase: raw_kit.fx_lfo_start_phase,
            fx_lfo_mode: raw_kit.fx_lfo_mode,
            fx_lfo_depth_msb: raw_kit.fx_lfo_depth_msb,
            fx_lfo_depth_lsb: raw_kit.fx_lfo_depth_lsb,

            perf_ctl: raw_kit.perf_ctl,
            scene_ctl: raw_kit.scene_ctl,

            __unknown_pad37: raw_kit.__unknown_pad37,

            current_scene_id: raw_kit.current_scene_id,

            __unknown_arr1b: raw_kit.__unknown_arr1b,
            __unknown_arr2: raw_kit.__unknown_arr2,
            __unknown_arr4: raw_kit.__unknown_arr4,
            __unknown_arr5: raw_kit.__unknown_arr5,
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
        })
    }

    #[parameter_range(range = "kit_index:0..=127")]
    pub fn try_default(kit_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            index: kit_index,
            sysex_meta: SysexMeta::try_default_for_kit(kit_index, None)?,
            version: 6,

            name: ObjectName::from_u8_array([0_u8; 15]),
            __pad_name: 0,

            track_levels: [0; 12],

            // TODO: Currently relevant indexes are omitted.
            // This array is not valid, it is temporary.
            sounds: [Sound::work_buffer_default(); 12],

            fx_delay_time: 0,
            fx_delay_pingpong: 0,
            fx_delay_stereo_width: 0,
            fx_delay_feedback: 0,
            fx_delay_hpf: 0,
            fx_delay_lpf: 0,
            fx_delay_reverb_send: 0,
            fx_delay_volume: 0,

            fx_dist_reverb_send: 0,
            fx_dist_delay_pre_post: 0,

            fx_reverb_pre: 0,
            fx_reverb_decay: 0,
            fx_reverb_freq: 0,
            fx_reverb_gain: 0,
            fx_reverb_hpf: 0,
            fx_reverb_lpf: 0,
            fx_reverb_volume: 0,

            fx_dist_reverb_pre_post: 0,
            fx_dist_amount: 0,
            fx_dist_sym: 0,

            fx_comp_threshold: 0,
            fx_comp_attack: 0,
            fx_comp_release: 0,
            fx_comp_ratio: 0,
            fx_comp_seq: 0,
            fx_comp_gain: 0,
            fx_comp_mix: 0,
            fx_comp_volume: 0,

            fx_lfo_speed: 0,
            fx_lfo_multiplier: 0,
            fx_lfo_fade: 0,
            fx_lfo_dest: 0,
            fx_lfo_wave: 0,
            fx_lfo_start_phase: 0,
            fx_lfo_mode: 0,
            fx_lfo_depth_msb: 0,
            fx_lfo_depth_lsb: 0,

            perf_ctl: [0; 48 * 4],
            scene_ctl: [0; 48 * 4],

            __unknown_pad37: 0,

            current_scene_id: 0,

            __unknown_arr1b: [0; 0x2],
            __unknown_arr2: [0; 0x4],
            __unknown_arr4: [0; 0x15],
            __unknown_arr5: [0; 54],
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
        })
    }

    pub fn work_buffer_default() -> Self {
        Self {
            index: 0,
            sysex_meta: SysexMeta::default_for_kit_in_work_buffer(None),
            version: 6,

            name: ObjectName::from_u8_array([0_u8; 15]),
            __pad_name: 0,

            track_levels: [0; 12],

            // TODO: Currently relevant indexes are omitted.
            // This array is not valid, it is temporary.
            sounds: [Sound::work_buffer_default(); 12],

            fx_delay_time: 0,
            fx_delay_pingpong: 0,
            fx_delay_stereo_width: 0,
            fx_delay_feedback: 0,
            fx_delay_hpf: 0,
            fx_delay_lpf: 0,
            fx_delay_reverb_send: 0,
            fx_delay_volume: 0,

            fx_dist_reverb_send: 0,
            fx_dist_delay_pre_post: 0,

            fx_reverb_pre: 0,
            fx_reverb_decay: 0,
            fx_reverb_freq: 0,
            fx_reverb_gain: 0,
            fx_reverb_hpf: 0,
            fx_reverb_lpf: 0,
            fx_reverb_volume: 0,

            fx_dist_reverb_pre_post: 0,
            fx_dist_amount: 0,
            fx_dist_sym: 0,

            fx_comp_threshold: 0,
            fx_comp_attack: 0,
            fx_comp_release: 0,
            fx_comp_ratio: 0,
            fx_comp_seq: 0,
            fx_comp_gain: 0,
            fx_comp_mix: 0,
            fx_comp_volume: 0,

            fx_lfo_speed: 0,
            fx_lfo_multiplier: 0,
            fx_lfo_fade: 0,
            fx_lfo_dest: 0,
            fx_lfo_wave: 0,
            fx_lfo_start_phase: 0,
            fx_lfo_mode: 0,
            fx_lfo_depth_msb: 0,
            fx_lfo_depth_lsb: 0,

            perf_ctl: [0; 48 * 4],
            scene_ctl: [0; 48 * 4],

            __unknown_pad37: 0,

            current_scene_id: 0,

            __unknown_arr1b: [0; 0x2],
            __unknown_arr2: [0; 0x4],
            __unknown_arr4: [0; 0x15],
            __unknown_arr5: [0; 54],
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
