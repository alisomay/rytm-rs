use super::types::FxDelayTimeOnTheGrid;
use crate::{
    error::{ConversionError, ParameterError, RytmError},
    util::{i8_to_u8_midpoint_of_u8_input_range, u8_to_i8_midpoint_of_u8_input_range},
};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_kit_t;

/// Delay parameters for the kit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FxDelay {
    time: u8,
    ping_pong: bool,
    stereo_width: i8,
    feedback: u8,
    hpf: u8,
    lpf: u8,
    reverb_send: u8,
    volume: u8,
}

impl Default for FxDelay {
    fn default() -> Self {
        Self {
            time: 23,
            ping_pong: false,
            stereo_width: 0,
            feedback: 49,
            hpf: 32,
            lpf: 96,
            reverb_send: 0,
            volume: 110,
        }
    }
}

impl TryFrom<&ar_kit_t> for FxDelay {
    type Error = ConversionError;
    fn try_from(raw_kit: &ar_kit_t) -> Result<Self, Self::Error> {
        // map 0..=127 to 0..=198
        let feedback = (raw_kit.fx_delay_feedback as f32 / 127.0 * 198.0) as u8;

        Ok(Self {
            time: raw_kit.fx_delay_time,
            ping_pong: raw_kit.fx_delay_pingpong != 0,
            stereo_width: u8_to_i8_midpoint_of_u8_input_range(
                raw_kit.fx_delay_stereo_width,
                0,
                127,
            ),
            feedback,
            hpf: raw_kit.fx_delay_hpf,
            lpf: raw_kit.fx_delay_lpf,
            reverb_send: raw_kit.fx_delay_reverb_send,
            volume: raw_kit.fx_delay_volume,
        })
    }
}

impl FxDelay {
    pub(crate) fn apply_to_raw_kit(self, raw_kit: &mut ar_kit_t) {
        // map 0..=198 to 0..=127
        let feedback = (self.feedback as f32 / 198.0 * 127.0) as u8;

        raw_kit.fx_delay_time = self.time;
        raw_kit.fx_delay_pingpong = self.ping_pong as u8;
        raw_kit.fx_delay_stereo_width =
            i8_to_u8_midpoint_of_u8_input_range(self.stereo_width, 0, 127);
        raw_kit.fx_delay_feedback = feedback;
        raw_kit.fx_delay_hpf = self.hpf;
        raw_kit.fx_delay_lpf = self.lpf;
        raw_kit.fx_delay_reverb_send = self.reverb_send;
        raw_kit.fx_delay_volume = self.volume;
    }

    /// Sets the time of the delay.
    ///
    /// Range: `0..=127`

    #[parameter_range(range = "time:0..=127")]
    pub fn set_time(&mut self, time: u8) -> Result<(), RytmError> {
        self.time = time;
        Ok(())
    }

    /// Sets the time of the delay on the grid.
    pub fn set_time_on_grid(&mut self, time: FxDelayTimeOnTheGrid) {
        self.time = time.into();
    }

    /// Sets the ping pong of the delay.
    pub fn set_ping_pong(&mut self, enable: bool) {
        self.ping_pong = enable;
    }

    /// Sets the stereo width of the delay.
    ///
    /// Range: `-64..=63`

    #[parameter_range(range = "stereo_width:-64..=63")]
    pub fn set_stereo_width(&mut self, stereo_width: i8) -> Result<(), RytmError> {
        self.stereo_width = stereo_width;
        Ok(())
    }

    /// Sets the feedback of the delay.
    ///
    /// Range: `0..=`198`
    #[parameter_range(range = "feedback:0..=198")]
    pub fn set_feedback(&mut self, feedback: u8) -> Result<(), RytmError> {
        self.feedback = feedback;
        Ok(())
    }

    /// Sets the high pass filter of the delay.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "hpf:0..=127")]
    pub fn set_hpf(&mut self, hpf: u8) -> Result<(), RytmError> {
        self.hpf = hpf;
        Ok(())
    }

    /// Sets the low pass filter of the delay.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "lpf:0..=127")]
    pub fn set_lpf(&mut self, lpf: u8) -> Result<(), RytmError> {
        self.lpf = lpf;
        Ok(())
    }

    /// Sets the reverb send of the delay.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "reverb_send:0..=127")]
    pub fn set_reverb_send(&mut self, reverb_send: u8) -> Result<(), RytmError> {
        self.reverb_send = reverb_send;
        Ok(())
    }

    /// Sets the volume of the delay.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "volume:0..=127")]
    pub fn set_volume(&mut self, volume: u8) -> Result<(), RytmError> {
        self.volume = volume;
        Ok(())
    }

    /// Returns the time of the delay.
    ///
    /// Range: `0..=127`
    pub const fn time(&self) -> usize {
        self.time as usize
    }

    /// Returns the ping pong state of the delay.
    pub const fn ping_pong(&self) -> bool {
        self.ping_pong
    }

    /// Returns the stereo width of the delay.
    ///
    /// Range: `-64..=63`
    pub const fn stereo_width(&self) -> isize {
        self.stereo_width as isize
    }

    /// Returns the feedback of the delay.
    ///
    /// Range: `0..=198`
    pub const fn feedback(&self) -> usize {
        self.feedback as usize
    }

    /// Returns the high pass filter of the delay.
    ///
    /// Range: `0..=127`
    pub const fn hpf(&self) -> usize {
        self.hpf as usize
    }

    /// Returns the low pass filter of the delay.
    ///
    /// Range: `0..=127`
    pub const fn lpf(&self) -> usize {
        self.lpf as usize
    }

    /// Returns the reverb send of the delay.
    ///
    /// Range: `0..=127`
    pub const fn reverb_send(&self) -> usize {
        self.reverb_send as usize
    }

    /// Returns the volume of the delay.
    ///
    /// Range: `0..=127`
    pub const fn volume(&self) -> usize {
        self.volume as usize
    }
}
