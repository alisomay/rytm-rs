// All casts in this file are intended or safe within the context of this library.
//
// One can change `allow` to `warn` to review them if necessary.
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
// TODO: I'm currently really lazy to write errors doc for plock impls. Will do it later..
#![allow(clippy::missing_errors_doc)]

/// Bd Acoustic machine parameters.
mod bd_acoustic;
/// Bd Classic machine parameters.
mod bd_classic;
/// Bd Fm machine parameters.
mod bd_fm;
/// Bd Hard machine parameters.
mod bd_hard;
/// Bd Plastic machine parameters.
mod bd_plastic;
/// Bd Sharp machine parameters.
mod bd_sharp;
/// Bd Silky machine parameters.
mod bd_silky;
/// Bt Classic machine parameters.
mod bt_classic;
/// Cb Classic machine parameters.
mod cb_classic;
/// Cb Metallic machine parameters.
mod cb_metallic;
/// Ch Classic machine parameters.
mod ch_classic;
/// Ch Metallic machine parameters.
mod ch_metallic;
/// Cp Classic machine parameters.
mod cp_classic;
/// Cy Classic machine parameters.
mod cy_classic;
/// Cy Metallic machine parameters.
mod cy_metallic;
/// Cy Ride machine parameters.
mod cy_ride;
/// Hh Basic machine parameters.
mod hh_basic;
/// Hh Lab machine parameters.
mod hh_lab;
/// Oh Classic machine parameters.
mod oh_classic;
/// Oh Metallic machine parameters.
mod oh_metallic;
/// Rs Classic machine parameters.
mod rs_classic;
/// Rs Hard machine parameters.
mod rs_hard;
/// Sd Acoustic machine parameters.
mod sd_acoustic;
/// Sd Classic machine parameters.
mod sd_classic;
/// Sd Fm machine parameters.
mod sd_fm;
/// Sd Hard machine parameters.
mod sd_hard;
/// Sd Natural machine parameters.
mod sd_natural;
/// Sy Chip machine parameters.
mod sy_chip;
/// Sy Dual Vco machine parameters.
mod sy_dual_vco;
/// Sy Raw machine parameters.
mod sy_raw;
/// Ut Impulse machine parameters.
mod ut_impulse;
/// Ut Noise machine parameters.
mod ut_noise;
/// Xt Classic machine parameters.
mod xt_classic;

use super::types::MachineType;
use crate::{
    error::{ParameterError, RytmError},
    object::pattern::plock::ParameterLockPool,
};
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
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
pub use sy_chip::*;
pub use sy_dual_vco::*;
pub use sy_raw::*;
pub use ut_impulse::*;
pub use ut_noise::*;
pub use xt_classic::*;

/// Machine parameters of a sound.
///
/// Every machine has distinct parameters and ranges for those parameters.
///
/// Not every machine can be assigned to every track.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl From<MachineType> for MachineParameters {
    fn from(machine_type: MachineType) -> Self {
        match machine_type {
            MachineType::BdHard => Self::BdHard(BdHardParameters::default()),
            MachineType::BdClassic => Self::BdClassic(BdClassicParameters::default()),
            MachineType::SdHard => Self::SdHard(SdHardParameters::default()),
            MachineType::SdClassic => Self::SdClassic(SdClassicParameters::default()),
            MachineType::RsHard => Self::RsHard(RsHardParameters::default()),
            MachineType::RsClassic => Self::RsClassic(RsClassicParameters::default()),
            MachineType::CpClassic => Self::CpClassic(CpClassicParameters::default()),
            MachineType::BtClassic => Self::BtClassic(BtClassicParameters::default()),
            MachineType::XtClassic => Self::XtClassic(XtClassicParameters::default()),
            MachineType::ChClassic => Self::ChClassic(ChClassicParameters::default()),
            MachineType::OhClassic => Self::OhClassic(OhClassicParameters::default()),
            MachineType::CyClassic => Self::CyClassic(CyClassicParameters::default()),
            MachineType::CbClassic => Self::CbClassic(CbClassicParameters::default()),
            MachineType::BdFm => Self::BdFm(BdFmParameters::default()),
            MachineType::SdFm => Self::SdFm(SdFmParameters::default()),
            MachineType::UtNoise => Self::UtNoise(UtNoiseParameters::default()),
            MachineType::UtImpulse => Self::UtImpulse(UtImpulseParameters::default()),
            MachineType::ChMetallic => Self::ChMetallic(ChMetallicParameters::default()),
            MachineType::OhMetallic => Self::OhMetallic(OhMetallicParameters::default()),
            MachineType::CyMetallic => Self::CyMetallic(CyMetallicParameters::default()),
            MachineType::CbMetallic => Self::CbMetallic(CbMetallicParameters::default()),
            MachineType::BdPlastic => Self::BdPlastic(BdPlasticParameters::default()),
            MachineType::BdSilky => Self::BdSilky(BdSilkyParameters::default()),
            MachineType::SdNatural => Self::SdNatural(SdNaturalParameters::default()),
            MachineType::HhBasic => Self::HhBasic(HhBasicParameters::default()),
            MachineType::CyRide => Self::CyRide(CyRideParameters::default()),
            MachineType::BdSharp => Self::BdSharp(BdSharpParameters::default()),
            MachineType::Disable => Self::Disable,
            MachineType::SyDualVco => Self::SyDualVco(SyDualVcoParameters::default()),
            MachineType::SyChip => Self::SyChip(SyChipParameters::default()),
            MachineType::BdAcoustic => Self::BdAcoustic(BdAcousticParameters::default()),
            MachineType::SdAcoustic => Self::SdAcoustic(SdAcousticParameters::default()),
            MachineType::SyRaw => Self::SyRaw(SyRawParameters::default()),
            MachineType::HhLab => Self::HhLab(HhLabParameters::default()),
            MachineType::Unset => Self::Unset,
        }
    }
}

