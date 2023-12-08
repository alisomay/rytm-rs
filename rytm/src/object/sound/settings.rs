use super::types::{Machine, SoundModTarget, SoundSettingsChromaticMode};
use crate::error::{ConversionError, ParameterError, RytmError};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sound_t;

/// A sound's settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct SoundSettings {
    machine: Machine,
    chromatic_mode: SoundSettingsChromaticMode,
    env_reset_filter: bool,
    velocity_to_volume: bool,
    legacy_fx_send: bool,

    velocity_modulation_amt_1: i8,
    velocity_modulation_target_1: SoundModTarget,
    velocity_modulation_amt_2: i8,
    velocity_modulation_target_2: SoundModTarget,
    velocity_modulation_amt_3: i8,
    velocity_modulation_target_3: SoundModTarget,
    velocity_modulation_amt_4: i8,
    velocity_modulation_target_4: SoundModTarget,

    after_touch_modulation_amt_1: i8,
    after_touch_modulation_target_1: SoundModTarget,
    after_touch_modulation_amt_2: i8,
    after_touch_modulation_target_2: SoundModTarget,
    after_touch_modulation_amt_3: i8,
    after_touch_modulation_target_3: SoundModTarget,
    after_touch_modulation_amt_4: i8,
    after_touch_modulation_target_4: SoundModTarget,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            machine: Machine::default(),
            chromatic_mode: SoundSettingsChromaticMode::default(),
            env_reset_filter: false,
            velocity_to_volume: false,
            legacy_fx_send: false,

            velocity_modulation_amt_1: 0,
            velocity_modulation_target_1: SoundModTarget::default(),
            velocity_modulation_amt_2: 0,
            velocity_modulation_target_2: SoundModTarget::default(),
            velocity_modulation_amt_3: 0,
            velocity_modulation_target_3: SoundModTarget::default(),
            velocity_modulation_amt_4: 0,
            velocity_modulation_target_4: SoundModTarget::default(),

            after_touch_modulation_amt_1: 0,
            after_touch_modulation_target_1: SoundModTarget::default(),
            after_touch_modulation_amt_2: 0,
            after_touch_modulation_target_2: SoundModTarget::default(),
            after_touch_modulation_amt_3: 0,
            after_touch_modulation_target_3: SoundModTarget::default(),
            after_touch_modulation_amt_4: 0,
            after_touch_modulation_target_4: SoundModTarget::default(),
        }
    }
}

impl TryFrom<&ar_sound_t> for SoundSettings {
    type Error = ConversionError;
    fn try_from(raw_sound: &ar_sound_t) -> Result<Self, Self::Error> {
        // bit 0  : ?
        // bit 1  : env reset filter switch
        // bit 2  : legacy fx send switch
        // bit 3  : ?
        // bit 4+5: chromatic mode  0=OFF, 1=SYNTH, 2=SAMPLE, 3=SYN+SMP
        // bit 6  : velocity to vol switch
        // bit 7  : ?
        let raw_mode_flags = raw_sound.mode_flags;
        let chromatic_mode_number = (raw_mode_flags & 0b0011_0000) >> 4;

        Ok(Self {
            machine: raw_sound.machine_type.try_into()?,
            chromatic_mode: chromatic_mode_number.try_into()?,
            env_reset_filter: raw_mode_flags & 0b0000_0010 != 0,
            velocity_to_volume: raw_mode_flags & 0b0100_0000 != 0,
            legacy_fx_send: raw_mode_flags & 0b0000_0100 != 0,

            velocity_modulation_amt_1: raw_sound.vel_amt_1 as i8 - 64,
            velocity_modulation_target_1: raw_sound.vel_target_1.try_into()?,
            velocity_modulation_amt_2: raw_sound.vel_amt_2 as i8 - 64,
            velocity_modulation_target_2: raw_sound.vel_target_2.try_into()?,
            velocity_modulation_amt_3: raw_sound.vel_amt_3 as i8 - 64,
            velocity_modulation_target_3: raw_sound.vel_target_3.try_into()?,
            velocity_modulation_amt_4: raw_sound.vel_amt_4 as i8 - 64,
            velocity_modulation_target_4: raw_sound.vel_target_4.try_into()?,

            after_touch_modulation_amt_1: raw_sound.at_amt_1 as i8 - 64,
            after_touch_modulation_target_1: raw_sound.at_target_1.try_into()?,
            after_touch_modulation_amt_2: raw_sound.at_amt_2 as i8 - 64,
            after_touch_modulation_target_2: raw_sound.at_target_2.try_into()?,
            after_touch_modulation_amt_3: raw_sound.at_amt_3 as i8 - 64,
            after_touch_modulation_target_3: raw_sound.at_target_3.try_into()?,
            after_touch_modulation_amt_4: raw_sound.at_amt_4 as i8 - 64,
            after_touch_modulation_target_4: raw_sound.at_target_4.try_into()?,
        })
    }
}

