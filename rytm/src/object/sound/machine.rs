// TODO:

use crate::error::{ParameterError, RytmError};
use rytm_rs_macro::parameter_range;

pub struct BdHard {
    lev: u16,
    tun: u16,
    dec: u16,
    hld: u16,
    swt: u16,
    snp: u16,
    wav: u16,
    tic: u16,
}

impl BdHard {
    /// Sets the level of the `BdHard` machine.
    ///
    /// Range: 0..=127
    #[parameter_range(range = "lev:0..=127")]
    pub fn set_lev(&mut self, lev: usize) -> Result<(), RytmError> {
        // map 0..=127 to 0..=32512
        let a = 0.0;
        let b = 127.0;
        let c = 0.0;
        let d = 32512.0;
        self.lev = ((lev as f32 - a) * (d - c) / (b - a) + c) as u16;
        Ok(())
    }

    /// Sets the tuning of the `BdHard` machine.
    #[parameter_range(range = "tun:-32.0..=32.0")]
    pub fn set_tun(&mut self, tun: f32) -> Result<(), RytmError> {
        // map -32.0..=32.0 to 8162..=24576
        let a = -32.0;
        let b = 32.0;
        let c = 8162.0;
        let d = 24576.0;
        self.tun = ((tun - a) * (d - c) / (b - a) + c) as u16;
        Ok(())
    }

    /// Sets the decay of the `BdHard` machine.

    #[parameter_range(range = "dec:0..=127")]
    pub fn set_dec(&mut self, dec: usize) -> Result<(), RytmError> {
        // map 0..=127 to 0..=32512
        let a = 0.0;
        let b = 127.0;
        let c = 0.0;
        let d = 32512.0;
        self.dec = ((dec as f32 - a) * (d - c) / (b - a) + c) as u16;
        Ok(())
    }

    /// Sets the hold of the `BdHard` machine.
    #[parameter_range(range = "hld:0..=127")]
    pub fn set_hld(&mut self, hld: usize) -> Result<(), RytmError> {
        // map 0..=127 to 0..=32512
        let a = 0.0;
        let b = 127.0;
        let c = 0.0;
        let d = 32512.0;
        self.hld = ((hld as f32 - a) * (d - c) / (b - a) + c) as u16;
        Ok(())
    }

    /// Sets the switch of the `BdHard` machine.
    #[parameter_range(range = "swt:0..=127")]
    pub fn set_swt(&mut self, swt: usize) -> Result<(), RytmError> {
        // map 0..=127 to 0..=32512
        let a = 0.0;
        let b = 127.0;
        let c = 0.0;
        let d = 32512.0;
        self.swt = ((swt as f32 - a) * (d - c) / (b - a) + c) as u16;
        Ok(())
    }

    /// Sets the snap of the `BdHard` machine.

    #[parameter_range(range = "snp:0..=127")]
    pub fn set_snp(&mut self, snp: usize) -> Result<(), RytmError> {
        // map 0..=127 to 0..=32512
        let a = 0.0;
        let b = 127.0;
        let c = 0.0;
        let d = 32512.0;
        self.snp = ((snp as f32 - a) * (d - c) / (b - a) + c) as u16;
        Ok(())
    }

    /// Sets the wave of the `BdHard` machine.
    #[parameter_range(range = "wav:0..=2")]
    pub fn set_wav(&mut self, wav: usize) -> Result<(), RytmError> {
        self.wav = (wav * 256) as u16;
        Ok(())
    }

    /// Sets the tick of the `BdHard` machine.
    #[parameter_range(range = "tic:0..=127")]
    pub fn set_tic(&mut self, tic: usize) -> Result<(), RytmError> {
        // map 0..=127 to 0..=32512
        let a = 0.0;
        let b = 127.0;
        let c = 0.0;
        let d = 32512.0;
        self.tic = ((tic as f32 - a) * (d - c) / (b - a) + c) as u16;
        Ok(())
    }
}

// BdHard,
// BdClassic,
// SdHard,
// SdClassic,
// RsHard,
// RsClassic,
// CpClassic,
// BtClassic,
// XtClassic,
// ChClassic,
// OhClassic,
// CyClassic,
// CbClassic,
// BdFm,
// SdFm,
// UtNoise,
// UtImpulse,
// ChMetallic,
// OhMetallic,
// CyMetallic,
// CbMetallic,
// BdPlastic,
// BdSilky,
// SdNatural,
// HhBasic,
// CyRide,
// BdSharp,
// Disable,
// SyDualVco,
// SyChip,
// BdAcoustic,
// SdAcoustic,
// SyRaw,
// HhLab,
// #[default]
// Unset,
