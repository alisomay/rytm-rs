<p align="center">
  <img src="https://raw.githubusercontent.com/alisomay/rytm-rs/main/assets/logo.png"/>
</p>

# rytm-rs

More than safe rust abstractions over [rytm-sys](https://github.com/alisomay/rytm-sys), an unofficial SDK for writing software for Analog Rytm running on firmware 1.70.

On top of `CC` and `NRPN` messages, Rytm also accepts sysex messages which are undocumented and not officially supported by Elektron.

The effort of reverse engineering the sysex format started with [libanalogrytm](https://github.com/bsp2/libanalogrytm) which is a `C` library powers parts of `rytm-rs` through `rytm-sys` bindings.

[libanalogrytm](https://github.com/bsp2/libanalogrytm) though a great foundation, is not accessible to many developers due to its low level nature and also lacks high level abstractions for common tasks. The scope of the [libanalogrytm](https://github.com/bsp2/libanalogrytm) is to provide the necessary types for the encoded and decoded sysex messages and focus on the low level details of the sysex protocol.

`rytm-rs` builds on top of [libanalogrytm](https://github.com/bsp2/libanalogrytm) and provides high level abstractions for common tasks and designed to provide an SDK like experience for developers with ease of use in mind abstracting the low level details completely.

It is thoroughly [documented](https://docs.rs/rytm-rs/latest/rytm_rs/#), to get you started right away.

## Features

- All structures in a Rytm project is completely represented with a nested struct called `RytmProject` with all the necessary fields and methods to receive manipulate and send the project to the device.
- All getter and setter methods have range and validity checks including comments about the range and validity of the values.
- The Rytm device project defaults are represented in all the struct `Default` implementations.
- Sysex encoding and decoding is completely abstracted away. Update the project with a single method call.
- Convert parts of the project to sysex with one method call and send it to the device with your choice of transport.
- Separate query types provided for `Pattern`, `Kit`, `Sound`, `Settings` and `Global` types which covers the entire Rytm project parameters except songs.
- Different methods provided for setting, getting, clearing parameter locks exhaustively and available in `Trig` struct.
- All 34 machine types are represented including parameter lock setters getters and clearers.
- All getters and setters use the actual range of values on the device not the internal ranges which are used in the sysex protocol.

## Purpose

The purpose of this crate is to provide a safe and easy to use SDK like experience for developers who would like to write software for Analog Rytm.

The first priority for this crate is to provide an easy to use api for developers who would like to

- Develop a software products for Analog Rytm
- Develop custom creative software for artistic purposes
- Discover and experiment with generative and algorithmic music but don't want to deal with the low level details of the sysex protocol communicating with the device.

The crate is not optimized for the best performance or memory. On the other hand the memory footprint is not that big and the performance is good enough since the performance bottleneck is the device itself when it comes to sysex communication.

I believe that Rytm uses a low priority thread for sysex communication in their internal RTOS. If you flood Rytm with sysex messages it will queue the responses and get back to you when it can. This is not an issue for most use cases but it is a nice to know.

## Layers

`rytm-rs` is composed of 3 main layers.

### `rytm-sys`

- Encoding/decoding sysex messages
- Providing `#[repr(C,packed)]` structs to identically represent the sysex messages in memory keeping the original memory layout of the messages.
- Exposing types from [libanalogrytm](https://github.com/bsp2/libanalogrytm) through `rytm-sys` bindings. Which is the main hub for reverse engineering.

### `rytm-rs`

Internal layer which deals with communicating with `rytm-sys` and deals with conversion from/to raw types (`#[repr(C,packed)]` structs).

User facing layer which provides high level abstractions for common tasks. Getters, setters etc.

## Usage

Starting with importing the prelude is a good idea since it brings the necessary traits and types into scope.

Also the [`midir`](https://github.com/Boddlnagg/midir) library will be used for midi communication with the device in these examples but you can use any midi library you want.

```rust
use std::sync::{Arc, Mutex};
use midir::{Ignore, MidiInputConnection, MidiOutputConnection};
use rytm_rs::prelude::*;

// We'll be using this connection for sending sysex messages to the device.
//
// Using an Arc<Mutex<MidiOutputConnection>> is a good idea since you can share the connection between threads.
// Which will be common in this context.
fn get_connection_to_rytm() -> Arc<Mutex<MidiOutputConnection>> {
    let output = port::MidiOut::new("rytm_test_out").unwrap();
    let rytm_out_identifier = "Elektron Analog Rytm MKII";
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
    let rytm_in_identifier = "Elektron Analog Rytm MKII";
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

fn main() {
    // Make a default rytm project
    let mut rytm = RytmProject::default();

    // Get a connection to the device
    let conn_out = get_connection_to_rytm();

    // Listen for incoming messages from the device
    let (_conn_in, rx) = make_input_message_forwarder();

    // Make a query for the pattern in the work buffer
    let query = PatternQuery::new_targeting_work_buffer();

    // Send the query to the device
    conn_out
        .lock()
        .unwrap()
        .send(&query.as_sysex().unwrap())
        .unwrap();

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
                        .unwrap()
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
}
```

### Tests

Tests are currently a mess. They're not meant to be run but used as a playground for reverse engineering and testing the library manually.

I'll write some automated integration tests in the future which requires a connection to the device. Which again should be run manually but could test the library in a more automated way.

## Contributing

Contributions are welcome!

I did this as a single individual and phew.. it was a lot of labour of love. Also weeks of tedious reverse engineering and manual testing work has gone into it. So I would be happy to see some contributions.

Also since I'm alone even if I tested the library thoroughly many times there might be some bugs. So if you find any please open an issue. I'd be grateful.

There are also some ideas which may be nice for the community in the future.

- People are quite excited about a Max/MSP external for Rytm. One can build that external on top of this crate. Check [median](https://github.com/Cycling74/median/blob/develop/median/README.md).
- [Neon](https://github.com/neon-bindings/neon) bindings might be very useful so people can use Node for Max to build Max patches or Live devices on top of this crate easily.
- Expanding the crate to support easy interfacing with `CC` and `NRPN` messages would be an idea.

For all communication and contributions for this repo the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) applies.

## License

This crate is licensed under the MIT license. You can basically do whatever you want with it but I'd be glad if you reach me out if you make good profit from it or use it for major commercial projects.

## Remarks

The people mentioned here are major contributors to the reverse engineering effort and I would like to thank them for their work.
This crate would not be possible in this form and time frame without their work.

### bsp2

The maintainer of [libanalogrytm](https://github.com/bsp2/libanalogrytm) and the original author of the reverse engineering effort. He is the one who started the reverse engineering effort and provided the initial `C` library which is the foundation of `rytm-rs`.

- <https://github.com/bsp2>

### mekohler

Author of the [Collider](https://www.elektronauts.com/t/collider-for-the-ipad/27479) app which is available for iPad in the app store.
Another contributor to the reverse engineering effort.

- <https://marcoskohler.com/>
- <https://github.com/mekohler>
- <https://www.elektronauts.com/u/mekohler/summary>

### void

Author of the [STROM](https://apps.apple.com/us/app/strom/id907044543) app which is available for iPad in the app store.
Another contributor to the reverse engineering effort.

- <https://www.elektronauts.com/u/void/summary>
- <https://soundcloud.com/jakob-penca>
