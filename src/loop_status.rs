use std::{error, fmt, str::FromStr};

use zbus::zvariant::{self, Type, Value};

/// A repeat / loop status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
pub enum LoopStatus {
    /// The playback will stop when there are no more tracks to play.
    None,
    /// The current track will start again from the begining once it has finished playing.
    Track,
    /// The playback loops through a list of tracks.
    Playlist,
}

impl LoopStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Track => "Track",
            Self::Playlist => "Playlist",
        }
    }
}

impl fmt::Display for LoopStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> TryFrom<Value<'a>> for LoopStatus {
    type Error = zvariant::Error;
    fn try_from(value: Value<'a>) -> Result<Self, Self::Error> {
        match value {
            Value::Str(s) => s
                .parse::<Self>()
                .map_err(|err| zvariant::Error::Message(err.to_string())),
            _ => Err(zvariant::Error::IncorrectType),
        }
    }
}

impl<'a> From<LoopStatus> for Value<'a> {
    fn from(status: LoopStatus) -> Self {
        Value::new(status.as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLoopStatusError {
    invalid: String,
}

impl fmt::Display for ParseLoopStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid loop status: {}", self.invalid)
    }
}

impl error::Error for ParseLoopStatusError {}

impl FromStr for LoopStatus {
    type Err = ParseLoopStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Track" => Ok(Self::Track),
            "Playlist" => Ok(Self::Playlist),
            _ => Err(ParseLoopStatusError {
                invalid: s.to_string(),
            }),
        }
    }
}
