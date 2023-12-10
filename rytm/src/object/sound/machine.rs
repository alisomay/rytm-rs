mod bd_acoustic;
mod bd_classic;
mod bd_fm;
mod bd_hard;
mod bd_plastic;
mod bd_sharp;
mod bd_silky;
mod bt_classic;
mod cb_classic;
mod cb_metallic;
mod ch_classic;
mod ch_metallic;
mod cp_classic;
mod cy_classic;
mod cy_metallic;
mod cy_ride;
mod hh_basic;
mod hh_lab;
mod oh_classic;
mod oh_metallic;
mod rs_classic;
mod rs_hard;
mod sd_acoustic;
mod sd_classic;
mod sd_fm;
mod sd_hard;
mod sd_natural;
mod sy_chip;
mod sy_dual_vco;
mod sy_raw;
mod ut_impulse;
mod ut_noise;
mod xt_classic;

use super::types::MachineType;
use crate::error::{ParameterError, RytmError};
pub use bd_acoustic::*;
pub use bd_classic::*;
pub use bd_fm::*;
pub use bd_hard::*;
pub use bd_plastic::*;
pub use bd_sharp::*;
pub use bd_silky::*;
pub use bt_classic::*;
pub use cb_classic::*;
pub use cb_metallic::*;
pub use ch_classic::*;
pub use ch_metallic::*;
pub use cp_classic::*;
pub use cy_classic::*;
pub use cy_metallic::*;
pub use cy_ride::*;
pub use hh_basic::*;
pub use hh_lab::*;
pub use oh_classic::*;
pub use oh_metallic::*;
pub use rs_classic::*;
pub use rs_hard::*;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_sound_t;
pub use sd_acoustic::*;
pub use sd_classic::*;
pub use sd_fm::*;
pub use sd_hard::*;
pub use sd_natural::*;
pub use sy_chip::*;
pub use sy_dual_vco::*;
pub use sy_raw::*;
pub use ut_impulse::*;
pub use ut_noise::*;
pub use xt_classic::*;

/// The machine parameters of a sound.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MachineParameters {
    BdHard(BdHardParameters),
    BdClassic(BdClassicParameters),
    SdHard(SdHardParameters),
    SdClassic(SdClassicParameters),
    RsHard(RsHardParameters),
    RsClassic(RsClassicParameters),
    CpClassic(CpClassicParameters),
    BtClassic(BtClassicParameters),
    XtClassic(XtClassicParameters),
    ChClassic(ChClassicParameters),
    OhClassic(OhClassicParameters),
    CyClassic(CyClassicParameters),
    CbClassic(CbClassicParameters),
    BdFm(BdFmParameters),
    SdFm(SdFmParameters),
    UtNoise(UtNoiseParameters),
    UtImpulse(UtImpulseParameters),
    ChMetallic(ChMetallicParameters),
    OhMetallic(OhMetallicParameters),
    CyMetallic(CyMetallicParameters),
    CbMetallic(CbMetallicParameters),
    BdPlastic(BdPlasticParameters),
    BdSilky(BdSilkyParameters),
    SdNatural(SdNaturalParameters),
    HhBasic(HhBasicParameters),
    CyRide(CyRideParameters),
    BdSharp(BdSharpParameters),
    Disable,
    SyDualVco(SyDualVcoParameters),
    SyChip(SyChipParameters),
    BdAcoustic(BdAcousticParameters),
    SdAcoustic(SdAcousticParameters),
    SyRaw(SyRawParameters),
    HhLab(HhLabParameters),
    Unset,
}

impl Default for MachineParameters {
    fn default() -> Self {
        Self::BdHard(BdHardParameters::default())
    }
}

