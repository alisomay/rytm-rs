use serde::{Deserialize, Serialize};

use crate::error::ConversionError;

/// Micro timing of a trig.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum MicroTime {
    /// `-23/384`
    N23B384,
    /// `-11/192`
    N11B192,
    /// `-7/128`
    N7B128,
    /// `-5/96`
    N5B96,
    /// `-19/384`
    N19B384,
    /// `-3/64`
    N3B64,
    /// `-17/384`
    N17B384,
    /// `-1/24`
    N1B24,
    /// `-5/128`
    N5B128,
    /// `-7/192`
    N7B192,
    /// `-13/384`
    N13B384,
    /// `-1/32`
    N32nd,
    /// `-11/384`
    N11B384,
    /// `-5/192`
    N5B192,
    /// `-3/128`
    N3B128,
    /// `-1/48`
    N1B48,
    /// `-7/384`
    N7B384,
    /// `-1/64`
    N64th,
    /// `-5/384`
    N5B384,
    /// `-1/96`
    N1B96,
    /// `-1/128`
    N1B128,
    /// `-1/192`
    N1B192,
    /// `-1/384`
    N1B384,
    #[default]
    /// The trig is on the grid.
    OnGrid,
    /// `1/384`
    P1B384,
    /// `1/192`
    P1B192,
    /// `1/128`
    P1B128,
    /// `1/96`
    P1B96,
    /// `5/384`
    P5B384,
    /// `1/64`
    P64th,
    /// `7/384`
    P7B384,
    /// `1/48`
    P1B48,
    /// `3/128`
    P3B128,
    /// `5/192`
    P5B192,
    /// `11/384`
    P11B384,
    /// `1/32`
    P32nd,
    /// `13/384`
    P13B384,
    /// `7/192`
    P7B192,
    /// `5/128`
    P5B128,
    /// `1/24`
    P1B24,
    /// `17/384`
    P17B384,
    /// `3/64`
    P3B64,
    /// `19/384`
    P19B384,
    /// `5/96`
    P5B96,
    /// `7/128`
    P7B128,
    /// `11/192`
    P11B192,
    /// `23/384`
    P23B384,
}

impl TryFrom<isize> for MicroTime {
    type Error = ConversionError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            -23 => Ok(Self::N23B384),
            -22 => Ok(Self::N11B192),
            -21 => Ok(Self::N7B128),
            -20 => Ok(Self::N5B96),
            -19 => Ok(Self::N19B384),
            -18 => Ok(Self::N3B64),
            -17 => Ok(Self::N17B384),
            -16 => Ok(Self::N1B24),
            -15 => Ok(Self::N5B128),
            -14 => Ok(Self::N7B192),
            -13 => Ok(Self::N13B384),
            -12 => Ok(Self::N32nd),
            -11 => Ok(Self::N11B384),
            -10 => Ok(Self::N5B192),
            -9 => Ok(Self::N3B128),
            -8 => Ok(Self::N1B48),
            -7 => Ok(Self::N7B384),
            -6 => Ok(Self::N64th),
            -5 => Ok(Self::N5B384),
            -4 => Ok(Self::N1B96),
            -3 => Ok(Self::N1B128),
            -2 => Ok(Self::N1B192),
            -1 => Ok(Self::N1B384),
            0 => Ok(Self::OnGrid),
            1 => Ok(Self::P1B384),
            2 => Ok(Self::P1B192),
            3 => Ok(Self::P1B128),
            4 => Ok(Self::P1B96),
            5 => Ok(Self::P5B384),
            6 => Ok(Self::P64th),
            7 => Ok(Self::P7B384),
            8 => Ok(Self::P1B48),
            9 => Ok(Self::P3B128),
            10 => Ok(Self::P5B192),
            11 => Ok(Self::P11B384),
            12 => Ok(Self::P32nd),
            13 => Ok(Self::P13B384),
            14 => Ok(Self::P7B192),
            15 => Ok(Self::P5B128),
            16 => Ok(Self::P1B24),
            17 => Ok(Self::P17B384),
            18 => Ok(Self::P3B64),
            19 => Ok(Self::P19B384),
            20 => Ok(Self::P5B96),
            21 => Ok(Self::P7B128),
            22 => Ok(Self::P11B192),
            23 => Ok(Self::P23B384),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "MicroTime".into(),
            }),
        }
    }
}

