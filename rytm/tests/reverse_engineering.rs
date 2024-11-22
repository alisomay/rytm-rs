//! This file contains tests that are used to reverse engineer the RytmProject.
//!
//! The tests are not run by default, and are not expected to pass.
//! The tests are used to figure out how to decode the sysex messages.
//! The tests are not expected to be stable, and may change at any time.

mod common;
use crate::common::util::decode_sysex_response_to_raw;
use common::*;
use parking_lot::Mutex;
use rytm_rs::{
    error::RytmError,
    object::{
        pattern::{track, Pattern},
        sound::types::LfoMultiplier,
    },
    prelude::*,
    query::{GlobalQuery, KitQuery, PatternQuery, SettingsQuery, SoundQuery},
    RytmProject, SysexCompatible,
};
use std::sync::Arc;

#[test]
fn settings() {
    let mut rytm = RytmProject::try_default().unwrap();
    let conn_out = get_connection_to_rytm();
    let (_conn_in, rx) = make_input_message_forwarder();

    let query = SettingsQuery::new();
    let callback =
        |response: &[u8], rytm: &mut RytmProject, elapsed: u64| -> Result<(), RytmError> {
            if !is_sysex(response) {
                // Pass..
                return Ok(());
            }

            let (response, _meta) = decode_sysex_response_to_raw(response)?;

            clearscreen::clear().unwrap();

            let version = &response[0..4];

            let proj_bpm_msb = &response[4..5];
            let proj_bpm_lsb = &response[5..6];
            let selected_track = &response[6..7];
            let selected_track_dup = &response[7..8];
            let selected_rs_menu = &response[8..9]; // 0..5
            let selected_fx_menu = &response[9..10]; // 0..5
            dbg!(selected_fx_menu);
            let page_select = &response[10..11]; // 0..3

            let unknown_0x0b = &response[11..12];

            let track_mute_msb = &response[0x0C..0x0D];
            let track_mute_lsb = &response[0x0D..0x0E];

            let unknown_area_1 = &response[0x0E..0x15];

            let normal_chain_song = &response[0x15..0x16]; // 0 norm, 1 chain, 2 song
            let pattern_change_mode = &response[0x16..0x17]; // from left 0, 1, 2, 2 duplicates for the last one

            let unknown_area_2 = &response[0x17..0x1A];

            let fixed_vel_enable = &response[0x1A..0x1B]; // 0 norm, 1 chain, 2 song
            let fix_vel_amt = &response[0x1B..0x1C]; // from left 0, 1, 2, 2 duplicates for the last one
            let sample_recorder_stuff = &response[0x1C..=0x1E];

            let unknown0x81f = &response[0x81F..0x820];

            let sample_recorder_rlen = &response[0x820..0x821];

            let unknown_area_3 = &response[0x821..=0x826];

            dbg!(unknown_0x0b);
            dbg!(unknown_area_1);
            dbg!(unknown_area_2);
            dbg!(unknown0x81f);
            dbg!(unknown_area_3);

            Ok(())
        };

    poll_with_query_blocking(&mut rytm, query, conn_out, rx, 1000, callback).unwrap();
}

