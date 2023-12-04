pub(in crate::pattern) mod trig;
pub(in crate::pattern) mod types;

use self::trig::{HoldsTrigFlags, TrigFlags};
use self::types::{PadScale, RootNote};
use super::Length;
use crate::util::to_s_u16_t_union_b;
use crate::{
    error::{ParameterError, RytmError},
    pattern::types::Speed,
    util::from_s_u16_t,
};
use bitstream_io::{BigEndian, BitRead, BitReader, BitWrite, BitWriter};
use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_pattern_track_t;
use std::io::Cursor;
use trig::Trig;

// TODO: Check for these.
//    sU8     notes[64];                   /* @0x0074..0x00B3   0xFF=unset, MIDI note otherwise (lower 7 bits)
//                                          *                    (default is C-4 == 0x3C, 0x3B="-1", 0x3D="+1")
//                                          *                   bit7: 1=no trig condition, 0=have trig condition
//                                          *                          ==> 0x7F = default note with trig cond
//                                          *                          ==> 0xFF = default note without trig cond
//                                          */
//    sU8     retrig_rates[64];            /* ?@0x01B4..0x01F3   Retrig rates (0(=1/1)..16(=1/80))
//                                          *                    Changing the trig condition of step 1 updates 0x1c4
//                                          */
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Track {
    trigs: [Trig; 64],

    default_trig_flags: TrigFlags,
    default_trig_note: u8,
    default_trig_velocity: u8,
    default_trig_note_length: Length,
    default_trig_probability: u8,

    number_of_steps: u8,
    quantize_amount: u8,
    sends_midi: bool,
    speed: Speed,

    euclidean_mode: bool,
    euclidean_pl1: u8,
    euclidean_pl2: u8,
    euclidean_ro1: u8,
    euclidean_ro2: u8,
    euclidean_tro: u8,

    pad_scale: PadScale,
    root_note: RootNote,

    /// MSB of default_trig_note.
    ///
    /// For now it is always 0.
    ///
    /// Maybe it means something?
    #[derivative(Debug = "ignore")]
    pub(crate) __maybe_useful_flag_from_default_trig_note: u8,

    /// Mid bits of flags_and_speed.
    ///
    /// For now they're always 0.
    ///
    /// Maybe they means something?
    #[derivative(Debug = "ignore")]
    pub(crate) __maybe_useful_flags_from_flags_and_speed: u8,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            trigs: [Trig::default(); 64],

            default_trig_flags: TrigFlags::default(),
            default_trig_note: 0,
            default_trig_velocity: 0,
            default_trig_note_length: Length::default(),
            default_trig_probability: 0,

            number_of_steps: 0,
            quantize_amount: 0,
            sends_midi: false,
            speed: Speed::default(),

            euclidean_mode: false,
            euclidean_pl1: 0,
            euclidean_pl2: 0,
            euclidean_ro1: 0,
            euclidean_ro2: 0,
            euclidean_tro: 0,

            pad_scale: PadScale::default(),
            root_note: RootNote::default(),

            __maybe_useful_flag_from_default_trig_note: 1,
            __maybe_useful_flags_from_flags_and_speed: 0,
        }
    }
}

impl TryFrom<&ar_pattern_track_t> for Track {
    type Error = RytmError;

