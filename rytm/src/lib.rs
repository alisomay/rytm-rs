#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    clippy::similar_names
)]
// TODO: Re-check later.
#![allow(clippy::must_use_candidate, clippy::unsafe_derive_deserialize)]
// TODO: Convert stack allocating arrays either to vectors ot Box<[T]> and re-enable this lint.
#![allow(clippy::large_stack_frames)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alisomay/rytm-rs/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/alisomay/rytm-rs/main/assets/favicon/favicon.ico"
)]

//! # rytm-rs
//!
//! More than safe rust abstractions over [rytm-sys](https://github.com/alisomay/rytm-sys), an unofficial SDK for writing software for Analog Rytm running on firmware 1.70.
//!
//! On top of `CC` and `NRPN` messages, Rytm also accepts sysex messages which are undocumented and not officially supported by Elektron.
//!
//! The effort of reverse engineering the sysex format started with [libanalogrytm](https://github.com/bsp2/libanalogrytm) which is a `C` library powers parts of `rytm-rs` through `rytm-sys` bindings.
//!
//! [libanalogrytm](https://github.com/bsp2/libanalogrytm) though a great foundation, is not accessible to many developers due to its low level nature and also lacks high level abstractions for common tasks. The scope of the [libanalogrytm](https://github.com/bsp2/libanalogrytm) is to provide the necessary types for the encoded and decoded sysex messages and focus on the low level details of the sysex protocol.
//!
//! `rytm-rs` builds on top of [libanalogrytm](https://github.com/bsp2/libanalogrytm) and provides high level abstractions for common tasks and designed to provide an SDK like experience for developers with ease of use in mind abstracting the low level details completely.
//!
//! ## Features
//!
//! - All structures in a Rytm project is completely represented with [`RytmProject`] including all the necessary fields and methods to receive manipulate and send the project to the device.
//! - All getter and setter methods have range and validity checks including comments about the range and validity of the values.
//! - The Rytm device project defaults are represented in all the struct `Default` implementations.
//! - Sysex encoding and decoding is completely abstracted away. Update the project with a single method call.
//! - Convert parts of the project to sysex with one method call and send it to the device with your choice of transport.
//! - Separate query types provided for [`Pattern`](crate::object::Pattern), [`Kit`](crate::object::Kit), [`Sound`](crate::object::Sound), [`Settings`](crate::object::Settings) and [`Global`](crate::object::Global) types which covers the entire Rytm project parameters except songs.
//! - Different methods provided for setting, getting, clearing parameter locks exhaustively and available in [`Trig`](crate::object::pattern::track::trig::Trig) struct.
//! - All 34 machine types are represented including parameter lock setters getters and clearers.
//! - All getters and setters use the actual range of values on the device not the internal ranges which are used in the sysex protocol.
//!
//! ## Purpose
//!
//! The purpose of this crate is to provide a safe and easy to use SDK like experience for developers who would like to write software for Analog Rytm.
//!
//! The first priority for this crate is to provide an easy to use api for developers who would like to
//!
//! - Develop a software products for Analog Rytm
//! - Develop custom creative software for artistic purposes
//! - Discover and experiment with generative and algorithmic music but don't want to deal with the low level details of the sysex protocol communicating with the device.
//!
//! The crate is not optimized for the best performance or memory. On the other hand the memory footprint is not that big and the performance is good enough since the performance bottleneck is the device itself when it comes to sysex communication.
//!
//! I believe that Rytm uses a low priority thread for sysex communication in their internal RTOS. If you flood Rytm with sysex messages it will queue the responses and get back to you when it can. This is not an issue for most use cases but it is a nice to know.
//!
//! ## Layers
//!
//! `rytm-rs` is composed of 3 main layers.
//!
//! ### [`rytm-sys`](https://docs.rs/rytm-sys/latest/rytm_sys/#)
//!
//! - Encoding/decoding sysex messages
//! - Providing `#[repr(C,packed)]` structs to identically represent the sysex messages in memory keeping the original memory layout of the messages.
//! - Exposing types from [libanalogrytm](https://github.com/bsp2/libanalogrytm) through `rytm-sys` bindings. Which is the main hub for reverse engineering.
//!
//! ### `rytm-rs`
//!
//! Internal layer which deals with communicating with `rytm-sys` and deals with conversion from/to raw types (`#[repr(C,packed)]` structs).
//!
//! User facing layer which provides high level abstractions for common tasks. Getters, setters etc.
//!
//!
//! ## Usage
//!
//! Starting with importing the prelude is a good idea since it brings the necessary traits and types into scope.
//!
//! Also the [`midir`](https://github.com/Boddlnagg/midir) library will be used for midi communication with the device in these examples but you can use any midi library you want.
//!
//! ```
//! use std::sync::Arc; use parking_lot::Mutex;
//! use midir::{Ignore, MidiInputConnection, MidiOutputConnection};
//! use rytm_rs::prelude::*;
//!
//! // We'll be using this connection for sending sysex messages to the device.
//! //
//! // Using an Arc<Mutex<MidiOutputConnection>> is a good idea since you can share the connection between threads.
//! // Which will be common in this context.
//! fn get_connection_to_rytm() -> Arc<Mutex<MidiOutputConnection>> {
//!     let output = port::MidiOut::new("rytm_test_out").unwrap();
//!     let rytm_out_identifier = "Elektron Analog Rytm MKII";
//!     let rytm_output_port = output.find_output_port(rytm_out_identifier).unwrap();
//!
//!     Arc::new(Mutex::new(
//!         output.make_output_connection(&rytm_output_port, 0).unwrap(),
//!     ))
//! }
//!
//! // We'll be using this connection for receiving sysex messages from the device and forwarding them to our main thread.
//! pub fn make_input_message_forwarder() -> (
//!     MidiInputConnection<()>,
//!     std::sync::mpsc::Receiver<(Vec<u8>, u64)>,
//! ) {
//!     let mut input = crate::port::MidiIn::new("rytm_test_in").unwrap();
//!     input.ignore(Ignore::None);
//!     let rytm_in_identifier = "Elektron Analog Rytm MKII";
//!     let rytm_input_port = input.find_input_port(rytm_in_identifier).unwrap();
//!
//!     let (tx, rx) = std::sync::mpsc::channel::<(Vec<u8>, u64)>();
//!
//!     let conn_in: midir::MidiInputConnection<()> = input
//!         .into_inner()
//!         .connect(
//!             &rytm_input_port,
//!             "rytm_test_in",
//!             move |stamp, message, _| {
//!                 // Do some filtering here if you like.
//!                 tx.send((message.to_vec(), stamp)).unwrap();
//!             },
//!             (),
//!         )
//!         .unwrap();
//!
//!     (conn_in, rx)
//! }
//!
//! fn main() {
//!     // Make a default rytm project
//!     let mut rytm = RytmProject::default();
//!
//!     // Get a connection to the device
//!     let conn_out = get_connection_to_rytm();
//!
//!     // Listen for incoming messages from the device
//!     let (_conn_in, rx) = make_input_message_forwarder();
//!
//!     // Make a query for the pattern in the work buffer
//!     let query = PatternQuery::new_targeting_work_buffer();
//!
//!     // Send the query to the device
//!     conn_out
//!         .lock()
//!         .unwrap()
//!         .send(&query.as_sysex().unwrap())
//!         .unwrap();
//!
//!     // Wait for the response
//!     match rx.recv() {
//!         Ok((message, _stamp)) => {
//!             match rytm.update_from_sysex_response(&message) {
//!                 Ok(_) => {
//!                     for track in rytm.work_buffer_mut().pattern_mut().tracks_mut() {
//!                         // Set the number of steps to 64
//!                         track.set_number_of_steps(64).unwrap();
//!                         for (i, trig) in track.trigs_mut().iter_mut().enumerate() {
//!                             // Enable every 4th trig.
//!                             // Set retrig on.
//!                             if i % 4 == 0 {
//!                                 trig.set_trig_enable(true);
//!                                 trig.set_retrig(true);
//!                             }
//!                         }
//!                     }
//!
//!                     // Send the updated pattern to the device if you like
//!                     conn_out
//!                         .lock()
//!                         .unwrap()
//!                         .send(&rytm.work_buffer().pattern().as_sysex().unwrap())
//!                         .unwrap();
//!                 }
//!                 Err(err) => {
//!                     println!("Error: {:?}", err);
//!                 }
//!             }
//!         }
//!         Err(err) => {
//!             println!("Error: {:?}", err);
//!         }
//!     }
//! }
//! ```
//!
//! ## Remarks
//!
//! The people mentioned here are major contributors to the reverse engineering effort and I would like to thank them for their work.
//! This crate would not be possible in this form and time frame without their work.
//!
//! ### bsp2
//!
//! The maintainer of [libanalogrytm](https://github.com/bsp2/libanalogrytm) and the original author of the reverse engineering effort. He is the one who started the reverse engineering effort and provided the initial `C` library which is the foundation of `rytm-rs`.
//!
//! - <https://github.com/bsp2>
//!
//! ### mekohler
//!
//! Author of the [Collider](https://www.elektronauts.com/t/collider-for-the-ipad/27479) app which is available for iPad in the app store.
//! Another contributor to the reverse engineering effort.
//!
//! - <https://marcoskohler.com/>
//! - <https://github.com/mekohler>
//! - <https://www.elektronauts.com/u/mekohler/summary>
//!
//! ### void
//!
//! Author of the [STROM](https://apps.apple.com/us/app/strom/id907044543) app which is available for iPad in the app store.
//! Another contributor to the reverse engineering effort.
//!
//! - <https://www.elektronauts.com/u/void/summary>
//! - <https://soundcloud.com/jakob-penca>
//!
//!
//! Many thanks to [Başak Ünal](https://basakunal.design) for the logo.

