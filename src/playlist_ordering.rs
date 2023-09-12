use std::fmt;

use serde::Deserialize;
use zbus::zvariant::{Type, Value};

/// Specifies the ordering of returned playlists.
///
/// # Rationale
///
/// Some media players may allow users to order playlists
/// as they wish. This ordering allows playlists to be retrieved
/// in that order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Type)]
#[zvariant(signature = "s")]
pub enum PlaylistOrdering {
    /// Alphabetical ordering by name, ascending.
    Alphabetical,
    /// Ordering by creation date, oldest first.
    CreationDate,
    /// Ordering by last modified date, oldest first.
    ModifiedDate,
    /// Ordering by date of last playback, oldest first.
    LastPlayDate,
    /// A user-defined ordering.
    UserDefined,
}

impl PlaylistOrdering {
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

impl<'a> From<PlaylistOrdering> for Value<'a> {
    fn from(status: PlaylistOrdering) -> Self {
        Value::new(status.as_str())
    }
}
