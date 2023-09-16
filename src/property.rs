use enumflags2::bitflags;

/// Used for emitting `PropertiesChanged` signals on
/// [`Server::properties_changed`] and [`LocalServer::properties_changed`].
///
/// [`Server::properties_changed`]: crate::Server::properties_changed
/// [`LocalServer::properties_changed`]: crate::LocalServer::properties_changed
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Property {
    CanQuit,
    Fullscreen,
    CanSetFullscreen,
    CanRaise,
    HasTrackList,
    Identity,
    DesktopEntry,
    SupportedUriSchemes,
    SupportedMimeTypes,
    PlaybackStatus,
    LoopStatus,
    Rate,
    Shuffle,
    Metadata,
    Volume,
    // Position (must use `Rate` property together with `Seeked` signal instead)
    MinimumRate,
    MaximumRate,
    CanGoNext,
    CanGoPrevious,
    CanPlay,
    CanPause,
    CanSeek,
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
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TrackListProperty {
    Tracks,
    CanEditTracks,
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
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlaylistsProperty {
    PlaylistCount,
    Orderings,
    ActivePlaylist,
}
