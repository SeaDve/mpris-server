use serde::Serialize;
use zbus::zvariant::{Type, Value};

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
    pub valid: bool,
    pub playlist: Playlist,
}

impl<'a> From<MaybePlaylist> for Value<'a> {
    fn from(mp: MaybePlaylist) -> Self {
        Value::from((mp.valid, mp.playlist))
    }
}

impl MaybePlaylist {
    pub fn as_playlist(&self) -> Option<&Playlist> {
        if self.valid {
            Some(&self.playlist)
        } else {
            None
        }
    }

    pub fn into_playlist(self) -> Option<Playlist> {
        if self.valid {
            Some(self.playlist)
        } else {
            None
        }
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
