//! Utilities for reverse engineering sysex responses from the Rytm.

pub(crate) mod port;
pub(crate) mod util;

use midir::{Ignore, MidiInputConnection, MidiOutputConnection};
use rytm_rs::{
    error::{RytmError, SysexConversionError},
    query::ObjectQuery,
    Rytm,
};
use std::sync::{Arc, Mutex};
use util::SysexMeta;

pub fn get_connection_to_rytm() -> Arc<Mutex<MidiOutputConnection>> {
    let output = port::MidiOut::new("rytm_test_out").unwrap();
    let rytm_out_identifier = "Elektron Analog Rytm MKII";
    let rytm_output_port = output.find_output_port(rytm_out_identifier).unwrap();

    Arc::new(Mutex::new(
        output.make_output_connection(&rytm_output_port, 0).unwrap(),
    ))
}

pub fn make_input_message_forwarder(
) -> (MidiInputConnection<()>, std::sync::mpsc::Receiver<Vec<u8>>) {
    let mut input = crate::port::MidiIn::new("rytm_test_in").unwrap();
    input.ignore(Ignore::None);
    let rytm_in_identifier = "Elektron Analog Rytm MKII";
    let rytm_input_port = input.find_input_port(rytm_in_identifier).unwrap();

    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();

    let conn_in: midir::MidiInputConnection<()> = input
        .into_inner()
        .connect(
            &rytm_input_port,
            "rytm_test_in",
            move |_stamp, message, _| {
                // Forward ro the receiver for continuous monitoring.
                tx.send(message.to_vec()).unwrap();
            },
            (),
        )
        .unwrap();

    (conn_in, rx)
}

pub fn poll_with_query_blocking(
    rytm: &mut Rytm,
    query: impl ObjectQuery,
    conn_out: Arc<Mutex<MidiOutputConnection>>,
    rx: std::sync::mpsc::Receiver<Vec<u8>>,
    interval_in_millis: u64,
    mut callback: impl FnMut(&[u8], &mut Rytm) -> Result<(), RytmError>,
) -> Result<(), RytmError> {
    loop {
        conn_out
            .lock()
            .unwrap()
            .send(&query.serialize_to_sysex().unwrap())
            .unwrap();

        match rx.recv() {
            Ok(message) => {
                callback(&message, rytm)?;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(interval_in_millis));
    }
}

pub fn is_sysex(response: &[u8]) -> bool {
    response[0] == 0xF0 && response[response.len() - 1] == 0xF7
}

pub fn decode_sysex_response_to_raw(response: &[u8]) -> Result<(Vec<u8>, SysexMeta), RytmError> {
    const LARGE_SYSEX_GUESSED_SIZE: usize = 1024 * 16;

    let meta = SysexMeta::default();
    let mut meta: rytm_sys::ar_sysex_meta_t = meta.into();
    let meta_p = &mut meta as *mut rytm_sys::ar_sysex_meta_t;

    let mut src_buf = response.as_ptr();
    let src_buf_p = &mut src_buf as *mut *const u8;
    let mut src_buf_size = response.len() as u32;
    let src_buf_size_p = &mut src_buf_size as *mut u32;

    // Will be calculated by the first call to ar_sysex_to_raw.
    let dst_buf_size = 0; // Big enough for the largest sysex message probably.
    let dest_buf_size_p = dst_buf_size as *mut u32;

    let mut dst_buf = vec![0_u8; LARGE_SYSEX_GUESSED_SIZE];
    let dst_buf_p = dst_buf.as_mut_slice().as_mut_ptr();

    unsafe {
        let return_code = rytm_sys::ar_sysex_to_raw(
            dst_buf_p,
            src_buf_p,
            src_buf_size_p,
            dest_buf_size_p,
            meta_p,
        ) as u8;

        if return_code != 0 {
            return Err(SysexConversionError::from(return_code).into());
        }
    }

    Ok((dst_buf, SysexMeta::from(&meta)))
}
