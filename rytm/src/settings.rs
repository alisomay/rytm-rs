pub mod types;

use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_settings_t;

use crate::error::{ParameterError, RytmError};
use crate::sysex::SysexMeta;

use self::types::{
    FxParameterMenuItem, ParameterMenuItem, PatternMode, SampleRecorderRecordingLength,
    SampleRecorderSource, SequencerMode,
};

#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Settings {
    sysex_meta: SysexMeta,
    /// Version of the kit structure.
    version: u32,

    bpm_project: f64,

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

    // The rest is not figured out:

    // Always the duplicate of selected_track
    #[derivative(Debug = "ignore")]
    __selected_track_duplicate: u8,
    // Always 0x00
    #[derivative(Debug = "ignore")]
    __unknown_0x000b: u8,
    // @0x000E..0x0014 All zeros.
    #[derivative(Debug = "ignore")]
    __unknown0x000e_0x0014: [u8; 7],
    //  @0x0017..0x0019 All zeros.
    #[derivative(Debug = "ignore")]
    __unknown0x0017_0x0019: [u8; 3],
    // The response continues with the repeating 16 byte pattern of 0xFF_FF_FF_FF 0x00_00_00_00 0x00_00_00_00 0x00_00_00_00
    // The repeating pattern repeats 128 times. Total length of 2048 bytes.
    #[derivative(Debug = "ignore")]
    __unknown0x001f: [u8; 16 * 128],
    // @0x081F Always 0x01
    #[derivative(Debug = "ignore")]
    __unknown0x081f: u8,
    // @0x0821..0x0826 All zeros.
    #[derivative(Debug = "ignore")]
    __unknown0x0821_0x0826: [u8; 6],
}

impl From<&Settings> for ar_settings_t {
    fn from(settings: &Settings) -> Self {
        let mut version = [0; 4];
        version[0] = (settings.version >> 24) as u8;
        version[1] = (settings.version >> 16) as u8;
        version[2] = (settings.version >> 8) as u8;
        version[3] = settings.version as u8;

        let bpm = (settings.bpm_project * 120.0) as u16;
        let bpm_msb = (bpm >> 8) as u8;
        let bpm_lsb = bpm as u8;

        let track_mute_msb = (settings.mute_flags >> 8) as u8;
        let track_mute_lsb = settings.mute_flags as u8;

        Self {
            version,
            bpm_msb,
            bpm_lsb,
            selected_track: settings.selected_track,
            _selected_track_duplicate: settings.selected_track,
            selected_trig_or_parameter_menu: settings.selected_parameter_menu_item.into(),
            selected_fx_menu: settings.selected_fx_menu_item.into(),
            selected_page: settings.selected_page,

            __unknown_0x000B: settings.__unknown_0x000b,

            track_mute_msb,
            track_mute_lsb,

            __unknown0x000E_0x0014: settings.__unknown0x000e_0x0014,

            selected_mode: settings.selected_mode.into(),
            selected_pattern_transition_mode: settings.selected_pattern_mode.into(),

            __unknown0x0017_0x0019: settings.__unknown0x0017_0x0019,

            fixed_velocity_enable: settings.fixed_velocity_enable.into(),
            fixed_velocity_amount: settings.fixed_velocity_amount,

            sample_recorder_src: settings.sample_recorder_src.into(),
            sample_recorder_thr: settings.sample_recorder_thr,
            sample_recorder_monitor: settings.sample_recorder_monitor_enable.into(),

            __unknown0x001F: settings.__unknown0x001f,
            __unknown0x081F: settings.__unknown0x081f,

            sample_recorder_rlen: settings.sample_recorder_rlen.into(),

            __unknown0x0821_0x0826: settings.__unknown0x0821_0x0826,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sysex_meta: SysexMeta::default_for_settings(None),
            version: 3,
            bpm_project: 120.0,
            selected_track: 0,
            __selected_track_duplicate: 0,
            selected_parameter_menu_item: ParameterMenuItem::default(),
            selected_fx_menu_item: FxParameterMenuItem::default(),
            selected_page: 0,
            __unknown_0x000b: 0,
            mute_flags: 0,
            __unknown0x000e_0x0014: [0; 7],
            selected_mode: SequencerMode::default(),
            selected_pattern_mode: PatternMode::default(),
            __unknown0x0017_0x0019: [0; 3],
            fixed_velocity_enable: false,
            fixed_velocity_amount: 0,
            sample_recorder_src: SampleRecorderSource::default(),
            sample_recorder_thr: 0,
            sample_recorder_monitor_enable: false,
            __unknown0x001f: [0; 16 * 128],
            __unknown0x081f: 0,
            sample_recorder_rlen: SampleRecorderRecordingLength::default(),
            __unknown0x0821_0x0826: [0; 6],
        }
    }
}

impl Settings {
    pub fn to_raw_parts(&self) -> (SysexMeta, ar_settings_t) {
        (self.sysex_meta, self.into())
    }