#[test]
fn global() {
    let mut rytm = RytmProject::try_default().unwrap();
    let conn_out = get_connection_to_rytm();
    let (_conn_in, rx) = make_input_message_forwarder();

    let query = GlobalQuery::new(0).unwrap();
    let callback =
        |response: &[u8], rytm: &mut RytmProject, elapsed: u64| -> Result<(), RytmError> {
            if !is_sysex(response) {
                // Pass..
                return Ok(());
            }

            let (response, _meta) = decode_sysex_response_to_raw(response)?;

            clearscreen::clear().unwrap();

            let version = &response[0..4];

            let click_active = &response[4..5];
            let click_time_sig_num = &response[5..6];
            let click_time_sig_den = &response[6..7];
            let pre_roll = &response[7..8];
            let volume = &response[8..9];

            let unknown_area_1 = &response[9..11];

            // MIDI CHANNEL PAGE
            let auto_channel = &response[11..12];
            let track_midi_channels = &response[12..24];
            let track_fx_channel = &response[24..25];
            let program_in_channel = &response[25..26];
            let program_out_channel = &response[26..27];
            let perf_channel = &response[27..28];

            // PORTS PAGE (TODO: Where is turbo speed?)
            let out_port_function = &response[28..29];
            let thru_port_function = &response[29..30];
            let input_from = &response[30..31];
            let output_to = &response[31..32];
            let param_output = &response[32..33];

            // SYNC MENU
            let clock_receive = &response[33..34];
            let clock_send = &response[34..35];
            let transport_receive = &response[35..36];
            let transport_send = &response[36..37];
            let program_change_receive = &response[37..38];
            let program_change_send = &response[38..39];

            // PORTS PAGE CONTINUES
            let receive_notes = &response[39..40];
            let receive_cc_nrpn = &response[40..41];

            let unknown_41 = &response[41..42];

            let pad_dest = &response[42..43];
            let pressure_dest = &response[43..44];
            let encoder_dest = &response[44..45];
            let mute_dest = &response[45..46];
            let ports_output_channel = &response[46..47];
            let kit_reload_on_change = &response[47..48];
            let quantize_live_rec = &response[48..49];

            let unknown_49 = &response[49..50];

            let route_to_main_msb = &response[50..51];
            let route_to_main_lsb = &response[51..52];
            let send_to_fx_msb = &response[52..53];
            let send_to_fx_lsb = &response[53..54];

            let unknown_area_2 = &response[57..70];

            let auto_track_switch = &response[70..71];
            let usb_in = &response[71..72];
            let usb_out = &response[72..73];
            let usb_to_main_db = &response[73..74];

            let unknown_area_3 = &response[74..80];

            dbg!(unknown_area_1);
            dbg!(unknown_41);
            dbg!(unknown_49);
            dbg!(unknown_area_2);
            dbg!(unknown_area_3);

            Ok(())
        };

    poll_with_query_blocking(&mut rytm, query, conn_out, rx, 1000, callback).unwrap();
}

#[test]
fn plock_seq() {
    let mut rytm = RytmProject::try_default().unwrap();
    let conn_out = get_connection_to_rytm();
    let (_conn_in, rx) = make_input_message_forwarder();

    let query = PatternQuery::new_targeting_work_buffer();

    // let mut found_types = Vec::new();
    let callback =
        |response: &[u8], rytm: &mut RytmProject, elapsed: u64| -> Result<(), RytmError> {
            if !is_sysex(response) {
                // Pass..
                return Ok(());
            }

            // let r = decode_sysex_response_to_raw(response)?;
            // std::fs::write("pattern.raw", r.0).unwrap();

            rytm.update_from_sysex_response(response)?;

            let pattern = rytm.work_buffer().pattern();
            clearscreen::clear().unwrap();
            // let types_and_tracks = pattern
            //     .parameter_lock_pool
            //     .borrow()
            //     .inner
            //     .iter()
            //     .map(|p| (p.plock_type, p.track_nr))
            //     .collect::<Vec<_>>();
            // dbg!(types_and_tracks);
            // let track = pattern.tracks()[0];
            // let plock_seqs = pattern.plock_seqs();
            // let mut for_first_trig_all = Vec::new();
            // let mut for_first_trig_values = Vec::new();
            // let mut for_first_trig_types = Vec::new();
            // for p in plock_seqs {
            //     for_first_trig_all.push((p.plock_type, p.track_nr, p.data[0]));
            //     for_first_trig_values.push(p.data[0]);
            //     for_first_trig_types.push(p.plock_type);
            // }

            // let first_trig = track.trigs()[0];

            // dbg!(track._maybe_useful_flag_from_default_trig_note);
            // dbg!(track._maybe_useful_flags_from_flags_and_speed);
            // dbg!(&for_first_trig_values[0..12]);
            // dbg!(&for_first_trig_values[12..24]);
            // dbg!(&for_first_trig_values[24..36]);
            // dbg!(&for_first_trig_values[36..48]);
            // dbg!(&for_first_trig_values[48..60]);
            // dbg!(&for_first_trig_values[60..72]);

            // dbg!(&for_first_trig_types[0..12]);
            // dbg!(&for_first_trig_types[12..24]);
            // dbg!(&for_first_trig_types[24..36]);
            // dbg!(&for_first_trig_types[36..48]);
            // dbg!(&for_first_trig_types[48..60]);
            // dbg!(&for_first_trig_types[60..72]);

            // for t in &for_first_trig_types {
            //     if !found_types.contains(t) {
            //         found_types.push(*t);
            //     }
            // }

            // if !found_types.is_empty() {
            //     if found_types.len() > 1 {
            //         dbg!(found_types[found_types.len() - 2]);
            //     }
            //     dbg!(found_types.last());
            // }

            // println!(
            //     "unknown_3msb_flags_retrig_rate: {:08b}",
            //     first_trig.unknown_3msb_flags_retrig_rate
            // );
            // println!(
            //     "unknown_flag_retrig_length: {:08b}",
            //     first_trig.unknown_flag_retrig_length
            // );
            // dbg!(track.default_trig_condition());
            // dbg!(track.default_trig_flags());
            // dbg!(track.default_trig_note());
            // dbg!(track.default_trig_note_length());
            // dbg!(track.default_trig_probability());
            // dbg!(track.default_trig_velocity());

            // let mut reverse = 0;
            // reverse |=
            //     (((encode_micro_timing_byte(&first_trig.micro_timing()) as u8) & 0b1100_0000) >> 2);
            // reverse |= ((first_trig.unknown_flag_retrig_length & 0b1000_0000) >> 4);
            // reverse |= ((first_trig.unknown_3msb_flags_retrig_rate & 0b1110_0000) >> 5);

            // dbg!(reverse);

            // dbg!(track);
            // How does plock effect

            //         [rytm/tests/reverse_engineering.rs:191] first_trig = Trig {
            //     index: 0,
            //     flags: 0000_0011_1000_0011 - ENABLE, RETRIG, SYN_PL_SW, SMP_PL_SW, ENV_PL_SW,
            //     note: 127, // How to set this? is 127 unset because it is 0xFF with trig_condition false?
            //     trig_condition: false, // What is this?
            //     velocity: 255, // How to set this?
            //     note_length: Unset, // ok I guess how to set it?
            //     micro_timing: OnGrid, // ok
            //     retrig_rate: _1B12, // ok
            //     retrig_length: _11D5, // ok
            //     retrig_velocity_offset: 91, // ok
            //     sound_lock: 255, // check
            // }
            // dbg!(plock_seqs);
            // panic!();
            Ok(())
        };

    poll_with_query_blocking(&mut rytm, query, conn_out, rx, 1000, callback).unwrap();
}

