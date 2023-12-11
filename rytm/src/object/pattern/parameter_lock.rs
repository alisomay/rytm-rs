use crate::{
    error::RytmError,
    object::sound::{
        machine::MachineParameters,
        page::{Amplitude, Filter, Lfo, Sample},
        types::{FilterType, LfoDestination, LfoMode, LfoMultiplier, LfoWaveform},
    },
    util::stable_partition,
    RytmError::ParameterLockMemoryFull,
};
use derivative::Derivative;
// pub(crate) mod parameter_lock_private {
//     trait ParameterLockCrate {
//         // TODO: Impl
//         fn get_parameter_lock_type(&self) -> u8;
//     }
// }

// // pub struct PlockSeq {
// //     pub plock_type: u8,
// //     pub track_nr: u8,
// //     pub data: [u8; 64],
// // }

// pub trait ParameterLock: parameter_lock_private::ParameterLockCrate {
//     fn get_track_number(&self) -> u8;
//     fn get_trig_number(&self) -> u8;
//     fn set_track_number(&mut self, track_number: u8);
//     fn set_value_for_trig(&mut self, trig_number: u8);
// }
// use num_traits::PrimInt;
// // 72 of these are possible.
// // They come in order.
// pub struct ParameterLock<T: PrimInt> {
//     // When unset 0xFF also default
//     pub track_number: u8,
//     // When unset 0xFF also default
//     pub type_number: u8,
//     // Internal data when nothing has ever been set for the track these are 0
//     // If a parameter lock is set for a track, the unset ones are 0xFF
//     pub values: [T; 64],
// }

// pub enum ParameterLock {
// syn_parameter_0: u16,
// syn_parameter_1: u16,
// syn_parameter_2: u16,
// syn_parameter_3: u16,
// syn_parameter_4: u16,
// syn_parameter_5: u16,
// syn_parameter_6: u16,
// syn_parameter_7: u16,

// smp_tune: i8,
// smp_fine: i8,
// smp_number: u8,
// smp_bit_reduction: u8,
// smp_start: u16,
// smp_end: u16,
// smp_loop_switch: bool,
// smp_level: u8,

// flt_attack: u8,
// flt_sustain: u8,
// flt_decay: u8,
// flt_release: u8,
// flt_frequency: u8,
// flt_resonance: u8,
// flt_type: FilterType,
// flt_env_depth: i8,

// amp_attack: u8,
// amp_hold: u8,
// amp_decay: u8,
// amp_drive: u8,
// amp_delay: u8,
// amp_reverb: u8,
// amp_pan: i8,
// amp_volume: u8,

// lfo_speed: u8,
// lfo_multiplier: LfoMultiplier,
// lfo_fade: u8,
// lfo_dest: LfoDestination,
// lfo_wav: LfoWaveform,
// lfo_start_phase: u8,
// lfo_mode: LfoMode,
// lfo_depth: i8,
// }

// pub struct TrigFxTrackParameterLock {
//     // TODO:
// }

// #define AR_PLOCK_TYPE_UNUSED        (0xFFu)

