use std::{error, fmt, str::FromStr};

use zbus::zvariant::{Type, Value};

/// A playback state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
pub enum PlaybackStatus {
    /// A track is currently playing.
    Playing,
    /// A track is currently paused.
    Paused,
    /// There is no track currently playing.
    Stopped,
}

impl PlaybackStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Playing => "Playing",
            Self::Paused => "Paused",
            Self::Stopped => "Stopped",
        }
    }
}

impl fmt::Display for PlaybackStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePlaybackStatusError;

impl fmt::Display for ParsePlaybackStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid playback status")
    }
}

impl error::Error for ParsePlaybackStatusError {}

impl FromStr for PlaybackStatus {
    type Err = ParsePlaybackStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Playing" => Ok(Self::Playing),
            "Paused" => Ok(Self::Paused),
            "Stopped" => Ok(Self::Stopped),
            _ => Err(ParsePlaybackStatusError),
        }
    }
}

impl<'a> From<PlaybackStatus> for Value<'a> {
    fn from(status: PlaybackStatus) -> Self {
        Value::new(status.as_str())
    }
}
