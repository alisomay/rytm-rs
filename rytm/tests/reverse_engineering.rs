//! This file contains tests that are used to reverse engineer the Rytm.
//!
//! The tests are not run by default, and are not expected to pass.
//! The tests are used to figure out how to decode the sysex messages.
//! The tests are not expected to be stable, and may change at any time.

mod common;
use common::*;
use rytm_rs::query::{GlobalQuery, SettingsQuery};
use rytm_rs::{error::RytmError, Rytm};

#[test]
fn settings() {
    let mut rytm = Rytm::default();
    let conn_out = get_connection_to_rytm();
    let (_conn_in, rx) = make_input_message_forwarder();

    let query = SettingsQuery::new();
    let callback = |response: &[u8], rytm: &mut Rytm| -> Result<(), RytmError> {
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
    let mut rytm = Rytm::default();
    let conn_out = get_connection_to_rytm();
    let (_conn_in, rx) = make_input_message_forwarder();

    let query = GlobalQuery::new(0);
    let callback = |response: &[u8], rytm: &mut Rytm| -> Result<(), RytmError> {
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

// Follow the same pattern for creating reverse engineering environments.