    fn try_from(raw_track: &ar_pattern_track_t) -> Result<Self, RytmError> {
        let mut trigs: [Trig; 64] = [Trig::default(); 64];

        let trig_cursor = Cursor::new(raw_track.trig_bits);
        let mut bit_reader = BitReader::endian(trig_cursor, BigEndian);

        for (i, trig) in trigs.iter_mut().enumerate() {
            let raw_trig_flags = bit_reader.read::<u16>(14).unwrap();
            *trig = Trig::new(
                i,
                raw_trig_flags,
                raw_track.notes[i],
                raw_track.note_lengths[i],
                raw_track.velocities[i],
                raw_track.micro_timings[i],
                raw_track.retrig_rates[i],
                raw_track.retrig_lengths[i],
                raw_track.retrig_velocity_offsets[i],
                raw_track.sound_locks[i],
            )?;
        }

        // Sends midi
        let sends_midi = raw_track.flags_and_speed & 0b1000_0000 != 0;
        // Speed
        let speed: Speed = (raw_track.flags_and_speed & 0b0000_0111).try_into()?;

        // TODO: They always seem to be 0.
        let __maybe_useful_flags_from_flags_and_speed = raw_track.flags_and_speed & 0b0111_1000;

        let __maybe_useful_flag_from_default_trig_note = raw_track.default_note & 0b1000_0000;
        let default_trig_note = raw_track.default_note & 0b0111_1111;

        Ok(Self {
            trigs,

            default_trig_note,
            default_trig_velocity: raw_track.default_velocity,
            default_trig_note_length: raw_track.default_note_length.try_into()?,
            default_trig_flags: unsafe { from_s_u16_t(&raw_track.default_trig_flags).into() },
            default_trig_probability: raw_track.trig_probability,

            number_of_steps: raw_track.num_steps,
            quantize_amount: raw_track.quantize_amount,
            sends_midi,
            speed,

            euclidean_mode: raw_track.euc_mode == 128,
            euclidean_pl1: raw_track.euc_pl1,
            euclidean_pl2: raw_track.euc_pl2,
            euclidean_ro1: raw_track.euc_ro1,
            euclidean_ro2: raw_track.euc_ro2,
            euclidean_tro: raw_track.euc_tro,

            pad_scale: raw_track.pad_scale.try_into()?,
            root_note: raw_track.root_note.try_into()?,

            __maybe_useful_flag_from_default_trig_note,
            __maybe_useful_flags_from_flags_and_speed,
        })
    }
}

impl From<&Track> for ar_pattern_track_t {
    fn from(track: &Track) -> Self {
        let mut notes: [u8; 64] = [0; 64];
        let mut velocities: [u8; 64] = [0; 64];
        let mut note_lengths: [u8; 64] = [0; 64];
        let mut micro_timings: [i8; 64] = [0; 64];
        let mut retrig_lengths: [u8; 64] = [0; 64];
        let mut retrig_rates: [u8; 64] = [0; 64];
        let mut retrig_velocity_offsets: [i8; 64] = [0; 64];
        let mut sound_locks: [u8; 64] = [0; 64];

        let mut encoded_trig_bits: [u8; 112] = [0; 112];
        let mut bit_writer =
            BitWriter::endian(Cursor::new(encoded_trig_bits.as_mut_slice()), BigEndian);

        for (i, trig) in track.trigs.iter().enumerate() {
            // Encode trig flags to packed 14 bits.
            bit_writer
                .write::<u16>(14, trig.raw_trig_flags())
                .expect("This shouldn't fail.");

            notes[i] = trig.encode_note();
            velocities[i] = trig.velocity() as u8;
            note_lengths[i] = trig.note_length().into();
            micro_timings[i] = trig.encode_micro_timing();
            retrig_lengths[i] = trig.encode_retrig_length();
            retrig_rates[i] = trig.encode_retrig_rate();
            retrig_velocity_offsets[i] = trig.retrig_velocity_offset() as i8;
            sound_locks[i] = trig.sound_lock() as u8;
        }

        // Encode flags and speed.
        let mut encoded_flags_and_speed: u8 = 0;
        encoded_flags_and_speed |= track.speed as u8;
        encoded_flags_and_speed |= if track.sends_midi { 0b1000_0000 } else { 0 };
        encoded_flags_and_speed |= track.__maybe_useful_flags_from_flags_and_speed;

        // Encoded euclidean mode.
        let encoded_euc_mode = if track.euclidean_mode { 128 } else { 0 };

        // Compile note and unknown flag.
        let encoded_default_trig_note =
            track.default_trig_note | track.__maybe_useful_flag_from_default_trig_note;

        Self {
            trig_bits: encoded_trig_bits,
            notes,
            velocities,
            note_lengths,
            micro_timings,
            retrig_lengths,
            retrig_rates,
            retrig_velocity_offsets,
            default_note: encoded_default_trig_note,
            default_velocity: track.default_trig_velocity,
            default_note_length: track.default_trig_note_length.into(),
            default_trig_flags: to_s_u16_t_union_b(*track.default_trig_flags),
            num_steps: track.number_of_steps,
            quantize_amount: track.quantize_amount,
            sound_locks,
            flags_and_speed: encoded_flags_and_speed,
            trig_probability: track.default_trig_probability,
            euc_mode: encoded_euc_mode,
            euc_pl1: track.euclidean_pl1,
            euc_pl2: track.euclidean_pl2,
            euc_ro1: track.euclidean_ro1,
            euc_ro2: track.euclidean_ro2,
            euc_tro: track.euclidean_tro,
            pad_scale: track.pad_scale.into(),
            root_note: track.root_note.into(),
        }
    }
}

