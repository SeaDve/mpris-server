use serde::Serialize;
use zbus::zvariant::{ObjectPath, Type, Value};

use crate::{PlaylistId, Uri};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct Playlist {
    pub id: PlaylistId,
    pub name: String,
    pub icon: Uri,
}

impl<'a> From<Playlist> for Value<'a> {
    fn from(p: Playlist) -> Self {
        Value::from((p.id, p.name, p.icon))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Type)]
pub struct MaybePlaylist {
    valid: bool,
    playlist: Playlist,
}

impl MaybePlaylist {
    pub fn some(playlist: Playlist) -> Self {
        Self {
            valid: true,
            playlist,
        }
    }

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
