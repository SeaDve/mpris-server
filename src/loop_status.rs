use std::fmt;

use serde::{Deserialize, Serialize};
use zbus::{Type, Variant};

/// A repeat / loop status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Deserialize, Serialize)]
#[zvariant(signature = "s")]
#[doc(alias = "Loop_Status")]
pub enum LoopStatus {
    /// The playback will stop when there are no more tracks to play.
    None,
    /// The current track will start again from the beginning once it has
    /// finished playing.
    Track,
    /// The playback loops through a list of tracks.
    Playlist,
}

impl LoopStatus {
    /// Returns the string representation of this loop status.
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

impl<'a> TryFrom<Variant<'a>> for LoopStatus {
    type Error = zbus::Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Self::Error> {
        match value {
            Variant::Str(s) => match s.as_str() {
                "None" => Ok(Self::None),
                "Track" => Ok(Self::Track),
                "Playlist" => Ok(Self::Playlist),
                _ => Err(zbus::Error::Failure(format!("invalid loop status: {s}"))),
            },
            _ => Err(zbus::Error::IncorrectType),
        }
    }
}

impl From<LoopStatus> for Variant<'_> {
    fn from(status: LoopStatus) -> Self {
        Variant::new(status.as_str())
    }
}
