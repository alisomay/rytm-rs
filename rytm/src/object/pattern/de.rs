use super::{Pattern, Trig};
use crate::object::pattern::{plock::ParameterLockPool, track::Track};
use parking_lot::Mutex;
use serde::de::{
    self, Deserialize, DeserializeSeed, Deserializer, Error, MapAccess, SeqAccess, Visitor,
};
use std::{fmt, sync::Arc};

#[allow(clippy::too_many_lines)]
impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PatternVisitor;

        impl<'de> Visitor<'de> for PatternVisitor {
            type Value = Pattern;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Pattern")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Pattern, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut sysex_meta = None;
                let mut index = None;
                let mut version = None;
                let mut tracks: Option<Vec<Track>> = None;
                let mut fx_track: Option<Arc<Mutex<Track>>> = None;
                let mut parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>> = None;
                let mut master_length = None;
                let mut master_change = None;
                let mut kit_number = None;
                let mut swing_amount = None;
                let mut time_mode = None;
                let mut speed = None;
                let mut global_quantize = None;
                let mut bpm = None;
                let mut pad_scale_per_pattern = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "sysex_meta" => sysex_meta = Some(map.next_value()?),
                        "index" => index = Some(map.next_value()?),
                        "version" => version = Some(map.next_value()?),
                        "fx_track" => {
                            // Context for the fx_track deserializer
                            let fx_track_seed = TrackDeserializerSeed {
                                fx_track_ref: &None, // fx_track does not have a parent, so pass None
                                parameter_lock_pool: &(parameter_lock_pool.clone()),
                            };
                            fx_track =
                                Some(Arc::new(Mutex::new(map.next_value_seed(fx_track_seed)?)));
                        }
                        "tracks" => {
                            // Context for the tracks array deserializer
                            let tracks_seed = TracksArrayDeserializerSeed {
                                fx_track_ref: &(fx_track.clone()),
                                parameter_lock_pool: &(parameter_lock_pool.clone()),
                            };
                            tracks = Some(map.next_value_seed(tracks_seed)?);
                        }
                        "master_length" => master_length = Some(map.next_value()?),
                        "master_change" => master_change = Some(map.next_value()?),
                        "kit_number" => kit_number = Some(map.next_value()?),
                        "swing_amount" => swing_amount = Some(map.next_value()?),
                        "time_mode" => time_mode = Some(map.next_value()?),
                        "speed" => speed = Some(map.next_value()?),
                        "global_quantize" => global_quantize = Some(map.next_value()?),
                        "bpm" => bpm = Some(map.next_value()?),
                        "pad_scale_per_pattern" => pad_scale_per_pattern = Some(map.next_value()?),
                        "parameter_lock_pool" => {
                            let parameter_lock_pool_value: ParameterLockPool = map.next_value()?;
                            parameter_lock_pool =
                                Some(Arc::new(Mutex::new(parameter_lock_pool_value)));
                        }
                        _ => return Err(V::Error::unknown_field(key, FIELDS)),
                    }
                }

                Ok(Pattern {
                    sysex_meta: sysex_meta.ok_or_else(|| V::Error::missing_field("sysex_meta"))?,
                    index: index.ok_or_else(|| V::Error::missing_field("index"))?,
                    version: version.ok_or_else(|| V::Error::missing_field("version"))?,
                    fx_track: fx_track.ok_or_else(|| V::Error::missing_field("fx_track"))?,
                    tracks: tracks.ok_or_else(|| V::Error::missing_field("tracks"))?,
                    master_length: master_length
                        .ok_or_else(|| V::Error::missing_field("master_length"))?,
                    master_change: master_change
                        .ok_or_else(|| V::Error::missing_field("master_change"))?,
                    kit_number: kit_number.ok_or_else(|| V::Error::missing_field("kit_number"))?,
                    swing_amount: swing_amount
                        .ok_or_else(|| V::Error::missing_field("swing_amount"))?,
                    time_mode: time_mode.ok_or_else(|| V::Error::missing_field("time_mode"))?,
                    speed: speed.ok_or_else(|| V::Error::missing_field("speed"))?,
                    global_quantize: global_quantize
                        .ok_or_else(|| V::Error::missing_field("global_quantize"))?,
                    bpm: bpm.ok_or_else(|| V::Error::missing_field("bpm"))?,
                    pad_scale_per_pattern: pad_scale_per_pattern
                        .ok_or_else(|| V::Error::missing_field("pad_scale_per_pattern"))?,
                    parameter_lock_pool: parameter_lock_pool
                        .ok_or_else(|| V::Error::missing_field("parameter_lock_pool"))?,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "sysex_meta",
            "index",
            "version",
            "tracks",
            "fx_track",
            "parameter_lock_pool",
            "master_length",
            "master_change",
            "kit_number",
            "swing_amount",
            "time_mode",
            "speed",
            "global_quantize",
            "bpm",
            "pad_scale_per_pattern",
        ];

        deserializer.deserialize_struct("Pattern", FIELDS, PatternVisitor)
    }
}

