use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_global_t;

use crate::error::{ParameterError, RytmError};
use crate::sysex::SysexMeta;

// typedef struct {
//    sU8 version[4];             /* @0x00..0x03  32 bit version number (0x00, 0x00, 0x00, 0x02) */
//    /* Click Menu */
//    sU8 click_active;           /* @0x04         0=OFF, 1=ON                                */
//    sU8 click_time_sig_num;     /* @0x05         range=0..15 maps to 1..16 on device        */
//    sU8 click_time_sig_den;     /* @0x06         0=1, 1=2, 2=4, 3=8, 4=16                   */
//    sU8 pre_roll;               /* @0x07         0=OFF, range=0..15 maps to 1..16 on device */
//    sU8 volume;                 /* @0x08         range=0..127                               */
//    sU8 __unknown0x09_0x0A[2];  /* @?0x09..0x0A  Currently reads  0x40, 0x00 */
//    /* Midi Channels Menu */
//    sU8 auto_channel;           /* @0x0B         255=OFF  range=0..15  */
//    sU8 track_channels[12];     /* @0x0C..0x17   255=OFF  range=0..15  */
//    sU8 track_fx_channel;       /* @0x18         255=OFF  range=0..15  */
//    sU8 prog_ch_in_channel;     /* @0x19         255=auto range=0..15  */
//    sU8 prog_ch_out_channel;    /* @0x1A         255=auto range=0..15  */
//    sU8 perf_channel;           /* @0x1B         255=OFF  range=0..15  */
//    /* Midi Port Config Menu Part 1 */
//    sU8 out_port_func;          /* @0x1C         0=MIDI, 1=DIN24, 2=DIN48               */
//    sU8 thru_port_func;         /* @0x1D         0=MIDI, 1=DIN24, 2=DIN48               */
//    sU8 input_from;             /* @0x1E         0=DISABLED, 1=MIDI, 2=USB. 3=MIDI+USB  */
//    sU8 output_to;              /* @0x1F         0=DISABLED, 1=MIDI, 2=USB, 3=MIDI+USB  */
//    sU8 param_output;           /* @0x20         0=NRPN, 1=CC                           */
//    /* Midi Sync Menu */
//    sU8 clock_receive;          /* @0x21         0=OFF, 1=ON */
//    sU8 clock_send;             /* @0x22         0=OFF, 1=ON */
//    sU8 transport_receive;      /* @0x23         0=OFF, 1=ON */
//    sU8 transport_send;         /* @0x24         0=OFF, 1=ON */
//    sU8 program_change_receive; /* @0x25         0=OFF, 1=ON */
//    sU8 program_change_send;    /* @0x26         0=OFF, 1=ON */
//    /* Midi Port Config Menu Part 2 */
//    sU8 receive_notes;          /* @0x27         0=OFF, 1=ON */
//    sU8 receive_cc_nrpn;        /* @0x28         0=OFF, 1=ON */
//    /* I believe this is `TURBO SPEED` since it is the only one left in the menu. */
//    /* But since I can not enable it without connecting a turbo speed capable MIDI interface I can not be sure.. */
//    sU8 __unknown0x29;          /* ?@0x29        0=OFF, 1=ON */
//    sU8 pad_dest;               /* @0x2A         0=INT, 1=INT+EXT, 2=EXT */
//    sU8 pressure_dest;          /* @0x2B         0=INT, 1=INT+EXT, 2=EXT */
//    sU8 encoder_dest;           /* @0x2C         0=INT, 1=INT+EXT        */
//    sU8 mute_dest;              /* @0x2D         0=INT, 1=INT+EXT, 2=EXT */
//    sU8 ports_output_channel;   /* @0x2E         0=AUTO CH, 1=TRK_CH     */
//    /* Sequencer Config Menu Part 1 */
//    sU8 kit_reload_on_chg;      /* @0x2F         0=OFF, 1=ON */
//    sU8 quantize_live_rec;      /* @0x30         0=OFF, 1=ON */
//    sU8 __unknown0x31;          /* ?@0x31        */
//    /* Routing Menu Part 1 (the semantics are not discovered yet) */
//    sU8 route_to_main_msb;      /* @?0x32        */
//    sU8 route_to_main_lsb;      /* @?0x33        */
//    sU8 send_to_fx_msb;         /* @?0x34        */
//    sU8 send_to_fx_lsb;         /* @?0x35        */
//    /* All zeros. It is suspicious since it is exactly 16 bytes long, maybe related to midi channels? */
//    sU8 __unknown0x36_0x45[16]; /* @?0x36..0x45  */
//    /* Sequencer Config Menu Part 2 */
//    sU8 auto_trk_switch;        /* @0x46         0=OFF, 1=ON */
//    /* Routing Menu Part 2 */
//    sU8 usb_in;                 /* ?@0x47        (the semantics are not discovered yet) */
//    sU8 usb_out;                /* ?@0x48        (the semantics are not discovered yet) */
//    sU8 usb_to_main_db;         /* @0x49         0=0, 1=+6, 2=+12, 3=+18                */
//    /* All zeros. */
//    sU8 __unknown0x50_0x4F[6];  /* @?0x50..0x4F  */
// } ar_global_t;

