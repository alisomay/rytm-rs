pub mod types;

use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sound_t;

use self::types::Machine;
use crate::error::RytmError;
use crate::sysex::SysexMeta;
use crate::ParameterError;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SoundType {
    Pool,
    #[default]
    WorkBuffer,
    KitQuery,
}

#[derive(Clone, Copy, Debug)]
pub struct Sound {
    /// Index of the sound.
    ///
    /// This can mean various things depending on the context
    ///
    /// - If this sound is retrieved from the sound pool, this is the index of the sound in the pool.
    /// - If this sound is retrieved from a track from the work buffer or a kit query, this is the index of the track.
    index: usize,
    /// Index of the sound if it was retrieved from the sound pool.
    pool_index: Option<usize>,
    /// Kit number if this sound is retrieved from a kit query
    kit_number: Option<usize>,
    /// Index of the sound if it was retrieved from a track from the work buffer.
    assigned_track: Option<usize>,
    sysex_meta: SysexMeta,
    /// Version of the sound structure.
    version: u32,
    machine: Machine,

    _unknown_arr1: [u8; 12],
}

impl From<&Sound> for ar_sound_t {
    fn from(sound: &Sound) -> Self {
        todo!("Conversion to ar_sound_t is not implemented yet.")
    }
}

impl Sound {
    pub fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_sound: &ar_sound_t,
        kit_number_and_assigned_track: Option<(usize, usize)>,
    ) -> Result<Self, RytmError> {
        let machine: Machine = raw_sound.machine_type.try_into()?;

        let version = ((raw_sound.__unknown_arr1[4] as u32) << 24)
            | ((raw_sound.__unknown_arr1[5] as u32) << 16)
            | ((raw_sound.__unknown_arr1[6] as u32) << 8)
            | (raw_sound.__unknown_arr1[7] as u32);

        let mut index: usize = 0;
        let mut assigned_track = None;
        let mut kit_number = None;
        let mut pool_index = None;
        if sysex_meta.is_targeting_work_buffer() {
            index = (sysex_meta.obj_nr & 0b0111_1111_1111_1111) as usize;
            assigned_track = Some(index as usize);
        }

        if let Some((kit_n, assigned_t)) = kit_number_and_assigned_track {
            index = assigned_t;
            assigned_track = Some(assigned_t);
            kit_number = Some(kit_n);
        }

        if kit_number_and_assigned_track.is_none() && !sysex_meta.is_targeting_work_buffer() {
            index = (sysex_meta.obj_nr & 0b0111_1111_1111_1111) as usize;
            pool_index = Some(index);
        }

        Ok(Self {
            _unknown_arr1: raw_sound.__unknown_arr1,
            index,
            pool_index,
            kit_number,
            assigned_track,
            sysex_meta,
            version,
            machine,
        })
    }

    /// Checks if the given machine is compatible for the given track.
    fn is_machine_compatible_for_track(track_index: usize, machine: Machine) -> bool {
        let compatible_machines = unsafe { rytm_sys::ar_sound_compatible_machines };
        let compatible_machines_for_track = compatible_machines[track_index];

        let mut compatible_machines_for_track_size = 0;
        loop {
            unsafe {
                let return_id = rytm_sys::ar_sound_get_machine_id_by_track_and_list_idx(
                    track_index as u32,
                    compatible_machines_for_track_size,
                );
                if return_id == -1 {
                    break;
                }
                compatible_machines_for_track_size += 1;
            }
        }

        let compatible_machines_for_track_slice = unsafe {
            std::slice::from_raw_parts(
                compatible_machines_for_track,
                compatible_machines_for_track_size as usize,
            )
        };

        compatible_machines_for_track_slice.contains(&((machine as u8) as i32))
    }

    pub fn set_machine(&mut self, machine: Machine) -> Result<(), RytmError> {
        if let Some(assigned_track) = self.assigned_track() {
            if !Sound::is_machine_compatible_for_track(assigned_track, machine) {
                return Err(ParameterError::Compatibility {
                    value: machine.to_string(),
                    parameter_name: "Machine".to_string(),
                    reason: Some(format!(
                        "Given machine {} is not compatible for track {}",
                        machine, self.index
                    )),
                }
                .into());
            }
        }

        self.machine = machine;
        Ok(())
    }

    pub fn sound_type(&self) -> SoundType {
        if self.is_pool_sound() {
            SoundType::Pool
        } else if self.is_work_buffer_sound() {
            SoundType::WorkBuffer
        } else {
            SoundType::KitQuery
        }
    }

    pub fn is_pool_sound(&self) -> bool {
        self.pool_index.is_some()
    }

    pub fn is_work_buffer_sound(&self) -> bool {
        self.assigned_track().is_some() && self.kit_number.is_none()
    }

    pub fn is_part_of_a_kit_query(&self) -> bool {
        self.kit_number.is_some()
    }

    /// Returns the assigned track if this is a track sound.
    ///
    /// Returns `None` if this is not a track sound.
    pub fn assigned_track(&self) -> Option<usize> {
        self.assigned_track
    }

    pub fn to_raw_parts(&self) -> (SysexMeta, ar_sound_t) {
        (self.sysex_meta, self.into())
    }

    // TODO: Find the right range.
    #[parameter_range(range = "sound_index:0..=127")]
    pub fn try_default(sound_index: usize) -> Result<Self, RytmError> {
        Ok(Self {
            // BE EF BA CE 00 00 00 04 00 00 00 00
            _unknown_arr1: [
                0xBE, 0xEF, 0xBA, 0xCE, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
            ],
            sysex_meta: SysexMeta::try_default_for_sound(sound_index, None)?,
            index: sound_index,
            pool_index: Some(sound_index),
            kit_number: None,
            assigned_track: None,
            machine: Machine::default(),
            version: 0x0000_0000_0000_0000_0000_0000_0000_0004,
        })
    }

    pub fn work_buffer_default() -> Self {
        Self {
            // BE EF BA CE 00 00 00 04 00 00 00 00
            _unknown_arr1: [
                0xBE, 0xEF, 0xBA, 0xCE, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
            ],
            sysex_meta: SysexMeta::default_for_sound_in_work_buffer(None),
            index: 0b0000_0000_0000_0000_1000_0000_0000_0000,
            pool_index: None,
            kit_number: None,
            assigned_track: Some(0),
            machine: Machine::default(),
            version: 0x0000_0000_0000_0000_0000_0000_0000_0004,
        }
    }
}
