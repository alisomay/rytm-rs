// All casts in this file are intended or safe within the context of this library.
//
// One can change `allow` to `warn` to review them if necessary.
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
#![allow(clippy::enum_glob_use)]
use super::machine::MachineParameters;
use crate::error::ConversionError;
use serde::{Deserialize, Serialize};

#[allow(unused)]
mod machines {
    use rytm_sys::{
        sSI, AR_NUM_SOUND_MACHINES, AR_SOUND_MACHINE_BD_ACOUSTIC, AR_SOUND_MACHINE_BD_CLASSIC,
        AR_SOUND_MACHINE_BD_FM, AR_SOUND_MACHINE_BD_HARD, AR_SOUND_MACHINE_BD_PLASTIC,
        AR_SOUND_MACHINE_BD_SHARP, AR_SOUND_MACHINE_BD_SILKY, AR_SOUND_MACHINE_BT_CLASSIC,
        AR_SOUND_MACHINE_CB_CLASSIC, AR_SOUND_MACHINE_CB_METALLIC, AR_SOUND_MACHINE_CH_CLASSIC,
        AR_SOUND_MACHINE_CH_METALLIC, AR_SOUND_MACHINE_CP_CLASSIC, AR_SOUND_MACHINE_CY_CLASSIC,
        AR_SOUND_MACHINE_CY_METALLIC, AR_SOUND_MACHINE_CY_RIDE, AR_SOUND_MACHINE_DISABLE,
        AR_SOUND_MACHINE_HH_BASIC, AR_SOUND_MACHINE_HH_LAB, AR_SOUND_MACHINE_OH_CLASSIC,
        AR_SOUND_MACHINE_OH_METALLIC, AR_SOUND_MACHINE_RS_CLASSIC, AR_SOUND_MACHINE_RS_HARD,
        AR_SOUND_MACHINE_SD_ACOUSTIC, AR_SOUND_MACHINE_SD_CLASSIC, AR_SOUND_MACHINE_SD_FM,
        AR_SOUND_MACHINE_SD_HARD, AR_SOUND_MACHINE_SD_NATURAL, AR_SOUND_MACHINE_SY_CHIP,
        AR_SOUND_MACHINE_SY_DUAL_VCO, AR_SOUND_MACHINE_SY_RAW, AR_SOUND_MACHINE_UT_IMPULSE,
        AR_SOUND_MACHINE_UT_NOISE, AR_SOUND_MACHINE_XT_CLASSIC,
    };

    pub const NUM_SOUND_MACHINES: u8 = AR_NUM_SOUND_MACHINES as u8;
    pub const BD_ACOUSTIC: u8 = AR_SOUND_MACHINE_BD_ACOUSTIC as u8;
    pub const BD_CLASSIC: u8 = AR_SOUND_MACHINE_BD_CLASSIC as u8;
    pub const BD_FM: u8 = AR_SOUND_MACHINE_BD_FM as u8;
    pub const BD_HARD: u8 = AR_SOUND_MACHINE_BD_HARD as u8;
    pub const BD_PLASTIC: u8 = AR_SOUND_MACHINE_BD_PLASTIC as u8;
    pub const BD_SHARP: u8 = AR_SOUND_MACHINE_BD_SHARP as u8;
    pub const BD_SILKY: u8 = AR_SOUND_MACHINE_BD_SILKY as u8;
    pub const BT_CLASSIC: u8 = AR_SOUND_MACHINE_BT_CLASSIC as u8;
    pub const CB_CLASSIC: u8 = AR_SOUND_MACHINE_CB_CLASSIC as u8;
    pub const CB_METALLIC: u8 = AR_SOUND_MACHINE_CB_METALLIC as u8;
    pub const CH_CLASSIC: u8 = AR_SOUND_MACHINE_CH_CLASSIC as u8;
    pub const CH_METALLIC: u8 = AR_SOUND_MACHINE_CH_METALLIC as u8;
    pub const CP_CLASSIC: u8 = AR_SOUND_MACHINE_CP_CLASSIC as u8;
    pub const CY_CLASSIC: u8 = AR_SOUND_MACHINE_CY_CLASSIC as u8;
    pub const CY_METALLIC: u8 = AR_SOUND_MACHINE_CY_METALLIC as u8;
    pub const CY_RIDE: u8 = AR_SOUND_MACHINE_CY_RIDE as u8;
    pub const DISABLE: u8 = AR_SOUND_MACHINE_DISABLE as u8;
    pub const HH_BASIC: u8 = AR_SOUND_MACHINE_HH_BASIC as u8;
    pub const HH_LAB: u8 = AR_SOUND_MACHINE_HH_LAB as u8;
    pub const OH_CLASSIC: u8 = AR_SOUND_MACHINE_OH_CLASSIC as u8;
    pub const OH_METALLIC: u8 = AR_SOUND_MACHINE_OH_METALLIC as u8;
    pub const RS_CLASSIC: u8 = AR_SOUND_MACHINE_RS_CLASSIC as u8;
    pub const RS_HARD: u8 = AR_SOUND_MACHINE_RS_HARD as u8;
    pub const SD_ACOUSTIC: u8 = AR_SOUND_MACHINE_SD_ACOUSTIC as u8;
    pub const SD_CLASSIC: u8 = AR_SOUND_MACHINE_SD_CLASSIC as u8;
    pub const SD_FM: u8 = AR_SOUND_MACHINE_SD_FM as u8;
    pub const SD_HARD: u8 = AR_SOUND_MACHINE_SD_HARD as u8;
    pub const SD_NATURAL: u8 = AR_SOUND_MACHINE_SD_NATURAL as u8;
    pub const SY_CHIP: u8 = AR_SOUND_MACHINE_SY_CHIP as u8;
    pub const SY_DUAL_VCO: u8 = AR_SOUND_MACHINE_SY_DUAL_VCO as u8;
    pub const SY_RAW: u8 = AR_SOUND_MACHINE_SY_RAW as u8;
    pub const UT_IMPULSE: u8 = AR_SOUND_MACHINE_UT_IMPULSE as u8;
    pub const UT_NOISE: u8 = AR_SOUND_MACHINE_UT_NOISE as u8;
    pub const XT_CLASSIC: u8 = AR_SOUND_MACHINE_XT_CLASSIC as u8;
    pub const UNSET: u8 = 0xFF;
}