// #define AR_PLOCK_TYPE_MP0           (0x00u)  /* <syn> first machine parameter.  Also see e.g. AR_M_BDCLASSIC_P* */
// #define AR_PLOCK_TYPE_MP1           (0x01u)  /* <syn> second machine parameter                                  */
// #define AR_PLOCK_TYPE_MP2           (0x02u)  /* ..                                                              */
// #define AR_PLOCK_TYPE_MP3           (0x03u)  /* ..                                                              */
// #define AR_PLOCK_TYPE_MP4           (0x04u)  /* ..                                                              */
// #define AR_PLOCK_TYPE_MP5           (0x05u)  /* ..                                                              */
// #define AR_PLOCK_TYPE_MP6           (0x06u)  /* ..                                                              */
// #define AR_PLOCK_TYPE_MP7           (0x07u)  /* <syn> 8th machine parameter                                     */
// #define AR_PLOCK_TYPE_SMP_TUNE      (0x08u)  /* <sample> tune (0x28=-24, 0x40=+0, 0x58=+24) */
// #define AR_PLOCK_TYPE_SMP_FINE      (0x09u)  /* <sample> fine (0x00=-64, 0x40=+0, 0x7F=+63) */
// #define AR_PLOCK_TYPE_SMP_NR        (0x0Au)  /* <sample> nr (0(off), 1..127)                */
// #define AR_PLOCK_TYPE_SMP_BITRDC    (0x0Bu)  /* <sample> bitreduction (0..127)              */
// #define AR_PLOCK_TYPE_SMP_START     (0x0Cu)  /* <sample> start (0..120)                     */
// #define AR_PLOCK_TYPE_SMP_END       (0x0Du)  /* <sample> end (0..120)                       */
// #define AR_PLOCK_TYPE_SMP_LOOPSW    (0x0Eu)  /* <sample> loopsw (0..1)                      */
// #define AR_PLOCK_TYPE_SMP_LEVEL     (0x0Fu)  /* <sample> level (0..127)                     */
// #define AR_PLOCK_TYPE_FLT_ATTACK    (0x10u)  /* <filter> attacktime (0..127)                                  */
// #define AR_PLOCK_TYPE_FLT_SUSTAIN   (0x11u)  /* <filter> sustainlevel (0..127)                                */
// #define AR_PLOCK_TYPE_FLT_DECAY     (0x12u)  /* <filter> decaytime (0..127)                                   */
// #define AR_PLOCK_TYPE_FLT_RELEASE   (0x13u)  /* <filter> releasetime (0..127)                                 */
// #define AR_PLOCK_TYPE_FLT_FREQ      (0x14u)  /* <filter> frequency (0..127)                                   */
// #define AR_PLOCK_TYPE_FLT_RESO      (0x15u)  /* <filter> resonance (0..127)                                   */
// #define AR_PLOCK_TYPE_FLT_TYPE      (0x16u)  /* <filter> type (0=lp2, 1=lp1, 2=bp, 3=hp1, 4=hp2, 5=bs, 6=pk). */
//                                              /*                See AR_FLT_TYPE_xxx                            */
// #define AR_PLOCK_TYPE_FLT_ENV       (0x17u)  /* <filter> envdepth (0(-64)..64(0)..127(+63))                   */
// #define AR_PLOCK_TYPE_AMP_ATTACK    (0x18u)  /* <amp> attacktime (0..127)                 */
// #define AR_PLOCK_TYPE_AMP_HOLD      (0x19u)  /* <amp> holdtime (0..127)                   */
// #define AR_PLOCK_TYPE_AMP_DECAY     (0x1Au)  /* <amp> decaytime (0..126,127=inf)          */
// #define AR_PLOCK_TYPE_AMP_DRIVE     (0x1Bu)  /* <amp> overdrive (0..127)                  */
// #define AR_PLOCK_TYPE_AMP_DELAY     (0x1Cu)  /* <amp> delaysend (0..127)                  */
// #define AR_PLOCK_TYPE_AMP_REVERB    (0x1Du)  /* <amp> reverbsend (0..127)                 */
// #define AR_PLOCK_TYPE_AMP_PAN       (0x1Eu)  /* <amp> pan (0(left)..64(ctr)..127(right))  */
// #define AR_PLOCK_TYPE_AMP_VOLUME    (0x1Fu)  /* <amp> volume (0..127)                     */
// #define AR_PLOCK_TYPE_UNKNOWN_20    (0x20u)  /* (todo?) */
// #define AR_PLOCK_TYPE_LFO_SPEED     (0x21u)  /* <lfo> speed (0(-63),64(0),127(+63))                               */
// #define AR_PLOCK_TYPE_LFO_MULTIPLY  (0x22u)  /* <lfo> multiplier (0=1, .., 0xb=2k)                                */
// #define AR_PLOCK_TYPE_LFO_FADE      (0x23u)  /* <lfo> fade (0(-63),64(0),127(+63))                                */
// #define AR_PLOCK_TYPE_LFO_DEST      (0x24u)  /* <lfo> dest (0=off, .., 0x29=reverbsend) (see AR_LFO_DEST_xxx)     */
// #define AR_PLOCK_TYPE_LFO_WAVEFORM  (0x25u)  /* <lfo> waveform (0=tri, 1=sin, 2=sqr, 3=saw, 4=exp, 5=rmp, 6=rnd). */
//                                              /*                 See AR_LFO_WAVEFORM_xxx                           */
// #define AR_PLOCK_TYPE_LFO_PHASE     (0x26u)  /* <lfo> startphase (0..127)                                         */
// #define AR_PLOCK_TYPE_LFO_TRIGMODE  (0x27u)  /* <lfo> trigmode (0=fre, 1=trg, 2=hld, 3=one, 4=hlf)                */
//                                              /*                 See AR_LFO_TRIGMODE_xxx                           */
// #define AR_PLOCK_TYPE_LFO_DEPTH     (0x28u)  /* <lfo> depth (0..127)                                              */
use num_traits::PrimInt;
use rytm_sys::ar_plock_seq_t;
use std::cell::{Ref, RefCell};

#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct ParameterLockPool {
    // #[derivative(Debug = "ignore")]
    pub inner: [rytm_sys::ar_plock_seq_t; 72],
}

impl Default for ParameterLockPool {
    fn default() -> Self {
        Self {
            inner: [rytm_sys::ar_plock_seq_t::default(); 72],
        }
    }
}

