use crate::{error::RytmError, util::stable_partition, RytmError::ParameterLockMemoryFull};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Derivative, Clone, Copy, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct PlockSeq {
    pub track_nr: u8,
    pub plock_type: u8,

    #[serde(with = "BigArray")]
    pub data: [u8; 64],
}

impl Default for PlockSeq {
    fn default() -> Self {
        Self {
            track_nr: 0xFF,
            plock_type: 0xFF,
            // This is not project default in AR but I think it is more sound.
            // The project default is 0x00 in AR.
            data: [0xFF; 64],
        }
    }
}

impl From<rytm_sys::ar_plock_seq_t> for PlockSeq {
    fn from(raw: rytm_sys::ar_plock_seq_t) -> Self {
        Self {
            track_nr: raw.track_nr,
            plock_type: raw.plock_type,
            data: raw.data,
        }
    }
}

impl From<&PlockSeq> for rytm_sys::ar_plock_seq_t {
    fn from(plock_seq: &PlockSeq) -> Self {
        Self {
            track_nr: plock_seq.track_nr,
            plock_type: plock_seq.plock_type,
            data: plock_seq.data,
        }
    }
}

/// Wrapper type for the parameter lock pool.
///
/// This represents the parameter lock pool for a single patterns.
///
/// Has a total of 72 slots for 72 different parameter locks.
///
/// Each slot can hold 64 parameter lock values which corresponds to the 64 possible trigs in a pattern.
#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct ParameterLockPool {
    pub owner_pattern_index: usize,
    pub inner: Vec<PlockSeq>,
    pub is_owner_pattern_work_buffer: bool,
}

impl Default for ParameterLockPool {
    fn default() -> Self {
        let mut inner = Vec::with_capacity(72);
        for _ in 0..72 {
            inner.push(PlockSeq::default());
        }

        Self {
            owner_pattern_index: 0,
            inner,
            is_owner_pattern_work_buffer: false,
        }
    }
}

impl ParameterLockPool {
    pub fn as_raw(&self) -> [rytm_sys::ar_plock_seq_t; 72] {
        self.inner
            .iter()
            .map(std::convert::Into::into)
            .collect::<Vec<_>>()
            .try_into()
            .expect("This can not fail until we change the size of 72 anywhere.")
    }

    // This type is around 4kb in size, copying is indeed inefficient.
    // But until now it didn't create any practical problems.
    // If we see a slowdown in the future we can change this.
    pub fn from_raw(
        raw: &[rytm_sys::ar_plock_seq_t; 72],
        owner_pattern_index: usize,
        is_owner_pattern_work_buffer: bool,
    ) -> Self {
        let inner = raw
            .iter()
            .map(|plock_seq| (*plock_seq).into())
            .collect::<Vec<_>>();

        Self {
            owner_pattern_index,
            inner,
            is_owner_pattern_work_buffer,
        }
    }

    pub fn set_basic_plock(
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

    pub fn set_compound_plock(
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
            // This is safe because if we could have set it it means these indexes are valid.
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
        if let Some((i, found_empty_slot)) = self
            .inner
            .iter_mut()
            .enumerate()
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
        if let Some((i, plock)) = self
            .inner
            .iter()
            .enumerate()
            .find(|(_, plock_seq)| -> bool {
                plock_seq.track_nr == track_index && plock_seq.plock_type == plock_type
            })
        {
            return Some(
                ((plock.data[trig_index] as u16) << 8)
                    | (self.inner[i + 1].data[trig_index] as u16),
            );
        }
        None
    }

    /// Clears the basic plock for the given trig.
    pub fn clear_basic_plock(&mut self, trig_index: usize, track_index: u8, plock_type: u8) {
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
    }

    pub fn clear_compound_plock(&mut self, trig_index: usize, track_index: u8, plock_type: u8) {
        let mut plock_seq_index_which_we_cleared_from: Option<usize> = None;

        // Check if we have this type of compound plock already set if so modify it.
        if let Some((i, plock)) = self.inner.iter_mut().enumerate().find(|(_, plock_seq)| {
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
    }

    pub fn set_fx_basic_plock(
        &mut self,
        trig_index: usize,
        plock_type: u8,
        value: u8,
    ) -> Result<(), RytmError> {
        self.set_basic_plock(trig_index, 12, plock_type, value)
    }

    pub fn set_fx_compound_plock(
        &mut self,
        trig_index: usize,
        plock_type: u8,
        value: u16,
    ) -> Result<(), RytmError> {
        self.set_compound_plock(trig_index, 12, plock_type, value)
    }

    pub fn get_fx_basic_plock(&self, trig_index: usize, plock_type: u8) -> Option<u8> {
        self.get_basic_plock(trig_index, 12, plock_type)
    }

    pub fn get_fx_compound_plock(&self, trig_index: usize, plock_type: u8) -> Option<u16> {
        self.get_compound_plock(trig_index, 12, plock_type)
    }

    pub fn clear_fx_basic_plock(&mut self, trig_index: usize, plock_type: u8) {
        self.clear_basic_plock(trig_index, 12, plock_type);
    }

    pub fn clear_fx_compound_plock(&mut self, trig_index: usize, plock_type: u8) {
        self.clear_compound_plock(trig_index, 12, plock_type);
    }

    pub fn clear_all_plocks(&mut self) {
        for plock_seq in &mut self.inner {
            plock_seq.track_nr = 0xFF;
            plock_seq.plock_type = 0xFF;
            for byte in &mut plock_seq.data {
                // Like the default.
                *byte = 0xFF;
            }
        }
    }

    pub fn clear_all_plocks_for_track(&mut self, track_index: u8) {
        for plock_seq in &mut self.inner {
            if plock_seq.track_nr == track_index {
                plock_seq.track_nr = 0xFF;
                plock_seq.plock_type = 0xFF;
                for byte in &mut plock_seq.data {
                    // Like the default.
                    *byte = 0xFF;
                }
            }
        }
    }
}