pub(crate) mod defaults;
pub mod error;
pub mod object;
pub mod prelude;
pub mod query;
pub(crate) mod sysex;
pub(crate) mod util;

use self::error::RytmError;
use crate::error::ParameterError;
use defaults::*;
use error::SysexConversionError;
use object::{
    global::Global,
    kit::Kit,
    pattern::Pattern,
    settings::Settings,
    sound::{Sound, SoundType},
};
use rytm_sys::{ar_global_t, ar_kit_t, ar_pattern_t, ar_settings_t, ar_sound_t};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use sysex::decode_sysex_response_to_raw;
pub use sysex::{AnySysexType, SysexCompatible, SysexType};

/// [`RytmProject`] represents the state of the analog rytm.
///
/// It contains all structures scoped to an Analog Rytm MKII FW 1.70 project.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RytmProject {
    work_buffer: Box<RytmProjectWorkBuffer>,
    patterns: Vec<Pattern>,
    #[serde(with = "BigArray")]
    pool_sounds: [Sound; POOL_SOUND_MAX_COUNT],
    kits: Vec<Kit>,
    globals: [Global; GLOBAL_MAX_COUNT],
    // TODO: Songs (16)
    settings: Settings,

    pub(crate) last_queried_pattern_index: Option<usize>,
    pub(crate) last_queried_kit_index: Option<usize>,
    pub(crate) last_queried_work_buffer_pattern_index: Option<usize>,
    pub(crate) last_queried_work_buffer_kit_index: Option<usize>,
}

