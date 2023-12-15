// All casts in this file are intended or safe within the context of this library.
//
// One can change `allow` to `warn` to review them if necessary.
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
// TODO: Re-check if bpm related casts are accurate.

pub mod types;
pub(crate) mod unknown;

use self::types::{
    FxParameterMenuItem, ParameterMenuItem, PatternMode, SampleRecorderRecordingLength,
    SampleRecorderSource, SequencerMode,
};
use crate::util::{assemble_u32_from_u8_array, break_u32_into_u8_array};
use crate::AnySysexType;
use crate::{
    error::{ParameterError, RytmError, SysexConversionError},
    impl_sysex_compatible,
    sysex::{SysexCompatible, SysexMeta, SysexType, SETTINGS_SYSEX_SIZE},
};
use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::{ar_settings_raw_to_syx, ar_settings_t, ar_sysex_meta_t};
use unknown::SettingsUnknown;

impl_sysex_compatible!(
    Settings,
    ar_settings_t,
    ar_settings_raw_to_syx,
    SysexType::Settings,
    SETTINGS_SYSEX_SIZE
);

/// Represents settings in the analog rytm.
///
/// It does not map identically to the relevant structure in the firmware.
///
/// Settings may not be a familiar structure for all, to understand what kind of settings are available, please check the methods of this struct.
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Settings {
    #[derivative(Debug = "ignore")]
    sysex_meta: SysexMeta,
    /// Version of the settings structure.
    version: u32,

    bpm_project: f32,

    selected_track: u8,
    selected_parameter_menu_item: ParameterMenuItem,
    selected_fx_menu_item: FxParameterMenuItem,
    selected_page: u8,
    selected_mode: SequencerMode,
    selected_pattern_mode: PatternMode,

    mute_flags: u16,

    fixed_velocity_enable: bool,
    fixed_velocity_amount: u8,

    sample_recorder_src: SampleRecorderSource,
    sample_recorder_thr: u8,
    sample_recorder_monitor_enable: bool,
    sample_recorder_rlen: SampleRecorderRecordingLength,

    #[derivative(Debug = "ignore")]
    __unknown: SettingsUnknown,
}

impl From<&Settings> for ar_settings_t {
    fn from(settings: &Settings) -> Self {
        let bpm = (settings.bpm_project * 120.0) as u16;
        let bpm_msb = (bpm >> 8) as u8;
        let bpm_lsb = bpm as u8;

        let track_mute_msb = (settings.mute_flags >> 8) as u8;
        let track_mute_lsb = settings.mute_flags as u8;

        let mut raw_settings = Self {
            version: break_u32_into_u8_array(settings.version),
            bpm_msb,
            bpm_lsb,
            selected_track: settings.selected_track,
            _selected_track_duplicate: settings.selected_track,
            selected_trig_or_parameter_menu: settings.selected_parameter_menu_item.into(),
            selected_fx_menu: settings.selected_fx_menu_item.into(),
            selected_page: settings.selected_page,

            track_mute_msb,
            track_mute_lsb,

            selected_mode: settings.selected_mode.into(),
            selected_pattern_transition_mode: settings.selected_pattern_mode.into(),

            fixed_velocity_enable: settings.fixed_velocity_enable.into(),
            fixed_velocity_amount: settings.fixed_velocity_amount,

            sample_recorder_src: settings.sample_recorder_src.into(),
            sample_recorder_thr: settings.sample_recorder_thr,
            sample_recorder_monitor: settings.sample_recorder_monitor_enable.into(),

            sample_recorder_rlen: settings.sample_recorder_rlen.into(),

            ..Default::default()
        };

        settings.__unknown.apply_to_raw_settings(&mut raw_settings);

        raw_settings
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sysex_meta: SysexMeta::default_for_settings(None),
            version: 3,
            bpm_project: 120.0,
            selected_track: 0,

            selected_parameter_menu_item: ParameterMenuItem::default(),
            selected_fx_menu_item: FxParameterMenuItem::default(),
            selected_page: 0,

            mute_flags: 0,

            selected_mode: SequencerMode::default(),
            selected_pattern_mode: PatternMode::default(),

            fixed_velocity_enable: false,
            fixed_velocity_amount: 100,
            sample_recorder_src: SampleRecorderSource::default(),
            sample_recorder_thr: 0,
            sample_recorder_monitor_enable: false,

            sample_recorder_rlen: SampleRecorderRecordingLength::default(),

            __unknown: SettingsUnknown::default(),
        }
    }
}

