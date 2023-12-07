use crate::error::ConversionError;

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

mod sound_mod_target {
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

use machines::*;

/// Targets for sound modulation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Destination of an LFO.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// The machine type of a sound.
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Filter type of a sound.
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

// TODO:
#[derive(Clone, Copy, Debug)]
pub struct SynthParameter {
    inner: u16,
}

impl SynthParameter {
    pub fn new(inner: u16) -> Self {
        Self { inner }
    }
}

//    s_u16_t synth_param_1;     /* @0x001C  (LSB is always 0)
//                                           0:bd hard     : lev
//                                           1:bd classic  : lev
//                                           2:sd hard     : lev
//                                           3:sd classic  : lev
//                                           4:rs hard     : lev
//                                           5:rs classic  : lev
//                                           6:cp classic  : lev
//                                           7:bt classic  : lev
//                                           8:xt classic  : lev
//                                           9:ch classic  : lev
//                                          10:oh classic  : lev
//                                          11:cy classic  : lev
//                                          12:cb classic  : lev
//                                          13:bd fm       : lev
//                                          14:sd fm       : lev
//                                          15:ut noise    : lev
//                                          16:ut impulse  : lev
//                                          17:ch metallic : lev
//                                          18:oh metallic : lev
//                                          19:cy metallic : lev
//                                          20:cb metallic : lev
//                                          21:bd plastic  : lev
//                                          22:bd silky    : lev
//                                          23:sd natural  : lev
//                                          24:hh basic    : lev
//                                          25:cy ride     : lev
//                                          26:bd sharp    : lev
//                                          27:DISABLE     : -
//                                          28:sy dual vco : lev
//                                          29:sy chip     : lev
//                                          30:bd acoustic : lev
//                                          31:sd acoustic : lev
//                                          32:sy raw      : lev
//                                          33:hh lab      : lev
//                                */
//    s_u16_t synth_param_2;     /* @0x001E  (some machines use LSB since FW1.70
//                                           0:bd hard     : tun (64=+0)
//                                           1:bd classic  : tun (64=+0)
//                                           2:sd hard     : tun (64=+0)
//                                           3:sd classic  : tun (64=+0)
//                                           4:rs hard     : tun (64=+0)
//                                           5:rs classic  : t1  (64=+0)
//                                           6:cp classic  : ton (0..127)
//                                           7:bt classic  : tun (64=+0)
//                                           8:xt classic  : tun (64=+0)
//                                           9:ch classic  : tun (64=+0)
//                                          10:oh classic  : tun (64=+0)
//                                          11:cy classic  : tun (64=+0)
//                                          12:cb classic  : tun (64=+0)
//                                          13:bd fm       : tun (64=+0)
//                                          14:sd fm       : tun (64=+0)
//                                          15:ut noise    : lpf
//                                          16:ut impulse  : atk
//                                          17:ch metallic : tun (64=+0)
//                                          18:oh metallic : tun (64=+0)
//                                          19:cy metallic : tun (64=+0)
//                                          20:cb metallic : tun (64=+0)
//                                          21:bd plastic  : tun (64=+0)
//                                          22:bd silky    : tun (64=+0)
//                                          23:sd natural  : tun (64=+0)
//                                          24:hh basic    : tun (64=+0)
//                                          25:cy ride     : tun (64=+0)
//                                          26:bd sharp    : tun (64=+0)
//                                          27:DISABLE     : -
//                                          28:sy dual vco : tun (64=+0)
//                                          29:sy chip     : tun (64=+0) (uses LSB)
//                                          30:bd acoustic : tun (64=+0) (uses LSB)
//                                          31:sd acoustic : tun (64=+0) (uses LSB)
//                                          32:sy raw      : tun (64=+0) (uses LSB)
//                                          33:hh lab      : osc1 (uses 8bit? LSB)
//                               */
//    s_u16_t synth_param_3;     /* @0x0020  (LSB is always 0)
//                                           0:bd hard     : dec
//                                           1:bd classic  : dec
//                                           2:sd hard     : dec
//                                           3:sd classic  : dec
//                                           4:rs hard     : dec
//                                           5:rs classic  : dec
//                                           6:cp classic  : nod
//                                           7:bt classic  : dec
//                                           8:xt classic  : dec
//                                           9:ch classic  : dec
//                                          10:oh classic  : dec
//                                          11:cy classic  : dec
//                                          12:cb classic  : dec
//                                          13:bd fm       : dec
//                                          14:sd fm       : dec
//                                          15:ut noise    : dec
//                                          16:ut impulse  : dec
//                                          17:ch metallic : dec
//                                          18:oh metallic : dec
//                                          19:cy metallic : dec
//                                          20:cb metallic : dec
//                                          21:bd plastic  : dec
//                                          22:bd silky    : dec
//                                          23:sd natural  : bdy
//                                          24:hh basic    : dec
//                                          25:cy ride     : dec
//                                          26:bd sharp    : dec
//                                          27:DISABLE     : -
//                                          28:sy dual vco : dec#1
//                                          29:sy chip     : dcy
//                                          30:bd acoustic : dec
//                                          31:sd acoustic : bdy
//                                          32:sy raw      : dcy (0..126,127=inf)
//                                          33:hh lab      : dec
//                               */
//    s_u16_t synth_param_4;     /* @0x0022  (some machines use LSB since FW1.70)
//                                           0:bd hard     : hld
//                                           1:bd classic  : hld
//                                           2:sd hard     : swd
//                                           3:sd classic  : det
//                                           4:rs hard     : swd
//                                           5:rs classic  : bal (64=+0)
//                                           6:cp classic  : num
//                                           7:bt classic  : -
//                                           8:xt classic  : swd
//                                           9:ch classic  : col (64=+0)
//                                          10:oh classic  : col (64=+0)
//                                          11:cy classic  : col (64=+0)
//                                          12:cb classic  : det
//                                          13:bd fm       : fma
//                                          14:sd fm       : fmt (64=+0)
//                                          15:ut noise    : hpf
//                                          16:ut impulse  : -
//                                          17:ch metallic : -
//                                          18:oh metallic : -
//                                          19:cy metallic : ton (64=+0)
//                                          20:cb metallic : det
//                                          21:bd plastic  : typ
//                                          22:bd silky    : hld
//                                          23:sd natural  : dec
//                                          24:hh basic    : ton (64=+0)
//                                          25:cy ride     : typ (0..3=A..D)
//                                          26:bd sharp    : hld
//                                          27:DISABLE     : -
//                                          28:sy dual vco : det
//                                          29:sy chip     : of2 (40=-24..64=+0..88=+24)
//                                          30:bd acoustic : hld
//                                          31:sd acoustic : nod
//                                          32:sy raw      : det (64=+0) (uses LSB)
//                                          33:hh lab      : osc2 (uses 8bit? LSB)
//                               */
//    s_u16_t synth_param_5;     /* @0x0024  ("hh lab" uses LSB since FW1.70)
//                                           0:bd hard     : swt
//                                           1:bd classic  : swt
//                                           2:sd hard     : tic
//                                           3:sd classic  : snp
//                                           4:rs hard     : tic
//                                           5:rs classic  : t2  (64=+0)
//                                           6:cp classic  : rat
//                                           7:bt classic  : nol
//                                           8:xt classic  : swt
//                                           9:ch classic  : -
//                                          10:oh classic  : -
//                                          11:cy classic  : ton (64=+0)
//                                          12:cb classic  : pw1 (64=+0)
//                                          13:bd fm       : swt
//                                          14:sd fm       : fmd
//                                          15:ut noise    : lpq
//                                          16:ut impulse  : -
//                                          17:ch metallic : -
//                                          18:oh metallic : -
//                                          19:cy metallic : trd
//                                          20:cb metallic : pw1 (64=+0)
//                                          21:bd plastic  : mod
//                                          22:bd silky    : swt
//                                          23:sd natural  : bal (0..127)
//                                          24:hh basic    : trd
//                                          25:cy ride     : hit
//                                          26:bd sharp    : swt
//                                          27:DISABLE     : -
//                                          28:sy dual vco : dec#2
//                                          29:sy chip     : of3 (40=-24..64=+0..88=+24)
//                                          30:bd acoustic : swt
//                                          31:sd acoustic : nol
//                                          32:sy raw      : nol
//                                          33:hh lab      : osc3 (uses 8bit? LSB)
//                               */
//    s_u16_t synth_param_6;     /* @0x0026  ("hh lab" uses LSB since FW1.70)
//                                           0:bd hard     : snp
//                                           1:bd classic  : swd
//                                           2:sd hard     : nod
//                                           3:sd classic  : nod
//                                           4:rs hard     : nol
//                                           5:rs classic  : sym (64=+0)
//                                           6:cp classic  : nol
//                                           7:bt classic  : snp (0..3)
//                                           8:xt classic  : nod
//                                           9:ch classic  : -
//                                          10:oh classic  : -
//                                          11:cy classic  : -
//                                          12:cb classic  : pw2 (64=+0)
//                                          13:bd fm       : fms
//                                          14:sd fm       : nod
//                                          15:ut noise    : atk
//                                          16:ut impulse  : -
//                                          17:ch metallic : -
//                                          18:oh metallic : -
//                                          19:cy metallic : -
//                                          20:cb metallic : pw2 (64=+0)
//                                          21:bd plastic  : swt
//                                          22:bd silky    : swd
//                                          23:sd natural  : lpf
//                                          24:hh basic    : rst (0 or 1)
//                                          25:cy ride     : c1
//                                          26:bd sharp    : swd
//                                          27:DISABLE     : -
//                                          28:sy dual vco : bal (64=+0)
//                                          29:sy chip     : of4 (40=-24..64=+0..88=+24)
//                                          30:bd acoustic : swd
//                                          31:sd acoustic : hld
//                                          32:sy raw      : wav1 (0=sin,1=asin,2=tri,3=ssaw,4=asaw,5=saw,6=ring)
//                                          33:hh lab      : osc4 (uses 8bit? LSB)
//                               */
//    s_u16_t synth_param_7;     /* @0x0028  ("hh lab" uses LSB since FW1.70)
//                                           0:bd hard     : wav
//                                           1:bd classic  : wav
//                                           2:sd hard     : nol
//                                           3:sd classic  : nol
//                                           4:rs hard     : syn
//                                           5:rs classic  : nol
//                                           6:cp classic  : rnd
//                                           7:bt classic  : swd (FW1.70)
//                                           8:xt classic  : nol
//                                           9:ch classic  : -
//                                          10:oh classic  : -
//                                          11:cy classic  : -
//                                          12:cb classic  : -
//                                          13:bd fm       : fmd
//                                          14:sd fm       : nol
//                                          15:ut noise    : swt
//                                          16:ut impulse  : -
//                                          17:ch metallic : -
//                                          18:oh metallic : -
//                                          19:cy metallic : -
//                                          20:cb metallic : -
//                                          21:bd plastic  : swd
//                                          22:bd silky    : dus
//                                          23:sd natural  : hpf
//                                          24:hh basic    : -
//                                          25:cy ride     : c2
//                                          26:bd sharp    : wav (0=sinA,1=sinB,2=asinA,3=asinB,4=triA,5=triB,6=ssawA,7=ssawB,8=sawA,9=sawB,10=sqrA,11=sqrB)
//                                          27:DISABLE     : -
//                                          28:sy dual vco : bnd (64=+0)
//                                          29:sy chip     : wav (0=sin,1=asin,2=tri,3=ssaw,4=saw,5=sqr,6=noise,
//                                                                7=anm1,8=anm2,9=anm3,10=anm4,11=anm5,
//                                                                12=pwm+,13=pwm-,
//                                                                14=triB,15=+tri,16=tri+,17=triX,
//                                                                18=sawB,19=+saw,20=saw+,21=sawX,
//                                                                22=sqrB,23=+sqr,24=sqr+,25=sqrX
//                                                                26=tbl1,27=tbl2,28=tbl3,
//                                                                29=p1%..127=p99%)
//                                          30:bd acoustic : wav (0=sinA,1=sinB,2=asinA,3=asinB,4=triA,5=triB,6=ssawA,7=ssawB,8=sawA,9=sawB,10=sqrA,11=sqrB)
//                                          31:sd acoustic : swd
//                                          32:sy raw      : wav2 (0=sineA,1=ssawA,2=sineB,3=ssawB)
//                                          33:hh lab      : osc5 (uses 8bit? LSB)
//                               */
//    s_u16_t synth_param_8;     /* @0x002A  ("hh lab" uses LSB since FW1.70)
//                                           0:bd hard     : tic
//                                           1:bd classic  : tra
//                                           2:sd hard     : swt
//                                           3:sd classic  : bal (64=+0)
//                                           4:rs hard     : swt
//                                           5:rs classic  : tic
//                                           6:cp classic  : cpd
//                                           7:bt classic  : -
//                                           8:xt classic  : ton (64=+0)
//                                           9:ch classic  : -
//                                          10:oh classic  : -
//                                          11:cy classic  : -
//                                          12:cb classic  : -
//                                          13:bd fm       : fmt (64=+0)
//                                          14:sd fm       : fma
//                                          15:ut noise    : swd (64=+0)
//                                          16:ut impulse  : pol (0 or 1)
//                                          17:ch metallic : -
//                                          18:oh metallic : -
//                                          19:cy metallic : -
//                                          20:cb metallic : -
//                                          21:bd plastic  : tic
//                                          22:bd silky    : clk
//                                          23:sd natural  : res
//                                          24:hh basic    : -
//                                          25:cy ride     : c3
//                                          26:bd sharp    : tic
//                                          27:DISABLE     : -
//                                          28:sy dual vco : cfg (0..79)
//                                          29:sy chip     : spd (0=128T,1=128,2=64T,3=128d,4=64,5=32T,6=64d,7=32,8=16T,9=32d,10=16,11=8T,
//                                                                12=16d,13=8,14=4T,15=8d,16=4,17=2T,18=4d,19=2,20=1T,21=2d,22=1,23=1d,24=1.0Hz,
//                                                                25=1.56Hz,26=1.88Hz,27=2Hz,28=3.13Hz,29=3.75Hz,30=4Hz,31=5Hz,32=6.25Hz,33=7.5Hz,34=10Hz,
//                                                                35=12.5Hz,36=15Hz,37=20Hz,38=25Hz,39=30Hz,40=40Hz,41=50Hz,42=60Hz,43=75Hz,
//                                                                44=100Hz,45=120Hz,46=150Hz,47=180Hz,48=200Hz,49=240Hz,50=250Hz,
//                                                                51=300Hz,52=350Hz,53=360Hz,54=400Hz,55=420Hz,56=480Hz,57=240 5Hz,
//                                                                58=200 5Hz,59=150 5Hz,60=120 5Hz,61=100 5Hz,62=60 5Hz,63=50 5Hz,64=30 5Hz,65=25 5Hz
//                                                                )
//                                          30:bd acoustic : imp
//                                          31:sd acoustic : imp
//                                          32:sy raw      : bal (64=+0)
//                                          33:hh lab      : osc6 (uses 8bit? LSB)
//                               */
pub enum SynthParameterType {}

/// LFO multiplier of a sound.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// The shape of the LFO wave.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// The mode of the LFO.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LfoMode {
    #[default]
    Free,
    Trig,
    Hold,
    One,
    Half,
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SoundSettingsChromaticMode {
    Off,
    Synth,
    Sample,
    #[default]
    SynthAndSample,
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