impl RytmProject {
    // TODO:
    #[allow(clippy::missing_errors_doc)]
    pub fn try_default() -> Result<Self, RytmError> {
        let mut patterns = Vec::with_capacity(PATTERN_MAX_COUNT);
        let mut kits = Vec::with_capacity(KIT_MAX_COUNT);

        // PATTERN_MAX_COUNT == KIT_MAX_COUNT is true.
        for i in 0..PATTERN_MAX_COUNT {
            let pattern = Pattern::try_default(i)?;
            let mut kit = Kit::try_default(i)?;
            kit.link_parameter_lock_pool(&pattern.parameter_lock_pool)?;
            patterns.push(Pattern::try_default(i)?);
            kits.push(Kit::try_default(i)?);
        }

        // TODO: ALSO FOR THE TRACK AND TRIG TYPES!

        Ok(Self {
            work_buffer: Box::new(RytmProjectWorkBuffer::try_default()?),
            patterns,
            pool_sounds: default_pool_sounds(),
            kits,
            globals: default_globals(),
            settings: Settings::default(),

            last_queried_pattern_index: None,
            last_queried_kit_index: None,
            last_queried_work_buffer_pattern_index: None,
            last_queried_work_buffer_kit_index: None,
        })
    }

    #[allow(clippy::too_many_lines)]
    /// Updates the Rytm struct from a sysex response.
    ///
    /// All encoding/decoding is done in [`RytmProject`], so this is the only method that needs to be called to update the struct when a sysex response is received.
    ///
    /// # Important
    ///
    /// This method will act as a no-op if the given slice does not contain a valid sysex message.
    /// The check is performed by checking the first and last byte of the slice.
    /// This behaviour is preferred to returning an error, as it is expected to be called in a midi callback which receives not just sysex messages.
    ///
    /// # Errors
    ///
    /// This method will return an error
    ///
    /// - If the sysex message is invalid in the context of Rytm
    /// - If the sysex message is valid, but the object type is not supported or implemented yet. Example: [`crate::error::RytmError::SysexConversionError::Unimplemented`] variant.
    /// - If the sysex message is incomplete, this sometimes happens in the initial parts of the transmission and is a behaviour of Rytm. You may check for the error [`crate::error::RytmError::SysexConversionError::ShortRead`] and ignore it.
    /// - If the sysex message is valid, but the size of the expected object does not match the size of the received object. This may happen if the firmware version of Rytm is different than the one this library supports which is currently FW 1.70 only. Never happened to me in practice but a cut transmission may also cause this in theory.
    /// - All other  [`crate::error::RytmError::SysexConversionError`] variants are possible which are inherited from [libanalogrytm](https://github.com/bsp2/libanalogrytm).
    pub fn update_from_sysex_response(&mut self, response: &[u8]) -> Result<(), RytmError> {
        if response.len() < 2 {
            return Err(SysexConversionError::ShortRead.into());
        }

        // Ignore non-sysex messages.
        if !(response[0] == 0xF0 && response[response.len() - 1] == 0xF7) {
            return Ok(());
        }

        // Raw buffer and metadata about the sysex message.
        let (mut raw, meta) = decode_sysex_response_to_raw(response)?;

        // `& 0b0111_1111` is to get rid of the bit that indicates if the object is in the work buffer or not.
        // `- 0x80` is to get the actual index of the object in the work buffer. Example, there could be 12 sounds in the work buffer.
        match meta.object_type()? {
            SysexType::Pattern => {
                let raw_pattern: &ar_pattern_t =
                    unsafe { &*(raw.as_mut_ptr() as *const ar_pattern_t) };
                let mut pattern = Pattern::try_from_raw(meta, raw_pattern)?;

                // Once a pattern is retrieved it's parameter lock pool will be linked to the kit we assume that it uses.
                // This doesn't mean the kit itself is updated, it can be as well out of sync from the device but it's the best we can do.
                // It is the responsibility of the user to update the kit if it is out of sync.

                let kit_number = pattern.kit_number();

                if kit_number == 0xFF {
                    // When the kit is not set, we assume that the pattern uses the kit in the work buffer.
                    let work_buffer_pattern_kit_number = self.work_buffer().pattern().kit_number();

                    // We update the kit which has than number with the parameter lock pool of the pattern.
                    self.kits_mut()[work_buffer_pattern_kit_number]
                        .link_parameter_lock_pool(&pattern.parameter_lock_pool)?;

                    // We also update the work buffer kit if it is the same kit.
                    let work_buffer_kit_mut = self.work_buffer_mut().kit_mut();
                    if work_buffer_kit_mut.index() == work_buffer_pattern_kit_number {
                        work_buffer_kit_mut
                            .link_parameter_lock_pool(&pattern.parameter_lock_pool)?;
                    }
                } else {
                    // When the kit is set then we directly update that kit.
                    self.kits_mut()[kit_number]
                        .link_parameter_lock_pool(&pattern.parameter_lock_pool)?;

                    // We also update the work buffer kit if it is the same kit.
                    let work_buffer_kit_mut = self.work_buffer_mut().kit_mut();
                    if work_buffer_kit_mut.index() == kit_number {
                        work_buffer_kit_mut
                            .link_parameter_lock_pool(&pattern.parameter_lock_pool)?;
                    }
                }

                if meta.is_targeting_work_buffer() {
                    let index = meta.get_normalized_object_index();
                    pattern.index = index;
                    self.work_buffer.pattern = pattern;

                    return Ok(());
                }

                let index = meta.get_normalized_object_index();
                self.patterns[index] = pattern;
                Ok(())
            }

            SysexType::Kit => {
                let raw_kit: &ar_kit_t = unsafe { &*(raw.as_mut_ptr() as *const ar_kit_t) };
                let mut kit = Kit::try_from_raw(meta, raw_kit)?;

                // When a kit is received, we check all the existing patterns to see if any of them is linked to the kit index queried or not.
                // Then we link the plock pool of those patterns to the updated kit.

                for pattern in self.patterns() {
                    if pattern.kit_number() == kit.index() {
                        kit.link_parameter_lock_pool(&pattern.parameter_lock_pool)?;
                    }
                }

                if self.work_buffer().pattern().kit_number() == kit.index() {
                    kit.link_parameter_lock_pool(
                        &self.work_buffer().pattern().parameter_lock_pool,
                    )?;
                }

                if meta.is_targeting_work_buffer() {
                    let index = meta.get_normalized_object_index();
                    kit.index = index;
                    self.work_buffer.kit = kit;
                    return Ok(());
                }

                let index = meta.get_normalized_object_index();
                self.kits[index] = kit;
                Ok(())
            }

            SysexType::Sound => {
                let raw_sound: &ar_sound_t = unsafe { &*(raw.as_mut_ptr() as *const ar_sound_t) };
                let mut sound = Sound::try_from_raw(meta, raw_sound, None)?;

                let index = meta.get_normalized_object_index();
                match sound.sound_type() {
                    SoundType::Pool => {
                        // Pool sounds will not have a linked parameter lock pool.
                        // TODO: Would we need linking here?
                        self.pool_sounds[index] = sound;
                    }
                    SoundType::WorkBuffer => {
                        // Work buffer sounds though will be linked to the work buffer pattern's parameter lock pool.
                        sound.link_parameter_lock_pool(
                            &self.work_buffer().pattern().parameter_lock_pool,
                        )?;
                        self.work_buffer.sounds[index] = sound;
                    }
                    SoundType::KitQuery => {
                        unreachable!("A response from a sound query can not contain a sound which is part of a kit.")
                    }
                };

                Ok(())
            }

            SysexType::Global => {
                let raw_global: &ar_global_t =
                    unsafe { &*(raw.as_mut_ptr() as *const ar_global_t) };
                let global = Global::try_from_raw(meta, raw_global)?;

                if meta.is_targeting_work_buffer() {
                    self.work_buffer.global = global;
                    return Ok(());
                }

                let index = (meta.obj_nr & 0b0111_1111) as usize;
                self.globals[index] = global;
                Ok(())
            }

            // There is only a single settings object for a project.
            SysexType::Settings => {
                let raw_settings: &ar_settings_t =
                    unsafe { &*(raw.as_mut_ptr() as *const ar_settings_t) };
                let settings = Settings::try_from_raw(meta, raw_settings)?;
                self.settings = settings;
                Ok(())
            }

            // TODO: Implement Song
            SysexType::Song => Err(SysexConversionError::Unimplemented("Song".to_owned()).into()),
        }
    }