impl From<&MicroTime> for isize {
    fn from(micro_time: &MicroTime) -> Self {
        match micro_time {
            MicroTime::N23B384 => -23,
            MicroTime::N11B192 => -22,
            MicroTime::N7B128 => -21,
            MicroTime::N5B96 => -20,
            MicroTime::N19B384 => -19,
            MicroTime::N3B64 => -18,
            MicroTime::N17B384 => -17,
            MicroTime::N1B24 => -16,
            MicroTime::N5B128 => -15,
            MicroTime::N7B192 => -14,
            MicroTime::N13B384 => -13,
            MicroTime::N32nd => -12,
            MicroTime::N11B384 => -11,
            MicroTime::N5B192 => -10,
            MicroTime::N3B128 => -9,
            MicroTime::N1B48 => -8,
            MicroTime::N7B384 => -7,
            MicroTime::N64th => -6,
            MicroTime::N5B384 => -5,
            MicroTime::N1B96 => -4,
            MicroTime::N1B128 => -3,
            MicroTime::N1B192 => -2,
            MicroTime::N1B384 => -1,
            MicroTime::OnGrid => 0,
            MicroTime::P1B384 => 1,
            MicroTime::P1B192 => 2,
            MicroTime::P1B128 => 3,
            MicroTime::P1B96 => 4,
            MicroTime::P5B384 => 5,
            MicroTime::P64th => 6,
            MicroTime::P7B384 => 7,
            MicroTime::P1B48 => 8,
            MicroTime::P3B128 => 9,
            MicroTime::P5B192 => 10,
            MicroTime::P11B384 => 11,
            MicroTime::P32nd => 12,
            MicroTime::P13B384 => 13,
            MicroTime::P7B192 => 14,
            MicroTime::P5B128 => 15,
            MicroTime::P1B24 => 16,
            MicroTime::P17B384 => 17,
            MicroTime::P3B64 => 18,
            MicroTime::P19B384 => 19,
            MicroTime::P5B96 => 20,
            MicroTime::P7B128 => 21,
            MicroTime::P11B192 => 22,
            MicroTime::P23B384 => 23,
        }
    }
}

