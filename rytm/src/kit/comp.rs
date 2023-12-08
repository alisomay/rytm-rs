use super::types::{FxCompAttack, FxCompRatio, FxCompRelease, FxCompSideChainEq};
use crate::error::{ConversionError, ParameterError, RytmError};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_kit_t;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FxCompressor {
    threshold: u8,
    attack: FxCompAttack,
    release: FxCompRelease,
    ratio: FxCompRatio,
    seq: FxCompSideChainEq,
    gain: u8,
    mix: u8,
    volume: u8,
}

impl Default for FxCompressor {
    fn default() -> Self {
        Self {
            threshold: 0,
            attack: FxCompAttack::default(),
            release: FxCompRelease::default(),
            ratio: FxCompRatio::default(),
            seq: FxCompSideChainEq::default(),
            gain: 0,
            mix: 0,
            volume: 0,
        }
    }
}

impl TryFrom<&ar_kit_t> for FxCompressor {
    type Error = ConversionError;
    fn try_from(raw_kit: &ar_kit_t) -> Result<Self, Self::Error> {
        Ok(Self {
            threshold: raw_kit.fx_comp_threshold,
            attack: raw_kit.fx_comp_attack.try_into()?,
            release: raw_kit.fx_comp_release.try_into()?,
            ratio: raw_kit.fx_comp_ratio.try_into()?,
            seq: raw_kit.fx_comp_seq.try_into()?,
            gain: raw_kit.fx_comp_gain,
            mix: raw_kit.fx_comp_mix,
            volume: raw_kit.fx_comp_volume,
        })
    }
}

impl FxCompressor {
    pub(crate) fn apply_to_raw_kit(&self, raw_kit: &mut ar_kit_t) {
        raw_kit.fx_comp_threshold = self.threshold;
        raw_kit.fx_comp_attack = self.attack as u8;
        raw_kit.fx_comp_release = self.release as u8;
        raw_kit.fx_comp_ratio = self.ratio as u8;
        raw_kit.fx_comp_seq = self.seq as u8;
        raw_kit.fx_comp_gain = self.gain;
        raw_kit.fx_comp_mix = self.mix;
        raw_kit.fx_comp_volume = self.volume;
    }

    /// Sets the threshold of the compressor.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "threshold:0..=127")]
    pub fn set_threshold(&mut self, threshold: u8) -> Result<(), RytmError> {
        self.threshold = threshold;
        Ok(())
    }

    /// Sets the attack of the compressor.
    pub fn set_attack(&mut self, attack: FxCompAttack) -> Result<(), RytmError> {
        self.attack = attack;
        Ok(())
    }

    /// Sets the release of the compressor.
    pub fn set_release(&mut self, release: FxCompRelease) -> Result<(), RytmError> {
        self.release = release;
        Ok(())
    }

    /// Sets the ratio of the compressor.
    pub fn set_ratio(&mut self, ratio: FxCompRatio) -> Result<(), RytmError> {
        self.ratio = ratio;
        Ok(())
    }

    /// Sets the side chain eq of the compressor.
    pub fn set_side_chain_eq(&mut self, seq: FxCompSideChainEq) -> Result<(), RytmError> {
        self.seq = seq;
        Ok(())
    }

    /// Sets the gain of the compressor.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "gain:0..=127")]
    pub fn set_gain(&mut self, gain: u8) -> Result<(), RytmError> {
        self.gain = gain;
        Ok(())
    }

    /// Sets the mix of the compressor.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "mix:0..=127")]
    pub fn set_mix(&mut self, mix: u8) -> Result<(), RytmError> {
        self.mix = mix;
        Ok(())
    }

    /// Sets the volume of the compressor.
    ///
    /// Range: `0..=127`
    #[parameter_range(range = "volume:0..=127")]
    pub fn set_volume(&mut self, volume: u8) -> Result<(), RytmError> {
        self.volume = volume;
        Ok(())
    }

    /// Returns the threshold of the compressor.
    ///
    /// Range: `0..=127`
    pub fn threshold(&self) -> usize {
        self.threshold as usize
    }

    /// Returns the attack of the compressor.
    pub fn attack(&self) -> &FxCompAttack {
        &self.attack
    }

    /// Returns the release of the compressor.
    pub fn release(&self) -> &FxCompRelease {
        &self.release
    }

    /// Returns the ratio of the compressor.
    pub fn ratio(&self) -> &FxCompRatio {
        &self.ratio
    }

    /// Returns the side chain eq of the compressor.
    pub fn side_chain_eq(&self) -> &FxCompSideChainEq {
        &self.seq
    }

    /// Returns the gain of the compressor.
    ///
    /// Range: `0..=127`
    pub fn gain(&self) -> usize {
        self.gain as usize
    }

    /// Returns the mix of the compressor.
    ///
    /// Range: `0..=127`
    pub fn mix(&self) -> usize {
        self.mix as usize
    }

    /// Returns the volume of the compressor.
    ///
    /// Range: `0..=127`
    pub fn volume(&self) -> usize {
        self.volume as usize
    }
}
