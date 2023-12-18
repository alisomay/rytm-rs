use crate::error::ConversionError;

const PAD_SCALE_ROOT_NOTE_BASE: u8 = 96;
// Import all the PADSCALE_* constants from rytm_sys
use rytm_sys::{
    AR_PADSCALE_AEOLIAN_MINOR, AR_PADSCALE_BLUES, AR_PADSCALE_COMBO_MINOR, AR_PADSCALE_DORIAN,
    AR_PADSCALE_DORIAN_B2, AR_PADSCALE_DOUBLE_HARMONIC_MAJOR, AR_PADSCALE_HALF_WHOLE_DIMINISHED,
    AR_PADSCALE_HARMONIC_MINOR, AR_PADSCALE_HIRAJOSHI, AR_PADSCALE_HUNGARIAN_MINOR,
    AR_PADSCALE_IN_SEN, AR_PADSCALE_IONIAN_2_5, AR_PADSCALE_IONIAN_MAJOR, AR_PADSCALE_IWATO,
    AR_PADSCALE_LOCRIAN, AR_PADSCALE_LOCRIAN_BB3_BB7, AR_PADSCALE_LYDIAN, AR_PADSCALE_LYDIAN_2_6,
    AR_PADSCALE_LYDIAN_AUGMENTED, AR_PADSCALE_LYDIAN_DOMINANT, AR_PADSCALE_MAJOR_LOCRIAN,
    AR_PADSCALE_MELODIC_MINOR, AR_PADSCALE_MIXOLYDIAN, AR_PADSCALE_ORIENTAL, AR_PADSCALE_PELOG,
    AR_PADSCALE_PENTATONIC_MAJOR, AR_PADSCALE_PENTATONIC_MINOR, AR_PADSCALE_PERSIAN,
    AR_PADSCALE_PHRYGIAN, AR_PADSCALE_PHRYGIAN_DOMINANT, AR_PADSCALE_SPANISH,
    AR_PADSCALE_SUPER_LOCRIAN, AR_PADSCALE_ULTRAPHRYGIAN, AR_PADSCALE_WHOLE_HALF_DIMINISHED,
    AR_PADSCALE_WHOLE_TONE,
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum RootNote {
    #[default]
    C,
    CSharp,
    D,
    EFlat,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    BFlat,
    B,
}

impl From<RootNote> for u8 {
    fn from(note: RootNote) -> Self {
        match note {
            RootNote::C => 0,
            RootNote::CSharp => 1,
            RootNote::D => 2,
            RootNote::EFlat => 3,
            RootNote::E => 4,
            RootNote::F => 5,
            RootNote::FSharp => 6,
            RootNote::G => 7,
            RootNote::GSharp => 8,
            RootNote::A => 9,
            RootNote::BFlat => 10,
            RootNote::B => 11,
        }
    }
}

impl TryFrom<u8> for RootNote {
    type Error = ConversionError;

    // TODO:
    // When a pattern is sent (or maybe something else happens) the notes are between 0 and 11
    // But by default they are between 96 and 108
    // This indicates probably a flag encoding somewhere in the data.
    // I'm a bit lazy to figure out what it is, so I'm just going to do a quick hack here.
    // But if a lost flag is waiting to be discovered in this context one can start analyzing this behaviour.
    fn try_from(note: u8) -> Result<Self, Self::Error> {
        match note {
            0 | 96 => Ok(Self::C),
            1 | 97 => Ok(Self::CSharp),
            2 | 98 => Ok(Self::D),
            3 | 99 => Ok(Self::EFlat),
            4 | 100 => Ok(Self::E),
            5 | 101 => Ok(Self::F),
            6 | 102 => Ok(Self::FSharp),
            7 | 103 => Ok(Self::G),
            8 | 104 => Ok(Self::GSharp),
            9 | 105 => Ok(Self::A),
            10 | 107 => Ok(Self::BFlat),
            11 | 108 => Ok(Self::B),
            _ => Err(ConversionError::Range {
                value: note.to_string(),
                type_name: "RootNote".into(),
            }),
        }
    }
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum PadScale {
    #[default]
    Chromatic,
    IonianMajor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    AeolianMinor,
    Locrian,
    PentatonicMinor,
    PentatonicMajor,
    MelodicMinor,
    HarmonicMinor,
    WholeTone,
    Blues,
    ComboMinor,
    Persian,
    Iwato,
    InSen,
    Hirajoshi,
    Pelog,
    PhrygianDominant,
    WholeHalfDiminished,
    HalfWholeDiminished,
    Spanish,
    MajorLocrian,
    SuperLocrian,
    DorianB2,
    LydianAugmented,
    LydianDominant,
    DoubleHarmonicMajor,
    Lydian26,
    Ultraphrygian,
    HungarianMinor,
    Oriental,
    Ionian25,
    LocrianBb3Bb7,
}

impl From<PadScale> for u8 {
    fn from(scale: PadScale) -> Self {
        let scale = match scale {
            PadScale::Chromatic => 0xFF,
            PadScale::IonianMajor => AR_PADSCALE_IONIAN_MAJOR,
            PadScale::Dorian => AR_PADSCALE_DORIAN,
            PadScale::Phrygian => AR_PADSCALE_PHRYGIAN,
            PadScale::Lydian => AR_PADSCALE_LYDIAN,
            PadScale::Mixolydian => AR_PADSCALE_MIXOLYDIAN,
            PadScale::AeolianMinor => AR_PADSCALE_AEOLIAN_MINOR,
            PadScale::Locrian => AR_PADSCALE_LOCRIAN,
            PadScale::PentatonicMinor => AR_PADSCALE_PENTATONIC_MINOR,
            PadScale::PentatonicMajor => AR_PADSCALE_PENTATONIC_MAJOR,
            PadScale::MelodicMinor => AR_PADSCALE_MELODIC_MINOR,
            PadScale::HarmonicMinor => AR_PADSCALE_HARMONIC_MINOR,
            PadScale::WholeTone => AR_PADSCALE_WHOLE_TONE,
            PadScale::Blues => AR_PADSCALE_BLUES,
            PadScale::ComboMinor => AR_PADSCALE_COMBO_MINOR,
            PadScale::Persian => AR_PADSCALE_PERSIAN,
            PadScale::Iwato => AR_PADSCALE_IWATO,
            PadScale::InSen => AR_PADSCALE_IN_SEN,
            PadScale::Hirajoshi => AR_PADSCALE_HIRAJOSHI,
            PadScale::Pelog => AR_PADSCALE_PELOG,
            PadScale::PhrygianDominant => AR_PADSCALE_PHRYGIAN_DOMINANT,
            PadScale::WholeHalfDiminished => AR_PADSCALE_WHOLE_HALF_DIMINISHED,
            PadScale::HalfWholeDiminished => AR_PADSCALE_HALF_WHOLE_DIMINISHED,
            PadScale::Spanish => AR_PADSCALE_SPANISH,
            PadScale::MajorLocrian => AR_PADSCALE_MAJOR_LOCRIAN,
            PadScale::SuperLocrian => AR_PADSCALE_SUPER_LOCRIAN,
            PadScale::DorianB2 => AR_PADSCALE_DORIAN_B2,
            PadScale::LydianAugmented => AR_PADSCALE_LYDIAN_AUGMENTED,
            PadScale::LydianDominant => AR_PADSCALE_LYDIAN_DOMINANT,
            PadScale::DoubleHarmonicMajor => AR_PADSCALE_DOUBLE_HARMONIC_MAJOR,
            PadScale::Lydian26 => AR_PADSCALE_LYDIAN_2_6,
            PadScale::Ultraphrygian => AR_PADSCALE_ULTRAPHRYGIAN,
            PadScale::HungarianMinor => AR_PADSCALE_HUNGARIAN_MINOR,
            PadScale::Oriental => AR_PADSCALE_ORIENTAL,
            PadScale::Ionian25 => AR_PADSCALE_IONIAN_2_5,
            PadScale::LocrianBb3Bb7 => AR_PADSCALE_LOCRIAN_BB3_BB7,
        };
        scale as Self
    }
}

impl TryFrom<u8> for PadScale {
    type Error = ConversionError;

    fn try_from(scale: u8) -> Result<Self, Self::Error> {
        match scale as u32 {
            0xFF => Ok(Self::Chromatic),
            AR_PADSCALE_IONIAN_MAJOR => Ok(Self::IonianMajor),
            AR_PADSCALE_DORIAN => Ok(Self::Dorian),
            AR_PADSCALE_PHRYGIAN => Ok(Self::Phrygian),
            AR_PADSCALE_LYDIAN => Ok(Self::Lydian),
            AR_PADSCALE_MIXOLYDIAN => Ok(Self::Mixolydian),
            AR_PADSCALE_AEOLIAN_MINOR => Ok(Self::AeolianMinor),
            AR_PADSCALE_LOCRIAN => Ok(Self::Locrian),
            AR_PADSCALE_PENTATONIC_MINOR => Ok(Self::PentatonicMinor),
            AR_PADSCALE_PENTATONIC_MAJOR => Ok(Self::PentatonicMajor),
            AR_PADSCALE_MELODIC_MINOR => Ok(Self::MelodicMinor),
            AR_PADSCALE_HARMONIC_MINOR => Ok(Self::HarmonicMinor),
            AR_PADSCALE_WHOLE_TONE => Ok(Self::WholeTone),
            AR_PADSCALE_BLUES => Ok(Self::Blues),
            AR_PADSCALE_COMBO_MINOR => Ok(Self::ComboMinor),
            AR_PADSCALE_PERSIAN => Ok(Self::Persian),
            AR_PADSCALE_IWATO => Ok(Self::Iwato),
            AR_PADSCALE_IN_SEN => Ok(Self::InSen),
            AR_PADSCALE_HIRAJOSHI => Ok(Self::Hirajoshi),
            AR_PADSCALE_PELOG => Ok(Self::Pelog),
            AR_PADSCALE_PHRYGIAN_DOMINANT => Ok(Self::PhrygianDominant),
            AR_PADSCALE_WHOLE_HALF_DIMINISHED => Ok(Self::WholeHalfDiminished),
            AR_PADSCALE_HALF_WHOLE_DIMINISHED => Ok(Self::HalfWholeDiminished),
            AR_PADSCALE_SPANISH => Ok(Self::Spanish),
            AR_PADSCALE_MAJOR_LOCRIAN => Ok(Self::MajorLocrian),
            AR_PADSCALE_SUPER_LOCRIAN => Ok(Self::SuperLocrian),
            AR_PADSCALE_DORIAN_B2 => Ok(Self::DorianB2),
            AR_PADSCALE_LYDIAN_AUGMENTED => Ok(Self::LydianAugmented),
            AR_PADSCALE_LYDIAN_DOMINANT => Ok(Self::LydianDominant),
            AR_PADSCALE_DOUBLE_HARMONIC_MAJOR => Ok(Self::DoubleHarmonicMajor),
            AR_PADSCALE_LYDIAN_2_6 => Ok(Self::Lydian26),
            AR_PADSCALE_ULTRAPHRYGIAN => Ok(Self::Ultraphrygian),
            AR_PADSCALE_HUNGARIAN_MINOR => Ok(Self::HungarianMinor),
            AR_PADSCALE_ORIENTAL => Ok(Self::Oriental),
            AR_PADSCALE_IONIAN_2_5 => Ok(Self::Ionian25),
            AR_PADSCALE_LOCRIAN_BB3_BB7 => Ok(Self::LocrianBb3Bb7),

            _ => Err(ConversionError::Range {
                value: scale.to_string(),
                type_name: "PadScale".into(),
            }),
        }
    }
}
