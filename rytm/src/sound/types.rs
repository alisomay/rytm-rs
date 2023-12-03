use crate::error::ConversionError;

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

use machines::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Machine {
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

impl std::fmt::Display for Machine {
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
        write!(f, "{}", machine)
    }
}

impl TryFrom<u8> for Machine {
    type Error = ConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

impl From<Machine> for u8 {
    fn from(value: Machine) -> Self {
        match value {
            Machine::BdHard => BD_HARD,
            Machine::BdClassic => BD_CLASSIC,
            Machine::SdHard => SD_HARD,
            Machine::SdClassic => SD_CLASSIC,
            Machine::RsHard => RS_HARD,
            Machine::RsClassic => RS_CLASSIC,
            Machine::CpClassic => CP_CLASSIC,
            Machine::BtClassic => BT_CLASSIC,
            Machine::XtClassic => XT_CLASSIC,
            Machine::ChClassic => CH_CLASSIC,
            Machine::OhClassic => OH_CLASSIC,
            Machine::CyClassic => CY_CLASSIC,
            Machine::CbClassic => CB_CLASSIC,
            Machine::BdFm => BD_FM,
            Machine::SdFm => SD_FM,
            Machine::UtNoise => UT_NOISE,
            Machine::UtImpulse => UT_IMPULSE,
            Machine::ChMetallic => CH_METALLIC,
            Machine::OhMetallic => OH_METALLIC,
            Machine::CyMetallic => CY_METALLIC,
            Machine::CbMetallic => CB_METALLIC,
            Machine::BdPlastic => BD_PLASTIC,
            Machine::BdSilky => BD_SILKY,
            Machine::SdNatural => SD_NATURAL,
            Machine::HhBasic => HH_BASIC,
            Machine::CyRide => CY_RIDE,
            Machine::BdSharp => BD_SHARP,
            Machine::Disable => DISABLE,
            Machine::SyDualVco => SY_DUAL_VCO,
            Machine::SyChip => SY_CHIP,
            Machine::BdAcoustic => BD_ACOUSTIC,
            Machine::SdAcoustic => SD_ACOUSTIC,
            Machine::SyRaw => SY_RAW,
            Machine::HhLab => HH_LAB,
            Machine::Unset => UNSET,
        }
    }
}