/// Length type which can be used for note lengths and retrig lengths.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Length {
    /// `1/128`
    _128th,
    /// `.188`
    _D188,
    /// `1/64`
    _64th,
    /// `.313`
    _D313,
    /// `.375`
    _D375,
    /// `.438`
    _D438,
    /// `1/32`
    _32nd,
    /// `.563`
    _D563,
    /// `.625`
    _D625,
    /// `.688`
    _D688,
    /// `.75`
    _D750,
    /// `.813`
    _D813,
    /// `.875`
    _D875,
    /// `.938`
    _D938,
    #[default]
    /// `1/16`
    _16th,
    /// `1.06`
    _1D06,
    /// `1.13`
    _1D13,
    /// `1.19`
    _1D19,
    /// `1.25`
    _1D25,
    /// `1.31`
    _1D31,
    /// `1.38`
    _1D38,
    /// `1.44`
    _1D44,
    /// `1.5`
    _1D50,
    /// `1.56`
    _1D56,
    /// `1.63`
    _1D63,
    /// `1.69`
    _1D69,
    /// `1.75`
    _1D75,
    /// `1.81`
    _1D81,
    /// `1.88`
    _1D88,
    /// `1.94`
    _1D94,
    /// `1/8`
    _8th,
    /// `2.13`
    _2D13,
    /// `2.25`
    _2D25,
    /// `2.38`
    _2D38,
    /// `2.5`
    _2D50,
    /// `2.63`
    _2D63,
    /// `2.75`
    _2D75,
    /// `2.88`
    _2D88,
    /// `3`
    _3,
    /// `3.13`
    _3D13,
    /// `3.25`
    _3D25,
    /// `3.38`
    _3D38,
    /// `3.5`
    _3D50,
    /// `3.63`
    _3D63,
    /// `3.75`
    _3D75,
    /// `3.88`
    _3D88,
    /// `1/4`
    Quarter,
    /// `4.25`
    _4D25,
    /// `4.5`
    _4D50,
    /// `4.75`
    _4D75,
    /// `5`
    _5,
    /// `5.25`
    _5D25,
    /// `5.5`
    _5D50,
    /// `5.75`
    _5D75,
    /// `6`
    _6,
    /// `6.25`
    _6D25,
    /// `6.5`
    _6D50,
    /// `6.75`
    _6D75,
    /// `7`
    _7,
    /// `7.25`
    _7D25,
    /// `7.5`
    _7D50,
    /// `7.75`
    _7D75,
    /// `1/2`
    Half,
    /// `8.5`
    _8D50,
    /// `9`
    _9,
    /// `9.5`
    _9D50,
    /// `10`
    _10,
    /// `10.5`
    _10D5,
    /// `11`
    _11,
    /// `11.5`
    _11D5,
    /// `12`
    _12,
    /// `12.5`
    _12D5,
    /// `13`
    _13,
    /// `13.5`
    _13D5,
    /// `14`
    _14,
    /// `14.5`
    _14D5,
    /// `15`
    _15,
    /// `15.5`
    _15D5,
    /// `1/1`
    Whole,
    /// `17`
    _17,
    /// `18`
    _18,
    /// `19`
    _19,
    /// `20`
    _20,
    /// `21`
    _21,
    /// `22`
    _22,
    /// `23`
    _23,
    /// `24`
    _24,
    /// `25`
    _25,
    /// `26`
    _26,
    /// `27`
    _27,
    /// `28`
    _28,
    /// `29`
    _29,
    /// `30`
    _30,
    /// `31`
    _31,
    /// `32`
    DoubleWhole,
    /// `34`
    _34,
    /// `36`
    _36,
    /// `38`
    _38,
    /// `40`
    _40,
    /// `42`
    _42,
    /// `44`
    _44,
    /// `46`
    _46,
    /// `48`
    _48,
    /// `50`
    _50,
    /// `52`
    _52,
    /// `54`
    _54,
    /// `56`
    _56,
    /// `58`
    _58,
    /// `60`
    _60,
    /// `62`
    _62,
    /// `64`
    _64,
    /// `68`
    _68,
    /// `72`
    _72,
    /// `76`
    _76,
    /// `80`
    _80,
    /// `84`
    _84,
    /// `88`
    _88,
    /// `92`
    _92,
    /// `96`
    _96,
    /// `100`
    _100,
    /// `104`
    _104,
    /// `108`
    _108,
    /// `112`
    _112,
    /// `116`
    _116,
    /// `120`
    _120,
    /// `124`
    _124,
    /// `128`
    _128,
    /// `inf`
    Infinite,
    /// Default value for unset values.
    ///
    /// `0xFF``
    Unset,
}

