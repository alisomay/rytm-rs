pub mod types;

use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sound_t;

use self::types::Machine;
use crate::error::RytmError;
use crate::object::ObjectName;
use crate::sysex::{SysexMeta, SysexType};
use crate::util::from_s_u16_t;
use crate::ParameterError;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SoundType {
    Pool,
    #[default]
    WorkBuffer,
    KitQuery,
}

// TODO:
#[derive(Clone, Copy, Debug)]
pub struct SynthParameter {
    inner: u16,
}

impl SynthParameter {
    pub fn new(inner: u16) -> Self {
        Self { inner }
    }
}

#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Sound {
    /// Index of the sound.
    ///
    /// This can mean various things depending on the context
    ///
    /// - If this sound is retrieved from the sound pool, this is the index of the sound in the pool.
    /// - If this sound is retrieved from a track from the work buffer or a kit query, this is the index of the track.
    index: usize,
    /// Index of the sound if it was retrieved from the sound pool.
    pool_index: Option<usize>,
    /// Kit number if this sound is retrieved from a kit query
    kit_number: Option<usize>,
    /// Index of the sound if it was retrieved from a track from the work buffer.
    assigned_track: Option<usize>,
    sysex_meta: SysexMeta,
    /// Version of the sound structure.
    version: u32,

    #[derivative(Debug = "ignore")]
    __unknown_arr1: [u8; 12], /* @0x0000 reads BE EF BA CE 00 00 00 04 00 00 00 00 */

    name: ObjectName,

    #[derivative(Debug = "ignore")]
    __pad_name: u8, /* @0x000C */

    // TODO: Complex lookup depending on machine type.
    synth_parameter: [SynthParameter; 8],

    sample_tune: u8, /* @0x002c  0x40=0, 0x41=+1, .. */
    // 40..=88 device -24..=24
    sample_fine_tune: u8, /* @0x002e  0x40=0, 0x41=+1, .. */
    // 0..=127 device -64..=63
    sample_number: u8, /* @0x0030  0=off, 1..127 */
    // 0..=127 device 0..=127
    //                                           (note) changing the sample also changes:
    //                                                   smp OFF->6:
    //                                                    off=48 (0x30) a=0x00 b=0x06
    //                                                    off=132 (0x84) a=0xff b=0x00
    //                                                    off=133 (0x85) a=0xff b=0x00
    //                                                    off=134 (0x86) a=0xff b=0x10
    //                                                    off=135 (0x87) a=0xff b=0xef
    //                                                    off=136 (0x88) a=0x00 b=0x2c
    //                                                    off=137 (0x89) a=0x00 b=0x6f
    //                                                    off=138 (0x8a) a=0x00 b=0xa8
    //                                                    off=139 (0x8b) a=0x00 b=0x07
    //                                                    off=142 (0x8e) a=0x00 b=0x52
    //                                                    off=143 (0x8f) a=0x00 b=0x4e
    //                                                    off=146 (0x92) a=0x00 b=0x12
    //                                                    off=147 (0x93) a=0x00 b=0x36
    //                               */
    sample_br: u8, /* @0x0032  sample bit reduction */
    // 0..=127 device 0..=127
    sample_start: u16, /* @0x0034  STA (LSB used since v5/FW1.70) */
    // 0..=30720 device 0.0..=120.0
    sample_end: u16, /* @0x0036  END (LSB used since v5/FW1.70) */
    // 0..=30720 device 0.0..=120.0
    sample_loop_flag: u8, /* @0x0038  0=off, 1=on */
    // 0..=1 device 0..=1
    sample_volume: u8, /* @0x003a */

    // 0..=127 device 0..=127
    flt_attack: u8, /* @0x003c */
    // 0..=127 device 0..=127
    flt_sustain: u8, /* @0x003e */
    // 0..=127 device 0..=127
    flt_decay: u8, /* @0x0040 */
    // 0..=127 device 0..=127
    flt_release: u8, /* @0x0042 */
    // 0..=127 device 0..=127
    flt_cutoff: u8, /* @0x0044 */
    // 0..=127 device 0..=127
    flt_res: u8, /* @0x0046 */
    // 0..=127 device 0..=127
    flt_type: u8, /* @0x0048 */
    // 0..=6 device LP2, LP1, BP, HP1, HP2, BS, PK
    flt_env: u8, /* @0x004a    64=0, 127=+63, 0=-64*/

    // 0..=127 device -64..=63
    amp_attack: u8, /* @0x004c */
    // 0..=127 device 0..=127
    amp_hold: u8, /* @0x004e */
    // 0..=127 device 0..=127
    amp_decay: u8, /* @0x0050 */
    // 0..=127 device 0..=127, 127=INF
    amp_overdrive: u8, /* @0x0052 */
    // 0..=127 device 0..=127
    amp_delay_send: u8, /* @0x0054 */
    // 0..=127 device 0..=127
    amp_reverb_send: u8, /* @0x0056 */
    // 0..=127 device 0..=127
    amp_pan: u8, /* @0x0058 */
    // 0..=127 device 0..=127 middle point=64 L64=0, R63=127
    amp_volume: u8, /* @0x005a */

    // 0..=127 device 0..=127
    accent_level: u8, /* @0x005c accent level [FUNC+B/F] */

    // TODO:
    lfo_speed: u8, /* @0x005e */
    // 0..=127 device -64..=63
    lfo_multiplier: u8, /* @0x0060 */
    // 0..=23 device
    // 0..=11
    // x1, x2, x4, x8, x16, x32, x64, x128, x256, x512, x1k, x2k,
    // 12..=23
    //.1, .2, .4, .8, .16, .32, .64, .128, .256, .512, .1k, .2k
    lfo_fade: u8, /* @0x0062  0x40=0 */
    // 0..=127 device -64..=63
    lfo_dest: u8, /* @0x0064  see LFO_DEST_xxx */
    // TODO: double check LFO_DEST_xxx
    lfo_wav: u8, /* @0x0066 (0=tri,1=sin,2=sqr,3=saw,4=exp,5=rmp,6=rnd) */
    // 0..=6
    lfo_start_phase: u8, /* @0x0068 (note) FW1.70: used as slew when wav is set to 6=rnd */
    // 0..=127 device 0..=127
    lfo_mode: u8, /* @0x006a (0=free,1=trig,2=hold,3=one,4=half) */
    // 0..=4
    lfo_depth: u16, /* @0x006c  */

    // 0..=32767 device -128.0..=127.99
    def_note: u8, /* @0x006e  0x3c=0, 0x3d=+1, 0x3b=-1 (initially 0x00 == +0 ?!) // TODO: Don't know what this is yet */
    //                                    (note) not used in sound dump ? (only in kit?!!)
    //                                */
    #[derivative(Debug = "ignore")]
    __unknown_006f: [u8; 0xd], /* @0x006f..0x007B   */

    // TODO: Not understood kit offsets?
    machine: Machine,

    pub mode_flags: u8, /* @0x007D bit 0  : ?
                        //                                          bit 1  : env reset filter switch
                        //                                          bit 2  : legacy fx send switch
                        //                                          bit 3  : ?
                        //                                          bit 4+5: chromatic mode  0=OFF, 1=SYNTH, 2=SAMPLE, 3=SYN+SMP
                        //                                          bit 6  : velocity to vol switch
                        //                                          bit 7  : ?
                        //                                          (note) FW1.70: moved extra veltovol,legacyfx,envreset bytes to bit fields
                        //                                */

    #[derivative(Debug = "ignore")]
    __unknown_007e: [u8; 16], /* @0x007E..0x008D */

    // All amounts are TODO: Try interpreting them as i8,  device -128..=+127
    vel_amt_1: u8,    /* @0x008E VELOCITY MOD */
    vel_target_1: u8, /* @0x008F */

    // Targets are enum style TODO: Check
    vel_amt_2: u8,    /* @0x0090 */
    vel_target_2: u8, /* @0x0091 */

    vel_amt_3: u8,    /* @0x0092 */
    vel_target_3: u8, /* @0x0093 */

    vel_amt_4: u8,    /* @0x0094 */
    vel_target_4: u8, /* @0x0095 */

    at_amt_1: u8,    /* @0x0096 AFTERTOUCH */
    at_target_1: u8, /* @0x0097 */

    at_amt_2: u8,    /* @0x0098 */
    at_target_2: u8, /* @0x0099 */

    at_amt_3: u8,    /* @0x009A */
    at_target_3: u8, /* @0x009B */

    at_amt_4: u8,    /* @0x009C */
    at_target_4: u8, /* @0x009D */

    #[derivative(Debug = "ignore")]
    __unknown_009e: [u8; 4], /* @0x009E..0x00A1 */

    #[derivative(Debug = "ignore")]
    __unused_pad9: u8, /* @0x002d (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad10: u8, /* @0x002f (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad11: u8, /* @0x0031 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad12: u8, /* @0x0033 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad15: u8, /* @0x0039 */
    #[derivative(Debug = "ignore")]
    __unused_pad16: u8, /* @0x003b (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad17: u8, /* @0x003d (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad18: u8, /* @0x003f (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad19: u8, /* @0x0041 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad20: u8, /* @0x0043 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad21: u8, /* @0x0045 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad22: u8, /* @0x0047 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad23: u8, /* @0x0049 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad24: u8, /* @0x004b (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad25: u8, /* @0x004d (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad26: u8, /* @0x004f (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad27: u8, /* @0x0051 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad28: u8, /* @0x0053 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad29: u8, /* @0x0055 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad30: u8, /* @0x0057 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad31: u8, /* @0x0059 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad32: u8, /* @0x005b (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused005D: u8, /* @0x005d (lsb, always 0)         */
    #[derivative(Debug = "ignore")]
    __unused_pad33: u8, /* @0x005f (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad34: u8, /* @0x0061 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad35: u8, /* @0x0063 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad36: u8, /* @0x0065 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad37: u8, /* @0x0067 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad38: u8, /* @0x0069 (lsb, always 0) */
    #[derivative(Debug = "ignore")]
    __unused_pad39: u8, /* @0x006b (lsb, always 0) */
}

