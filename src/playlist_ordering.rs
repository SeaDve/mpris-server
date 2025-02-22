use std::fmt;

use serde::Deserialize;
use zbus::zvariant::{Type, Value};

/// Specifies the ordering of returned playlists.
///
/// <details><summary>Rationale</summary>
///
/// Some media players may allow users to order playlists
/// as they wish. This ordering allows playlists to be retrieved
/// in that order.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Type)]
#[zvariant(signature = "s")]
#[doc(alias = "Playlist_Ordering")]
pub enum PlaylistOrdering {
    /// Alphabetical ordering by name, ascending.
    #[serde(rename = "Alphabetical")]
    Alphabetical,
    /// Ordering by creation date, oldest first.
    #[serde(rename = "Created")]
    CreationDate,
    /// Ordering by last modified date, oldest first.
    #[serde(rename = "Modified")]
    ModifiedDate,
    /// Ordering by date of last playback, oldest first.
    #[serde(rename = "Played")]
    LastPlayDate,
    /// A user-defined ordering.
    #[serde(rename = "User")]
    UserDefined,
}

impl PlaylistOrdering {
    /// Returns the string representation of this playlist ordering.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alphabetical => "Alphabetical",
            Self::CreationDate => "Created",
            Self::ModifiedDate => "Modified",
            Self::LastPlayDate => "Played",
            Self::UserDefined => "User",
        }
    }
}

impl fmt::Display for PlaylistOrdering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<PlaylistOrdering> for Value<'_> {
    fn from(status: PlaylistOrdering) -> Self {
        Value::new(status.as_str())
    }
}