impl From<Length> for u8 {
    #[allow(clippy::too_many_lines)]
    fn from(item: Length) -> Self {
        match item {
            Length::_128th => 0,
            Length::_D188 => 1,
            Length::_64th => 2,
            Length::_D313 => 3,
            Length::_D375 => 4,
            Length::_D438 => 5,
            Length::_32nd => 6,
            Length::_D563 => 7,
            Length::_D625 => 8,
            Length::_D688 => 9,
            Length::_D750 => 10,
            Length::_D813 => 11,
            Length::_D875 => 12,
            Length::_D938 => 13,
            Length::_16th => 14,
            Length::_1D06 => 15,
            Length::_1D13 => 16,
            Length::_1D19 => 17,
            Length::_1D25 => 18,
            Length::_1D31 => 19,
            Length::_1D38 => 20,
            Length::_1D44 => 21,
            Length::_1D50 => 22,
            Length::_1D56 => 23,
            Length::_1D63 => 24,
            Length::_1D69 => 25,
            Length::_1D75 => 26,
            Length::_1D81 => 27,
            Length::_1D88 => 28,
            Length::_1D94 => 29,
            Length::_8th => 30,
            Length::_2D13 => 31,
            Length::_2D25 => 32,
            Length::_2D38 => 33,
            Length::_2D50 => 34,
            Length::_2D63 => 35,
            Length::_2D75 => 36,
            Length::_2D88 => 37,
            Length::_3 => 38,
            Length::_3D13 => 39,
            Length::_3D25 => 40,
            Length::_3D38 => 41,
            Length::_3D50 => 42,
            Length::_3D63 => 43,
            Length::_3D75 => 44,
            Length::_3D88 => 45,
            Length::Quarter => 46,
            Length::_4D25 => 47,
            Length::_4D50 => 48,
            Length::_4D75 => 49,
            Length::_5 => 50,
            Length::_5D25 => 51,
            Length::_5D50 => 52,
            Length::_5D75 => 53,
            Length::_6 => 54,
            Length::_6D25 => 55,
            Length::_6D50 => 56,
            Length::_6D75 => 57,
            Length::_7 => 58,
            Length::_7D25 => 59,
            Length::_7D50 => 60,
            Length::_7D75 => 61,
            Length::Half => 62,
            Length::_8D50 => 63,
            Length::_9 => 64,
            Length::_9D50 => 65,
            Length::_10 => 66,
            Length::_10D5 => 67,
            Length::_11 => 68,
            Length::_11D5 => 69,
            Length::_12 => 70,
            Length::_12D5 => 71,
            Length::_13 => 72,
            Length::_13D5 => 73,
            Length::_14 => 74,
            Length::_14D5 => 75,
            Length::_15 => 76,
            Length::_15D5 => 77,
            Length::Whole => 78,
            Length::_17 => 79,
            Length::_18 => 80,
            Length::_19 => 81,
            Length::_20 => 82,
            Length::_21 => 83,
            Length::_22 => 84,
            Length::_23 => 85,
            Length::_24 => 86,
            Length::_25 => 87,
            Length::_26 => 88,
            Length::_27 => 89,
            Length::_28 => 90,
            Length::_29 => 91,
            Length::_30 => 92,
            Length::_31 => 93,
            Length::DoubleWhole => 94,
            Length::_34 => 95,
            Length::_36 => 96,
            Length::_38 => 97,
            Length::_40 => 98,
            Length::_42 => 99,
            Length::_44 => 100,
            Length::_46 => 101,
            Length::_48 => 102,
            Length::_50 => 103,
            Length::_52 => 104,
            Length::_54 => 105,
            Length::_56 => 106,
            Length::_58 => 107,
            Length::_60 => 108,
            Length::_62 => 109,
            Length::_64 => 110,
            Length::_68 => 111,
            Length::_72 => 112,
            Length::_76 => 113,
            Length::_80 => 114,
            Length::_84 => 115,
            Length::_88 => 116,
            Length::_92 => 117,
            Length::_96 => 118,
            Length::_100 => 119,
            Length::_104 => 120,
            Length::_108 => 121,
            Length::_112 => 122,
            Length::_116 => 123,
            Length::_120 => 124,
            Length::_124 => 125,
            Length::_128 => 126,
            Length::Infinite => 127,
            Length::Unset => 0xFF,
        }
    }
}

