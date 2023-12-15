use crate::error::ConversionError;
use rytm_sys::ar_global_t;

/// Sequencer configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequencerConfig {
    kit_reload_on_chg: bool,
    quantize_live_rec: bool,
    auto_trk_switch: bool,
}

impl Default for SequencerConfig {
    fn default() -> Self {
        Self {
            kit_reload_on_chg: false,
            quantize_live_rec: false,
            auto_trk_switch: true,
        }
    }
}

impl TryFrom<&ar_global_t> for SequencerConfig {
    type Error = ConversionError;
    fn try_from(raw_global: &ar_global_t) -> Result<Self, Self::Error> {
        Ok(Self {
            kit_reload_on_chg: raw_global.kit_reload_on_chg != 0,
            quantize_live_rec: raw_global.quantize_live_rec != 0,
            auto_trk_switch: raw_global.auto_trk_switch != 0,
        })
    }
}

impl SequencerConfig {
    pub(crate) fn apply_to_raw_global(self, raw_global: &mut ar_global_t) {
        raw_global.kit_reload_on_chg = self.kit_reload_on_chg as u8;
        raw_global.quantize_live_rec = self.quantize_live_rec as u8;
        raw_global.auto_trk_switch = self.auto_trk_switch as u8;
    }

    pub fn set_kit_reload_on_chg(&mut self, kit_reload_on_chg: bool) {
        self.kit_reload_on_chg = kit_reload_on_chg;
    }

    pub fn set_quantize_live_rec(&mut self, quantize_live_rec: bool) {
        self.quantize_live_rec = quantize_live_rec;
    }

    pub fn set_auto_trk_switch(&mut self, auto_trk_switch: bool) {
        self.auto_trk_switch = auto_trk_switch;
    }

    pub const fn kit_reload_on_chg(&self) -> bool {
        self.kit_reload_on_chg
    }

    pub const fn quantize_live_rec(&self) -> bool {
        self.quantize_live_rec
    }

    pub const fn auto_trk_switch(&self) -> bool {
        self.auto_trk_switch
    }
}