impl Settings {
    pub(crate) fn as_raw_parts(&self) -> (SysexMeta, ar_settings_t) {
        (self.sysex_meta, self.into())
    }

    pub(crate) fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_settings: &ar_settings_t,
    ) -> Result<Self, RytmError> {
        let bpm_project = (raw_settings.bpm_msb as u16) << 8 | raw_settings.bpm_lsb as u16;
        let bpm_project = bpm_project as f32 / 120.0;

        let mute_flags =
            (raw_settings.track_mute_msb as u16) << 8 | raw_settings.track_mute_lsb as u16;

        Ok(Self {
            sysex_meta,
            version: assemble_u32_from_u8_array(&raw_settings.version),
            bpm_project,
            selected_track: raw_settings.selected_track,

            selected_parameter_menu_item: raw_settings
                .selected_trig_or_parameter_menu
                .try_into()?,
            selected_fx_menu_item: raw_settings.selected_fx_menu.try_into()?,
            selected_page: raw_settings.selected_page,

            mute_flags,

            selected_mode: raw_settings.selected_mode.try_into()?,
            selected_pattern_mode: raw_settings.selected_pattern_transition_mode.try_into()?,

            fixed_velocity_enable: raw_settings.fixed_velocity_enable != 0,
            fixed_velocity_amount: raw_settings.fixed_velocity_amount,

            sample_recorder_src: raw_settings.sample_recorder_src.try_into()?,
            sample_recorder_thr: raw_settings.sample_recorder_thr,
            sample_recorder_monitor_enable: raw_settings.sample_recorder_monitor != 0,
            sample_recorder_rlen: raw_settings.sample_recorder_rlen.try_into()?,

            __unknown: raw_settings.into(),
        })
    }

    /// Sets the BPM for the entire project.
    ///
    /// Range `30.0..=300.0`
    ///
    /// This is only effective when project level bpm is enabled.
    #[parameter_range(range = "bpm:30.0..=300.0")]
    pub fn set_bpm(&mut self, bpm: f32) -> Result<(), RytmError> {
        self.bpm_project = bpm;
        Ok(())
    }

    /// Sets the selected track.
    ///
    /// Range `0..=11`
    #[parameter_range(range = "track_index:0..=11")]
    pub fn set_selected_track(&mut self, track_index: usize) -> Result<(), RytmError> {
        self.selected_track = track_index as u8;
        Ok(())
    }

    /// Sets the selected parameter menu item.
    ///
    /// The six sequential square buttons on the right side of the Analog Rytm MKII.
    pub fn set_selected_parameter_menu_item(&mut self, parameter_menu_item: ParameterMenuItem) {
        self.selected_parameter_menu_item = parameter_menu_item;
    }

    /// Sets the selected fx menu item.
    ///
    /// The six sequential square buttons on the right side of the Analog Rytm MKII.
    ///
    /// The fx menu is only available when the FX button is pressed.
    pub fn set_selected_fx_menu_item(&mut self, fx_menu_item: FxParameterMenuItem) {
        self.selected_fx_menu_item = fx_menu_item;
    }

    /// Sets the selected page.
    ///
    /// The `[PAGE]` button on the Analog Rytm MKII.
    ///
    /// Range `0..=3`
    #[parameter_range(range = "page_index:0..=3")]
    pub fn set_selected_page(&mut self, page_index: usize) -> Result<(), RytmError> {
        self.selected_page = page_index as u8;
        Ok(())
    }

    /// Sets the selected sequencer mode.
    pub fn set_selected_mode(&mut self, sequencer_mode: SequencerMode) {
        self.selected_mode = sequencer_mode;
    }

    /// Sets the selected pattern mode.  
    pub fn set_selected_pattern_mode(&mut self, pattern_mode: PatternMode) {
        self.selected_pattern_mode = pattern_mode;
    }

    /// Sets the mute flags for sounds.
    ///
    /// Range `0..=0b1111_1111_1111`
    #[parameter_range(range = "mute_flags:0..=4095")]
    pub fn set_mute_flags(&mut self, mute_flags: u16) -> Result<(), RytmError> {
        self.mute_flags = mute_flags;
        Ok(())
    }

    /// Mutes a sound by sound index.
    ///
    /// Range `0..=11`
    #[parameter_range(range = "sound_index:0..=11")]
    pub fn mute_sound(&mut self, sound_index: usize) -> Result<(), RytmError> {
        self.mute_flags |= 1 << sound_index;
        Ok(())
    }

    /// Unmute a sound by sound index.
    ///
    /// Range `0..=11`
    #[parameter_range(range = "sound_index:0..=11")]
    pub fn unmute_sound(&mut self, sound_index: usize) -> Result<(), RytmError> {
        self.mute_flags &= !(1 << sound_index);
        Ok(())
    }

    /// Toggles the mute state of a sound by sound index.
    ///
    /// Range `0..=11`
    #[parameter_range(range = "sound_index:0..=11")]
    pub fn toggle_mute_sound(&mut self, sound_index: usize) -> Result<(), RytmError> {
        self.mute_flags ^= 1 << sound_index;
        Ok(())
    }

    /// Mutes a range of sounds.
    ///
    /// Maximum range `0..=11`
    ///
    /// # Errors
    ///
    /// Returns an error if the range is out of bounds.
    pub fn mute_range_of_sounds(&mut self, range: std::ops::Range<usize>) -> Result<(), RytmError> {
        if range.end > 11 {
            return Err(RytmError::Parameter(ParameterError::Range {
                value: format!("{range:?}",),
                parameter_name: "range".to_string(),
            }));
        }

        for sound_index in range {
            self.mute_sound(sound_index)?;
        }

        Ok(())
    }

    /// Unmute a range of sounds.
    ///
    /// Maximum range `0..=11`
    ///
    /// # Errors
    ///
    /// Returns an error if the range is out of bounds.
    pub fn unmute_range_of_sounds(
        &mut self,
        range: std::ops::Range<usize>,
    ) -> Result<(), RytmError> {
        for sound_index in range {
            self.unmute_sound(sound_index)?;
        }
        Ok(())
    }

    /// Toggles the mute state of a range of sounds.
    ///
    /// Maximum range `0..=11`
    ///
    /// # Errors
    ///
    /// Returns an error if the range is out of bounds.
    pub fn toggle_mute_range_of_sounds(
        &mut self,
        range: std::ops::Range<usize>,
    ) -> Result<(), RytmError> {
        for sound_index in range {
            self.toggle_mute_sound(sound_index)?;
        }
        Ok(())
    }

    /// Sets the fixed velocity enable state.
    pub fn set_fixed_velocity_enable(&mut self, fixed_velocity_enable: bool) {
        self.fixed_velocity_enable = fixed_velocity_enable;
    }

    /// Sets the fixed velocity amount.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "fixed_velocity_amount:0..=127")]
    pub fn set_fixed_velocity_amount(
        &mut self,
        fixed_velocity_amount: usize,
    ) -> Result<(), RytmError> {
        self.fixed_velocity_amount = fixed_velocity_amount as u8;
        Ok(())
    }

    /// Sets the sample recorder source.
    pub fn set_sample_recorder_source(&mut self, sample_recorder_source: SampleRecorderSource) {
        self.sample_recorder_src = sample_recorder_source;
    }

    /// Sets the sample recorder threshold.
    ///
    /// Range `0..=127`
    #[parameter_range(range = "sample_recorder_threshold:0..=127")]
    pub fn set_sample_recorder_threshold(
        &mut self,
        sample_recorder_threshold: usize,
    ) -> Result<(), RytmError> {
        self.sample_recorder_thr = sample_recorder_threshold as u8;
        Ok(())
    }

    /// Sets the sample recorder monitor state.
    pub fn set_sample_recorder_monitor_enable(&mut self, set_sample_recorder_monitor_enable: bool) {
        self.sample_recorder_monitor_enable = set_sample_recorder_monitor_enable;
    }

    /// Sets the sample recorder recording length.
    pub fn set_sample_recorder_recording_length(
        &mut self,
        sample_recorder_recording_length: SampleRecorderRecordingLength,
    ) {
        self.sample_recorder_rlen = sample_recorder_recording_length;
    }

    /// Returns the BPM for the entire project.
    ///
    /// Range `30.0..=300.0`
    ///
    /// This is only effective when project level bpm is enabled.
    pub const fn bpm(&self) -> f32 {
        self.bpm_project
    }

    /// Returns the selected track.
    ///
    /// Range `0..=11`
    pub const fn selected_track(&self) -> usize {
        self.selected_track as usize
    }

    /// Returns the selected parameter menu item.
    pub const fn selected_parameter_menu_item(&self) -> ParameterMenuItem {
        self.selected_parameter_menu_item
    }

    /// Returns the selected fx menu item.
    pub const fn selected_fx_menu_item(&self) -> FxParameterMenuItem {
        self.selected_fx_menu_item
    }

    /// Returns the selected page.
    ///
    /// Range `0..=3`
    pub const fn selected_page(&self) -> usize {
        self.selected_page as usize
    }

    /// Returns the selected sequencer mode.
    pub const fn selected_mode(&self) -> SequencerMode {
        self.selected_mode
    }

    /// Returns the selected pattern mode.
    pub const fn selected_pattern_mode(&self) -> PatternMode {
        self.selected_pattern_mode
    }

    /// Returns the raw mute flags for sounds.
    pub const fn raw_mute_flags(&self) -> u16 {
        self.mute_flags
    }

    /// Returns the collection of muted sound indexes.
    pub fn muted_sound_indexes(&self) -> Vec<usize> {
        let mut muted_track_numbers = Vec::new();
        for sound_index in 0..=11 {
            if self.mute_flags & (1 << sound_index) != 0 {
                muted_track_numbers.push(sound_index);
            }
        }
        muted_track_numbers
    }

    /// Returns the collection of unmuted sound indexes.
    pub fn unmuted_sound_indexes(&self) -> Vec<usize> {
        let mut unmuted_sound_indexes = Vec::new();
        for sound_index in 0..=11 {
            if self.mute_flags & (1 << sound_index) == 0 {
                unmuted_sound_indexes.push(sound_index);
            }
        }
        unmuted_sound_indexes
    }

    /// Returns the fixed velocity enable state.
    pub const fn fixed_velocity_enabled(&self) -> bool {
        self.fixed_velocity_enable
    }

    /// Returns the fixed velocity amount.
    ///
    /// Range `0..=127`
    pub const fn fixed_velocity_amount(&self) -> usize {
        self.fixed_velocity_amount as usize
    }

    /// Returns the sample recorder source.
    pub const fn sample_recorder_source(&self) -> SampleRecorderSource {
        self.sample_recorder_src
    }

    /// Returns the sample recorder threshold.
    ///
    /// Range `0..=127`
    pub const fn sample_recorder_threshold(&self) -> usize {
        self.sample_recorder_thr as usize
    }

    /// Returns the sample recorder monitor state.
    pub const fn sample_recorder_monitor_enabled(&self) -> bool {
        self.sample_recorder_monitor_enable
    }

    /// Returns the sample recorder recording length.
    pub const fn sample_recorder_recording_length(&self) -> SampleRecorderRecordingLength {
        self.sample_recorder_rlen
    }

    /// Returns the version of the settings structure.
    pub const fn structure_version(&self) -> u32 {
        self.version
    }
}