impl TryFrom<u8> for Length {
    type Error = ConversionError;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::_128th),
            1 => Ok(Self::_D188),
            2 => Ok(Self::_64th),
            3 => Ok(Self::_D313),
            4 => Ok(Self::_D375),
            5 => Ok(Self::_D438),
            6 => Ok(Self::_32nd),
            7 => Ok(Self::_D563),
            8 => Ok(Self::_D625),
            9 => Ok(Self::_D688),
            10 => Ok(Self::_D750),
            11 => Ok(Self::_D813),
            12 => Ok(Self::_D875),
            13 => Ok(Self::_D938),
            14 => Ok(Self::_16th),
            15 => Ok(Self::_1D06),
            16 => Ok(Self::_1D13),
            17 => Ok(Self::_1D19),
            18 => Ok(Self::_1D25),
            19 => Ok(Self::_1D31),
            20 => Ok(Self::_1D38),
            21 => Ok(Self::_1D44),
            22 => Ok(Self::_1D50),
            23 => Ok(Self::_1D56),
            24 => Ok(Self::_1D63),
            25 => Ok(Self::_1D69),
            26 => Ok(Self::_1D75),
            27 => Ok(Self::_1D81),
            28 => Ok(Self::_1D88),
            29 => Ok(Self::_1D94),
            30 => Ok(Self::_8th),
            31 => Ok(Self::_2D13),
            32 => Ok(Self::_2D25),
            33 => Ok(Self::_2D38),
            34 => Ok(Self::_2D50),
            35 => Ok(Self::_2D63),
            36 => Ok(Self::_2D75),
            37 => Ok(Self::_2D88),
            38 => Ok(Self::_3),
            39 => Ok(Self::_3D13),
            40 => Ok(Self::_3D25),
            41 => Ok(Self::_3D38),
            42 => Ok(Self::_3D50),
            43 => Ok(Self::_3D63),
            44 => Ok(Self::_3D75),
            45 => Ok(Self::_3D88),
            46 => Ok(Self::Quarter),
            47 => Ok(Self::_4D25),
            48 => Ok(Self::_4D50),
            49 => Ok(Self::_4D75),
            50 => Ok(Self::_5),
            51 => Ok(Self::_5D25),
            52 => Ok(Self::_5D50),
            53 => Ok(Self::_5D75),
            54 => Ok(Self::_6),
            55 => Ok(Self::_6D25),
            56 => Ok(Self::_6D50),
            57 => Ok(Self::_6D75),
            58 => Ok(Self::_7),
            59 => Ok(Self::_7D25),
            60 => Ok(Self::_7D50),
            61 => Ok(Self::_7D75),
            62 => Ok(Self::Half),
            63 => Ok(Self::_8D50),
            64 => Ok(Self::_9),
            65 => Ok(Self::_9D50),
            66 => Ok(Self::_10),
            67 => Ok(Self::_10D5),
            68 => Ok(Self::_11),
            69 => Ok(Self::_11D5),
            70 => Ok(Self::_12),
            71 => Ok(Self::_12D5),
            72 => Ok(Self::_13),
            73 => Ok(Self::_13D5),
            74 => Ok(Self::_14),
            75 => Ok(Self::_14D5),
            76 => Ok(Self::_15),
            77 => Ok(Self::_15D5),
            78 => Ok(Self::Whole),
            79 => Ok(Self::_17),
            80 => Ok(Self::_18),
            81 => Ok(Self::_19),
            82 => Ok(Self::_20),
            83 => Ok(Self::_21),
            84 => Ok(Self::_22),
            85 => Ok(Self::_23),
            86 => Ok(Self::_24),
            87 => Ok(Self::_25),
            88 => Ok(Self::_26),
            89 => Ok(Self::_27),
            90 => Ok(Self::_28),
            91 => Ok(Self::_29),
            92 => Ok(Self::_30),
            93 => Ok(Self::_31),
            94 => Ok(Self::DoubleWhole),
            95 => Ok(Self::_34),
            96 => Ok(Self::_36),
            97 => Ok(Self::_38),
            98 => Ok(Self::_40),
            99 => Ok(Self::_42),
            100 => Ok(Self::_44),
            101 => Ok(Self::_46),
            102 => Ok(Self::_48),
            103 => Ok(Self::_50),
            104 => Ok(Self::_52),
            105 => Ok(Self::_54),
            106 => Ok(Self::_56),
            107 => Ok(Self::_58),
            108 => Ok(Self::_60),
            109 => Ok(Self::_62),
            110 => Ok(Self::_64),
            111 => Ok(Self::_68),
            112 => Ok(Self::_72),
            113 => Ok(Self::_76),
            114 => Ok(Self::_80),
            115 => Ok(Self::_84),
            116 => Ok(Self::_88),
            117 => Ok(Self::_92),
            118 => Ok(Self::_96),
            119 => Ok(Self::_100),
            120 => Ok(Self::_104),
            121 => Ok(Self::_108),
            122 => Ok(Self::_112),
            123 => Ok(Self::_116),
            124 => Ok(Self::_120),
            125 => Ok(Self::_124),
            126 => Ok(Self::_128),
            127 => Ok(Self::Infinite),
            0xFF => Ok(Self::Unset),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "Length".into(),
            }),
        }
    }
}

