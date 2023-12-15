//! Re-exports all object types. Kits, patterns, sounds, globals and settings are called objects.
//!
//! To know more about the objects, please check their own module documentation.

/// Holds the global object structure.
pub mod global;
/// Holds the kit object structure.
pub mod kit;
/// Holds the pattern object structure.
pub mod pattern;
/// Holds the settings object structure.
pub mod settings;
/// Holds the sound object structure.
pub mod sound;
/// Types which are common to all object types.
pub mod types;

pub use global::Global;
pub use kit::Kit;
pub use pattern::Pattern;
pub use settings::Settings;
pub use sound::Sound;