impl SoundSettings {
    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        raw_sound.machine_type = self.machine.into();

        let chromatic_mode_number: u8 = self.chromatic_mode.into();
        raw_sound.mode_flags = chromatic_mode_number << 4;
        raw_sound.mode_flags |= (self.env_reset_filter as u8) << 1;
        raw_sound.mode_flags |= (self.legacy_fx_send as u8) << 2;
        raw_sound.mode_flags |= (self.velocity_to_volume as u8) << 6;

        // All amounts are TODO: Try interpreting them as i8,  device -128..=+127
        raw_sound.vel_amt_1 = self.velocity_modulation_amt_1 as u8 + 64;
        raw_sound.vel_target_1 = self.velocity_modulation_target_1.into();
        raw_sound.vel_amt_2 = self.velocity_modulation_amt_2 as u8 + 64;
        raw_sound.vel_target_2 = self.velocity_modulation_target_2.into();
        raw_sound.vel_amt_3 = self.velocity_modulation_amt_3 as u8 + 64;
        raw_sound.vel_target_3 = self.velocity_modulation_target_3.into();
        raw_sound.vel_amt_4 = self.velocity_modulation_amt_4 as u8 + 64;
        raw_sound.vel_target_4 = self.velocity_modulation_target_4.into();

        raw_sound.at_amt_1 = self.after_touch_modulation_amt_1 as u8 + 64;
        raw_sound.at_target_1 = self.after_touch_modulation_target_1.into();
        raw_sound.at_amt_2 = self.after_touch_modulation_amt_2 as u8 + 64;
        raw_sound.at_target_2 = self.after_touch_modulation_target_2.into();
        raw_sound.at_amt_3 = self.after_touch_modulation_amt_3 as u8 + 64;
        raw_sound.at_target_3 = self.after_touch_modulation_target_3.into();
        raw_sound.at_amt_4 = self.after_touch_modulation_amt_4 as u8 + 64;
        raw_sound.at_target_4 = self.after_touch_modulation_target_4.into();
    }

    /// Sets the machine type of the sound.
    pub(crate) fn set_machine(&mut self, machine: Machine) {
        self.machine = machine;
    }

    /// Sets the chromatic mode of the sound.
    pub fn set_chromatic_mode(&mut self, chromatic_mode: SoundSettingsChromaticMode) {
        self.chromatic_mode = chromatic_mode;
    }

    /// Sets the env reset filter switch of the sound.
    pub fn set_env_reset_filter(&mut self, env_reset_filter: bool) {
        self.env_reset_filter = env_reset_filter;
    }

    /// Sets the velocity to volume switch of the sound.
    pub fn set_velocity_to_volume(&mut self, velocity_to_volume: bool) {
        self.velocity_to_volume = velocity_to_volume;
    }

    /// Sets the legacy fx send switch of the sound.
    pub fn set_legacy_fx_send(&mut self, legacy_fx_send: bool) {
        self.legacy_fx_send = legacy_fx_send;
    }

    /// Sets the velocity modulation amount 1 of the sound.
    ///
    /// Range: `-127..=128`
    #[parameter_range(range = "velocity_modulation_amt_1:-127..=128")]
    pub fn set_velocity_modulation_amt_1(
        &mut self,
        velocity_modulation_amt_1: isize,
    ) -> Result<(), RytmError> {
        self.velocity_modulation_amt_1 = velocity_modulation_amt_1 as i8;
        Ok(())
    }

    /// Sets the velocity modulation target 1 of the sound.
    pub fn set_velocity_modulation_target_1(
        &mut self,
        velocity_modulation_target_1: SoundModTarget,
    ) {
        self.velocity_modulation_target_1 = velocity_modulation_target_1;
    }

    /// Sets the velocity modulation amount 2 of the sound.
    ///
    /// Range: `-127..=128`
    #[parameter_range(range = "velocity_modulation_amt_2:-127..=128")]
    pub fn set_velocity_modulation_amt_2(
        &mut self,
        velocity_modulation_amt_2: isize,
    ) -> Result<(), RytmError> {
        self.velocity_modulation_amt_2 = velocity_modulation_amt_2 as i8;
        Ok(())
    }

    /// Sets the velocity modulation target 2 of the sound.
    pub fn set_velocity_modulation_target_2(
        &mut self,
        velocity_modulation_target_2: SoundModTarget,
    ) {
        self.velocity_modulation_target_2 = velocity_modulation_target_2;
    }

    /// Sets the velocity modulation amount 3 of the sound.
    ///
    /// Range: `-127..=128`
    #[parameter_range(range = "velocity_modulation_amt_3:-127..=128")]
    pub fn set_velocity_modulation_amt_3(
        &mut self,
        velocity_modulation_amt_3: isize,
    ) -> Result<(), RytmError> {
        self.velocity_modulation_amt_3 = velocity_modulation_amt_3 as i8;
        Ok(())
    }

    /// Sets the velocity modulation target 3 of the sound.
    pub fn set_velocity_modulation_target_3(
        &mut self,
        velocity_modulation_target_3: SoundModTarget,
    ) {
        self.velocity_modulation_target_3 = velocity_modulation_target_3;
    }

    /// Sets the velocity modulation amount 4 of the sound.
    ///
    /// Range: `-127..=128`
    #[parameter_range(range = "velocity_modulation_amt_4:-127..=128")]
    pub fn set_velocity_modulation_amt_4(
        &mut self,
        velocity_modulation_amt_4: isize,
    ) -> Result<(), RytmError> {
        self.velocity_modulation_amt_4 = velocity_modulation_amt_4 as i8;
        Ok(())
    }

    /// Sets the velocity modulation target 4 of the sound.
    pub fn set_velocity_modulation_target_4(
        &mut self,
        velocity_modulation_target_4: SoundModTarget,
    ) {
        self.velocity_modulation_target_4 = velocity_modulation_target_4;
    }

    /// Sets the after touch modulation amount 1 of the sound.
    ///
    /// Range: `-127..=128`
    #[parameter_range(range = "after_touch_modulation_amt_1:-127..=128")]
    pub fn set_after_touch_modulation_amt_1(
        &mut self,
        after_touch_modulation_amt_1: isize,
    ) -> Result<(), RytmError> {
        self.after_touch_modulation_amt_1 = after_touch_modulation_amt_1 as i8;
        Ok(())
    }

    /// Sets the after touch modulation target 1 of the sound.
    pub fn set_after_touch_modulation_target_1(
        &mut self,
        after_touch_modulation_target_1: SoundModTarget,
    ) {
        self.after_touch_modulation_target_1 = after_touch_modulation_target_1;
    }

    /// Sets the after touch modulation amount 2 of the sound.
    ///
    /// Range: `-127..=128`
    #[parameter_range(range = "after_touch_modulation_amt_2:-127..=128")]
    pub fn set_after_touch_modulation_amt_2(
        &mut self,
        after_touch_modulation_amt_2: isize,
    ) -> Result<(), RytmError> {
        self.after_touch_modulation_amt_2 = after_touch_modulation_amt_2 as i8;
        Ok(())
    }

    /// Sets the after touch modulation target 2 of the sound.
    ///
    /// Range: `-127..=128`
    pub fn set_after_touch_modulation_target_2(
        &mut self,
        after_touch_modulation_target_2: SoundModTarget,
    ) {
        self.after_touch_modulation_target_2 = after_touch_modulation_target_2;
    }

    /// Sets the after touch modulation amount 3 of the sound.
    ///
    /// Range: `-127..=128`
    #[parameter_range(range = "after_touch_modulation_amt_3:-127..=128")]
    pub fn set_after_touch_modulation_amt_3(
        &mut self,
        after_touch_modulation_amt_3: isize,
    ) -> Result<(), RytmError> {
        self.after_touch_modulation_amt_3 = after_touch_modulation_amt_3 as i8;
        Ok(())
    }

    /// Sets the after touch modulation target 3 of the sound.
    pub fn set_after_touch_modulation_target_3(
        &mut self,
        after_touch_modulation_target_3: SoundModTarget,
    ) {
        self.after_touch_modulation_target_3 = after_touch_modulation_target_3;
    }

    /// Sets the after touch modulation amount 4 of the sound.
    ///
    /// Range: `-127..=128`
    #[parameter_range(range = "after_touch_modulation_amt_4:-127..=128")]
    pub fn set_after_touch_modulation_amt_4(
        &mut self,
        after_touch_modulation_amt_4: isize,
    ) -> Result<(), RytmError> {
        self.after_touch_modulation_amt_4 = after_touch_modulation_amt_4 as i8;
        Ok(())
    }

    /// Sets the after touch modulation target 4 of the sound.
    pub fn set_after_touch_modulation_target_4(
        &mut self,
        after_touch_modulation_target_4: SoundModTarget,
    ) {
        self.after_touch_modulation_target_4 = after_touch_modulation_target_4;
    }

    /// Returns the machine type of the sound.
    pub fn machine(&self) -> Machine {
        self.machine
    }

    /// Returns the chromatic mode of the sound.

    pub fn chromatic_mode(&self) -> SoundSettingsChromaticMode {
        self.chromatic_mode
    }

    /// Returns the env reset filter switch of the sound.
    pub fn env_reset_filter(&self) -> bool {
        self.env_reset_filter
    }

    /// Returns the velocity to volume switch of the sound.
    pub fn velocity_to_volume(&self) -> bool {
        self.velocity_to_volume
    }

    /// Returns the legacy fx send switch of the sound.
    pub fn legacy_fx_send(&self) -> bool {
        self.legacy_fx_send
    }

    /// Returns the velocity modulation amount 1 of the sound.
    ///
    /// Range: `-127..=128`
    pub fn velocity_modulation_amt_1(&self) -> isize {
        self.velocity_modulation_amt_1 as isize
    }

    /// Returns the velocity modulation target 1 of the sound.
    pub fn velocity_modulation_target_1(&self) -> SoundModTarget {
        self.velocity_modulation_target_1
    }

    /// Returns the velocity modulation amount 2 of the sound.
    ///    
    /// Range: `-127..=128`
    pub fn velocity_modulation_amt_2(&self) -> isize {
        self.velocity_modulation_amt_2 as isize
    }

    /// Returns the velocity modulation target 2 of the sound.
    pub fn velocity_modulation_target_2(&self) -> SoundModTarget {
        self.velocity_modulation_target_2
    }

    /// Returns the velocity modulation amount 3 of the sound.
    ///
    /// Range: `-127..=128`
    pub fn velocity_modulation_amt_3(&self) -> isize {
        self.velocity_modulation_amt_3 as isize
    }

    /// Returns the velocity modulation target 3 of the sound.
    pub fn velocity_modulation_target_3(&self) -> SoundModTarget {
        self.velocity_modulation_target_3
    }

    /// Returns the velocity modulation amount 4 of the sound
    ///
    /// Range: `-127..=128.
    pub fn velocity_modulation_amt_4(&self) -> isize {
        self.velocity_modulation_amt_4 as isize
    }

    /// Returns the velocity modulation target 4 of the sound.
    pub fn velocity_modulation_target_4(&self) -> SoundModTarget {
        self.velocity_modulation_target_4
    }

    /// Returns the after touch modulation amount 1 of the sound.
    ///
    /// Range: `-127..=128.
    pub fn after_touch_modulation_amt_1(&self) -> isize {
        self.after_touch_modulation_amt_1 as isize
    }

    /// Returns the after touch modulation target 1 of the sound.
    pub fn after_touch_modulation_target_1(&self) -> SoundModTarget {
        self.after_touch_modulation_target_1
    }

    /// Returns the after touch modulation amount 2 of the sound.
    ///
    /// Range: `-127..=128.
    pub fn after_touch_modulation_amt_2(&self) -> isize {
        self.after_touch_modulation_amt_2 as isize
    }

    /// Returns the after touch modulation target 2 of the sound.
    pub fn after_touch_modulation_target_2(&self) -> SoundModTarget {
        self.after_touch_modulation_target_2
    }

    /// Returns the after touch modulation amount 3 of the sound.
    ///
    /// Range: `-127..=128.
    pub fn after_touch_modulation_amt_3(&self) -> isize {
        self.after_touch_modulation_amt_3 as isize
    }

    /// Returns the after touch modulation target 3 of the sound.
    pub fn after_touch_modulation_target_3(&self) -> SoundModTarget {
        self.after_touch_modulation_target_3
    }

    /// Returns the after touch modulation amount 4 of the sound.
    ///
    /// Range: `-127..=128.
    pub fn after_touch_modulation_amt_4(&self) -> isize {
        self.after_touch_modulation_amt_4 as isize
    }

    /// Returns the after touch modulation target 4 of the sound.
    pub fn after_touch_modulation_target_4(&self) -> SoundModTarget {
        self.after_touch_modulation_target_4
    }
}