#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Global {
    index: usize,
    sysex_meta: SysexMeta,
    /// Version of the kit structure.
    version: u32,

    click_active: u8, /* @0x04         0=OFF, 1=ON                                */
    click_time_sig_num: u8, /* @0x05         range=0..15 maps to 1..16 on device        */
    click_time_sig_den: u8, /* @0x06         0=1, 1=2, 2=4, 3=8, 4=16                   */
    click_pre_roll: u8, /* @0x07         0=OFF, range=0..15 maps to 1..16 on device */
    click_volume: u8, /* @0x08         range=0..127                               */

    midi_channels_auto_channel: u8, /* @0x0B         255=OFF  range=0..15  */
    midi_channels_track_channels: [u8; 12], /* @0x0C..0x17   255=OFF  range=0..15  */
    midi_channels_track_fx_channel: u8, /* @0x18         255=OFF  range=0..15  */
    midi_channels_prog_ch_in_channel: u8, /* @0x19         255=auto range=0..15  */
    midi_channels_prog_ch_out_channel: u8, /* @0x1A         255=auto range=0..15  */
    midi_channels_perf_channel: u8, /* @0x1B         255=OFF  range=0..15  */

    midi_port_out_port_func: u8, /* @0x1C         0=MIDI, 1=DIN24, 2=DIN48               */
    midi_port_thru_port_func: u8, /* @0x1D         0=MIDI, 1=DIN24, 2=DIN48               */
    midi_port_input_from: u8,    /* @0x1E         0=DISABLED, 1=MIDI, 2=USB. 3=MIDI+USB  */
    midi_port_output_to: u8,     /* @0x1F         0=DISABLED, 1=MIDI, 2=USB, 3=MIDI+USB  */
    midi_port_param_output: u8,  /* @0x20         0=NRPN, 1=CC                           */
    midi_port_receive_notes: u8, /* @0x27         0=OFF, 1=ON */
    midi_port_receive_cc_nrpn: u8, /* @0x28         0=OFF, 1=ON */

    midi_port_pad_dest: u8,      /* @0x2A         0=INT, 1=INT+EXT, 2=EXT */
    midi_port_pressure_dest: u8, /* @0x2B         0=INT, 1=INT+EXT, 2=EXT */
    midi_port_encoder_dest: u8,  /* @0x2C         0=INT, 1=INT+EXT        */
    midi_port_mute_dest: u8,     /* @0x2D         0=INT, 1=INT+EXT, 2=EXT */
    midi_port_ports_output_channel: u8, /* @0x2E         0=AUTO CH, 1=TRK_CH     */

    midi_sync_clock_receive: u8,          /* @0x21         0=OFF, 1=ON */
    midi_sync_clock_send: u8,             /* @0x22         0=OFF, 1=ON */
    midi_sync_transport_receive: u8,      /* @0x23         0=OFF, 1=ON */
    midi_sync_transport_send: u8,         /* @0x24         0=OFF, 1=ON */
    midi_sync_program_change_receive: u8, /* @0x25         0=OFF, 1=ON */
    midi_sync_program_change_send: u8,    /* @0x26         0=OFF, 1=ON */

    // Sequencer Config Menu Part 1 */
    sequencer_config_kit_reload_on_chg: u8, /* @0x2F         0=OFF, 1=ON */
    sequencer_config_quantize_live_rec: u8, /* @0x30         0=OFF, 1=ON */
    sequencer_config_auto_trk_switch: u8,   /* @0x46         0=OFF, 1=ON */

    routing_route_to_main_msb: u8, /* @?0x32        */
    routing_route_to_main_lsb: u8, /* @?0x33        */
    routing_send_to_fx_msb: u8,    /* @?0x34        */
    routing_send_to_fx_lsb: u8,    /* @?0x35        */
    routing_usb_in: u8, /* ?@0x47        (bit1, bit0)= 0..3 0=PRE-FX 1=POST-FX, 2=TRACK ROUTING 3=SAMPLER ONLY (bit4 bit3 bit2) 0..=7 Tracks to Left (bit7 bit6 bit5) 0..=7 Tracks to Right */
    routing_usb_out: u8, /* ?@0x48        (bit1, bit0)= 0..3 0=MAIN OUT 1=TRACK ROUTING, 2=AUDIO IN 3=OFF (bit4 bit3 bit2) 0..=7 Tracks to Left (bit7 bit6 bit5) 0..=7 Tracks to Right */
    routing_usb_to_main_db: u8, /* @0x49         0=0, 1=+6, 2=+12, 3=+18                */

    #[derivative(Debug = "ignore")]
    pub(crate) __unknown0x09_0x0a: [u8; 2], /* @?0x09..0x0A  Currently reads  0x40, 0x00 */
    // I believe this is `TURBO SPEED` since it is the only one left in the menu. */
    // But since I can not enable it without connecting a turbo speed capable MIDI interface I can not be sure.. */
    #[derivative(Debug = "ignore")]
    pub(crate) __unknown0x29: u8, /* ?@0x29        0=OFF, 1=ON */
    #[derivative(Debug = "ignore")]
    pub(crate) __unknown0x31: u8, /* ?@0x31        */
    #[derivative(Debug = "ignore")]
    // All zeros. It is suspicious since it is exactly 16 bytes long, maybe related to midi channels?
    pub(crate) __unknown0x36_0x45: [u8; 16], /* @?0x36..0x45  */
    #[derivative(Debug = "ignore")]
    // All zeros.
    pub(crate) __unknown0x50_0x4f: [u8; 6], /* @?0x50..0x4F  */
}