    /// Get all patterns.
    ///
    /// Total of 128 patterns.
    pub fn patterns(&self) -> &[Pattern] {
        &self.patterns
    }

    /// Get all kits.
    ///
    /// Total of 128 kits.
    pub fn kits(&self) -> &[Kit] {
        &self.kits
    }

    /// Get all sounds in the pool.
    ///
    /// Total of 128 sounds.
    pub const fn pool_sounds(&self) -> &[Sound] {
        &self.pool_sounds
    }

    /// Get all global slots.
    ///
    /// Total of 4 global slots.
    pub const fn globals(&self) -> &[Global] {
        &self.globals
    }

    /// Get the settings.
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Get all patterns mutably.
    ///
    /// Total of 128 patterns.
    pub fn patterns_mut(&mut self) -> &mut [Pattern] {
        &mut self.patterns
    }

    /// Get all kits mutably.
    ///
    /// Total of 128 kits.
    pub fn kits_mut(&mut self) -> &mut [Kit] {
        &mut self.kits
    }

    /// Get all sounds in the pool mutably.
    ///
    /// Total of 128 sounds.
    pub fn pool_sounds_mut(&mut self) -> &mut [Sound] {
        &mut self.pool_sounds
    }

    /// Get all global slots mutably.
    ///
    /// Total of 4 global slots.
    pub fn globals_mut(&mut self) -> &mut [Global] {
        &mut self.globals
    }

