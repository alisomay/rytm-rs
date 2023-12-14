<!-- <p align="center">
  <img src="https://raw.githubusercontent.com/alisomay/rytm-rs/main/assets/logo_transparent.png"/>
</p> -->

# rytm-rs

More than safe rust abstractions over [rytm-sys](https://github.com/alisomay/rytm-sys), an unofficial SDK for writing software for Analog Rytm running on firmware 1.70.

On top of `CC` and `NRPN` messages, Rytm also accepts sysex messages which are undocumented and not officially supported by Elektron.

The effort of reverse engineering the sysex format started with [libanalogrytm]() which is a `C` library powers parts of `rytm-rs` through `rytm-sys` bindings.

[libanalogrytm](https://github.com/bsp2/libanalogrytm) though a great foundation, is not accessible to many developers due to its low level nature and also lacks high level abstractions for common tasks. The scope of the [libanalogrytm](https://github.com/bsp2/libanalogrytm) is to provide the necessary types for the encoded and decoded sysex messages and focus on the low level details of the sysex protocol.

`rytm-rs` builds on top of [libanalogrytm](https://github.com/bsp2/libanalogrytm) and provides high level abstractions for common tasks and designed to provide an SDK like experience for developers with ease of use in mind abstracting the low level details completely.

It is thoroughly [documented](), and enriched with various [examples](https://github.com/alisomay/rytm-rs/tree/main/examples) to get you started right away.

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

## Usage and examples

soon..

### Running the examples and tests

soon..

## Contributing

soon..

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
