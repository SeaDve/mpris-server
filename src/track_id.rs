use std::{fmt, ops};

use serde::{de, Deserialize, Deserializer, Serialize};
use zbus::zvariant::{Error, ObjectPath, Result, Type, Value};

/// Unique track identifier.
///
/// If the media player implements the TrackList interface and allows
/// the same track to appear multiple times in the tracklist, this
/// must be unique within the scope of the tracklist.
///
/// Note that this should be a valid D-Bus object id, although clients
/// should not assume that any object is actually exported with any
/// interfaces at that path.
///
/// Media players may not use any paths starting with /org/mpris unless
/// explicitly allowed by this specification. Such paths are intended to
/// have special meaning, such as /org/mpris/MediaPlayer2/TrackList/NoTrack
/// to indicate "no track".
///
/// ## Rationale
///
/// This is a D-Bus object id as that is the definitive way to have unique
/// identifiers on D-Bus. It also allows for future optional expansions
/// to the specification where tracks are exported to D-Bus with an
/// interface similar to org.gnome.UPnP.MediaItem2.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Type)]
#[serde(transparent)]
#[doc(alias = "Track_Id")]
pub struct TrackId(ObjectPath<'static>);

impl TrackId {
    /// A special track ID to indicate "no track".
    pub const NO_TRACK: TrackId = TrackId(ObjectPath::from_static_str_unchecked(
        "/org/mpris/MediaPlayer2/TrackList/NoTrack",
    ));

    /// Returns the track ID as an [`ObjectPath`].
    pub fn into_inner(self) -> ObjectPath<'static> {
        self.0
    }
}

impl ops::Deref for TrackId {
    type Target = ObjectPath<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TrackId> for ObjectPath<'static> {
    fn from(o: TrackId) -> Self {
        o.into_inner()
    }
}

impl From<TrackId> for Value<'static> {
    fn from(o: TrackId) -> Self {
        o.into_inner().into()
    }
}

impl<'unowned, 'owned: 'unowned> From<&'owned TrackId> for ObjectPath<'unowned> {
    fn from(o: &'owned TrackId) -> Self {
        ObjectPath::from_str_unchecked(o.as_str())
    }
}

impl<'a> From<ObjectPath<'a>> for TrackId {
    fn from(o: ObjectPath<'a>) -> Self {
        TrackId(o.into_owned())
    }
}

impl TryFrom<&'_ str> for TrackId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Ok(Self::from(ObjectPath::try_from(value)?))
    }
}

impl TryFrom<String> for TrackId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(Self::from(ObjectPath::try_from(value)?))
    }
}

impl TryFrom<Value<'_>> for TrackId {
    type Error = Error;

    fn try_from(value: Value<'_>) -> Result<Self> {
        ObjectPath::try_from(value).map(|o| Self(o.to_owned()))
    }
}

impl<'de> Deserialize<'de> for TrackId {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|s| ObjectPath::try_from(s).map_err(|e| de::Error::custom(e.to_string())))
            .map(Self)
    }
}

impl fmt::Display for TrackId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), f)
    }
}