    /// Get the settings mutably.
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Get the work buffer structures.
    pub const fn work_buffer(&self) -> &RytmProjectWorkBuffer {
        &self.work_buffer
    }

    /// Get the work buffer structures mutably.
    pub fn work_buffer_mut(&mut self) -> &mut RytmProjectWorkBuffer {
        &mut self.work_buffer
    }
}

/// [`RytmProjectWorkBuffer`] represents the state of the analog rytm work buffer.
///
/// It contains all structures scoped to an Analog Rytm MKII FW 1.70 project work buffer.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RytmProjectWorkBuffer {
    pattern: Pattern,
    kit: Kit,
    sounds: [Sound; TRACK_MAX_COUNT],
    global: Global,
    // TODO: Work buffer song
}

impl RytmProjectWorkBuffer {
    // TODO:
    #[allow(clippy::missing_errors_doc)]
    pub fn try_default() -> Result<Self, RytmError> {
        let pattern = Pattern::work_buffer_default();
        let mut kit = Kit::work_buffer_default();
        kit.link_parameter_lock_pool(&pattern.parameter_lock_pool)?;

        // TODO: What are work buffer sounds.. hmm..
        // Should we link their parameter lock pool also?

        Ok(Self {
            pattern,
            kit,
            sounds: default_work_buffer_sounds(),
            global: Global::work_buffer_default(),
        })
    }

    /// Get the pattern in the work buffer.
    pub const fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// Get the kit in the work buffer.
    pub const fn kit(&self) -> &Kit {
        &self.kit
    }

    /// Get the sounds in the work buffer.
    ///
    /// Total of 12 sounds for 12 tracks.
    pub const fn sounds(&self) -> &[Sound] {
        &self.sounds
    }

    /// Get the global in the work buffer.
    pub const fn global(&self) -> &Global {
        &self.global
    }

    /// Get the pattern in the work buffer mutably.
    pub fn pattern_mut(&mut self) -> &mut Pattern {
        &mut self.pattern
    }

    /// Get the kit in the work buffer mutably.
    pub fn kit_mut(&mut self) -> &mut Kit {
        &mut self.kit
    }

    /// Get the sounds in the work buffer mutably.
    ///
    /// Total of 12 sounds for 12 tracks.
    pub fn sounds_mut(&mut self) -> &mut [Sound] {
        &mut self.sounds
    }

    /// Get the global in the work buffer mutably.
    pub fn global_mut(&mut self) -> &mut Global {
        &mut self.global
    }
}