impl Track {
    /// Returns a mutable reference to the trigs in this track.
    ///
    /// 64 trigs in total.
    pub fn trigs_mut(&mut self) -> &mut [Trig; 64] {
        &mut self.trigs
    }

    /// Sets the default note for any trig in this track.
    ///
    /// Range `0..=127`
    ///
    /// Follows the midi note convention. C-4 is `0x3C`.
    #[parameter_range(range = "default_trig_note:0..=127")]
    pub fn set_default_trig_note(&mut self, default_trig_note: usize) -> Result<(), RytmError> {
        self.default_trig_note = default_trig_note as u8;
        Ok(())
    }

    /// Sets the default velocity for any trig in this track.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "default_trig_velocity:0..=127")]
    pub fn set_default_trig_velocity(
        &mut self,
        default_trig_velocity: usize,
    ) -> Result<(), RytmError> {
        self.default_trig_velocity = default_trig_velocity as u8;
        Ok(())
    }

    /// Sets the default note length for any trig in this track.
    ///
    /// Range `0..=127`
    pub fn set_default_trig_note_length(&mut self, default_trig_note_length: Length) {
        self.default_trig_note_length = default_trig_note_length;
    }

    // TODO: Double check if this api is good.
    /// Sets the default trig flags for any trig in this track.
    pub fn set_default_trig_flags<F: Into<TrigFlags>>(&mut self, default_trig_flags: F) {
        self.default_trig_flags = default_trig_flags.into();
    }

    /// Sets the default trig probability for any trig in this track.
    ///
    /// Range `0..=100`
    #[parameter_range(range = "default_trig_probability:0..=100")]
    pub fn set_default_trig_probability(
        &mut self,
        default_trig_probability: usize,
    ) -> Result<(), RytmError> {
        self.default_trig_probability = default_trig_probability as u8;
        Ok(())
    }

    /// Sets the number of steps in this track.
    ///
    /// Range `1..=64`
    #[parameter_range(range = "number_of_steps:1..=64")]
    pub fn set_number_of_steps(&mut self, number_of_steps: usize) -> Result<(), RytmError> {
        self.number_of_steps = number_of_steps as u8;
        Ok(())
    }

    /// Sets the quantize amount for this track.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "quantize_amount:0..=127")]
    pub fn set_quantize_amount(&mut self, quantize_amount: usize) -> Result<(), RytmError> {
        self.quantize_amount = quantize_amount as u8;
        Ok(())
    }

    /// Sets whether this track sends MIDI.
    pub fn set_sends_midi(&mut self, sends_midi: bool) {
        self.sends_midi = sends_midi;
    }

    /// Sets the speed for this track.
    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    /// Sets whether this track is in euclidean mode.
    pub fn set_euclidean_mode(&mut self, euclidean_mode: bool) {
        self.euclidean_mode = euclidean_mode;
    }

    /// Sets the euclidean pulse length 1 for this track.
    ///
    /// Number of pulses.
    ///
    /// Range `0..=64`
    #[parameter_range(range = "euclidean_pl1:0..=64")]
    pub fn set_euclidean_pl1(&mut self, euclidean_pl1: usize) -> Result<(), RytmError> {
        self.euclidean_pl1 = euclidean_pl1 as u8;
        Ok(())
    }

    /// Sets the euclidean pulse length 2 for this track.
    ///
    /// Number of pulses.
    ///
    /// Range `0..=64`
    #[parameter_range(range = "euclidean_pl2:0..=64")]
    pub fn set_euclidean_pl2(&mut self, euclidean_pl2: usize) -> Result<(), RytmError> {
        self.euclidean_pl2 = euclidean_pl2 as u8;
        Ok(())
    }

    /// Sets the euclidean rotation 1 for this track.
    ///
    /// Range `0..=126`
    ///
    /// Middle point `63`
    #[parameter_range(range = "euclidean_ro1:0..=126")]
    pub fn set_euclidean_ro1(&mut self, euclidean_ro1: usize) -> Result<(), RytmError> {
        self.euclidean_ro1 = euclidean_ro1 as u8;
        Ok(())
    }