// Tracks array deserializer
struct TracksArrayDeserializerSeed<'a> {
    fx_track_ref: &'a Option<Arc<Mutex<Track>>>,
    parameter_lock_pool: &'a Option<Arc<Mutex<ParameterLockPool>>>,
}

impl<'a, 'de> DeserializeSeed<'de> for TracksArrayDeserializerSeed<'a> {
    type Value = Vec<Track>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArrayVisitor<'a> {
            fx_track_ref: &'a Option<Arc<Mutex<Track>>>,
            parameter_lock_pool: &'a Option<Arc<Mutex<ParameterLockPool>>>,
        }

        impl<'a, 'de> Visitor<'de> for ArrayVisitor<'a> {
            type Value = Vec<Track>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of 12 tracks")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Vec<Track>, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut tracks = Vec::with_capacity(12);
                for i in 0..12 {
                    let track_seed = TrackDeserializerSeed {
                        fx_track_ref: self.fx_track_ref,
                        parameter_lock_pool: self.parameter_lock_pool,
                    };
                    tracks.push(
                        seq.next_element_seed(track_seed)?
                            .ok_or_else(|| Error::invalid_length(i, &self))?,
                    );
                }
                Ok(tracks)
            }
        }

        deserializer.deserialize_seq(ArrayVisitor {
            fx_track_ref: self.fx_track_ref,
            parameter_lock_pool: self.parameter_lock_pool,
        })
    }
}

// Context for a single track deserializer
pub struct TrackDeserializerSeed<'a> {
    pub fx_track_ref: &'a Option<Arc<Mutex<Track>>>,
    pub parameter_lock_pool: &'a Option<Arc<Mutex<ParameterLockPool>>>,
}

impl<'a, 'de> de::DeserializeSeed<'de> for TrackDeserializerSeed<'a> {
    type Value = Track;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "Track",
            FIELDS,
            TrackVisitor {
                fx_track_ref: self.fx_track_ref,
                parameter_lock_pool: self.parameter_lock_pool,
            },
        )
    }
}

struct TrackVisitor<'a> {
    fx_track_ref: &'a Option<Arc<Mutex<Track>>>,
    parameter_lock_pool: &'a Option<Arc<Mutex<ParameterLockPool>>>,
}

impl<'a, 'de> de::Visitor<'de> for TrackVisitor<'a> {
    type Value = Track;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Track")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V>(self, mut map: V) -> Result<Track, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        // Initialize all Track fields to None or default value
        let mut is_owner_pattern_work_buffer = None;
        let mut owner_pattern_index = None;
        let mut index = None;
        let mut trigs = None;
        let mut default_trig_flags = None;
        let mut default_trig_note = None;
        let mut default_trig_velocity = None;
        let mut default_trig_note_length = None;
        let mut default_trig_probability = None;
        let mut number_of_steps = None;
        let mut quantize_amount = None;
        let mut sends_midi = None;
        let mut speed = None;
        let mut euclidean_mode = None;
        let mut euclidean_pl1 = None;
        let mut euclidean_pl2 = None;
        let mut euclidean_ro1 = None;
        let mut euclidean_ro2 = None;
        let mut euclidean_tro = None;
        let mut pad_scale = None;
        let mut root_note = None;
        let mut __maybe_useful_flag_from_default_trig_note = None;
        let mut __maybe_useful_flags_from_flags_and_speed = None;

