use crate::{
    LoopStatus, MaybePlaylist, Metadata, PlaybackRate, PlaybackStatus, PlaylistOrdering, Volume,
};

/// Used for emitting `PropertiesChanged` signals on
/// [`Server::properties_changed`] and [`LocalServer::properties_changed`].
///
/// [`Server::properties_changed`]: crate::Server::properties_changed
/// [`LocalServer::properties_changed`]: crate::LocalServer::properties_changed
#[derive(Clone, Debug, PartialEq)]
pub enum Property {
    CanQuit(bool),
    Fullscreen(bool),
    CanSetFullscreen(bool),
    CanRaise(bool),
    HasTrackList(bool),
    Identity(String),
    DesktopEntry(String),
    SupportedUriSchemes(Vec<String>),
    SupportedMimeTypes(Vec<String>),
    PlaybackStatus(PlaybackStatus),
    LoopStatus(LoopStatus),
    Rate(PlaybackRate),
    Shuffle(bool),
    Metadata(Metadata),
    Volume(Volume),
    // Position (must use `Rate` property together with `Seeked` signal instead)
    MinimumRate(PlaybackRate),
    MaximumRate(PlaybackRate),
    CanGoNext(bool),
    CanGoPrevious(bool),
    CanPlay(bool),
    CanPause(bool),
    CanSeek(bool),
    // CanControl (not expected to change)
}

/// Used for emitting `PropertiesChanged` signals on
/// [`Server::track_list_properties_changed`] and
/// [`LocalServer::track_list_properties_changed`], if `T` implements
/// [`TrackListInterface`] or [`LocalTrackListInterface`].
///
/// [`Server::track_list_properties_changed`]: crate::Server::track_list_properties_changed
/// [`LocalServer::track_list_properties_changed`]: crate::LocalServer::track_list_properties_changed
/// [`TrackListInterface`]: crate::TrackListInterface
/// [`LocalTrackListInterface`]: crate::LocalTrackListInterface
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackListProperty {
    // The new value must not be sent according to the spec.
    Tracks,
    CanEditTracks(bool),
}

/// Used for emitting `PropertiesChanged` signals on
/// [`Server::playlists_properties_changed`] and
/// [`LocalServer::playlists_properties_changed`], if `T` implements
/// [`PlaylistsInterface`] or [`LocalPlaylistsInterface`].
///
/// [`Server::playlists_properties_changed`]: crate::Server::playlists_properties_changed
/// [`LocalServer::playlists_properties_changed`]: crate::LocalServer::playlists_properties_changed
/// [`PlaylistsInterface`]: crate::PlaylistsInterface
/// [`LocalPlaylistsInterface`]: crate::LocalPlaylistsInterface
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlaylistsProperty {
    PlaylistCount(u32),
    Orderings(Vec<PlaylistOrdering>),
    ActivePlaylist(MaybePlaylist),
}