/// Retrig rate of a trig.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum RetrigRate {
    /// `1/1`
    _1B1,
    /// `1/2`
    _1B2,
    /// `1/3`
    _1B3,
    /// `1/4`
    _1B4,
    /// `1/5`
    _1B5,
    /// `1/6`
    _1B6,
    /// `1/8`
    _1B8,
    /// `1/10`
    _1B10,
    /// `1/12`
    _1B12,
    #[default]
    /// `1/16`
    _1B16,
    /// `1/20`
    _1B20,
    /// `1/24`
    _1B24,
    /// `1/32`
    _1B32,
    /// `1/40`
    _1B40,
    /// `1/48`
    _1B48,
    /// `1/64`
    _1B64,
    /// `1/80`
    _1B80,
}

impl TryFrom<u8> for RetrigRate {
    type Error = ConversionError;

    fn try_from(rate: u8) -> Result<Self, Self::Error> {
        match rate {
            0 => Ok(Self::_1B1),
            1 => Ok(Self::_1B2),
            2 => Ok(Self::_1B3),
            3 => Ok(Self::_1B4),
            4 => Ok(Self::_1B5),
            5 => Ok(Self::_1B6),
            6 => Ok(Self::_1B8),
            7 => Ok(Self::_1B10),
            8 => Ok(Self::_1B12),
            9 => Ok(Self::_1B16),
            10 => Ok(Self::_1B20),
            11 => Ok(Self::_1B24),
            12 => Ok(Self::_1B32),
            13 => Ok(Self::_1B40),
            14 => Ok(Self::_1B48),
            15 => Ok(Self::_1B64),
            16 => Ok(Self::_1B80),
            _ => Err(Self::Error::Range {
                value: rate.to_string(),
                type_name: "RetrigRate".into(),
            }),
        }
    }
}