impl MachineParameters {
    #[parameter_range(range = "track_index[opt]:0..=11")]
    pub(crate) fn try_from_raw_sound(
        raw_sound: &ar_sound_t,
        track_index: Option<usize>,
    ) -> Result<Self, RytmError> {
        let machine_type: MachineType = raw_sound.machine_type.try_into()?;
        match machine_type {
            MachineType::BdHard => Ok(Self::BdHard(BdHardParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::BdClassic => Ok(Self::BdClassic(BdClassicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::BdAcoustic => Ok(Self::BdAcoustic(BdAcousticParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::BdFm => Ok(Self::BdFm(BdFmParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::BdPlastic => Ok(Self::BdPlastic(BdPlasticParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::BdSilky => Ok(Self::BdSilky(BdSilkyParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::BdSharp => Ok(Self::BdSharp(BdSharpParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::BtClassic => Ok(Self::BtClassic(BtClassicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::CbClassic => Ok(Self::CbClassic(CbClassicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::CbMetallic => Ok(Self::CbMetallic(CbMetallicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::ChClassic => Ok(Self::ChClassic(ChClassicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::ChMetallic => Ok(Self::ChMetallic(ChMetallicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::CpClassic => Ok(Self::CpClassic(CpClassicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::CyClassic => Ok(Self::CyClassic(CyClassicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::CyMetallic => Ok(Self::CyMetallic(CyMetallicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::CyRide => Ok(Self::CyRide(CyRideParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::HhBasic => Ok(Self::HhBasic(HhBasicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::HhLab => Ok(Self::HhLab(HhLabParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::OhClassic => Ok(Self::OhClassic(OhClassicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::OhMetallic => Ok(Self::OhMetallic(OhMetallicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::RsClassic => Ok(Self::RsClassic(RsClassicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::RsHard => Ok(Self::RsHard(RsHardParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::Disable => Ok(Self::Disable),
            MachineType::Unset => Ok(Self::Unset),
            MachineType::SdAcoustic => Ok(Self::SdAcoustic(SdAcousticParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::SdClassic => Ok(Self::SdClassic(SdClassicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::SdFm => Ok(Self::SdFm(SdFmParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::SdHard => Ok(Self::SdHard(SdHardParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::SdNatural => Ok(Self::SdNatural(SdNaturalParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::SyChip => Ok(Self::SyChip(SyChipParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::SyDualVco => Ok(Self::SyDualVco(SyDualVcoParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::SyRaw => Ok(Self::SyRaw(SyRawParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::UtImpulse => Ok(Self::UtImpulse(UtImpulseParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::UtNoise => Ok(Self::UtNoise(UtNoiseParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
            MachineType::XtClassic => Ok(Self::XtClassic(XtClassicParameters::from_raw_sound(
                raw_sound,
                track_index,
            )?)),
        }
    }

    /// Returns the default machine parameters for a given track.
    ///
    /// Range `0..=11`
    #[parameter_range(range = "track_index:0..=11")]
    pub fn try_default_for_track(track_index: usize) -> Result<Self, RytmError> {
        Ok(match track_index {
            0 => Self::BdHard(BdHardParameters::default()),
            1 => Self::SdHard(SdHardParameters::default()),
            2 => Self::RsHard(RsHardParameters::default()),
            3 => Self::CpClassic(CpClassicParameters::default()),
            4 => Self::BtClassic(BtClassicParameters::default()),
            5 => Self::XtClassic(XtClassicParameters::default_for_lt()),
            6 => Self::XtClassic(XtClassicParameters::default_for_mt()),
            7 => Self::XtClassic(XtClassicParameters::default_for_ht()),
            8 => Self::ChClassic(ChClassicParameters::default()),
            9 => Self::OhClassic(OhClassicParameters::default()),
            10 => Self::CyClassic(CyClassicParameters::default()),
            11 => Self::CbClassic(CbClassicParameters::default()),
            _ => unreachable!("This can never happened the range is pre-checked."),
        })
    }

    pub(crate) fn apply_to_raw_sound(&self, raw_sound: &mut ar_sound_t) {
        match self {
            Self::BdHard(bd_hard) => bd_hard.apply_to_raw_sound(raw_sound),
            Self::BdClassic(bd_classic) => bd_classic.apply_to_raw_sound(raw_sound),
            Self::BdAcoustic(bd_acoustic) => bd_acoustic.apply_to_raw_sound(raw_sound),
            Self::BdFm(bd_fm) => bd_fm.apply_to_raw_sound(raw_sound),
            Self::BdPlastic(bd_plastic) => bd_plastic.apply_to_raw_sound(raw_sound),
            Self::BdSilky(bd_silky) => bd_silky.apply_to_raw_sound(raw_sound),
            Self::BdSharp(bd_sharp) => bd_sharp.apply_to_raw_sound(raw_sound),
            Self::BtClassic(bt_classic) => bt_classic.apply_to_raw_sound(raw_sound),
            Self::CbClassic(cb_classic) => cb_classic.apply_to_raw_sound(raw_sound),
            Self::CbMetallic(cb_metallic) => cb_metallic.apply_to_raw_sound(raw_sound),
            Self::ChClassic(ch_classic) => ch_classic.apply_to_raw_sound(raw_sound),
            Self::ChMetallic(ch_metallic) => ch_metallic.apply_to_raw_sound(raw_sound),
            Self::CpClassic(cp_classic) => cp_classic.apply_to_raw_sound(raw_sound),
            Self::CyClassic(cy_classic) => cy_classic.apply_to_raw_sound(raw_sound),
            Self::CyMetallic(cy_metallic) => cy_metallic.apply_to_raw_sound(raw_sound),
            Self::CyRide(cy_ride) => cy_ride.apply_to_raw_sound(raw_sound),
            Self::HhBasic(hh_basic) => hh_basic.apply_to_raw_sound(raw_sound),
            Self::HhLab(hh_lab) => hh_lab.apply_to_raw_sound(raw_sound),
            Self::OhClassic(oh_classic) => oh_classic.apply_to_raw_sound(raw_sound),
            Self::OhMetallic(oh_metallic) => oh_metallic.apply_to_raw_sound(raw_sound),
            Self::RsClassic(rs_classic) => rs_classic.apply_to_raw_sound(raw_sound),
            Self::RsHard(rs_hard) => rs_hard.apply_to_raw_sound(raw_sound),
            Self::Disable => {
                // Empirical knowledge:
                //
                // These are the parameters which are sent from the rytm when a sound is queried and the machine is disabled.
                raw_sound.synth_param_1 = crate::util::to_s_u16_t_union_a(16384);
                raw_sound.synth_param_2 = crate::util::to_s_u16_t_union_a(0);
                raw_sound.synth_param_3 = crate::util::to_s_u16_t_union_a(6400);
                raw_sound.synth_param_4 = crate::util::to_s_u16_t_union_a(6400);
                raw_sound.synth_param_5 = crate::util::to_s_u16_t_union_a(0);
                raw_sound.synth_param_6 = crate::util::to_s_u16_t_union_a(12800);
                raw_sound.synth_param_7 = crate::util::to_s_u16_t_union_a(0);
                raw_sound.synth_param_8 = crate::util::to_s_u16_t_union_a(0);
            }
            Self::SdAcoustic(sd_acoustic) => sd_acoustic.apply_to_raw_sound(raw_sound),
            Self::SdClassic(sd_classic) => sd_classic.apply_to_raw_sound(raw_sound),
            Self::SdFm(sd_fm) => sd_fm.apply_to_raw_sound(raw_sound),
            Self::SdHard(sd_hard) => sd_hard.apply_to_raw_sound(raw_sound),
            Self::SdNatural(sd_natural) => sd_natural.apply_to_raw_sound(raw_sound),
            Self::SyChip(sy_chip) => sy_chip.apply_to_raw_sound(raw_sound),
            Self::SyDualVco(sy_dual_vco) => sy_dual_vco.apply_to_raw_sound(raw_sound),
            Self::SyRaw(sy_raw) => sy_raw.apply_to_raw_sound(raw_sound),
            Self::UtImpulse(ut_impulse) => ut_impulse.apply_to_raw_sound(raw_sound),
            Self::UtNoise(ut_noise) => ut_noise.apply_to_raw_sound(raw_sound),
            Self::XtClassic(xt_classic) => xt_classic.apply_to_raw_sound(raw_sound),
            Self::Unset => unreachable!("If you encounter this error, please report it to the maintainer. It means a machine can be unset."),
        }
    }

    pub(crate) fn link_parameter_lock_pool(
        &mut self,
        parameter_lock_pool: Arc<Mutex<ParameterLockPool>>,
    ) {
        match self {
            Self::BdHard(bd_hard) => bd_hard.link_parameter_lock_pool(parameter_lock_pool),
            Self::BdClassic(bd_classic) => bd_classic.link_parameter_lock_pool(parameter_lock_pool),
            Self::BdAcoustic(bd_acoustic) => {
                bd_acoustic.link_parameter_lock_pool(parameter_lock_pool);
            }
            Self::BdFm(bd_fm) => bd_fm.link_parameter_lock_pool(parameter_lock_pool),
            Self::BdPlastic(bd_plastic) => bd_plastic.link_parameter_lock_pool(parameter_lock_pool),
            Self::BdSilky(bd_silky) => bd_silky.link_parameter_lock_pool(parameter_lock_pool),
            Self::BdSharp(bd_sharp) => bd_sharp.link_parameter_lock_pool(parameter_lock_pool),
            Self::BtClassic(bt_classic) => bt_classic.link_parameter_lock_pool(parameter_lock_pool),
            Self::CbClassic(cb_classic) => cb_classic.link_parameter_lock_pool(parameter_lock_pool),
            Self::CbMetallic(cb_metallic) => {
                cb_metallic.link_parameter_lock_pool(parameter_lock_pool);
            }
            Self::ChClassic(ch_classic) => ch_classic.link_parameter_lock_pool(parameter_lock_pool),
            Self::ChMetallic(ch_metallic) => {
                ch_metallic.link_parameter_lock_pool(parameter_lock_pool);
            }
            Self::CpClassic(cp_classic) => cp_classic.link_parameter_lock_pool(parameter_lock_pool),
            Self::CyClassic(cy_classic) => cy_classic.link_parameter_lock_pool(parameter_lock_pool),
            Self::CyMetallic(cy_metallic) => {
                cy_metallic.link_parameter_lock_pool(parameter_lock_pool);
            }
            Self::CyRide(cy_ride) => cy_ride.link_parameter_lock_pool(parameter_lock_pool),
            Self::HhBasic(hh_basic) => hh_basic.link_parameter_lock_pool(parameter_lock_pool),
            Self::HhLab(hh_lab) => hh_lab.link_parameter_lock_pool(parameter_lock_pool),
            Self::OhClassic(oh_classic) => oh_classic.link_parameter_lock_pool(parameter_lock_pool),
            Self::OhMetallic(oh_metallic) => {
                oh_metallic.link_parameter_lock_pool(parameter_lock_pool);
            }
            Self::RsClassic(rs_classic) => rs_classic.link_parameter_lock_pool(parameter_lock_pool),
            Self::RsHard(rs_hard) => rs_hard.link_parameter_lock_pool(parameter_lock_pool),
            Self::SdAcoustic(sd_acoustic) => {
                sd_acoustic.link_parameter_lock_pool(parameter_lock_pool);
            }
            Self::SdClassic(sd_classic) => sd_classic.link_parameter_lock_pool(parameter_lock_pool),
            Self::SdFm(sd_fm) => sd_fm.link_parameter_lock_pool(parameter_lock_pool),
            Self::SdHard(sd_hard) => sd_hard.link_parameter_lock_pool(parameter_lock_pool),
            Self::SdNatural(sd_natural) => sd_natural.link_parameter_lock_pool(parameter_lock_pool),
            Self::SyChip(sy_chip) => sy_chip.link_parameter_lock_pool(parameter_lock_pool),
            Self::SyDualVco(sy_dual_vco) => {
                sy_dual_vco.link_parameter_lock_pool(parameter_lock_pool);
            }
            Self::SyRaw(sy_raw) => sy_raw.link_parameter_lock_pool(parameter_lock_pool),
            Self::UtImpulse(ut_impulse) => ut_impulse.link_parameter_lock_pool(parameter_lock_pool),
            Self::UtNoise(ut_noise) => ut_noise.link_parameter_lock_pool(parameter_lock_pool),
            Self::XtClassic(xt_classic) => xt_classic.link_parameter_lock_pool(parameter_lock_pool),
            Self::Unset | Self::Disable => {
                // Ignore
            }
        }
    }
}