mod lfo_destination {
    use rytm_sys::{
        AR_SOUND_LFO_DEST_AMP_ACC, AR_SOUND_LFO_DEST_AMP_ATK, AR_SOUND_LFO_DEST_AMP_DEC,
        AR_SOUND_LFO_DEST_AMP_DLY, AR_SOUND_LFO_DEST_AMP_HLD, AR_SOUND_LFO_DEST_AMP_OVR,
        AR_SOUND_LFO_DEST_AMP_PAN, AR_SOUND_LFO_DEST_AMP_REV, AR_SOUND_LFO_DEST_AMP_VOL,
        AR_SOUND_LFO_DEST_FLT_ATK, AR_SOUND_LFO_DEST_FLT_DEC, AR_SOUND_LFO_DEST_FLT_ENV,
        AR_SOUND_LFO_DEST_FLT_FRQ, AR_SOUND_LFO_DEST_FLT_REL, AR_SOUND_LFO_DEST_FLT_RES,
        AR_SOUND_LFO_DEST_FLT_SUS, AR_SOUND_LFO_DEST_NONE, AR_SOUND_LFO_DEST_SMP_BR,
        AR_SOUND_LFO_DEST_SMP_END, AR_SOUND_LFO_DEST_SMP_FIN, AR_SOUND_LFO_DEST_SMP_LEV,
        AR_SOUND_LFO_DEST_SMP_LOP, AR_SOUND_LFO_DEST_SMP_SMP, AR_SOUND_LFO_DEST_SMP_STA,
        AR_SOUND_LFO_DEST_SMP_TUN, AR_SOUND_LFO_DEST_SYN_1, AR_SOUND_LFO_DEST_SYN_2,
        AR_SOUND_LFO_DEST_SYN_3, AR_SOUND_LFO_DEST_SYN_4, AR_SOUND_LFO_DEST_SYN_5,
        AR_SOUND_LFO_DEST_SYN_6, AR_SOUND_LFO_DEST_SYN_7, AR_SOUND_LFO_DEST_SYN_8,
    };

    pub const NONE: u8 = AR_SOUND_LFO_DEST_NONE as u8;
    pub const SYN_1: u8 = AR_SOUND_LFO_DEST_SYN_1 as u8;
    pub const SYN_2: u8 = AR_SOUND_LFO_DEST_SYN_2 as u8;
    pub const SYN_3: u8 = AR_SOUND_LFO_DEST_SYN_3 as u8;
    pub const SYN_4: u8 = AR_SOUND_LFO_DEST_SYN_4 as u8;
    pub const SYN_5: u8 = AR_SOUND_LFO_DEST_SYN_5 as u8;
    pub const SYN_6: u8 = AR_SOUND_LFO_DEST_SYN_6 as u8;
    pub const SYN_7: u8 = AR_SOUND_LFO_DEST_SYN_7 as u8;
    pub const SYN_8: u8 = AR_SOUND_LFO_DEST_SYN_8 as u8;
    pub const SMP_TUN: u8 = AR_SOUND_LFO_DEST_SMP_TUN as u8;
    pub const SMP_FIN: u8 = AR_SOUND_LFO_DEST_SMP_FIN as u8;
    pub const SMP_SMP: u8 = AR_SOUND_LFO_DEST_SMP_SMP as u8;
    pub const SMP_BR: u8 = AR_SOUND_LFO_DEST_SMP_BR as u8;
    pub const SMP_STA: u8 = AR_SOUND_LFO_DEST_SMP_STA as u8;
    pub const SMP_END: u8 = AR_SOUND_LFO_DEST_SMP_END as u8;
    pub const SMP_LOP: u8 = AR_SOUND_LFO_DEST_SMP_LOP as u8;
    pub const SMP_LEV: u8 = AR_SOUND_LFO_DEST_SMP_LEV as u8;
    pub const FLT_ENV: u8 = AR_SOUND_LFO_DEST_FLT_ENV as u8;
    pub const FLT_ATK: u8 = AR_SOUND_LFO_DEST_FLT_ATK as u8;
    pub const FLT_DEC: u8 = AR_SOUND_LFO_DEST_FLT_DEC as u8;
    pub const FLT_SUS: u8 = AR_SOUND_LFO_DEST_FLT_SUS as u8;
    pub const FLT_REL: u8 = AR_SOUND_LFO_DEST_FLT_REL as u8;
    pub const FLT_FRQ: u8 = AR_SOUND_LFO_DEST_FLT_FRQ as u8;
    pub const FLT_RES: u8 = AR_SOUND_LFO_DEST_FLT_RES as u8;
    pub const AMP_ATK: u8 = AR_SOUND_LFO_DEST_AMP_ATK as u8;
    pub const AMP_HLD: u8 = AR_SOUND_LFO_DEST_AMP_HLD as u8;
    pub const AMP_DEC: u8 = AR_SOUND_LFO_DEST_AMP_DEC as u8;
    pub const AMP_OVR: u8 = AR_SOUND_LFO_DEST_AMP_OVR as u8;
    pub const AMP_VOL: u8 = AR_SOUND_LFO_DEST_AMP_VOL as u8;
    pub const AMP_PAN: u8 = AR_SOUND_LFO_DEST_AMP_PAN as u8;
    pub const AMP_ACC: u8 = AR_SOUND_LFO_DEST_AMP_ACC as u8;
    pub const AMP_DLY: u8 = AR_SOUND_LFO_DEST_AMP_DLY as u8;
    pub const AMP_REV: u8 = AR_SOUND_LFO_DEST_AMP_REV as u8;
}

pub(crate) mod sound_mod_target {
    use rytm_sys::{
        AR_SOUND_MOD_DEST_AMP_ACC, AR_SOUND_MOD_DEST_AMP_ATK, AR_SOUND_MOD_DEST_AMP_DEC,
        AR_SOUND_MOD_DEST_AMP_DLY, AR_SOUND_MOD_DEST_AMP_HLD, AR_SOUND_MOD_DEST_AMP_OVR,
        AR_SOUND_MOD_DEST_AMP_PAN, AR_SOUND_MOD_DEST_AMP_REV, AR_SOUND_MOD_DEST_AMP_VOL,
        AR_SOUND_MOD_DEST_FLT_ATK, AR_SOUND_MOD_DEST_FLT_DEC, AR_SOUND_MOD_DEST_FLT_ENV,
        AR_SOUND_MOD_DEST_FLT_FRQ, AR_SOUND_MOD_DEST_FLT_REL, AR_SOUND_MOD_DEST_FLT_RES,
        AR_SOUND_MOD_DEST_FLT_SUS, AR_SOUND_MOD_DEST_LFO_DEPTH, AR_SOUND_MOD_DEST_LFO_FADE,
        AR_SOUND_MOD_DEST_LFO_MULTIPLIER, AR_SOUND_MOD_DEST_LFO_PHASE, AR_SOUND_MOD_DEST_LFO_SPEED,
        AR_SOUND_MOD_DEST_LFO_TRIGMODE, AR_SOUND_MOD_DEST_LFO_WAVEFORM, AR_SOUND_MOD_DEST_NONE,
        AR_SOUND_MOD_DEST_SMP_BR, AR_SOUND_MOD_DEST_SMP_END, AR_SOUND_MOD_DEST_SMP_FIN,
        AR_SOUND_MOD_DEST_SMP_LEV, AR_SOUND_MOD_DEST_SMP_LOP, AR_SOUND_MOD_DEST_SMP_SMP,
        AR_SOUND_MOD_DEST_SMP_STA, AR_SOUND_MOD_DEST_SMP_TUN, AR_SOUND_MOD_DEST_SYN_1,
        AR_SOUND_MOD_DEST_SYN_2, AR_SOUND_MOD_DEST_SYN_3, AR_SOUND_MOD_DEST_SYN_4,
        AR_SOUND_MOD_DEST_SYN_5, AR_SOUND_MOD_DEST_SYN_6, AR_SOUND_MOD_DEST_SYN_7,
        AR_SOUND_MOD_DEST_SYN_8,
    };