impl TryFrom<&ar_sound_t> for MachineParameters {
    type Error = RytmError;
    fn try_from(value: &ar_sound_t) -> Result<Self, Self::Error> {
        let machine_type: MachineType = value.machine_type.try_into()?;
        match machine_type {
            MachineType::BdHard => Ok(Self::BdHard(value.into())),
            MachineType::BdClassic => Ok(Self::BdClassic(value.into())),
            MachineType::BdAcoustic => Ok(Self::BdAcoustic(value.into())),
            MachineType::BdFm => Ok(Self::BdFm(value.into())),
            MachineType::BdPlastic => Ok(Self::BdPlastic(value.into())),
            MachineType::BdSilky => Ok(Self::BdSilky(value.into())),
            MachineType::BdSharp => Ok(Self::BdSharp(value.into())),
            MachineType::BtClassic => Ok(Self::BtClassic(value.into())),
            MachineType::CbClassic => Ok(Self::CbClassic(value.into())),
            MachineType::CbMetallic => Ok(Self::CbMetallic(value.into())),
            MachineType::ChClassic => Ok(Self::ChClassic(value.into())),
            MachineType::ChMetallic => Ok(Self::ChMetallic(value.into())),
            MachineType::CpClassic => Ok(Self::CpClassic(value.into())),
            MachineType::CyClassic => Ok(Self::CyClassic(value.into())),
            MachineType::CyMetallic => Ok(Self::CyMetallic(value.into())),
            MachineType::CyRide => Ok(Self::CyRide(value.into())),
            MachineType::HhBasic => Ok(Self::HhBasic(value.into())),
            MachineType::HhLab => Ok(Self::HhLab(value.into())),
            MachineType::OhClassic => Ok(Self::OhClassic(value.into())),
            MachineType::OhMetallic => Ok(Self::OhMetallic(value.into())),
            MachineType::RsClassic => Ok(Self::RsClassic(value.into())),
            MachineType::RsHard => Ok(Self::RsHard(value.into())),
            MachineType::Disable => Ok(Self::Disable),
            MachineType::SdAcoustic => Ok(Self::SdAcoustic(value.into())),
            MachineType::SdClassic => Ok(Self::SdClassic(value.into())),
            MachineType::SdFm => Ok(Self::SdFm(value.into())),
            MachineType::SdHard => Ok(Self::SdHard(value.into())),
            MachineType::SdNatural => Ok(Self::SdNatural(value.into())),
            MachineType::SyChip => Ok(Self::SyChip(value.into())),
            MachineType::SyDualVco => Ok(Self::SyDualVco(value.into())),
            MachineType::SyRaw => Ok(Self::SyRaw(value.into())),
            MachineType::UtImpulse => Ok(Self::UtImpulse(value.into())),
            MachineType::UtNoise => Ok(Self::UtNoise(value.into())),
            MachineType::XtClassic => Ok(Self::XtClassic(value.into())),
            _ => todo!("Conversion error"),
        }
    }
}

