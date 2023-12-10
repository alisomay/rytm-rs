use crate::error::ConversionError;

#[derive(Clone, Copy)]
pub struct ObjectName([u8; 15]);

impl ObjectName {
    pub fn from_u8_array(raw_sound_name: [u8; 15]) -> Self {
        Self(raw_sound_name)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap().trim_end()
    }

    pub fn copy_inner(&self) -> [u8; 15] {
        self.0
    }
}

impl TryFrom<&str> for ObjectName {
    type Error = ConversionError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if !s.is_ascii() {
            return Err(ConversionError::ObjectNameNotAscii(s.to_string()));
        }
        if s.len() > 15 {
            return Err(ConversionError::ObjectNameTooLong(s.to_owned(), s.len()));
        }
        let mut raw_sound_name = [0u8; 15];
        raw_sound_name[..s.len()].copy_from_slice(s.as_bytes());
        Ok(Self(raw_sound_name))
    }
}

impl TryFrom<String> for ObjectName {
    type Error = ConversionError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
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