impl ParameterLockPool {
    pub(crate) fn to_raw(&self) -> [rytm_sys::ar_plock_seq_t; 72] {
        self.inner
    }

    pub(crate) fn from_raw(raw: [rytm_sys::ar_plock_seq_t; 72]) -> Self {
        Self { inner: raw }
    }

    pub(crate) fn set_fx_basic_plock(
        &mut self,
        trig_index: usize,
        plock_type: u8,
        value: u8,
    ) -> Result<(), RytmError> {
        self.set_basic_plock(trig_index, 12, plock_type, value)
    }

    pub(crate) fn set_fx_compound_plock(
        &mut self,
        trig_index: usize,
        plock_type: u8,
        value: u16,
    ) -> Result<(), RytmError> {
        self.set_compound_plock(trig_index, 12, plock_type, value)
    }

    pub(crate) fn set_basic_plock(
        &mut self,
        trig_index: usize,
        track_index: u8,
        plock_type: u8,
        value: u8,
    ) -> Result<(), RytmError> {
        let mut pool = self.inner.iter_mut();

        // Check if we have this type of basic plock already set if so modify it.
        if let Some(plock) = pool.find(|plock_seq| {
            plock_seq.track_nr == track_index && plock_seq.plock_type == plock_type
        }) {
            plock.data[trig_index] = value;
            return Ok(());
        }

        // I don't know if || is the right thing to do here.
        // Maybe && we'll see..
        // Find gets the first found one right?

        // Check if we have an available slot anywhere in the array.
        if let Some(empty_slot) =
            pool.find(|plock_seq| plock_seq.track_nr == 0xFF || plock_seq.plock_type == 0xFF)
        {
            // We know at this point that an empty slot is available.
            empty_slot.track_nr = track_index;
            empty_slot.plock_type = plock_type;
            empty_slot.data[trig_index] = value;

            return Ok(());
        }

        Err(ParameterLockMemoryFull)
    }

    pub fn get_basic_plock(
        &self,
        trig_index: usize,
        track_index: u8,
        plock_type: u8,
    ) -> Option<u8> {
        let mut pool = self.inner.iter();

        // Check if we have this type of basic plock already set if so modify it.
        if let Some(plock) = pool.find(|plock_seq| {
            plock_seq.track_nr == track_index && plock_seq.plock_type == plock_type
        }) {
            return Some(plock.data[trig_index]);
        }
        None
    }

    pub fn get_compound_plock(
        &self,
        trig_index: usize,
        track_index: u8,
        plock_type: u8,
    ) -> Option<u16> {
        let mut pool = self.inner.iter();

        // Check if we have this type of basic plock already set if so modify it.
        if let Some(plock) = pool.find(|plock_seq| {
            plock_seq.track_nr == track_index && plock_seq.plock_type == plock_type
        }) {
            return Some(
                (plock.data[trig_index] as u16) << 8 | self.inner[1].data[trig_index] as u16,
            );
        }
        None
    }

    // TODO: Check the order of the msb lsb.
    pub(crate) fn set_compound_plock(
        &mut self,
        trig_index: usize,
        track_index: u8,
        plock_type: u8,
        value: u16,
    ) -> Result<(), RytmError> {
        const ADJACENT_PLOCK_SLOT_TRACK_NUMBER_BYTE: u8 = 128;
        const ADJACENT_PLOCK_SLOT_TYPE_BYTE: u8 = 128;

        let value_msb = (value >> 8) as u8;
        let value_lsb = value as u8;

        // Partition the pool preserving the order so empty slots are stacked at the end.
        stable_partition(&mut self.inner[..], |plock_seq| {
            plock_seq.track_nr != 0xFF || plock_seq.plock_type != 0xFF
        });

        let last_slot = &self.inner[self.inner.len() - 1];
        let last_slot_available = last_slot.track_nr == 0xFF || last_slot.plock_type == 0xFF;

        let mut pool_iter = self.inner.iter_mut().enumerate();

        // Check if we have this type of compound plock already set if so modify it.
        if let Some((i, found_plock)) = pool_iter.find(|(_, plock_seq)| {
            plock_seq.track_nr == track_index && plock_seq.plock_type == plock_type
        }) {
            // This is safe because if we could have set it it means these indexes are valid.
            // TODO: Check msb lsb order.
            found_plock.data[trig_index] = value_msb;
            self.inner[i + 1].data[trig_index] = value_lsb;
            return Ok(());
        }

        // Check if we have an available slot in the end of the array because if not we can't set the companion byte.
        // Thus we return memory full error.
        if !last_slot_available {
            return Err(ParameterLockMemoryFull);
        }

        // Then we have enough slots to set the companion byte.
        // Let's find the first available slot.
        if let Some((i, found_empty_slot)) = pool_iter
            .find(|(_, plock_seq)| plock_seq.track_nr == 0xFF || plock_seq.plock_type == 0xFF)
        {
            // We know at this point that 2 empty slots are available.
            found_empty_slot.track_nr = track_index;
            found_empty_slot.plock_type = plock_type;
            found_empty_slot.data[trig_index] = value_msb;

            self.inner[i + 1].track_nr = ADJACENT_PLOCK_SLOT_TRACK_NUMBER_BYTE;
            self.inner[i + 1].plock_type = ADJACENT_PLOCK_SLOT_TYPE_BYTE;
            self.inner[i + 1].data[trig_index] = value_lsb;

            return Ok(());
        }

        Err(ParameterLockMemoryFull)
    }

