use super::super::types::{
    MidiParameterOutput, MidiPortFunction, MidiPortsOutputChannel, MidiTransportLayer,
    ParameterDestination, TimeSignature,
};
use crate::error::{ConversionError, ParameterError, RytmError};
use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_global_t;

// routing_route_to_main_msb: u8, /* @?0x32        */
// routing_route_to_main_lsb: u8, /* @?0x33        */
// routing_send_to_fx_msb: u8,    /* @?0x34        */
// routing_send_to_fx_lsb: u8,    /* @?0x35        */
// routing_usb_in: u8, /* ?@0x47        (bit1, bit0)= 0..3 0=PRE-FX 1=POST-FX, 2=TRACK ROUTING 3=SAMPLER ONLY (bit4 bit3 bit2) 0..=7 Tracks to Left (bit7 bit6 bit5) 0..=7 Tracks to Right */
// routing_usb_out: u8, /* ?@0x48        (bit1, bit0)= 0..3 0=MAIN OUT 1=TRACK ROUTING, 2=AUDIO IN 3=OFF (bit4 bit3 bit2) 0..=7 Tracks to Left (bit7 bit6 bit5) 0..=7 Tracks to Right */
// routing_usb_to_main_db: u8, /* @0x49         0=0, 1=+6, 2=+12, 3=+18                */
