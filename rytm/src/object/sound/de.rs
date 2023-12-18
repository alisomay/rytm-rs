use crate::object::{pattern::plock::ParameterLockPool, sound::machine::MachineParameters};

use super::Sound;
use serde::{
    de::{Deserializer, Error, MapAccess, Visitor},
    Deserialize,
};
use std::{
    fmt,
    sync::{Arc, Mutex},
};

impl<'de> Deserialize<'de> for Sound {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SoundVisitor;

        impl<'de> Visitor<'de> for SoundVisitor {
            type Value = Sound;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Sound")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Sound, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut sysex_meta = None;
                let mut version = None;
                let mut index = None;
                let mut pool_index = None;
                let mut kit_number = None;
                let mut assigned_track = None;
                let mut name = None;
                let mut accent_level = None;
                let mut def_note = None;
                let mut sample = None;
                let mut filter = None;
                let mut amplitude = None;
                let mut lfo = None;
                let mut settings = None;
                let mut machine_parameters: Option<MachineParameters> = None;
                let mut __unknown = None;
                let mut parameter_lock_pool: Option<Option<Arc<Mutex<ParameterLockPool>>>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "sysex_meta" => sysex_meta = Some(map.next_value()?),
                        "version" => version = Some(map.next_value()?),
                        "index" => index = Some(map.next_value()?),
                        "pool_index" => pool_index = Some(map.next_value()?),
                        "kit_number" => kit_number = Some(map.next_value()?),
                        "assigned_track" => assigned_track = Some(map.next_value()?),
                        "name" => name = Some(map.next_value()?),
                        "accent_level" => accent_level = Some(map.next_value()?),
                        "def_note" => def_note = Some(map.next_value()?),
                        "sample" => sample = Some(map.next_value()?),
                        "filter" => filter = Some(map.next_value()?),
                        "amplitude" => amplitude = Some(map.next_value()?),
                        "lfo" => lfo = Some(map.next_value()?),
                        "settings" => settings = Some(map.next_value()?),
                        "machine_parameters" => machine_parameters = Some(map.next_value()?),
                        "__unknown" => __unknown = Some(map.next_value()?),
                        // Inject to the machine parameters if a previously linked parameter lock pool is present.
                        "parameter_lock_pool" => {
                            let parameter_lock_pool_value: Option<ParameterLockPool> =
                                map.next_value()?;
                            if let Some(parameter_lock_pool_value) = parameter_lock_pool_value {
                                let pool = Arc::new(Mutex::new(parameter_lock_pool_value));
                                if let Some(mp) = machine_parameters.as_mut() {
                                    // TODO: When deserializing machine parameters we can also check validity but maybe overkill.
                                    mp.link_parameter_lock_pool(Arc::clone(&pool));
                                }
                                parameter_lock_pool = Some(Some(pool))
                            } else {
                                parameter_lock_pool = Some(None);
                            }
                        }
                        _ => return Err(V::Error::unknown_field(key, FIELDS)),
                    }
                }

                Ok(Sound {
                    sysex_meta: sysex_meta.ok_or_else(|| V::Error::missing_field("sysex_meta"))?,
                    version: version.ok_or_else(|| V::Error::missing_field("version"))?,
                    index: index.ok_or_else(|| V::Error::missing_field("index"))?,
                    pool_index: pool_index.ok_or_else(|| V::Error::missing_field("pool_index"))?,
                    kit_number: kit_number.ok_or_else(|| V::Error::missing_field("kit_number"))?,
                    assigned_track: assigned_track
                        .ok_or_else(|| V::Error::missing_field("assigned_track"))?,
                    name: name.ok_or_else(|| V::Error::missing_field("name"))?,
                    accent_level: accent_level
                        .ok_or_else(|| V::Error::missing_field("accent_level"))?,
                    def_note: def_note.ok_or_else(|| V::Error::missing_field("def_note"))?,
                    sample: sample.ok_or_else(|| V::Error::missing_field("sample"))?,
                    filter: filter.ok_or_else(|| V::Error::missing_field("filter"))?,
                    amplitude: amplitude.ok_or_else(|| V::Error::missing_field("amplitude"))?,
                    lfo: lfo.ok_or_else(|| V::Error::missing_field("lfo"))?,
                    settings: settings.ok_or_else(|| V::Error::missing_field("settings"))?,
                    machine_parameters: machine_parameters
                        .ok_or_else(|| V::Error::missing_field("machine_parameters"))?,
                    __unknown: __unknown.ok_or_else(|| V::Error::missing_field("__unknown"))?,
                    parameter_lock_pool: parameter_lock_pool
                        .ok_or_else(|| V::Error::missing_field("parameter_lock_pool"))?,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "sysex_meta",
            "version",
            "index",
            "pool_index",
            "kit_number",
            "assigned_track",
            "name",
            "accent_level",
            "def_note",
            "sample",
            "filter",
            "amplitude",
            "lfo",
            "settings",
            "machine_parameters",
            "__unknown",
            "parameter_lock_pool",
        ];

        deserializer.deserialize_struct("Sound", FIELDS, SoundVisitor)
    }
}