impl From<RetrigRate> for u8 {
    fn from(rate: RetrigRate) -> Self {
        match rate {
            RetrigRate::_1B1 => 0,
            RetrigRate::_1B2 => 1,
            RetrigRate::_1B3 => 2,
            RetrigRate::_1B4 => 3,
            RetrigRate::_1B5 => 4,
            RetrigRate::_1B6 => 5,
            RetrigRate::_1B8 => 6,
            RetrigRate::_1B10 => 7,
            RetrigRate::_1B12 => 8,
            RetrigRate::_1B16 => 9,
            RetrigRate::_1B20 => 10,
            RetrigRate::_1B24 => 11,
            RetrigRate::_1B32 => 12,
            RetrigRate::_1B40 => 13,
            RetrigRate::_1B48 => 14,
            RetrigRate::_1B64 => 15,
            RetrigRate::_1B80 => 16,
        }
    }
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
/// Trig condition of a trig.
pub enum TrigCondition {
    /// 1% probability.
    P1,
    /// 3% probability.
    P3,
    /// 4% probability.   
    P4,
    /// 6% probability.
    P6,
    /// 9% probability.
    P9,
    /// 13% probability.
    P13,
    /// 19% probability.
    P19,
    /// 25% probability.
    P25,
    /// 33% probability.
    P33,
    /// 41% probability.
    P41,
    /// 50% probability.
    P50,
    /// 59% probability.
    P59,
    /// 67% probability.
    P67,
    /// 75% probability.
    P75,
    /// 81% probability.
    P81,
    /// 87% probability.
    P87,
    /// 91% probability.
    P91,
    /// 94% probability.
    P94,
    /// 96% probability.
    P96,
    /// 98% probability.
    P98,
    /// 99% probability.
    P99,
    /// 100% probability.
    P100,
    Fill,
    FillNot,
    Pre,
    PreNot,
    Nei,
    NeiNot,
    _1st,
    _1stNot,
    _1B2,
    _2B2,
    _1B3,
    _2B3,
    _3B3,
    _1B4,
    _2B4,
    _3B4,
    _4B4,
    _1B5,
    _2B5,
    _3B5,
    _4B5,
    _5B5,
    _1B6,
    _2B6,
    _3B6,
    _4B6,
    _5B6,
    _6B6,
    _1B7,
    _2B7,
    _3B7,
    _4B7,
    _5B7,
    _6B7,
    _7B7,
    _1B8,
    _2B8,
    _3B8,
    _4B8,
    _5B8,
    _6B8,
    _7B8,
    _8B8,
    #[default]
    Unset,
}

impl TryFrom<u8> for TrigCondition {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::P1),
            1 => Ok(Self::P3),
            2 => Ok(Self::P4),
            3 => Ok(Self::P6),
            4 => Ok(Self::P9),
            5 => Ok(Self::P13),
            6 => Ok(Self::P19),
            7 => Ok(Self::P25),
            8 => Ok(Self::P33),
            9 => Ok(Self::P41),
            10 => Ok(Self::P50),
            11 => Ok(Self::P59),
            12 => Ok(Self::P67),
            13 => Ok(Self::P75),
            14 => Ok(Self::P81),
            15 => Ok(Self::P87),
            16 => Ok(Self::P91),
            17 => Ok(Self::P94),
            18 => Ok(Self::P96),
            19 => Ok(Self::P98),
            20 => Ok(Self::P99),
            21 => Ok(Self::P100),
            22 => Ok(Self::Fill),
            23 => Ok(Self::FillNot),
            24 => Ok(Self::Pre),
            25 => Ok(Self::PreNot),
            26 => Ok(Self::Nei),
            27 => Ok(Self::NeiNot),
            28 => Ok(Self::_1st),
            29 => Ok(Self::_1stNot),
            30 => Ok(Self::_1B2),
            31 => Ok(Self::_2B2),
            32 => Ok(Self::_1B3),
            33 => Ok(Self::_2B3),
            34 => Ok(Self::_3B3),
            35 => Ok(Self::_1B4),
            36 => Ok(Self::_2B4),
            37 => Ok(Self::_3B4),
            38 => Ok(Self::_4B4),
            39 => Ok(Self::_1B5),
            40 => Ok(Self::_2B5),
            41 => Ok(Self::_3B5),
            42 => Ok(Self::_4B5),
            43 => Ok(Self::_5B5),
            44 => Ok(Self::_1B6),
            45 => Ok(Self::_2B6),
            46 => Ok(Self::_3B6),
            47 => Ok(Self::_4B6),
            48 => Ok(Self::_5B6),
            49 => Ok(Self::_6B6),
            50 => Ok(Self::_1B7),
            51 => Ok(Self::_2B7),
            52 => Ok(Self::_3B7),
            53 => Ok(Self::_4B7),
            54 => Ok(Self::_5B7),
            55 => Ok(Self::_6B7),
            56 => Ok(Self::_7B7),
            57 => Ok(Self::_1B8),
            58 => Ok(Self::_2B8),
            59 => Ok(Self::_3B8),
            60 => Ok(Self::_4B8),
            61 => Ok(Self::_5B8),
            62 => Ok(Self::_6B8),
            63 => Ok(Self::_7B8),
            64 => Ok(Self::_8B8),
            0x7F => Ok(Self::Unset),
            _ => Err(ConversionError::Range {
                value: value.to_string(),
                type_name: "TrigCondition".into(),
            }),
        }
    }
}