        while let Some(key) = map.next_key()? {
            match key {
                "is_owner_pattern_work_buffer" => {
                    is_owner_pattern_work_buffer = Some(map.next_value()?);
                }
                "owner_pattern_index" => owner_pattern_index = Some(map.next_value()?),
                "index" => index = Some(map.next_value()?),
                "trigs" => {
                    // Pass context into the trigs array deserializer
                    trigs = Some(map.next_value_seed(TrigsArrayVisitor {
                        parameter_lock_pool: self.parameter_lock_pool.clone(),
                        fx_track_ref: self.fx_track_ref.clone(),
                    })?);
                }
                "default_trig_flags" => default_trig_flags = Some(map.next_value()?),
                "default_trig_note" => default_trig_note = Some(map.next_value()?),
                "default_trig_velocity" => default_trig_velocity = Some(map.next_value()?),
                "default_trig_note_length" => default_trig_note_length = Some(map.next_value()?),
                "default_trig_probability" => default_trig_probability = Some(map.next_value()?),
                "number_of_steps" => number_of_steps = Some(map.next_value()?),
                "quantize_amount" => quantize_amount = Some(map.next_value()?),
                "sends_midi" => sends_midi = Some(map.next_value()?),
                "speed" => speed = Some(map.next_value()?),
                "euclidean_mode" => euclidean_mode = Some(map.next_value()?),
                "euclidean_pl1" => euclidean_pl1 = Some(map.next_value()?),
                "euclidean_pl2" => euclidean_pl2 = Some(map.next_value()?),
                "euclidean_ro1" => euclidean_ro1 = Some(map.next_value()?),
                "euclidean_ro2" => euclidean_ro2 = Some(map.next_value()?),
                "euclidean_tro" => euclidean_tro = Some(map.next_value()?),
                "pad_scale" => pad_scale = Some(map.next_value()?),
                "root_note" => root_note = Some(map.next_value()?),
                "__maybe_useful_flag_from_default_trig_note" => {
                    __maybe_useful_flag_from_default_trig_note = Some(map.next_value()?);
                }
                "__maybe_useful_flags_from_flags_and_speed" => {
                    __maybe_useful_flags_from_flags_and_speed = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, FIELDS)),
            }
        }

        Ok(Track {
            is_owner_pattern_work_buffer: is_owner_pattern_work_buffer
                .ok_or_else(|| de::Error::missing_field("is_owner_pattern_work_buffer"))?,
            owner_pattern_index: owner_pattern_index
                .ok_or_else(|| de::Error::missing_field("owner_pattern_index"))?,
            index: index.ok_or_else(|| de::Error::missing_field("index"))?,
            trigs: trigs.ok_or_else(|| de::Error::missing_field("trigs"))?,
            default_trig_flags: default_trig_flags
                .ok_or_else(|| de::Error::missing_field("default_trig_flags"))?,
            default_trig_note: default_trig_note
                .ok_or_else(|| de::Error::missing_field("default_trig_note"))?,
            default_trig_velocity: default_trig_velocity
                .ok_or_else(|| de::Error::missing_field("default_trig_velocity"))?,
            default_trig_note_length: default_trig_note_length
                .ok_or_else(|| de::Error::missing_field("default_trig_note_length"))?,
            default_trig_probability: default_trig_probability
                .ok_or_else(|| de::Error::missing_field("default_trig_probability"))?,
            number_of_steps: number_of_steps
                .ok_or_else(|| de::Error::missing_field("number_of_steps"))?,
            quantize_amount: quantize_amount
                .ok_or_else(|| de::Error::missing_field("quantize_amount"))?,
            sends_midi: sends_midi.ok_or_else(|| de::Error::missing_field("sends_midi"))?,
            speed: speed.ok_or_else(|| de::Error::missing_field("speed"))?,
            euclidean_mode: euclidean_mode
                .ok_or_else(|| de::Error::missing_field("euclidean_mode"))?,
            euclidean_pl1: euclidean_pl1
                .ok_or_else(|| de::Error::missing_field("euclidean_pl1"))?,
            euclidean_pl2: euclidean_pl2
                .ok_or_else(|| de::Error::missing_field("euclidean_pl2"))?,
            euclidean_ro1: euclidean_ro1
                .ok_or_else(|| de::Error::missing_field("euclidean_ro1"))?,
            euclidean_ro2: euclidean_ro2
                .ok_or_else(|| de::Error::missing_field("euclidean_ro2"))?,
            euclidean_tro: euclidean_tro
                .ok_or_else(|| de::Error::missing_field("euclidean_tro"))?,
            pad_scale: pad_scale.ok_or_else(|| de::Error::missing_field("pad_scale"))?,
            root_note: root_note.ok_or_else(|| de::Error::missing_field("root_note"))?,
            __maybe_useful_flag_from_default_trig_note: __maybe_useful_flag_from_default_trig_note
                .ok_or_else(|| {
                    de::Error::missing_field("__maybe_useful_flag_from_default_trig_note")
                })?,
            __maybe_useful_flags_from_flags_and_speed: __maybe_useful_flags_from_flags_and_speed
                .ok_or_else(|| {
                    de::Error::missing_field("__maybe_useful_flags_from_flags_and_speed")
                })?,
            // Inject the context to the track
            fx_track_ref: self.fx_track_ref.clone(),
            parameter_lock_pool: self.parameter_lock_pool.clone(),
        })
    }
}