#[test]
fn sound() {
    let mut rytm = RytmProject::try_default().unwrap();
    let conn_out = get_connection_to_rytm();
    let (_conn_in, rx) = make_input_message_forwarder();

    let track_index = 0;

    let query = SoundQuery::new(0).unwrap();
    let query = SoundQuery::new_targeting_work_buffer(track_index).unwrap();

    let callback =
        |response: &[u8], rytm: &mut RytmProject, elapsed: u64| -> Result<(), RytmError> {
            if !is_sysex(response) {
                // Pass..
                return Ok(());
            }

            rytm.update_from_sysex_response(response)?;
            let sound = rytm.work_buffer().sounds();

            clearscreen::clear().unwrap();

            // convert unix epoch to human readable milliseconds
            // let response_time = elapsed / 1_000_000;

            // dbg!(elapsed);
            // dbg!(sound[track_index].machine_parameters());
            dbg!(sound[0].name(), sound[0].amplitude());

            Ok(())
        };

    poll_with_query_blocking(&mut rytm, query, conn_out, rx, 2000, callback).unwrap();
}

#[test]
fn kit() {
    let mut rytm = RytmProject::try_default().unwrap();
    let conn_out = get_connection_to_rytm();
    let (_conn_in, rx) = make_input_message_forwarder();

    let query = KitQuery::new(0).unwrap();
    let query = KitQuery::new_targeting_work_buffer();

    let callback =
        |response: &[u8], rytm: &mut RytmProject, elapsed: u64| -> Result<(), RytmError> {
            if !is_sysex(response) {
                // Pass..
                return Ok(());
            }

            rytm.update_from_sysex_response(response)?;
            let kit = &rytm.kits()[0];
            let kit = rytm.work_buffer().kit();

            clearscreen::clear().unwrap();

            dbg!(kit);

            // println!("mode_flags: {:08b}", kit.sounds()[6].mode_flags);

            Ok(())
        };

    poll_with_query_blocking(&mut rytm, query, conn_out, rx, 1000, callback).unwrap();
}

