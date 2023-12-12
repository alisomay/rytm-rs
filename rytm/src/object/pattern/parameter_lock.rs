use crate::{error::RytmError, util::stable_partition, RytmError::ParameterLockMemoryFull};
use derivative::Derivative;

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
    pub(crate) fn as_raw(&self) -> [rytm_sys::ar_plock_seq_t; 72] {
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

    pub(crate) fn clear_fx_basic_plock(
        &mut self,
        trig_index: usize,
        plock_type: u8,
    ) -> Result<(), RytmError> {
        self.clear_basic_plock(trig_index, 12, plock_type)
    }

    pub(crate) fn clear_fx_compound_plock(
        &mut self,
        trig_index: usize,
        plock_type: u8,
    ) -> Result<(), RytmError> {
        self.clear_compound_plock(trig_index, 12, plock_type)
    }

    pub(crate) fn get_fx_basic_plock(&self, trig_index: usize, plock_type: u8) -> Option<u8> {
        self.get_basic_plock(trig_index, 12, plock_type)
    }

    pub(crate) fn get_fx_compound_plock(&self, trig_index: usize, plock_type: u8) -> Option<u16> {
        self.get_compound_plock(trig_index, 12, plock_type)
    }

    pub(crate) fn set_basic_plock(
        &mut self,
        trig_index: usize,
        track_index: u8,
        plock_type: u8,
        value: u8,
    ) -> Result<(), RytmError> {
        // Check if we have this type of basic plock already set if so modify it.
        if let Some(plock) = self.inner.iter_mut().find(|plock_seq| {
            plock_seq.track_nr == track_index && plock_seq.plock_type == plock_type
        }) {
            plock.data[trig_index] = value;
            return Ok(());
        }

        // Check if we have an available slot anywhere in the array.
        if let Some(empty_slot) = self
            .inner
            .iter_mut()
            .find(|plock_seq| plock_seq.track_nr == 0xFF || plock_seq.plock_type == 0xFF)
        {
            // We know at this point that an empty slot is available.
            empty_slot.track_nr = track_index;
            empty_slot.plock_type = plock_type;
            empty_slot.data[trig_index] = value;

            return Ok(());
        }

        Err(ParameterLockMemoryFull)
    }

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

        // Check if we have this type of compound plock already set if so modify it.
        if let Some((i, found_plock)) =
            self.inner
                .iter_mut()
                .enumerate()
                .find(|(_, plock_seq)| -> bool {
                    plock_seq.track_nr == track_index && plock_seq.plock_type == plock_type
                })
        {
            // dbg!(found_plock);
            // panic!("PLOCK FOUND");
            // This is safe because if we could have set it it means these indexes are valid.
            found_plock.data[trig_index] = value_lsb;
            self.inner[i + 1].data[trig_index] = value_msb;

            return Ok(());
        }

        // Check if we have an available slot in the end of the array because if not we can't set the companion byte.
        // Thus we return memory full error.
        if !last_slot_available {
            return Err(ParameterLockMemoryFull);
        }

        // Then we have enough slots to set the companion byte.
        // Let's find the first available slot.
        if let Some((i, found_empty_slot)) = self
            .inner
            .iter_mut()
            .enumerate()
            .find(|(_, plock_seq)| plock_seq.track_nr == 0xFF || plock_seq.plock_type == 0xFF)
        {
            // We know at this point that 2 empty slots are available.
            found_empty_slot.track_nr = track_index;
            found_empty_slot.plock_type = plock_type;
            found_empty_slot.data[trig_index] = value_lsb;

            self.inner[i + 1].track_nr = ADJACENT_PLOCK_SLOT_TRACK_NUMBER_BYTE;
            self.inner[i + 1].plock_type = ADJACENT_PLOCK_SLOT_TYPE_BYTE;
            self.inner[i + 1].data[trig_index] = value_msb;

            // let first_way = ((self.inner[i + 1].data[trig_index] as u16) << 8)
            //     | self.inner[i].data[trig_index] as u16;
            // let second_way_reverse = ((self.inner[i].data[trig_index] as u16) << 8)
            //     | self.inner[i + 1].data[trig_index] as u16;

            // dbg!(self.inner[i], self.inner[i + 1]);
            // dbg!(first_way);
            // dbg!(crate::util::scale_u16_to_f32(
            //     first_way, 0u16, 32767u16, -128f32, 127.99f32,
            // ));
            // panic!("PLOCK NEW");
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
        // Check if we have this type of basic plock already set if so modify it.
        if let Some(plock) = self.inner.iter().find(|plock_seq| {
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
        // Check if we have this type of basic plock already set if so modify it.
        if let Some(plock) = self.inner.iter().find(|plock_seq| -> bool {
            plock_seq.track_nr == track_index && plock_seq.plock_type == plock_type
        }) {
            return Some(
                (plock.data[trig_index] as u16) | (self.inner[1].data[trig_index] as u16) << 8,
            );
        }
        None
    }

    /// Clears the basic plock for the given trig.
    pub(crate) fn clear_basic_plock(
        &mut self,
        trig_index: usize,
        track_index: u8,
        plock_type: u8,
    ) -> Result<(), RytmError> {
        let mut plock_seq_index_which_we_cleared_from: Option<usize> = None;

        // Check if we have this type of basic plock already set if so modify it.
        if let Some((i, plock)) = self.inner.iter_mut().enumerate().find(|(_, plock_seq)| {
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
        let mut plock_seq_index_which_we_cleared_from: Option<usize> = None;

        // Check if we have this type of compound plock already set if so modify it.
        if let Some((i, plock)) = self.inner.iter_mut().enumerate().find(|(i, plock_seq)| {
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