impl From<TrigCondition> for u8 {
    fn from(value: TrigCondition) -> Self {
        match value {
            TrigCondition::P1 => 0,
            TrigCondition::P3 => 1,
            TrigCondition::P4 => 2,
            TrigCondition::P6 => 3,
            TrigCondition::P9 => 4,
            TrigCondition::P13 => 5,
            TrigCondition::P19 => 6,
            TrigCondition::P25 => 7,
            TrigCondition::P33 => 8,
            TrigCondition::P41 => 9,
            TrigCondition::P50 => 10,
            TrigCondition::P59 => 11,
            TrigCondition::P67 => 12,
            TrigCondition::P75 => 13,
            TrigCondition::P81 => 14,
            TrigCondition::P87 => 15,
            TrigCondition::P91 => 16,
            TrigCondition::P94 => 17,
            TrigCondition::P96 => 18,
            TrigCondition::P98 => 19,
            TrigCondition::P99 => 20,
            TrigCondition::P100 => 21,
            TrigCondition::Fill => 22,
            TrigCondition::FillNot => 23,
            TrigCondition::Pre => 24,
            TrigCondition::PreNot => 25,
            TrigCondition::Nei => 26,
            TrigCondition::NeiNot => 27,
            TrigCondition::_1st => 28,
            TrigCondition::_1stNot => 29,
            TrigCondition::_1B2 => 30,
            TrigCondition::_2B2 => 31,
            TrigCondition::_1B3 => 32,
            TrigCondition::_2B3 => 33,
            TrigCondition::_3B3 => 34,
            TrigCondition::_1B4 => 35,
            TrigCondition::_2B4 => 36,
            TrigCondition::_3B4 => 37,
            TrigCondition::_4B4 => 38,
            TrigCondition::_1B5 => 39,
            TrigCondition::_2B5 => 40,
            TrigCondition::_3B5 => 41,
            TrigCondition::_4B5 => 42,
            TrigCondition::_5B5 => 43,
            TrigCondition::_1B6 => 44,
            TrigCondition::_2B6 => 45,
            TrigCondition::_3B6 => 46,
            TrigCondition::_4B6 => 47,
            TrigCondition::_5B6 => 48,
            TrigCondition::_6B6 => 49,
            TrigCondition::_1B7 => 50,
            TrigCondition::_2B7 => 51,
            TrigCondition::_3B7 => 52,
            TrigCondition::_4B7 => 53,
            TrigCondition::_5B7 => 54,
            TrigCondition::_6B7 => 55,
            TrigCondition::_7B7 => 56,
            TrigCondition::_1B8 => 57,
            TrigCondition::_2B8 => 58,
            TrigCondition::_3B8 => 59,
            TrigCondition::_4B8 => 60,
            TrigCondition::_5B8 => 61,
            TrigCondition::_6B8 => 62,
            TrigCondition::_7B8 => 63,
            TrigCondition::_8B8 => 64,
            TrigCondition::Unset => 0x7F,
        }
    }
}
