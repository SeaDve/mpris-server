use std::fmt;

use zbus::zvariant::{Type, Value};

/// A playback state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
#[doc(alias = "Playback_Status")]
pub enum PlaybackStatus {
    /// A track is currently playing.
    Playing,
    /// A track is currently paused.
    Paused,
    /// There is no track currently playing.
    Stopped,
}

impl PlaybackStatus {
    /// Returns the string representation of this playback status.
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

impl From<PlaybackStatus> for Value<'_> {
    fn from(status: PlaybackStatus) -> Self {
        Value::new(status.as_str())
    }
}
