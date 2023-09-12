use serde::Serialize;
use zbus::zvariant::{ObjectPath, Type, Value};

use crate::{PlaylistId, Uri};

/// A data structure describing a playlist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct Playlist {
    /// A unique identifier for the playlist.
    ///
    /// This should remain the same if the playlist is renamed.
    pub id: PlaylistId,
    /// The name of the playlist, typically given by the user.
    pub name: String,
    /// The URI of an (optional) icon.
    pub icon: Uri,
}

impl<'a> From<Playlist> for Value<'a> {
    fn from(p: Playlist) -> Self {
        Value::from((p.id, p.name, p.icon))
    }
}

/// A data structure describing a playlist, or nothing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct MaybePlaylist {
    /// Whether this structure refers to a valid playlist.
    valid: bool,
    /// The playlist, providing `valid` is true, otherwise undefined.
    ///
    /// When constructing this type, it should be noted that the playlist ID
    /// must be a valid object path, or D-Bus implementations may reject it.
    /// This is true even when Valid is false. It is suggested that "/" is
    /// used as the playlist ID in this case.
    playlist: Playlist,
}

impl MaybePlaylist {
    /// Construct a valid `MaybePlaylist` from the given playlist.
    pub fn some(playlist: Playlist) -> Self {
        Self {
            valid: true,
            playlist,
        }
    }

    /// Construct a `MaybePlaylist` that contains invalid/no playlist.
    pub fn none() -> Self {
        Self {
            valid: false,
            playlist: Playlist {
                id: ObjectPath::from_static_str_unchecked("/").into(),
                name: String::new(),
                icon: Uri::new(),
            },
        }
    }
}

impl From<Playlist> for MaybePlaylist {
    fn from(playlist: Playlist) -> Self {
        Self::some(playlist)
    }
}

impl From<Option<Playlist>> for MaybePlaylist {
    fn from(opt: Option<Playlist>) -> Self {
        match opt {
            Some(playlist) => Self::some(playlist),
            None => Self::none(),
        }
    }
}

impl<'a> From<MaybePlaylist> for Value<'a> {
    fn from(mp: MaybePlaylist) -> Self {
        Value::from((mp.valid, mp.playlist))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_signatures() {
        assert_eq!(Playlist::signature(), "(oss)");
        assert_eq!(MaybePlaylist::signature(), "(b(oss))");
    }
}