    pub const NONE: u8 = AR_SOUND_MOD_DEST_NONE as u8;
    pub const LFO_MULTIPLIER: u8 = AR_SOUND_MOD_DEST_LFO_MULTIPLIER as u8;
    pub const LFO_WAVEFORM: u8 = AR_SOUND_MOD_DEST_LFO_WAVEFORM as u8;
    pub const LFO_TRIGMODE: u8 = AR_SOUND_MOD_DEST_LFO_TRIGMODE as u8;
    pub const LFO_SPEED: u8 = AR_SOUND_MOD_DEST_LFO_SPEED as u8;
    pub const LFO_FADE: u8 = AR_SOUND_MOD_DEST_LFO_FADE as u8;
    pub const LFO_PHASE: u8 = AR_SOUND_MOD_DEST_LFO_PHASE as u8;
    pub const LFO_DEPTH: u8 = AR_SOUND_MOD_DEST_LFO_DEPTH as u8;
    pub const SYN_1: u8 = AR_SOUND_MOD_DEST_SYN_1 as u8;
    pub const SYN_2: u8 = AR_SOUND_MOD_DEST_SYN_2 as u8;
    pub const SYN_3: u8 = AR_SOUND_MOD_DEST_SYN_3 as u8;
    pub const SYN_4: u8 = AR_SOUND_MOD_DEST_SYN_4 as u8;
    pub const SYN_5: u8 = AR_SOUND_MOD_DEST_SYN_5 as u8;
    pub const SYN_6: u8 = AR_SOUND_MOD_DEST_SYN_6 as u8;
    pub const SYN_7: u8 = AR_SOUND_MOD_DEST_SYN_7 as u8;
    pub const SYN_8: u8 = AR_SOUND_MOD_DEST_SYN_8 as u8;
    pub const SMP_TUN: u8 = AR_SOUND_MOD_DEST_SMP_TUN as u8;
    pub const SMP_FIN: u8 = AR_SOUND_MOD_DEST_SMP_FIN as u8;
    pub const SMP_SMP: u8 = AR_SOUND_MOD_DEST_SMP_SMP as u8;
    pub const SMP_BR: u8 = AR_SOUND_MOD_DEST_SMP_BR as u8;
    pub const SMP_STA: u8 = AR_SOUND_MOD_DEST_SMP_STA as u8;
    pub const SMP_END: u8 = AR_SOUND_MOD_DEST_SMP_END as u8;
    pub const SMP_LOP: u8 = AR_SOUND_MOD_DEST_SMP_LOP as u8;
    pub const SMP_LEV: u8 = AR_SOUND_MOD_DEST_SMP_LEV as u8;
    pub const FLT_ENV: u8 = AR_SOUND_MOD_DEST_FLT_ENV as u8;
    pub const FLT_ATK: u8 = AR_SOUND_MOD_DEST_FLT_ATK as u8;
    pub const FLT_DEC: u8 = AR_SOUND_MOD_DEST_FLT_DEC as u8;
    pub const FLT_SUS: u8 = AR_SOUND_MOD_DEST_FLT_SUS as u8;
    pub const FLT_REL: u8 = AR_SOUND_MOD_DEST_FLT_REL as u8;
    pub const FLT_FRQ: u8 = AR_SOUND_MOD_DEST_FLT_FRQ as u8;
    pub const FLT_RES: u8 = AR_SOUND_MOD_DEST_FLT_RES as u8;
    pub const AMP_ATK: u8 = AR_SOUND_MOD_DEST_AMP_ATK as u8;
    pub const AMP_HLD: u8 = AR_SOUND_MOD_DEST_AMP_HLD as u8;
    pub const AMP_DEC: u8 = AR_SOUND_MOD_DEST_AMP_DEC as u8;
    pub const AMP_OVR: u8 = AR_SOUND_MOD_DEST_AMP_OVR as u8;
    pub const AMP_VOL: u8 = AR_SOUND_MOD_DEST_AMP_VOL as u8;
    pub const AMP_PAN: u8 = AR_SOUND_MOD_DEST_AMP_PAN as u8;
    pub const AMP_ACC: u8 = AR_SOUND_MOD_DEST_AMP_ACC as u8;
    pub const AMP_DLY: u8 = AR_SOUND_MOD_DEST_AMP_DLY as u8;
    pub const AMP_REV: u8 = AR_SOUND_MOD_DEST_AMP_REV as u8;
}

/// Machine type of a sound.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum MachineType {
    BdHard,
    BdClassic,
    SdHard,
    SdClassic,
    RsHard,
    RsClassic,
    CpClassic,
    BtClassic,
    XtClassic,
    ChClassic,
    OhClassic,
    CyClassic,
    CbClassic,
    BdFm,
    SdFm,
    UtNoise,
    UtImpulse,
    ChMetallic,
    OhMetallic,
    CyMetallic,
    CbMetallic,
    BdPlastic,
    BdSilky,
    SdNatural,
    HhBasic,
    CyRide,
    BdSharp,
    Disable,
    SyDualVco,
    SyChip,
    BdAcoustic,
    SdAcoustic,
    SyRaw,
    HhLab,
    #[default]
    Unset,
}

// bdhard, bdclassic, sdhard ..
impl TryFrom<&str> for MachineType {
    type Error = ConversionError;
    fn try_from(machine: &str) -> Result<Self, Self::Error> {
        use MachineType::*;
        match machine {
            "bdhard" => Ok(BdHard),
            "bdclassic" => Ok(BdClassic),
            "sdhard" => Ok(SdHard),
            "sdclassic" => Ok(SdClassic),
            "rshard" => Ok(RsHard),
            "rsclassic" => Ok(RsClassic),
            "cpclassic" => Ok(CpClassic),
            "btclassic" => Ok(BtClassic),
            "xtclassic" => Ok(XtClassic),
            "chclassic" => Ok(ChClassic),
            "ohclassic" => Ok(OhClassic),
            "cyclassic" => Ok(CyClassic),
            "cbclassic" => Ok(CbClassic),
            "bdfm" => Ok(BdFm),
            "sdfm" => Ok(SdFm),
            "utnoise" => Ok(UtNoise),
            "utimpulse" => Ok(UtImpulse),
            "chmetallic" => Ok(ChMetallic),
            "ohmetallic" => Ok(OhMetallic),
            "cymetallic" => Ok(CyMetallic),
            "cbmetallic" => Ok(CbMetallic),
            "bdplastic" => Ok(BdPlastic),
            "bdsilky" => Ok(BdSilky),
            "sdnatural" => Ok(SdNatural),
            "hhbasic" => Ok(HhBasic),
            "cyride" => Ok(CyRide),
            "bdsharp" => Ok(BdSharp),
            "disable" => Ok(Disable),
            "sydualvco" => Ok(SyDualVco),
            "sychip" => Ok(SyChip),
            "bdacoustic" => Ok(BdAcoustic),
            "sdacoustic" => Ok(SdAcoustic),
            "syraw" => Ok(SyRaw),
            "hhlab" => Ok(HhLab),
            "unset" => Ok(Unset),
            _ => Err(ConversionError::Range {
                value: machine.to_string(),
                type_name: "Machine".to_string(),
            }),
        }
    }
}