impl From<&Global> for ar_global_t {
    fn from(kit: &Global) -> Self {
        todo!("Conversion to ar_global_t is not implemented yet.")
    }
}

impl Global {
    pub fn to_raw_parts(&self) -> (SysexMeta, ar_global_t) {
        (self.sysex_meta, self.into())
    }

    pub fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_global: &ar_global_t,
    ) -> Result<Self, RytmError> {
        let version = ((raw_global.version[0] as u32) << 24)
            | ((raw_global.version[1] as u32) << 16)
            | ((raw_global.version[2] as u32) << 8)
            | (raw_global.version[3] as u32);

        let slot_number = if sysex_meta.is_targeting_work_buffer() {
            // TODO: Double check
            0
        } else {
            sysex_meta.obj_nr as usize
        };

        Ok(Self {
            index: slot_number,
            sysex_meta,
            version,

            click_active: raw_global.click_active,
            click_time_sig_num: raw_global.click_time_sig_num,
            click_time_sig_den: raw_global.click_time_sig_den,
            click_pre_roll: raw_global.pre_roll,
            click_volume: raw_global.volume,

            midi_channels_auto_channel: raw_global.auto_channel,
            midi_channels_track_channels: raw_global.track_channels,
            midi_channels_track_fx_channel: raw_global.track_fx_channel,
            midi_channels_prog_ch_in_channel: raw_global.prog_ch_in_channel,
            midi_channels_prog_ch_out_channel: raw_global.prog_ch_out_channel,
            midi_channels_perf_channel: raw_global.perf_channel,

            midi_port_out_port_func: raw_global.out_port_func,
            midi_port_thru_port_func: raw_global.thru_port_func,
            midi_port_input_from: raw_global.input_from,
            midi_port_output_to: raw_global.output_to,
            midi_port_param_output: raw_global.param_output,
            midi_port_receive_notes: raw_global.receive_notes,
            midi_port_receive_cc_nrpn: raw_global.receive_cc_nrpn,
            midi_port_pad_dest: raw_global.pad_dest,
            midi_port_pressure_dest: raw_global.pressure_dest,
            midi_port_encoder_dest: raw_global.encoder_dest,
            midi_port_mute_dest: raw_global.mute_dest,
            midi_port_ports_output_channel: raw_global.ports_output_channel,

            midi_sync_clock_receive: raw_global.clock_receive,
            midi_sync_clock_send: raw_global.clock_send,
            midi_sync_transport_receive: raw_global.transport_receive,
            midi_sync_transport_send: raw_global.transport_send,
            midi_sync_program_change_receive: raw_global.program_change_receive,
            midi_sync_program_change_send: raw_global.program_change_send,

            sequencer_config_kit_reload_on_chg: raw_global.kit_reload_on_chg,
            sequencer_config_quantize_live_rec: raw_global.quantize_live_rec,
            sequencer_config_auto_trk_switch: raw_global.auto_trk_switch,

            routing_route_to_main_msb: raw_global.route_to_main_msb,
            routing_route_to_main_lsb: raw_global.route_to_main_lsb,
            routing_send_to_fx_msb: raw_global.send_to_fx_msb,
            routing_send_to_fx_lsb: raw_global.send_to_fx_lsb,
            routing_usb_in: raw_global.usb_in,
            routing_usb_out: raw_global.usb_out,
            routing_usb_to_main_db: raw_global.usb_to_main_db,

            __unknown0x09_0x0a: raw_global.__unknown0x09_0x0A,
            __unknown0x29: raw_global.__unknown0x29,
            __unknown0x31: raw_global.__unknown0x31,
            __unknown0x36_0x45: raw_global.__unknown0x36_0x45,
            __unknown0x50_0x4f: raw_global.__unknown0x50_0x4F,
        })
    }

    #[parameter_range(range = "global_slot:0..=3")]
    pub fn try_default(global_slot: usize) -> Result<Self, RytmError> {
        Ok(Self {
            index: global_slot,
            sysex_meta: SysexMeta::try_default_for_global(global_slot, None)?,
            version: 2,

            click_active: 0,
            click_time_sig_num: 0,
            click_time_sig_den: 0,
            click_pre_roll: 0,
            click_volume: 0,

            midi_channels_auto_channel: 0,
            midi_channels_track_channels: [0; 12],
            midi_channels_track_fx_channel: 0,
            midi_channels_prog_ch_in_channel: 0,
            midi_channels_prog_ch_out_channel: 0,
            midi_channels_perf_channel: 0,

            midi_port_out_port_func: 0,
            midi_port_thru_port_func: 0,
            midi_port_input_from: 0,
            midi_port_output_to: 0,
            midi_port_param_output: 0,
            midi_port_receive_notes: 0,
            midi_port_receive_cc_nrpn: 0,
            midi_port_pad_dest: 0,
            midi_port_pressure_dest: 0,
            midi_port_encoder_dest: 0,
            midi_port_mute_dest: 0,
            midi_port_ports_output_channel: 0,

            midi_sync_clock_receive: 0,
            midi_sync_clock_send: 0,
            midi_sync_transport_receive: 0,
            midi_sync_transport_send: 0,
            midi_sync_program_change_receive: 0,
            midi_sync_program_change_send: 0,

            sequencer_config_kit_reload_on_chg: 0,
            sequencer_config_quantize_live_rec: 0,
            sequencer_config_auto_trk_switch: 0,

            routing_route_to_main_msb: 0,
            routing_route_to_main_lsb: 0,
            routing_send_to_fx_msb: 0,
            routing_send_to_fx_lsb: 0,
            routing_usb_in: 0,
            routing_usb_out: 0,
            routing_usb_to_main_db: 0,

            __unknown0x09_0x0a: [0; 2],
            __unknown0x29: 0,
            __unknown0x31: 0,
            __unknown0x36_0x45: [0; 16],
            __unknown0x50_0x4f: [0; 6],
        })
    }

    pub fn work_buffer_default() -> Self {
        Self {
            index: 0,
            sysex_meta: SysexMeta::default_for_global_in_work_buffer(None),
            version: 2,

            click_active: 0,
            click_time_sig_num: 0,
            click_time_sig_den: 0,
            click_pre_roll: 0,
            click_volume: 0,

            midi_channels_auto_channel: 0,
            midi_channels_track_channels: [0; 12],
            midi_channels_track_fx_channel: 0,
            midi_channels_prog_ch_in_channel: 0,
            midi_channels_prog_ch_out_channel: 0,
            midi_channels_perf_channel: 0,

            midi_port_out_port_func: 0,
            midi_port_thru_port_func: 0,
            midi_port_input_from: 0,
            midi_port_output_to: 0,
            midi_port_param_output: 0,
            midi_port_receive_notes: 0,
            midi_port_receive_cc_nrpn: 0,
            midi_port_pad_dest: 0,
            midi_port_pressure_dest: 0,
            midi_port_encoder_dest: 0,
            midi_port_mute_dest: 0,
            midi_port_ports_output_channel: 0,

            midi_sync_clock_receive: 0,
            midi_sync_clock_send: 0,
            midi_sync_transport_receive: 0,
            midi_sync_transport_send: 0,
            midi_sync_program_change_receive: 0,
            midi_sync_program_change_send: 0,

            sequencer_config_kit_reload_on_chg: 0,
            sequencer_config_quantize_live_rec: 0,
            sequencer_config_auto_trk_switch: 0,

            routing_route_to_main_msb: 0,
            routing_route_to_main_lsb: 0,
            routing_send_to_fx_msb: 0,
            routing_send_to_fx_lsb: 0,
            routing_usb_in: 0,
            routing_usb_out: 0,
            routing_usb_to_main_db: 0,

            __unknown0x09_0x0a: [0; 2],
            __unknown0x29: 0,
            __unknown0x31: 0,
            __unknown0x36_0x45: [0; 16],
            __unknown0x50_0x4f: [0; 6],
        }
    }
}