#[test]
fn global_type() {
    let mut rytm = RytmProject::try_default().unwrap();
    let conn_out = get_connection_to_rytm();
    let (_conn_in, rx) = make_input_message_forwarder();

    let query = GlobalQuery::new(0).unwrap();
    let query = GlobalQuery::new_targeting_work_buffer();

    let callback =
        |response: &[u8], rytm: &mut RytmProject, elapsed: u64| -> Result<(), RytmError> {
            if !is_sysex(response) {
                // Pass..
                return Ok(());
            }

            rytm.update_from_sysex_response(response)?;
            let global = rytm.globals()[0];
            let global = rytm.work_buffer().global();

            clearscreen::clear().unwrap();

            // let rt: u16 = ((global.routing_route_to_main_msb as u16) << 8)
            //     | (global.routing_route_to_main_lsb as u16);

            // println!(
            //     "{:04b}_{:04b}_{:04b}_{:04b} ",
            //     (rt >> 12) & 0b1111,
            //     (rt >> 8) & 0b1111,
            //     (rt >> 4) & 0b1111,
            //     rt & 0b1111,
            // );

            // println!(
            //     "{:03b}_{:03b}_{:02b}",
            //     (global.routing_usb_in & 0b11100000) >> 5,
            //     (global.routing_usb_in & 0b00011100) >> 2,
            //     global.routing_usb_in & 0b00000011,
            // );

            // dbg!();
            // println!("usb_out: {:08b}", global.routing_usb_out);
            // println!("other: {}", global.routing_usb_out >> 2);
            // println!("usb_out: {:08b}", global.routing_usb_out);

            Ok(())
        };

    poll_with_query_blocking(&mut rytm, query, conn_out, rx, 200, callback).unwrap();
}

#[test]
fn settings_type() {
    let mut rytm = RytmProject::try_default().unwrap();
    let conn_out = get_connection_to_rytm();
    let (_conn_in, rx) = make_input_message_forwarder();

    let query = SettingsQuery::new();

    let callback =
        |response: &[u8], rytm: &mut RytmProject, elapsed: u64| -> Result<(), RytmError> {
            if !is_sysex(response) {
                // Pass..
                return Ok(());
            }

            rytm.update_from_sysex_response(response)?;
            let settings = rytm.settings();

            // clearscreen::clear().unwrap();

            // let mute: u16 = ((settings.track_mute_msb as u16) << 8) | (settings.track_mute_lsb as u16);

            // println!(
            //     "{:04b}_{:04b}_{:04b}_{:04b} ",
            //     (mute >> 12) & 0b1111,
            //     (mute >> 8) & 0b1111,
            //     (mute >> 4) & 0b1111,
            //     mute & 0b1111,
            // );

            dbg!(settings);

            // println!("usb_out: {:08b}", global.routing_usb_out);
            // println!("other: {}", global.routing_usb_out >> 2);

            Ok(())
        };

    poll_with_query_blocking(&mut rytm, query, conn_out, rx, 1000, callback).unwrap();
}

#[test]
fn pattern_type() {
    let mut rytm = RytmProject::try_default().unwrap();
    let conn_out = get_connection_to_rytm();
    let (_conn_in, rx) = make_input_message_forwarder();

    let query = PatternQuery::new_targeting_work_buffer();

    let out = conn_out.clone();
    let callback =
        |response: &[u8], rytm: &mut RytmProject, elapsed: u64| -> Result<(), RytmError> {
            if !is_sysex(response) {
                // Pass..
                return Ok(());
            }

            rytm.update_from_sysex_response(response)?;
            let pattern = rytm.work_buffer_mut().pattern_mut();
            dbg!(&pattern.tracks()[0].trigs()[0]);

            // let t = &mut pattern.tracks_mut()[0];
            // let trigs = t.trigs_mut();
            // let trig0 = &mut trigs[0];

            // trig0.plock_set_lfo_depth(-66.0).unwrap();
            // trig0.plock_set_lfo_speed(-63).unwrap();
            // trig0.plock_set_lfo_multiplier(LfoMultiplier::X1).unwrap();
            // out.lock().send(&pattern.as_sysex()?).unwrap();
            // clearscreen::clear().unwrap();

            // let mute: u16 = ((settings.track_mute_msb as u16) << 8) | (settings.track_mute_lsb as u16);

            // println!(
            //     "{:04b}_{:04b}_{:04b}_{:04b} ",
            //     (mute >> 12) & 0b1111,
            //     (mute >> 8) & 0b1111,
            //     (mute >> 4) & 0b1111,
            //     mute & 0b1111,
            // );

            // dbg!(pattern.tracks()[0]);

            // println!("usb_out: {:08b}", global.routing_usb_out);
            // println!("other: {}", global.routing_usb_out >> 2);

            Ok(())
        };

    poll_with_query_blocking(&mut rytm, query, conn_out, rx, 3000, callback).unwrap();
}