impl MachineParameters {
    #[parameter_range(range = "track_index:0..=11")]
    pub fn try_default_for_track(track_index: usize) -> Result<Self, RytmError> {
        Ok(match track_index {
            0 => MachineParameters::BdHard(BdHardParameters::default()),
            1 => MachineParameters::SdHard(SdHardParameters::default()),
            2 => MachineParameters::RsHard(RsHardParameters::default()),
            3 => MachineParameters::CpClassic(CpClassicParameters::default()),
            4 => MachineParameters::BtClassic(BtClassicParameters::default()),
            5 => MachineParameters::XtClassic(XtClassicParameters::default_for_lt()),
            6 => MachineParameters::XtClassic(XtClassicParameters::default_for_mt()),
            7 => MachineParameters::XtClassic(XtClassicParameters::default_for_ht()),
            8 => MachineParameters::ChClassic(ChClassicParameters::default()),
            9 => MachineParameters::OhClassic(OhClassicParameters::default()),
            10 => MachineParameters::CyClassic(CyClassicParameters::default()),
            11 => MachineParameters::CbClassic(CbClassicParameters::default()),
            _ => unreachable!(),
        })
    }

    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        match self {
            MachineParameters::BdHard(bd_hard) => bd_hard.apply_to_raw_sound(raw_sound),
            MachineParameters::BdClassic(bd_classic) => bd_classic.apply_to_raw_sound(raw_sound),
            MachineParameters::BdAcoustic(bd_acoustic) => bd_acoustic.apply_to_raw_sound(raw_sound),
            MachineParameters::BdFm(bd_fm) => bd_fm.apply_to_raw_sound(raw_sound),
            MachineParameters::BdPlastic(bd_plastic) => bd_plastic.apply_to_raw_sound(raw_sound),
            MachineParameters::BdSilky(bd_silky) => bd_silky.apply_to_raw_sound(raw_sound),
            MachineParameters::BdSharp(bd_sharp) => bd_sharp.apply_to_raw_sound(raw_sound),
            MachineParameters::BtClassic(bt_classic) => bt_classic.apply_to_raw_sound(raw_sound),
            MachineParameters::CbClassic(cb_classic) => cb_classic.apply_to_raw_sound(raw_sound),
            MachineParameters::CbMetallic(cb_metallic) => cb_metallic.apply_to_raw_sound(raw_sound),
            MachineParameters::ChClassic(ch_classic) => ch_classic.apply_to_raw_sound(raw_sound),
            MachineParameters::ChMetallic(ch_metallic) => ch_metallic.apply_to_raw_sound(raw_sound),
            MachineParameters::CpClassic(cp_classic) => cp_classic.apply_to_raw_sound(raw_sound),
            MachineParameters::CyClassic(cy_classic) => cy_classic.apply_to_raw_sound(raw_sound),
            MachineParameters::CyMetallic(cy_metallic) => cy_metallic.apply_to_raw_sound(raw_sound),
            MachineParameters::CyRide(cy_ride) => cy_ride.apply_to_raw_sound(raw_sound),
            MachineParameters::HhBasic(hh_basic) => hh_basic.apply_to_raw_sound(raw_sound),
            MachineParameters::HhLab(hh_lab) => hh_lab.apply_to_raw_sound(raw_sound),
            MachineParameters::OhClassic(oh_classic) => oh_classic.apply_to_raw_sound(raw_sound),
            MachineParameters::OhMetallic(oh_metallic) => oh_metallic.apply_to_raw_sound(raw_sound),
            MachineParameters::RsClassic(rs_classic) => rs_classic.apply_to_raw_sound(raw_sound),
            MachineParameters::RsHard(rs_hard) => rs_hard.apply_to_raw_sound(raw_sound),
            MachineParameters::Disable => todo!(),
            MachineParameters::SdAcoustic(sd_acoustic) => sd_acoustic.apply_to_raw_sound(raw_sound),
            MachineParameters::SdClassic(sd_classic) => sd_classic.apply_to_raw_sound(raw_sound),
            MachineParameters::SdFm(sd_fm) => sd_fm.apply_to_raw_sound(raw_sound),
            MachineParameters::SdHard(sd_hard) => sd_hard.apply_to_raw_sound(raw_sound),
            MachineParameters::SdNatural(sd_natural) => sd_natural.apply_to_raw_sound(raw_sound),
            MachineParameters::SyChip(sy_chip) => sy_chip.apply_to_raw_sound(raw_sound),
            MachineParameters::SyDualVco(sy_dual_vco) => sy_dual_vco.apply_to_raw_sound(raw_sound),
            MachineParameters::SyRaw(sy_raw) => sy_raw.apply_to_raw_sound(raw_sound),
            MachineParameters::UtImpulse(ut_impulse) => ut_impulse.apply_to_raw_sound(raw_sound),
            MachineParameters::UtNoise(ut_noise) => ut_noise.apply_to_raw_sound(raw_sound),
            MachineParameters::XtClassic(xt_classic) => xt_classic.apply_to_raw_sound(raw_sound),
            MachineParameters::Unset => todo!(),
        }
    }

    pub(crate) fn internal_machine_type_as_u8(&self) -> u8 {
        match self {
            MachineParameters::BdHard(_) => 0,
            MachineParameters::BdClassic(_) => 1,
            MachineParameters::BdAcoustic(_) => 30,
            MachineParameters::BdFm(_) => 13,
            MachineParameters::BdPlastic(_) => 21,
            MachineParameters::BdSilky(_) => 22,
            MachineParameters::BdSharp(_) => 26,
            MachineParameters::BtClassic(_) => 7,
            MachineParameters::CbClassic(_) => 12,
            MachineParameters::CbMetallic(_) => 20,
            MachineParameters::ChClassic(_) => 9,
            MachineParameters::ChMetallic(_) => 17,
            MachineParameters::CpClassic(_) => 6,
            MachineParameters::CyClassic(_) => 11,
            MachineParameters::CyMetallic(_) => 19,
            MachineParameters::CyRide(_) => 25,
            MachineParameters::HhBasic(_) => 24,
            MachineParameters::HhLab(_) => 33,
            MachineParameters::OhClassic(_) => 10,
            MachineParameters::OhMetallic(_) => 18,
            MachineParameters::RsClassic(_) => 5,
            MachineParameters::RsHard(_) => 4,
            MachineParameters::Disable => 27,
            MachineParameters::SdAcoustic(_) => 31,
            MachineParameters::SdClassic(_) => 3,
            MachineParameters::SdFm(_) => 14,
            MachineParameters::SdHard(_) => 2,
            MachineParameters::SdNatural(_) => 23,
            MachineParameters::SyChip(_) => 29,
            MachineParameters::SyDualVco(_) => 28,
            MachineParameters::SyRaw(_) => 32,
            MachineParameters::UtImpulse(_) => 16,
            MachineParameters::UtNoise(_) => 15,
            MachineParameters::XtClassic(_) => 8,
            // TODO: Double check
            MachineParameters::Unset => 255,
        }

        // TODO: Write enum getter also
    }
}
