use crate::{
    error::{ParameterError, RytmError},
    object::pattern::{Length, RetrigRate},
    util::{from_s_u16_t, to_s_u16_t_union_a},
};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_kit_t;
use serde::{Deserialize, Serialize};

/// Represents the retrig settings for a track.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TrackRetrigMenu {
    track_index: usize,
    rate: RetrigRate,
    length: Length,
    velocity_curve: i8,
    always_on: bool,
}

impl TrackRetrigMenu {
    pub(crate) fn get_default_for_13_tracks() -> [Self; 13] {
        [
            Self::try_default_for_track(0).unwrap(),
            Self::try_default_for_track(1).unwrap(),
            Self::try_default_for_track(2).unwrap(),
            Self::try_default_for_track(3).unwrap(),
            Self::try_default_for_track(4).unwrap(),
            Self::try_default_for_track(5).unwrap(),
            Self::try_default_for_track(6).unwrap(),
            Self::try_default_for_track(7).unwrap(),
            Self::try_default_for_track(8).unwrap(),
            Self::try_default_for_track(9).unwrap(),
            Self::try_default_for_track(10).unwrap(),
            Self::try_default_for_track(11).unwrap(),
            Self::try_default_for_track(12).unwrap(),
        ]
    }

    #[parameter_range(range = "track_index:0..=12")]
    pub fn try_default_for_track(track_index: usize) -> Result<Self, RytmError> {
        let rate = RetrigRate::default();
        let length = Length::default();
        let velocity_curve = 0;
        let always_on = false;
        Ok(Self {
            track_index,
            rate,
            length,
            velocity_curve,
            always_on,
        })
    }

    #[parameter_range(range = "track_index:0..=12")]
    pub(crate) fn try_from_raw(track_index: usize, raw_kit: &ar_kit_t) -> Result<Self, RytmError> {
        let flags = unsafe { from_s_u16_t(raw_kit.retrig_always_on) };
        let always_on = flags & (1 << track_index) != 0;

        let raw_retrig = raw_kit.retrig[track_index];
        Ok(Self {
            track_index,
            rate: raw_retrig.retrig.try_into()?,
            length: raw_retrig.length.try_into()?,
            velocity_curve: raw_retrig.vel_curve,
            always_on,
        })
    }

    pub(crate) fn apply_to_raw_kit(&self, raw_kit: &mut ar_kit_t) {
        let raw_retrig = &mut raw_kit.retrig[self.track_index];
        raw_retrig.retrig = self.rate.into();
        raw_retrig.length = self.length.into();
        raw_retrig.vel_curve = self.velocity_curve;
        let flags = unsafe { from_s_u16_t(raw_kit.retrig_always_on) };
        let new_flags = if self.always_on {
            flags | (1 << self.track_index)
        } else {
            flags & !(1 << self.track_index)
        };
        raw_kit.retrig_always_on = to_s_u16_t_union_a(new_flags);
    }

    /// Sets the retrig rate.
    pub fn set_rate(&mut self, rate: RetrigRate) {
        self.rate = rate;
    }

    /// Sets the retrig length.
    pub fn set_length(&mut self, length: Length) {
        self.length = length;
    }

    /// Sets the velocity curve.
    ///
    /// Range: `-128..=127`
    #[parameter_range(range = "velocity_curve:-128..=127")]
    pub fn set_velocity_curve(&mut self, velocity_curve: i8) -> Result<(), RytmError> {
        self.velocity_curve = velocity_curve;
        Ok(())
    }

    /// Sets whether the retrig is always on.
    ///
    /// If `true`, the retrig is always on.
    pub fn set_always_on(&mut self, always_on: bool) {
        self.always_on = always_on;
    }

    /// Returns the track index. The track which this setting belongs to.
    ///
    /// Range: `0..=12`
    pub const fn track_index(&self) -> usize {
        self.track_index
    }

    /// Returns the retrig rate.
    pub const fn rate(&self) -> RetrigRate {
        self.rate
    }

    /// Returns the retrig length.
    pub const fn length(&self) -> Length {
        self.length
    }

    /// Returns the velocity curve.
    ///
    /// Range: `-128..=127`
    pub const fn velocity_curve(&self) -> i8 {
        self.velocity_curve
    }

    /// Returns whether the retrig is always on.
    ///
    /// If `true`, the retrig is always on.
    pub const fn always_on(&self) -> bool {
        self.always_on
    }
}