impl From<&Sound> for ar_sound_t {
    fn from(sound: &Sound) -> Self {
        todo!("Conversion to ar_sound_t is not implemented yet.")
    }
}

impl Sound {
    pub fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_sound: &ar_sound_t,
        kit_number_and_assigned_track: Option<(usize, usize)>,
    ) -> Result<Self, RytmError> {
        let machine: Machine = raw_sound.machine_type.try_into()?;

        let version = ((raw_sound.__unknown_arr1[4] as u32) << 24)
            | ((raw_sound.__unknown_arr1[5] as u32) << 16)
            | ((raw_sound.__unknown_arr1[6] as u32) << 8)
            | (raw_sound.__unknown_arr1[7] as u32);

        let mut index: usize = 0;
        let mut assigned_track = None;
        let mut kit_number = None;
        let mut pool_index = None;

        match sysex_meta.object_type()? {
            SysexType::Sound => {
                if sysex_meta.is_targeting_work_buffer() {
                    index = (sysex_meta.obj_nr & 0b0111_1111) as usize;
                    assigned_track = Some(index);
                }

                if let Some((kit_n, assigned_t)) = kit_number_and_assigned_track {
                    index = assigned_t;
                    assigned_track = Some(assigned_t);
                    kit_number = Some(kit_n);
                }

                if kit_number_and_assigned_track.is_none() && !sysex_meta.is_targeting_work_buffer()
                {
                    index = (sysex_meta.obj_nr & 0b0111_1111) as usize;
                    pool_index = Some(index);
                }
            }
            SysexType::Kit => {
                // When this sound is part of a kit query...
                if let Some((kit_n, assigned_t)) = kit_number_and_assigned_track {
                    index = assigned_t;
                    assigned_track = Some(assigned_t);
                    kit_number = Some(kit_n);
                } else {
                    // TODO: Maybe better handle all these.
                    todo!("Error here, this is not a sound query. Kit queries should provide the kit number and assigned track.")
                }
            }
            _ => unreachable!(" TODO: This is not a sound or kit query handle error."),
        }

        Ok(Self {
            index,
            pool_index,
            kit_number,
            assigned_track,
            sysex_meta,
            version,
            __unknown_arr1: raw_sound.__unknown_arr1,
            name: ObjectName::from_u8_array(raw_sound.name),
            __pad_name: raw_sound.__pad_name,
            synth_parameter: [
                SynthParameter::new(unsafe { from_s_u16_t(&raw_sound.synth_param_1) }),
                SynthParameter::new(unsafe { from_s_u16_t(&raw_sound.synth_param_2) }),
                SynthParameter::new(unsafe { from_s_u16_t(&raw_sound.synth_param_3) }),
                SynthParameter::new(unsafe { from_s_u16_t(&raw_sound.synth_param_4) }),
                SynthParameter::new(unsafe { from_s_u16_t(&raw_sound.synth_param_5) }),
                SynthParameter::new(unsafe { from_s_u16_t(&raw_sound.synth_param_6) }),
                SynthParameter::new(unsafe { from_s_u16_t(&raw_sound.synth_param_7) }),
                SynthParameter::new(unsafe { from_s_u16_t(&raw_sound.synth_param_8) }),
            ],
            sample_tune: raw_sound.sample_tune,
            sample_fine_tune: raw_sound.sample_fine_tune,
            sample_number: raw_sound.sample_nr,
            sample_br: raw_sound.sample_br,
            sample_start: unsafe { from_s_u16_t(&raw_sound.sample_start) },
            sample_end: unsafe { from_s_u16_t(&raw_sound.sample_end) },
            sample_loop_flag: raw_sound.sample_loop_flag,
            sample_volume: raw_sound.sample_volume,

            flt_attack: raw_sound.flt_attack,
            flt_sustain: raw_sound.flt_sustain,
            flt_decay: raw_sound.flt_decay,
            flt_release: raw_sound.flt_release,
            flt_cutoff: raw_sound.flt_cutoff,
            flt_res: raw_sound.flt_res,
            flt_type: raw_sound.flt_type,
            flt_env: raw_sound.flt_env,

            amp_attack: raw_sound.amp_attack,
            amp_hold: raw_sound.amp_hold,
            amp_decay: raw_sound.amp_decay,
            amp_overdrive: raw_sound.amp_overdrive,
            amp_delay_send: raw_sound.amp_delay_send,
            amp_reverb_send: raw_sound.amp_reverb_send,
            amp_pan: raw_sound.amp_pan,
            amp_volume: raw_sound.amp_volume,

            accent_level: raw_sound.accent_level,

            lfo_speed: raw_sound.lfo_speed,
            lfo_multiplier: raw_sound.lfo_multiplier,
            lfo_fade: raw_sound.lfo_fade,
            lfo_dest: raw_sound.lfo_dest,
            lfo_wav: raw_sound.lfo_wav,
            lfo_start_phase: raw_sound.lfo_start_phase,
            lfo_mode: raw_sound.lfo_mode,
            lfo_depth: unsafe { from_s_u16_t(&raw_sound.lfo_depth) },

            def_note: raw_sound.def_note,
            machine,

            mode_flags: raw_sound.mode_flags,

            vel_amt_1: raw_sound.vel_amt_1,
            vel_target_1: raw_sound.vel_target_1,

            vel_amt_2: raw_sound.vel_amt_2,
            vel_target_2: raw_sound.vel_target_2,

            vel_amt_3: raw_sound.vel_amt_3,
            vel_target_3: raw_sound.vel_target_3,

            vel_amt_4: raw_sound.vel_amt_4,
            vel_target_4: raw_sound.vel_target_4,

            at_amt_1: raw_sound.at_amt_1,
            at_target_1: raw_sound.at_target_1,

            at_amt_2: raw_sound.at_amt_2,
            at_target_2: raw_sound.at_target_2,

            at_amt_3: raw_sound.at_amt_3,
            at_target_3: raw_sound.at_target_3,

            at_amt_4: raw_sound.at_amt_4,
            at_target_4: raw_sound.at_target_4,

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
            __unused005D: raw_sound.__unused005D,
            __unused_pad33: raw_sound.__unused_pad33,
            __unused_pad34: raw_sound.__unused_pad34,
            __unused_pad35: raw_sound.__unused_pad35,
            __unused_pad36: raw_sound.__unused_pad36,
            __unused_pad37: raw_sound.__unused_pad37,
            __unused_pad38: raw_sound.__unused_pad38,
            __unused_pad39: raw_sound.__unused_pad39,
        })
    }

    /// Checks if the given machine is compatible for the given track.
    fn is_machine_compatible_for_track(track_index: usize, machine: Machine) -> bool {
        let compatible_machines = unsafe { rytm_sys::ar_sound_compatible_machines };
        let compatible_machines_for_track = compatible_machines[track_index];

        let mut compatible_machines_for_track_size = 0;
        loop {
            unsafe {
                let return_id = rytm_sys::ar_sound_get_machine_id_by_track_and_list_idx(
                    track_index as u32,
                    compatible_machines_for_track_size,
                );
                if return_id == -1 {
                    break;
                }
                compatible_machines_for_track_size += 1;
            }
        }

        let compatible_machines_for_track_slice = unsafe {
            std::slice::from_raw_parts(
                compatible_machines_for_track,
                compatible_machines_for_track_size as usize,
            )
        };

        compatible_machines_for_track_slice.contains(&((machine as u8) as i32))
    }

    pub fn set_machine(&mut self, machine: Machine) -> Result<(), RytmError> {
        if let Some(assigned_track) = self.assigned_track() {
            if !Sound::is_machine_compatible_for_track(assigned_track, machine) {
                return Err(ParameterError::Compatibility {
                    value: machine.to_string(),
                    parameter_name: "Machine".to_string(),
                    reason: Some(format!(
                        "Given machine {} is not compatible for track {}",
                        machine, self.index
                    )),
                }
                .into());
            }
        }

        self.machine = machine;
        Ok(())
    }

    pub fn sound_type(&self) -> SoundType {
        if self.is_pool_sound() {
            SoundType::Pool
        } else if self.is_work_buffer_sound() {
            SoundType::WorkBuffer
        } else {
            SoundType::KitQuery
        }
    }

    pub fn is_pool_sound(&self) -> bool {
        self.pool_index.is_some()
    }

    pub fn is_work_buffer_sound(&self) -> bool {
        self.assigned_track().is_some() && self.kit_number.is_none()
    }

    pub fn is_part_of_a_kit_query(&self) -> bool {
        self.kit_number.is_some()
    }

    /// Returns the assigned track if this is a track sound.
    ///
    /// Returns `None` if this is not a track sound.
    pub fn assigned_track(&self) -> Option<usize> {
        self.assigned_track
    }

    pub fn to_raw_parts(&self) -> (SysexMeta, ar_sound_t) {
        (self.sysex_meta, self.into())
    }

    // TODO: Find the right range.
    #[parameter_range(range = "sound_index:0..=127")]
    pub fn try_default(sound_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            // BE EF BA CE 00 00 00 04 00 00 00 00
            __unknown_arr1: [
                0xBE, 0xEF, 0xBA, 0xCE, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
            ],
            sysex_meta: SysexMeta::try_default_for_sound(sound_index, None)?,
            index: sound_index,
            pool_index: Some(sound_index),
            kit_number: None,
            assigned_track: None,

            machine: Machine::default(),
            version: 0x00_00_00_04,

            name: ObjectName::from_u8_array([0; 15]),
            __pad_name: 0,

            synth_parameter: [
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
            ],

            sample_tune: 0,
            sample_fine_tune: 0,
            sample_number: 0,
            sample_br: 0,
            sample_start: 0,
            sample_end: 0,
            sample_loop_flag: 0,
            sample_volume: 0,

            flt_attack: 0,
            flt_sustain: 0,
            flt_decay: 0,
            flt_release: 0,
            flt_cutoff: 0,
            flt_res: 0,
            flt_type: 0,
            flt_env: 0,

            amp_attack: 0,
            amp_hold: 0,
            amp_decay: 0,
            amp_overdrive: 0,
            amp_delay_send: 0,
            amp_reverb_send: 0,
            amp_pan: 0,
            amp_volume: 0,

            accent_level: 0,

            lfo_speed: 0,
            lfo_multiplier: 0,
            lfo_fade: 0,
            lfo_dest: 0,
            lfo_wav: 0,
            lfo_start_phase: 0,
            lfo_mode: 0,
            lfo_depth: 0,

            def_note: 0,

            __unknown_006f: [0; 0xd],
            __unknown_009e: [0; 4],
            __unknown_007e: [0; 16],
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
            __unused005D: 0,
            __unused_pad33: 0,
            __unused_pad34: 0,
            __unused_pad35: 0,
            __unused_pad36: 0,
            __unused_pad37: 0,
            __unused_pad38: 0,
            __unused_pad39: 0,

            mode_flags: 0,

            vel_amt_1: 0,
            vel_target_1: 0,

            vel_amt_2: 0,
            vel_target_2: 0,

            vel_amt_3: 0,
            vel_target_3: 0,

            vel_amt_4: 0,
            vel_target_4: 0,

            at_amt_1: 0,
            at_target_1: 0,

            at_amt_2: 0,
            at_target_2: 0,

            at_amt_3: 0,
            at_target_3: 0,

            at_amt_4: 0,
            at_target_4: 0,
        })
    }

    pub fn work_buffer_default() -> Self {
        Self {
            // BE EF BA CE 00 00 00 04 00 00 00 00
            __unknown_arr1: [
                0xBE, 0xEF, 0xBA, 0xCE, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
            ],
            sysex_meta: SysexMeta::default_for_sound_in_work_buffer(None),
            index: 0b1000_0000,
            pool_index: None,
            kit_number: None,
            assigned_track: Some(0),

            // TODO: Default for track?
            machine: Machine::default(),
            version: 0x00_00_00_04,

            name: ObjectName::from_u8_array([0; 15]),
            __pad_name: 0,

            synth_parameter: [
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
                SynthParameter::new(0),
            ],

            sample_tune: 0,
            sample_fine_tune: 0,
            sample_number: 0,
            sample_br: 0,
            sample_start: 0,
            sample_end: 0,
            sample_loop_flag: 0,
            sample_volume: 0,

            flt_attack: 0,
            flt_sustain: 0,
            flt_decay: 0,
            flt_release: 0,
            flt_cutoff: 0,
            flt_res: 0,
            flt_type: 0,
            flt_env: 0,

            amp_attack: 0,
            amp_hold: 0,
            amp_decay: 0,
            amp_overdrive: 0,
            amp_delay_send: 0,
            amp_reverb_send: 0,
            amp_pan: 0,
            amp_volume: 0,

            accent_level: 0,

            lfo_speed: 0,
            lfo_multiplier: 0,
            lfo_fade: 0,
            lfo_dest: 0,
            lfo_wav: 0,
            lfo_start_phase: 0,
            lfo_mode: 0,
            lfo_depth: 0,

            def_note: 0,

            __unknown_006f: [0; 0xd],
            __unknown_009e: [0; 4],
            __unknown_007e: [0; 16],
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
            __unused005D: 0,
            __unused_pad33: 0,
            __unused_pad34: 0,
            __unused_pad35: 0,
            __unused_pad36: 0,
            __unused_pad37: 0,
            __unused_pad38: 0,
            __unused_pad39: 0,

            mode_flags: 0,

            vel_amt_1: 0,
            vel_target_1: 0,

            vel_amt_2: 0,
            vel_target_2: 0,

            vel_amt_3: 0,
            vel_target_3: 0,

            vel_amt_4: 0,
            vel_target_4: 0,

            at_amt_1: 0,
            at_target_1: 0,

            at_amt_2: 0,
            at_target_2: 0,

            at_amt_3: 0,
            at_target_3: 0,

            at_amt_4: 0,
            at_target_4: 0,
        }
    }
}