#[test]
fn usage() {
    use midir::{Ignore, MidiInputConnection, MidiOutputConnection};
    use parking_lot::Mutex;
    use rytm_rs::prelude::*;
    use std::sync::Arc;

    // We'll be using this connection for sending sysex messages to the device.
    fn get_connection_to_rytm() -> Arc<Mutex<MidiOutputConnection>> {
        let output = port::MidiOut::new("rytm_test_out").unwrap();
        let rytm_out_identifier = "to Max 1";
        let rytm_output_port = output.find_output_port(rytm_out_identifier).unwrap();

        Arc::new(Mutex::new(
            output.make_output_connection(&rytm_output_port, 0).unwrap(),
        ))
    }

    // We'll be using this connection for receiving sysex messages from the device and forwarding them to our main thread.
    pub fn make_input_message_forwarder() -> (
        MidiInputConnection<()>,
        std::sync::mpsc::Receiver<(Vec<u8>, u64)>,
    ) {
        let mut input = crate::port::MidiIn::new("rytm_test_in").unwrap();
        input.ignore(Ignore::None);
        let rytm_in_identifier = "from Max 1";
        let rytm_input_port = input.find_input_port(rytm_in_identifier).unwrap();

        let (tx, rx) = std::sync::mpsc::channel::<(Vec<u8>, u64)>();

        let conn_in: midir::MidiInputConnection<()> = input
            .into_inner()
            .connect(
                &rytm_input_port,
                "rytm_test_in",
                move |stamp, message, _| {
                    // Do some filtering here if you like.
                    tx.send((message.to_vec(), stamp)).unwrap();
                },
                (),
            )
            .unwrap();

        (conn_in, rx)
    }

    // Make a default rytm project
    let mut rytm = RytmProject::try_default().unwrap();

    // Get a connection to the device
    let conn_out = get_connection_to_rytm();

    // Listen for incoming messages from the device
    let (_conn_in, rx) = make_input_message_forwarder();

    // Make a query for the pattern in the work buffer
    let query = PatternQuery::new_targeting_work_buffer();

    // Send the query to the device
    conn_out.lock().send(&query.as_sysex().unwrap()).unwrap();

    // Wait for the response
    match rx.recv() {
        Ok((message, _stamp)) => {
            match rytm.update_from_sysex_response(&message) {
                Ok(_) => {
                    for track in rytm.work_buffer_mut().pattern_mut().tracks_mut() {
                        // Set the number of steps to 64
                        track.set_number_of_steps(64).unwrap();
                        for (i, trig) in track.trigs_mut().iter_mut().enumerate() {
                            // Enable every 4th trig.
                            // Set retrig on.
                            if i % 4 == 0 {
                                trig.set_trig_enable(true);
                                trig.set_retrig(true);
                            }
                        }
                    }

                    // Send the updated pattern to the device if you like
                    conn_out
                        .lock()
                        .send(&rytm.work_buffer().pattern().as_sysex().unwrap())
                        .unwrap();
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

    // let mut rytm = RytmProject::try_default().unwrap();

    // let patterns = rytm.patterns_mut();

    // let pattern_2 = &mut patterns[2];

    // let track_0 = &mut pattern_2.tracks_mut()[0];

    // let trigs = track_0.trigs_mut();

    // for trig in trigs {
    //     trig.plock_set_lfo_depth(-66.0).unwrap();
    //     trig.plock_set_lfo_speed(-63).unwrap();
    //     trig.plock_set_lfo_multiplier(LfoMultiplier::X1).unwrap();
    // }

    // // Use shared ref.

    // let rytm_ref = &rytm;
    // rytm_ref
    //     .encode_work_buffer_pattern_as_sysex_message()
    //     .unwrap();

    // drop(rytm_ref);

    // // Use mut ref again.

    // for trig in trigs {
    //     trig.plock_set_lfo_depth(-66.0).unwrap();
    //     trig.plock_set_lfo_speed(-63).unwrap();
    //     trig.plock_set_lfo_multiplier(LfoMultiplier::X1).unwrap();
    // }

    // use rytm_rs::SysexCompatible;

    // {
    //     let patterns = rytm.patterns_mut();
    //     let pattern_2 = &mut patterns[2];
    //     let track_0 = &mut pattern_2.tracks_mut()[0];
    //     let trigs = track_0.trigs_mut();

    //     for trig in trigs {
    //         trig.plock_set_lfo_depth(-66.0).unwrap();
    //         trig.plock_set_lfo_speed(-63).unwrap();
    //         trig.plock_set_lfo_multiplier(LfoMultiplier::X1).unwrap();
    //     }

    //     let msg = pattern_2.as_sysex().unwrap();
    // } // Mutable borrows end here

    // Now it's safe to borrow rytm immutably
    // let rytm_ref = &rytm;
    // rytm_ref
    //     .encode_work_buffer_pattern_as_sysex_message()
    //     .unwrap();

    // // Start a new scope for further mutable borrows
    // {
    //     let patterns = rytm.patterns_mut();
    //     let pattern_2 = &mut patterns[2];
    //     let track_0 = &mut pattern_2.tracks_mut()[0];
    //     let trigs = track_0.trigs_mut();

    //     for trig in trigs {
    //         trig.plock_set_lfo_depth(-66.0).unwrap();
    //         trig.plock_set_lfo_speed(-63).unwrap();
    //         trig.plock_set_lfo_multiplier(LfoMultiplier::X1).unwrap();
    //     }
    // }
}

#[test]
fn stuff() {
    let mut rytm = RytmProject::try_default().unwrap();
    let ser = serde_json::to_string(&rytm).unwrap();
    std::fs::write("proj.json", ser).unwrap();
    // let out = get_connection_to_rytm();
    // let (_conn_in, rx) = make_input_message_forwarder();

    // let query = SoundQuery::new_targeting_work_buffer(0).unwrap();

    // let mut rytm = Arc::new(Mutex::new(RytmProject::try_default().unwrap()));
    // let out = get_connection_to_rytm();

    // let rytm1 = rytm.clone();
    // let out1 = out.clone();

    // std::thread::spawn(move || loop {
    //     out1.lock()
    //         .unwrap()
    //         .send(&query.as_sysex().unwrap())
    //         .unwrap();

    //     match rx.recv() {
    //         Ok((message, _stamp)) => {
    //             callback(&message, rytm, elapsed)?;
    //         }
    //         Err(err) => {
    //             println!("Error: {:?}", err);
    //         }
    //     }

    //     std::thread::sleep(std::time::Duration::from_millis(interval_in_millis));
    // });

    // let mut out = out.lock();
    // out.send(&query.as_sysex().unwrap()).unwrap();

    // while let Ok((msg, _)) = rx.recv() {
    //     if !is_sysex(&msg) {
    //         continue;
    //     }

    //     if rytm.update_from_sysex_response(&msg).is_err() {
    //         continue;
    //     }

    //     let sound = &mut rytm.work_buffer_mut().sounds_mut()[0];
    //     sound
    //         .set_machine_type(rytm_rs::object::sound::types::MachineType::BdAcoustic)
    //         .unwrap();

    //     out.send(&sound.as_sysex().unwrap()).unwrap();
    // }

    // rytm.update_from_sysex_response(response)?;
    // let pattern = rytm.work_buffer_mut().pattern_mut();

    // let t = &mut pattern.tracks_mut()[0];
    // let trigs = t.trigs_mut();
    // let trig0 = &mut trigs[0];

    // trig0.plock_set_lfo_depth(-66.0).unwrap();
    // trig0.plock_set_lfo_speed(-63).unwrap();
    // trig0.plock_set_lfo_multiplier(LfoMultiplier::X1).unwrap();
    // out.lock().send(&pattern.as_sysex()?).unwrap();
}

// pub fn poll_with_query_blocking(
//     rytm: &mut RytmProject,
//     query: impl ObjectQuery,
//     conn_out: Arc<Mutex<MidiOutputConnection>>,
//     rx: std::sync::mpsc::Receiver<(Vec<u8>, u64)>,
//     interval_in_millis: u64,
//     mut callback: impl FnMut(&[u8], &mut RytmProject, u64) -> Result<(), RytmError>,
// ) -> Result<(), RytmError> {
//     loop {
//         conn_out
//             .lock()
//             .unwrap()
//             .send(&query.as_sysex().unwrap())
//             .unwrap();
//         // Timestamp
//         let query_start = std::time::SystemTime::now()
//             .duration_since(std::time::UNIX_EPOCH)
//             .unwrap()
//             .as_millis() as u64;

//         match rx.recv() {
//             Ok((message, _stamp)) => {
//                 let response_received = std::time::SystemTime::now()
//                     .duration_since(std::time::UNIX_EPOCH)
//                     .unwrap()
//                     .as_millis() as u64;
//                 let elapsed = response_received - query_start;
//                 callback(&message, rytm, elapsed)?;
//             }
//             Err(err) => {
//                 println!("Error: {:?}", err);
//             }
//         }

//         std::thread::sleep(std::time::Duration::from_millis(interval_in_millis));
//     }
// }