const FIELDS: &[&str] = &[
    "is_owner_pattern_work_buffer",
    "owner_pattern_index",
    "index",
    "trigs",
    "default_trig_flags",
    "default_trig_note",
    "default_trig_velocity",
    "default_trig_note_length",
    "default_trig_probability",
    "number_of_steps",
    "quantize_amount",
    "sends_midi",
    "speed",
    "euclidean_mode",
    "euclidean_pl1",
    "euclidean_pl2",
    "euclidean_ro1",
    "euclidean_ro2",
    "euclidean_tro",
    "pad_scale",
    "root_note",
    "__maybe_useful_flag_from_default_trig_note",
    "__maybe_useful_flags_from_flags_and_speed",
];

// Trigs array deserializer
pub struct TrigsArrayVisitor {
    pub parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,
    pub fx_track_ref: Option<Arc<Mutex<Track>>>,
}

impl<'de> de::DeserializeSeed<'de> for TrigsArrayVisitor {
    type Value = Vec<Trig>;

    fn deserialize<D>(self, deserializer: D) -> Result<Vec<Trig>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArrayVisitor {
            pub parameter_lock_pool: Option<Arc<Mutex<ParameterLockPool>>>,
            pub fx_track_ref: Option<Arc<Mutex<Track>>>,
        }

        impl<'de> Visitor<'de> for ArrayVisitor {
            type Value = Vec<Trig>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of 64 elements")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Vec<Trig>, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut trigs = Vec::with_capacity(64);
                for i in 0..64 {
                    let mut trig: Trig = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;

                    trig.parameter_lock_pool
                        .clone_from(&self.parameter_lock_pool);
                    trig.fx_track_ref.clone_from(&self.fx_track_ref);

                    trigs.push(trig);
                }
                Ok(trigs)
            }
        }

        deserializer.deserialize_seq(ArrayVisitor {
            parameter_lock_pool: self.parameter_lock_pool.clone(),
            fx_track_ref: self.fx_track_ref,
        })
    }
}