impl std::fmt::Display for MachineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let machine = match self {
            Self::BdHard => "BD_HARD",
            Self::BdClassic => "BD_CLASSIC",
            Self::SdHard => "SD_HARD",
            Self::SdClassic => "SD_CLASSIC",
            Self::RsHard => "RS_HARD",
            Self::RsClassic => "RS_CLASSIC",
            Self::CpClassic => "CP_CLASSIC",
            Self::BtClassic => "BT_CLASSIC",
            Self::XtClassic => "XT_CLASSIC",
            Self::ChClassic => "CH_CLASSIC",
            Self::OhClassic => "OH_CLASSIC",
            Self::CyClassic => "CY_CLASSIC",
            Self::CbClassic => "CB_CLASSIC",
            Self::BdFm => "BD_FM",
            Self::SdFm => "SD_FM",
            Self::UtNoise => "UT_NOISE",
            Self::UtImpulse => "UT_IMPULSE",
            Self::ChMetallic => "CH_METALLIC",
            Self::OhMetallic => "OH_METALLIC",
            Self::CyMetallic => "CY_METALLIC",
            Self::CbMetallic => "CB_METALLIC",
            Self::BdPlastic => "BD_PLASTIC",
            Self::BdSilky => "BD_SILKY",
            Self::SdNatural => "SD_NATURAL",
            Self::HhBasic => "HH_BASIC",
            Self::CyRide => "CY_RIDE",
            Self::BdSharp => "BD_SHARP",
            Self::Disable => "DISABLE",
            Self::SyDualVco => "SY_DUAL_VCO",
            Self::SyChip => "SY_CHIP",
            Self::BdAcoustic => "BD_ACOUSTIC",
            Self::SdAcoustic => "SD_ACOUSTIC",
            Self::SyRaw => "SY_RAW",
            Self::HhLab => "HH_LAB",
            Self::Unset => "UNSET",
        };
        write!(f, "{machine}")
    }
}

impl TryFrom<u8> for MachineType {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use machines::*;
        match value {
            BD_HARD => Ok(Self::BdHard),
            BD_CLASSIC => Ok(Self::BdClassic),
            SD_HARD => Ok(Self::SdHard),
            SD_CLASSIC => Ok(Self::SdClassic),
            RS_HARD => Ok(Self::RsHard),
            RS_CLASSIC => Ok(Self::RsClassic),
            CP_CLASSIC => Ok(Self::CpClassic),
            BT_CLASSIC => Ok(Self::BtClassic),
            XT_CLASSIC => Ok(Self::XtClassic),
            CH_CLASSIC => Ok(Self::ChClassic),
            OH_CLASSIC => Ok(Self::OhClassic),
            CY_CLASSIC => Ok(Self::CyClassic),
            CB_CLASSIC => Ok(Self::CbClassic),
            BD_FM => Ok(Self::BdFm),
            SD_FM => Ok(Self::SdFm),
            UT_NOISE => Ok(Self::UtNoise),
            UT_IMPULSE => Ok(Self::UtImpulse),
            CH_METALLIC => Ok(Self::ChMetallic),
            OH_METALLIC => Ok(Self::OhMetallic),
            CY_METALLIC => Ok(Self::CyMetallic),
            CB_METALLIC => Ok(Self::CbMetallic),
            BD_PLASTIC => Ok(Self::BdPlastic),
            BD_SILKY => Ok(Self::BdSilky),
            SD_NATURAL => Ok(Self::SdNatural),
            HH_BASIC => Ok(Self::HhBasic),
            CY_RIDE => Ok(Self::CyRide),
            BD_SHARP => Ok(Self::BdSharp),
            DISABLE => Ok(Self::Disable),
            SY_DUAL_VCO => Ok(Self::SyDualVco),
            SY_CHIP => Ok(Self::SyChip),
            BD_ACOUSTIC => Ok(Self::BdAcoustic),
            SD_ACOUSTIC => Ok(Self::SdAcoustic),
            SY_RAW => Ok(Self::SyRaw),
            HH_LAB => Ok(Self::HhLab),
            UNSET => Ok(Self::Unset),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "Machine".to_string(),
            }),
        }
    }
}

impl From<MachineType> for u8 {
    fn from(value: MachineType) -> Self {
        use machines::*;
        match value {
            MachineType::BdHard => BD_HARD,
            MachineType::BdClassic => BD_CLASSIC,
            MachineType::SdHard => SD_HARD,
            MachineType::SdClassic => SD_CLASSIC,
            MachineType::RsHard => RS_HARD,
            MachineType::RsClassic => RS_CLASSIC,
            MachineType::CpClassic => CP_CLASSIC,
            MachineType::BtClassic => BT_CLASSIC,
            MachineType::XtClassic => XT_CLASSIC,
            MachineType::ChClassic => CH_CLASSIC,
            MachineType::OhClassic => OH_CLASSIC,
            MachineType::CyClassic => CY_CLASSIC,
            MachineType::CbClassic => CB_CLASSIC,
            MachineType::BdFm => BD_FM,
            MachineType::SdFm => SD_FM,
            MachineType::UtNoise => UT_NOISE,
            MachineType::UtImpulse => UT_IMPULSE,
            MachineType::ChMetallic => CH_METALLIC,
            MachineType::OhMetallic => OH_METALLIC,
            MachineType::CyMetallic => CY_METALLIC,
            MachineType::CbMetallic => CB_METALLIC,
            MachineType::BdPlastic => BD_PLASTIC,
            MachineType::BdSilky => BD_SILKY,
            MachineType::SdNatural => SD_NATURAL,
            MachineType::HhBasic => HH_BASIC,
            MachineType::CyRide => CY_RIDE,
            MachineType::BdSharp => BD_SHARP,
            MachineType::Disable => DISABLE,
            MachineType::SyDualVco => SY_DUAL_VCO,
            MachineType::SyChip => SY_CHIP,
            MachineType::BdAcoustic => BD_ACOUSTIC,
            MachineType::SdAcoustic => SD_ACOUSTIC,
            MachineType::SyRaw => SY_RAW,
            MachineType::HhLab => HH_LAB,
            MachineType::Unset => UNSET,
        }
    }
}