    /// Sets the euclidean rotation 2 for this track.
    ///
    /// Range `0..=126`
    ///
    /// Middle point `63`
    #[parameter_range(range = "euclidean_ro2:0..=126")]
    pub fn set_euclidean_ro2(&mut self, euclidean_ro2: usize) -> Result<(), RytmError> {
        self.euclidean_ro2 = euclidean_ro2 as u8;
        Ok(())
    }

    /// Sets the euclidean track rotation for this track.
    ///
    /// Range `0..=126`
    ///
    /// Middle point `63`
    #[parameter_range(range = "euclidean_tro:0..=126")]
    pub fn set_euclidean_tro(&mut self, euclidean_tro: usize) -> Result<(), RytmError> {
        self.euclidean_tro = euclidean_tro as u8;
        Ok(())
    }

    /// Sets the pad scale for this track.
    pub fn set_pad_scale(&mut self, pad_scale: PadScale) {
        self.pad_scale = pad_scale;
    }

    /// Sets the root note for this track.
    pub fn set_root_note(&mut self, root_note: RootNote) {
        self.root_note = root_note;
    }

    /// Returns a reference to the trigs in this track.
    ///
    /// 64 trigs in total.
    pub fn trigs(&self) -> &[Trig; 64] {
        &self.trigs
    }

    /// Returns the default note for any trig in this track.
    ///
    /// Range `0..=127`
    ///
    /// Follows the midi note convention. C-4 is `0x3C`.
    pub fn default_trig_note(&self) -> usize {
        self.default_trig_note as usize
    }

    /// Returns the default velocity for any trig in this track.
    ///
    /// Range `0..=127`
    pub fn default_trig_velocity(&self) -> usize {
        self.default_trig_velocity as usize
    }

    /// Returns the default note length for any trig in this track.
    pub fn default_trig_note_length(&self) -> Length {
        self.default_trig_note_length
    }

    /// Returns the default trig flags for any trig in this track.
    pub fn default_trig_flags(&self) -> TrigFlags {
        self.default_trig_flags
    }

    /// Returns the default trig probability for any trig in this track.
    ///
    /// Range `0..=100`
    pub fn default_trig_probability(&self) -> usize {
        self.default_trig_probability as usize
    }

    /// Returns the number of steps in this track.
    ///
    /// Range `1..=64`
    pub fn number_of_steps(&self) -> usize {
        self.number_of_steps as usize
    }

    /// Returns the quantize amount for this track.
    ///
    /// Range `0..=127`
    pub fn quantize_amount(&self) -> usize {
        self.quantize_amount as usize
    }

    /// Returns whether this track sends MIDI.
    pub fn sends_midi(&self) -> bool {
        self.sends_midi
    }

    /// Returns the speed for this track.
    pub fn speed(&self) -> Speed {
        self.speed
    }

    /// Returns whether this track is in euclidean mode.
    pub fn euclidean_mode(&self) -> bool {
        self.euclidean_mode
    }

    /// Returns the euclidean pulse length 1 for this track.
    ///
    /// Number of pulses.
    ///
    /// Range `0..=64`
    pub fn euclidean_pl1(&self) -> usize {
        self.euclidean_pl1 as usize
    }

    /// Returns the euclidean pulse length 2 for this track.
    ///
    /// Number of pulses.
    ///
    /// Range `0..=64`
    pub fn euclidean_pl2(&self) -> usize {
        self.euclidean_pl2 as usize
    }

    /// Returns the euclidean rotation 1 for this track.
    ///
    /// Range `0..=126`
    ///
    /// Middle point `63`
    pub fn euclidean_ro1(&self) -> usize {
        self.euclidean_ro1 as usize
    }

    /// Returns the euclidean rotation 2 for this track.
    ///
    /// Range `0..=126`
    ///
    /// Middle point `63`
    pub fn euclidean_ro2(&self) -> usize {
        self.euclidean_ro2 as usize
    }

    /// Returns the euclidean track rotation for this track.
    ///
    /// Range `0..=126`
    ///
    /// Middle point `63`
    pub fn euclidean_tro(&self) -> usize {
        self.euclidean_tro as usize
    }

    /// Returns the pad scale for this track.
    pub fn pad_scale(&self) -> PadScale {
        self.pad_scale
    }

    /// Returns the root note for this track.
    pub fn root_note(&self) -> RootNote {
        self.root_note
    }
}
