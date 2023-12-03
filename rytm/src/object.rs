#[derive(Clone, Copy)]
pub struct ObjectName([u8; 15]);

impl ObjectName {
    pub fn from_u8_array(raw_sound_name: [u8; 15]) -> Self {
        Self(raw_sound_name)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }
}

impl std::fmt::Display for ObjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str().trim_end_matches(char::from(0)))
    }
}

impl std::fmt::Debug for ObjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.as_str().trim_end_matches(char::from(0)))
    }
}