impl From<MachineParameters> for MachineType {
    fn from(machine: MachineParameters) -> Self {
        use MachineParameters::*;
        match machine {
            BdHard(_) => Self::BdHard,
            BdClassic(_) => Self::BdClassic,
            SdHard(_) => Self::SdHard,
            SdClassic(_) => Self::SdClassic,
            RsHard(_) => Self::RsHard,
            RsClassic(_) => Self::RsClassic,
            CpClassic(_) => Self::CpClassic,
            BtClassic(_) => Self::BtClassic,
            XtClassic(_) => Self::XtClassic,
            ChClassic(_) => Self::ChClassic,
            OhClassic(_) => Self::OhClassic,
            CyClassic(_) => Self::CyClassic,
            CbClassic(_) => Self::CbClassic,
            BdFm(_) => Self::BdFm,
            SdFm(_) => Self::SdFm,
            UtNoise(_) => Self::UtNoise,
            UtImpulse(_) => Self::UtImpulse,
            ChMetallic(_) => Self::ChMetallic,
            OhMetallic(_) => Self::OhMetallic,
            CyMetallic(_) => Self::CyMetallic,
            CbMetallic(_) => Self::CbMetallic,
            BdPlastic(_) => Self::BdPlastic,
            BdSilky(_) => Self::BdSilky,
            SdNatural(_) => Self::SdNatural,
            HhBasic(_) => Self::HhBasic,
            CyRide(_) => Self::CyRide,
            BdSharp(_) => Self::BdSharp,
            Disable => Self::Disable,
            SyDualVco(_) => Self::SyDualVco,
            SyChip(_) => Self::SyChip,
            BdAcoustic(_) => Self::BdAcoustic,
            SdAcoustic(_) => Self::SdAcoustic,
            SyRaw(_) => Self::SyRaw,
            HhLab(_) => Self::HhLab,
            Unset => Self::Unset,
        }
    }
}

/// Destination of an LFO.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum LfoDestination {
    #[default]
    Unset,
    Syn1,
    Syn2,
    Syn3,
    Syn4,
    Syn5,
    Syn6,
    Syn7,
    Syn8,
    SampleTune,
    SampleFineTune,
    SampleSlice,
    SampleBitReduction,
    SampleStart,
    SampleEnd,
    SampleLoop,
    SampleLevel,
    FilterEnvelope,
    FilterAttack,
    FilterDecay,
    FilterSustain,
    FilterRelease,
    FilterFrequency,
    FilterResonance,
    AmpAttack,
    AmpHold,
    AmpDecay,
    AmpOverdrive,
    AmpVolume,
    AmpPan,
    AmpAccent,
    AmpDelaySend,
    AmpReverbSend,
}