    /// Clears the basic plock for the given trig.
    pub(crate) fn clear_basic_plock(
        &mut self,
        trig_index: usize,
        track_index: u8,
        plock_type: u8,
    ) -> Result<(), RytmError> {
        let mut pool = self.inner.iter_mut().enumerate();

        let mut plock_seq_index_which_we_cleared_from: Option<usize> = None;

        // Check if we have this type of basic plock already set if so modify it.
        if let Some((i, plock)) = pool.find(|(_, plock_seq)| {
            plock_seq.track_nr == track_index && plock_seq.plock_type == plock_type
        }) {
            plock.data[trig_index] = 0xFF;

            plock_seq_index_which_we_cleared_from = Some(i);
        }

        if let Some(i) = plock_seq_index_which_we_cleared_from {
            let plock = &mut self.inner[i];
            if plock.data.iter_mut().all(|byte| *byte == 0xFF) {
                // Release slot.
                plock.track_nr = 0xFF;
                plock.plock_type = 0xFF;
            }
        }

        Ok(())
    }

    pub(crate) fn clear_compound_plock(
        &mut self,
        trig_index: usize,
        track_index: u8,
        plock_type: u8,
    ) -> Result<(), RytmError> {
        let mut pool = self.inner.iter_mut().enumerate();

        let mut plock_seq_index_which_we_cleared_from: Option<usize> = None;

        // Check if we have this type of compound plock already set if so modify it.
        if let Some((i, plock)) = pool.find(|(i, plock_seq)| {
            plock_seq.track_nr == track_index && plock_seq.plock_type == plock_type
        }) {
            plock.data[trig_index] = 0xFF;
            self.inner[i + 1].data[trig_index] = 0xFF;

            plock_seq_index_which_we_cleared_from = Some(i);
        }

        if let Some(i) = plock_seq_index_which_we_cleared_from {
            let plock = &mut self.inner[i];
            if plock.data.iter_mut().all(|byte| *byte == 0xFF) {
                // Release both slots.
                plock.track_nr = 0xFF;
                plock.plock_type = 0xFF;
                self.inner[i + 1].track_nr = 0xFF;
                self.inner[i + 1].plock_type = 0xFF;
            }
        }

        Ok(())
    }

    pub(crate) fn clear_all_plocks(&mut self) {
        for plock_seq in self.inner.iter_mut() {
            plock_seq.track_nr = 0xFF;
            plock_seq.plock_type = 0xFF;
            for byte in plock_seq.data.iter_mut() {
                // Like the default.
                *byte = 0;
            }
        }
    }

    pub(crate) fn clear_all_plocks_for_track(&mut self, track_index: u8) {
        for plock_seq in self.inner.iter_mut() {
            if plock_seq.track_nr == track_index {
                plock_seq.track_nr = 0xFF;
                plock_seq.plock_type = 0xFF;
                for byte in plock_seq.data.iter_mut() {
                    // Like the default.
                    *byte = 0;
                }
            }
        }
    }

    pub(crate) fn clear_all_plocks_for_plock_type(&mut self, plock_type: u8) {
        for plock_seq in self.inner.iter_mut() {
            if plock_seq.plock_type == plock_type {
                plock_seq.track_nr = 0xFF;
                plock_seq.plock_type = 0xFF;
                for byte in plock_seq.data.iter_mut() {
                    // Like the default.
                    *byte = 0;
                }
            }
        }
    }

    // TODO: Update and write an interface for this or do it in a higher level.
    pub(crate) fn get_plocks_assigned_for_a_trigger(
        &self,
        trig_index: usize,
    ) -> Vec<(&rytm_sys::ar_plock_seq_t, u8)> {
        let mut plocks = Vec::new();

        for plock_seq in self.inner.iter() {
            if plock_seq.data[trig_index] != 0xFF {
                plocks.push((plock_seq, plock_seq.data[trig_index]));
            }
        }

        plocks
    }

    // TODO: A couple of more variants might make sense.
}