    pub fn try_from_raw(
        sysex_meta: SysexMeta,
        raw_settings: &ar_settings_t,
    ) -> Result<Self, RytmError> {
        let version = ((raw_settings.version[0] as u32) << 24)
            | ((raw_settings.version[1] as u32) << 16)
            | ((raw_settings.version[2] as u32) << 8)
            | (raw_settings.version[3] as u32);

        let bpm_project = (raw_settings.bpm_msb as u16) << 8 | raw_settings.bpm_lsb as u16;
        let bpm_project = bpm_project as f64 / 120.0;

        let mute_flags =
            (raw_settings.track_mute_msb as u16) << 8 | raw_settings.track_mute_lsb as u16;

        Ok(Self {
            sysex_meta,
            version,
            bpm_project,
            selected_track: raw_settings.selected_track,
            __selected_track_duplicate: raw_settings._selected_track_duplicate,
            selected_parameter_menu_item: raw_settings
                .selected_trig_or_parameter_menu
                .try_into()?,
            selected_fx_menu_item: raw_settings.selected_fx_menu.try_into()?,
            selected_page: raw_settings.selected_page,

            __unknown_0x000b: raw_settings.__unknown_0x000B,

            mute_flags,

            __unknown0x000e_0x0014: raw_settings.__unknown0x000E_0x0014,

            selected_mode: raw_settings.selected_mode.try_into()?,
            selected_pattern_mode: raw_settings.selected_pattern_transition_mode.try_into()?,

            __unknown0x0017_0x0019: raw_settings.__unknown0x0017_0x0019,

            fixed_velocity_enable: raw_settings.fixed_velocity_enable != 0,
            fixed_velocity_amount: raw_settings.fixed_velocity_amount,

            sample_recorder_src: raw_settings.sample_recorder_src.try_into()?,
            sample_recorder_thr: raw_settings.sample_recorder_thr,
            sample_recorder_monitor_enable: raw_settings.sample_recorder_monitor != 0,

            __unknown0x001f: raw_settings.__unknown0x001F,
            __unknown0x081f: raw_settings.__unknown0x081F,

            sample_recorder_rlen: raw_settings.sample_recorder_rlen.try_into()?,

            __unknown0x0821_0x0826: raw_settings.__unknown0x0821_0x0826,
        })
    }

    /// Sets the BPM for the entire project.
    ///
    /// Range `30.0..=300.0`
    ///
    /// This is only effective when project level bpm is enabled.
    #[parameter_range(range = "bpm:30.0..=300.0")]
    pub fn set_bpm(&mut self, bpm: f64) -> Result<(), RytmError> {
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
    pub fn mute_range_of_sounds(&mut self, range: std::ops::Range<usize>) -> Result<(), RytmError> {
        for sound_index in range {
            self.mute_sound(sound_index)?;
        }
        Ok(())
    }

    /// Unmute a range of sounds.
    ///
    /// Maximum range `0..=11`
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
    pub fn bpm(&self) -> f64 {
        self.bpm_project
    }

    /// Returns the selected track.
    ///
    /// Range `0..=11`
    pub fn selected_track(&self) -> usize {
        self.selected_track as usize
    }

    /// Returns the selected parameter menu item.
    pub fn selected_parameter_menu_item(&self) -> ParameterMenuItem {
        self.selected_parameter_menu_item
    }

    /// Returns the selected fx menu item.
    pub fn selected_fx_menu_item(&self) -> FxParameterMenuItem {
        self.selected_fx_menu_item
    }

    /// Returns the selected page.
    ///
    /// Range `0..=3`
    pub fn selected_page(&self) -> usize {
        self.selected_page as usize
    }

    /// Returns the selected sequencer mode.
    pub fn selected_mode(&self) -> SequencerMode {
        self.selected_mode
    }

    /// Returns the selected pattern mode.
    pub fn selected_pattern_mode(&self) -> PatternMode {
        self.selected_pattern_mode
    }

    /// Returns the raw mute flags for sounds.
    pub fn raw_mute_flags(&self) -> u16 {
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
    pub fn fixed_velocity_enabled(&self) -> bool {
        self.fixed_velocity_enable
    }

    /// Returns the fixed velocity amount.
    ///
    /// Range `0..=127`
    pub fn fixed_velocity_amount(&self) -> usize {
        self.fixed_velocity_amount as usize
    }

    /// Returns the sample recorder source.
    pub fn sample_recorder_source(&self) -> SampleRecorderSource {
        self.sample_recorder_src
    }

    /// Returns the sample recorder threshold.
    ///
    /// Range `0..=127`
    pub fn sample_recorder_threshold(&self) -> usize {
        self.sample_recorder_thr as usize
    }

    /// Returns the sample recorder monitor state.
    pub fn sample_recorder_monitor_enabled(&self) -> bool {
        self.sample_recorder_monitor_enable
    }

    /// Returns the sample recorder recording length.
    pub fn sample_recorder_recording_length(&self) -> SampleRecorderRecordingLength {
        self.sample_recorder_rlen
    }

    /// Returns the version of the pattern structure.
    pub fn structure_version(&self) -> u32 {
        self.version
    }
}