impl TryFrom<&str> for LfoDestination {
    type Error = ConversionError;
    fn try_from(destination: &str) -> Result<Self, Self::Error> {
        use LfoDestination::*;
        match destination {
            "syn1" => Ok(Syn1),
            "syn2" => Ok(Syn2),
            "syn3" => Ok(Syn3),
            "syn4" => Ok(Syn4),
            "syn5" => Ok(Syn5),
            "syn6" => Ok(Syn6),
            "syn7" => Ok(Syn7),
            "syn8" => Ok(Syn8),
            "sampletune" => Ok(SampleTune),
            "samplefinetune" => Ok(SampleFineTune),
            "sampleslice" => Ok(SampleSlice),
            "samplebitreduction" => Ok(SampleBitReduction),
            "samplestart" => Ok(SampleStart),
            "sampleend" => Ok(SampleEnd),
            "sampleloop" => Ok(SampleLoop),
            "samplelevel" => Ok(SampleLevel),
            "filterenvelope" => Ok(FilterEnvelope),
            "filterattack" => Ok(FilterAttack),
            "filterdecay" => Ok(FilterDecay),
            "filtersustain" => Ok(FilterSustain),
            "filterrelease" => Ok(FilterRelease),
            "filterfrequency" => Ok(FilterFrequency),
            "filterresonance" => Ok(FilterResonance),
            "ampattack" => Ok(AmpAttack),
            "amphold" => Ok(AmpHold),
            "ampdecay" => Ok(AmpDecay),
            "ampoverdrive" => Ok(AmpOverdrive),
            "ampvolume" => Ok(AmpVolume),
            "amppan" => Ok(AmpPan),
            "ampaccent" => Ok(AmpAccent),
            "ampdelaysend" => Ok(AmpDelaySend),
            "ampreverb_send" => Ok(AmpReverbSend),
            "unset" => Ok(Unset),
            _ => Err(ConversionError::Range {
                value: destination.to_string(),
                type_name: "LfoDestination".to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for LfoDestination {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use lfo_destination::*;
        match value {
            NONE => Ok(Self::Unset),
            SYN_1 => Ok(Self::Syn1),
            SYN_2 => Ok(Self::Syn2),
            SYN_3 => Ok(Self::Syn3),
            SYN_4 => Ok(Self::Syn4),
            SYN_5 => Ok(Self::Syn5),
            SYN_6 => Ok(Self::Syn6),
            SYN_7 => Ok(Self::Syn7),
            SYN_8 => Ok(Self::Syn8),
            SMP_TUN => Ok(Self::SampleTune),
            SMP_FIN => Ok(Self::SampleFineTune),
            SMP_SMP => Ok(Self::SampleSlice),
            SMP_BR => Ok(Self::SampleBitReduction),
            SMP_STA => Ok(Self::SampleStart),
            SMP_END => Ok(Self::SampleEnd),
            SMP_LOP => Ok(Self::SampleLoop),
            SMP_LEV => Ok(Self::SampleLevel),
            FLT_ENV => Ok(Self::FilterEnvelope),
            FLT_ATK => Ok(Self::FilterAttack),
            FLT_DEC => Ok(Self::FilterDecay),
            FLT_SUS => Ok(Self::FilterSustain),
            FLT_REL => Ok(Self::FilterRelease),
            FLT_FRQ => Ok(Self::FilterFrequency),
            FLT_RES => Ok(Self::FilterResonance),
            AMP_ATK => Ok(Self::AmpAttack),
            AMP_HLD => Ok(Self::AmpHold),
            AMP_DEC => Ok(Self::AmpDecay),
            AMP_OVR => Ok(Self::AmpOverdrive),
            AMP_VOL => Ok(Self::AmpVolume),
            AMP_PAN => Ok(Self::AmpPan),
            AMP_ACC => Ok(Self::AmpAccent),
            AMP_DLY => Ok(Self::AmpDelaySend),
            AMP_REV => Ok(Self::AmpReverbSend),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "LfoDestination".to_string(),
            }),
        }
    }
}

impl From<LfoDestination> for u8 {
    fn from(value: LfoDestination) -> Self {
        use lfo_destination::*;
        match value {
            LfoDestination::Unset => NONE,
            LfoDestination::Syn1 => SYN_1,
            LfoDestination::Syn2 => SYN_2,
            LfoDestination::Syn3 => SYN_3,
            LfoDestination::Syn4 => SYN_4,
            LfoDestination::Syn5 => SYN_5,
            LfoDestination::Syn6 => SYN_6,
            LfoDestination::Syn7 => SYN_7,
            LfoDestination::Syn8 => SYN_8,
            LfoDestination::SampleTune => SMP_TUN,
            LfoDestination::SampleFineTune => SMP_FIN,
            LfoDestination::SampleSlice => SMP_SMP,
            LfoDestination::SampleBitReduction => SMP_BR,
            LfoDestination::SampleStart => SMP_STA,
            LfoDestination::SampleEnd => SMP_END,
            LfoDestination::SampleLoop => SMP_LOP,
            LfoDestination::SampleLevel => SMP_LEV,
            LfoDestination::FilterEnvelope => FLT_ENV,
            LfoDestination::FilterAttack => FLT_ATK,
            LfoDestination::FilterDecay => FLT_DEC,
            LfoDestination::FilterSustain => FLT_SUS,
            LfoDestination::FilterRelease => FLT_REL,
            LfoDestination::FilterFrequency => FLT_FRQ,
            LfoDestination::FilterResonance => FLT_RES,
            LfoDestination::AmpAttack => AMP_ATK,
            LfoDestination::AmpHold => AMP_HLD,
            LfoDestination::AmpDecay => AMP_DEC,
            LfoDestination::AmpOverdrive => AMP_OVR,
            LfoDestination::AmpVolume => AMP_VOL,
            LfoDestination::AmpPan => AMP_PAN,
            LfoDestination::AmpAccent => AMP_ACC,
            LfoDestination::AmpDelaySend => AMP_DLY,
            LfoDestination::AmpReverbSend => AMP_REV,
        }
    }
}

/// Targets for sound modulation.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SoundModTarget {
    /// No target.
    #[default]
    Unset,
    /// LFO multiplier.
    LfoMultiplier,
    /// LFO waveform.
    LfoWaveform,
    /// LFO trigger mode.
    LfoTrigMode,
    /// LFO speed.
    LfoSpeed,
    /// LFO fade.
    LfoFade,
    /// LFO phase.
    LfoPhase,
    /// LFO depth.
    LfoDepth,
    /// Synth 1.
    Syn1,
    /// Synth 2.
    Syn2,
    /// Synth 3.
    Syn3,
    /// Synth 4.
    Syn4,
    /// Synth 5.
    Syn5,
    /// Synth 6.
    Syn6,
    /// Synth 7.
    Syn7,
    /// Synth 8.
    Syn8,
    /// Sample tune.
    SampleTune,
    /// Sample fine tune.
    SampleFineTune,
    /// Sample slice.
    SampleSlice,
    /// Sample bit reduction.
    SampleBitReduction,
    /// Sample start.
    SampleStart,
    /// Sample end.
    SampleEnd,
    /// Sample loop.
    SampleLoop,
    /// Sample level.
    SampleLevel,
    /// Filter envelope.
    FilterEnvelope,
    /// Filter attack.
    FilterAttack,
    /// Filter decay.
    FilterDecay,
    /// Filter sustain.
    FilterSustain,
    /// Filter release.
    FilterRelease,
    /// Filter frequency.
    FilterFrequency,
    /// Filter resonance.
    FilterResonance,
    /// Amp attack.
    AmpAttack,
    /// Amp hold.
    AmpHold,
    /// Amp decay.
    AmpDecay,
    /// Amp overdrive.
    AmpOverdrive,
    /// Amp volume.
    AmpVolume,
    /// Amp pan.
    AmpPan,
    /// Amp accent.
    AmpAccent,
    /// Amp delay send.
    AmpDelaySend,
    /// Amp reverb send.
    AmpReverbSend,
}

impl TryFrom<&str> for SoundModTarget {
    type Error = ConversionError;
    fn try_from(target: &str) -> Result<Self, Self::Error> {
        use SoundModTarget::*;
        match target {
            "unset" => Ok(Unset),
            "lfomultiplier" => Ok(LfoMultiplier),
            "lfowaveform" => Ok(LfoWaveform),
            "lfotrigmode" => Ok(LfoTrigMode),
            "lfospeed" => Ok(LfoSpeed),
            "lfofade" => Ok(LfoFade),
            "lfophase" => Ok(LfoPhase),
            "lfodepth" => Ok(LfoDepth),
            "syn1" => Ok(Syn1),
            "syn2" => Ok(Syn2),
            "syn3" => Ok(Syn3),
            "syn4" => Ok(Syn4),
            "syn5" => Ok(Syn5),
            "syn6" => Ok(Syn6),
            "syn7" => Ok(Syn7),
            "syn8" => Ok(Syn8),
            "sampletune" => Ok(SampleTune),
            "samplefinetune" => Ok(SampleFineTune),
            "sampleslice" => Ok(SampleSlice),
            "samplebitreduction" => Ok(SampleBitReduction),
            "samplestart" => Ok(SampleStart),
            "sampleend" => Ok(SampleEnd),
            "sampleloop" => Ok(SampleLoop),
            "samplelevel" => Ok(SampleLevel),
            "filterenvelope" => Ok(FilterEnvelope),
            "filterattack" => Ok(FilterAttack),
            "filterdecay" => Ok(FilterDecay),
            "filtersustain" => Ok(FilterSustain),
            "filterrelease" => Ok(FilterRelease),
            "filterfrequency" => Ok(FilterFrequency),
            "filterresonance" => Ok(FilterResonance),
            "ampattack" => Ok(AmpAttack),
            "amphold" => Ok(AmpHold),
            "ampdecay" => Ok(AmpDecay),
            "ampoverdrive" => Ok(AmpOverdrive),
            "ampvolume" => Ok(AmpVolume),
            "amppan" => Ok(AmpPan),
            "ampaccent" => Ok(AmpAccent),
            "ampdelaysend" => Ok(AmpDelaySend),
            "ampreverbsend" => Ok(AmpReverbSend),
            _ => Err(ConversionError::Range {
                value: target.to_string(),
                type_name: "SoundModTarget".to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for SoundModTarget {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use sound_mod_target::*;
        match value {
            NONE => Ok(Self::Unset),
            LFO_MULTIPLIER => Ok(Self::LfoMultiplier),
            LFO_WAVEFORM => Ok(Self::LfoWaveform),
            LFO_TRIGMODE => Ok(Self::LfoTrigMode),
            LFO_SPEED => Ok(Self::LfoSpeed),
            LFO_FADE => Ok(Self::LfoFade),
            LFO_PHASE => Ok(Self::LfoPhase),
            LFO_DEPTH => Ok(Self::LfoDepth),
            SYN_1 => Ok(Self::Syn1),
            SYN_2 => Ok(Self::Syn2),
            SYN_3 => Ok(Self::Syn3),
            SYN_4 => Ok(Self::Syn4),
            SYN_5 => Ok(Self::Syn5),
            SYN_6 => Ok(Self::Syn6),
            SYN_7 => Ok(Self::Syn7),
            SYN_8 => Ok(Self::Syn8),
            SMP_TUN => Ok(Self::SampleTune),
            SMP_FIN => Ok(Self::SampleFineTune),
            SMP_SMP => Ok(Self::SampleSlice),
            SMP_BR => Ok(Self::SampleBitReduction),
            SMP_STA => Ok(Self::SampleStart),
            SMP_END => Ok(Self::SampleEnd),
            SMP_LOP => Ok(Self::SampleLoop),
            SMP_LEV => Ok(Self::SampleLevel),
            FLT_ENV => Ok(Self::FilterEnvelope),
            FLT_ATK => Ok(Self::FilterAttack),
            FLT_DEC => Ok(Self::FilterDecay),
            FLT_SUS => Ok(Self::FilterSustain),
            FLT_REL => Ok(Self::FilterRelease),
            FLT_FRQ => Ok(Self::FilterFrequency),
            FLT_RES => Ok(Self::FilterResonance),
            AMP_ATK => Ok(Self::AmpAttack),
            AMP_HLD => Ok(Self::AmpHold),
            AMP_DEC => Ok(Self::AmpDecay),
            AMP_OVR => Ok(Self::AmpOverdrive),
            AMP_VOL => Ok(Self::AmpVolume),
            AMP_PAN => Ok(Self::AmpPan),
            AMP_ACC => Ok(Self::AmpAccent),
            AMP_DLY => Ok(Self::AmpDelaySend),
            AMP_REV => Ok(Self::AmpReverbSend),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "SoundModTarget".to_string(),
            }),
        }
    }
}

impl From<SoundModTarget> for u8 {
    fn from(value: SoundModTarget) -> Self {
        use sound_mod_target::*;
        match value {
            SoundModTarget::Unset => NONE,
            SoundModTarget::LfoMultiplier => LFO_MULTIPLIER,
            SoundModTarget::LfoWaveform => LFO_WAVEFORM,
            SoundModTarget::LfoTrigMode => LFO_TRIGMODE,
            SoundModTarget::LfoSpeed => LFO_SPEED,
            SoundModTarget::LfoFade => LFO_FADE,
            SoundModTarget::LfoPhase => LFO_PHASE,
            SoundModTarget::LfoDepth => LFO_DEPTH,
            SoundModTarget::Syn1 => SYN_1,
            SoundModTarget::Syn2 => SYN_2,
            SoundModTarget::Syn3 => SYN_3,
            SoundModTarget::Syn4 => SYN_4,
            SoundModTarget::Syn5 => SYN_5,
            SoundModTarget::Syn6 => SYN_6,
            SoundModTarget::Syn7 => SYN_7,
            SoundModTarget::Syn8 => SYN_8,
            SoundModTarget::SampleTune => SMP_TUN,
            SoundModTarget::SampleFineTune => SMP_FIN,
            SoundModTarget::SampleSlice => SMP_SMP,
            SoundModTarget::SampleBitReduction => SMP_BR,
            SoundModTarget::SampleStart => SMP_STA,
            SoundModTarget::SampleEnd => SMP_END,
            SoundModTarget::SampleLoop => SMP_LOP,
            SoundModTarget::SampleLevel => SMP_LEV,
            SoundModTarget::FilterEnvelope => FLT_ENV,
            SoundModTarget::FilterAttack => FLT_ATK,
            SoundModTarget::FilterDecay => FLT_DEC,
            SoundModTarget::FilterSustain => FLT_SUS,
            SoundModTarget::FilterRelease => FLT_REL,
            SoundModTarget::FilterFrequency => FLT_FRQ,
            SoundModTarget::FilterResonance => FLT_RES,
            SoundModTarget::AmpAttack => AMP_ATK,
            SoundModTarget::AmpHold => AMP_HLD,
            SoundModTarget::AmpDecay => AMP_DEC,
            SoundModTarget::AmpOverdrive => AMP_OVR,
            SoundModTarget::AmpVolume => AMP_VOL,
            SoundModTarget::AmpPan => AMP_PAN,
            SoundModTarget::AmpAccent => AMP_ACC,
            SoundModTarget::AmpDelaySend => AMP_DLY,
            SoundModTarget::AmpReverbSend => AMP_REV,
        }
    }
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
/// Filter type of a filter.
pub enum FilterType {
    #[default]
    Lp2,
    Lp1,
    Bp,
    Hp1,
    Hp2,
    Bs,
    Pk,
}

impl TryFrom<&str> for FilterType {
    type Error = ConversionError;
    fn try_from(filter: &str) -> Result<Self, Self::Error> {
        use FilterType::*;
        match filter {
            "lp2" => Ok(Lp2),
            "lp1" => Ok(Lp1),
            "bp" => Ok(Bp),
            "hp1" => Ok(Hp1),
            "hp2" => Ok(Hp2),
            "bs" => Ok(Bs),
            "pk" => Ok(Pk),
            _ => Err(ConversionError::Range {
                value: filter.to_string(),
                type_name: "FilterType".to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for FilterType {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Lp2),
            1 => Ok(Self::Lp1),
            2 => Ok(Self::Bp),
            3 => Ok(Self::Hp1),
            4 => Ok(Self::Hp2),
            5 => Ok(Self::Bs),
            6 => Ok(Self::Pk),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "FilterType".to_string(),
            }),
        }
    }
}

impl From<FilterType> for u8 {
    fn from(value: FilterType) -> Self {
        match value {
            FilterType::Lp2 => 0,
            FilterType::Lp1 => 1,
            FilterType::Bp => 2,
            FilterType::Hp1 => 3,
            FilterType::Hp2 => 4,
            FilterType::Bs => 5,
            FilterType::Pk => 6,
        }
    }
}

/// Multiplier for an LFO.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum LfoMultiplier {
    /// x1
    X1,
    /// x2
    X2,
    /// x4
    X4,
    /// x8
    X8,
    #[default]
    /// x16
    X16,
    /// x32
    X32,
    /// x64
    X64,
    /// x128
    X128,
    /// x256
    X256,
    /// x512
    X512,
    /// x1k
    X1k,
    /// x2k
    X2k,
    /// .1
    _D1,
    /// .2
    _D2,
    /// .4
    _D4,
    /// .8
    _D8,
    /// .16
    _D16,
    /// .32
    _D32,
    /// .64
    _D64,
    /// .128
    _D128,
    /// .256
    _D256,
    /// .512
    _D512,
    /// .1k
    _D1k,
    /// .2k
    _D2k,
}

impl TryFrom<&str> for LfoMultiplier {
    type Error = ConversionError;
    fn try_from(multiplier: &str) -> Result<Self, Self::Error> {
        use LfoMultiplier::*;
        match multiplier {
            "x1" => Ok(X1),
            "x2" => Ok(X2),
            "x4" => Ok(X4),
            "x8" => Ok(X8),
            "x16" => Ok(X16),
            "x32" => Ok(X32),
            "x64" => Ok(X64),
            "x128" => Ok(X128),
            "x256" => Ok(X256),
            "x512" => Ok(X512),
            "x1k" => Ok(X1k),
            "x2k" => Ok(X2k),
            ".1" => Ok(_D1),
            ".2" => Ok(_D2),
            ".4" => Ok(_D4),
            ".8" => Ok(_D8),
            ".16" => Ok(_D16),
            ".32" => Ok(_D32),
            ".64" => Ok(_D64),
            ".128" => Ok(_D128),
            ".256" => Ok(_D256),
            ".512" => Ok(_D512),
            ".1k" => Ok(_D1k),
            ".2k" => Ok(_D2k),
            _ => Err(ConversionError::Range {
                value: multiplier.to_string(),
                type_name: "LfoMultiplier".to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for LfoMultiplier {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::X1),
            1 => Ok(Self::X2),
            2 => Ok(Self::X4),
            3 => Ok(Self::X8),
            4 => Ok(Self::X16),
            5 => Ok(Self::X32),
            6 => Ok(Self::X64),
            7 => Ok(Self::X128),
            8 => Ok(Self::X256),
            9 => Ok(Self::X512),
            10 => Ok(Self::X1k),
            11 => Ok(Self::X2k),
            12 => Ok(Self::_D1),
            13 => Ok(Self::_D2),
            14 => Ok(Self::_D4),
            15 => Ok(Self::_D8),
            16 => Ok(Self::_D16),
            17 => Ok(Self::_D32),
            18 => Ok(Self::_D64),
            19 => Ok(Self::_D128),
            20 => Ok(Self::_D256),
            21 => Ok(Self::_D512),
            22 => Ok(Self::_D1k),
            23 => Ok(Self::_D2k),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "LfoMultiplier".to_string(),
            }),
        }
    }
}

impl From<LfoMultiplier> for u8 {
    fn from(value: LfoMultiplier) -> Self {
        match value {
            LfoMultiplier::X1 => 0,
            LfoMultiplier::X2 => 1,
            LfoMultiplier::X4 => 2,
            LfoMultiplier::X8 => 3,
            LfoMultiplier::X16 => 4,
            LfoMultiplier::X32 => 5,
            LfoMultiplier::X64 => 6,
            LfoMultiplier::X128 => 7,
            LfoMultiplier::X256 => 8,
            LfoMultiplier::X512 => 9,
            LfoMultiplier::X1k => 10,
            LfoMultiplier::X2k => 11,
            LfoMultiplier::_D1 => 12,
            LfoMultiplier::_D2 => 13,
            LfoMultiplier::_D4 => 14,
            LfoMultiplier::_D8 => 15,
            LfoMultiplier::_D16 => 16,
            LfoMultiplier::_D32 => 17,
            LfoMultiplier::_D64 => 18,
            LfoMultiplier::_D128 => 19,
            LfoMultiplier::_D256 => 20,
            LfoMultiplier::_D512 => 21,
            LfoMultiplier::_D1k => 22,
            LfoMultiplier::_D2k => 23,
        }
    }
}

/// The shape of an LFO wave.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum LfoWaveform {
    #[default]
    Tri,
    Sin,
    Sqr,
    Saw,
    Exp,
    Rmp,
    Rnd,
}

impl TryFrom<&str> for LfoWaveform {
    type Error = ConversionError;
    fn try_from(waveform: &str) -> Result<Self, Self::Error> {
        use LfoWaveform::*;
        match waveform {
            "tri" => Ok(Tri),
            "sin" => Ok(Sin),
            "sqr" => Ok(Sqr),
            "saw" => Ok(Saw),
            "exp" => Ok(Exp),
            "rmp" => Ok(Rmp),
            "rnd" => Ok(Rnd),
            _ => Err(ConversionError::Range {
                value: waveform.to_string(),
                type_name: "LfoWaveform".to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for LfoWaveform {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Tri),
            1 => Ok(Self::Sin),
            2 => Ok(Self::Sqr),
            3 => Ok(Self::Saw),
            4 => Ok(Self::Exp),
            5 => Ok(Self::Rmp),
            6 => Ok(Self::Rnd),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "LfoWaveform".to_string(),
            }),
        }
    }
}

impl From<LfoWaveform> for u8 {
    fn from(value: LfoWaveform) -> Self {
        match value {
            LfoWaveform::Tri => 0,
            LfoWaveform::Sin => 1,
            LfoWaveform::Sqr => 2,
            LfoWaveform::Saw => 3,
            LfoWaveform::Exp => 4,
            LfoWaveform::Rmp => 5,
            LfoWaveform::Rnd => 6,
        }
    }
}

/// The mode of an LFO.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum LfoMode {
    #[default]
    Free,
    Trig,
    Hold,
    One,
    Half,
}

impl TryFrom<&str> for LfoMode {
    type Error = ConversionError;
    fn try_from(mode: &str) -> Result<Self, Self::Error> {
        use LfoMode::*;
        match mode {
            "free" => Ok(Free),
            "trig" => Ok(Trig),
            "hold" => Ok(Hold),
            "one" => Ok(One),
            "half" => Ok(Half),
            _ => Err(ConversionError::Range {
                value: mode.to_string(),
                type_name: "LfoMode".to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for LfoMode {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Free),
            1 => Ok(Self::Trig),
            2 => Ok(Self::Hold),
            3 => Ok(Self::One),
            4 => Ok(Self::Half),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "LfoMode".to_string(),
            }),
        }
    }
}

impl From<LfoMode> for u8 {
    fn from(value: LfoMode) -> Self {
        match value {
            LfoMode::Free => 0,
            LfoMode::Trig => 1,
            LfoMode::Hold => 2,
            LfoMode::One => 3,
            LfoMode::Half => 4,
        }
    }
}

/// The chromatic mode of a sound.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SoundSettingsChromaticMode {
    Off,
    Synth,
    Sample,
    #[default]
    SynthAndSample,
}

impl TryFrom<&str> for SoundSettingsChromaticMode {
    type Error = ConversionError;
    fn try_from(mode: &str) -> Result<Self, Self::Error> {
        use SoundSettingsChromaticMode::*;
        // TODO: Double check naming
        match mode {
            "off" => Ok(Off),
            "syn" => Ok(Synth),
            "samp" => Ok(Sample),
            "syn+samp" => Ok(SynthAndSample),
            _ => Err(ConversionError::Range {
                value: mode.to_string(),
                type_name: "SoundSettingsChromaticMode".to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for SoundSettingsChromaticMode {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::Synth),
            2 => Ok(Self::Sample),
            3 => Ok(Self::SynthAndSample),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "SoundSettingsChromaticMode".to_string(),
            }),
        }
    }
}

impl From<SoundSettingsChromaticMode> for u8 {
    fn from(value: SoundSettingsChromaticMode) -> Self {
        match value {
            SoundSettingsChromaticMode::Off => 0,
            SoundSettingsChromaticMode::Synth => 1,
            SoundSettingsChromaticMode::Sample => 2,
            SoundSettingsChromaticMode::SynthAndSample => 3,
        }
    }
}
