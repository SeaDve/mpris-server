use enumflags2::bitflags;

/// Used for emitting `PropertiesChanged` signals on [`Server`] via
/// [`Server::properties_changed`].
///
/// [`Server`]: crate::Server
/// [`Server::properties_changed`]: crate::Server::properties_changed
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

/// Used for emitting `PropertiesChanged` signals on [`Server`] via
/// [`Server::track_list_properties_changed`], if `T` implements
/// [`TrackListInterface`].
///
/// [`Server`]: crate::Server
/// [`Server::track_list_properties_changed`]: crate::Server::track_list_properties_changed
/// [`TrackListInterface`]: crate::TrackListInterface
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TrackListProperty {
    Tracks,
    CanEditTracks,
}

/// Used for emitting `PropertiesChanged` signals on [`Server`] via
/// [`Server::playlists_properties_changed`], if `T` implements
/// [`PlaylistsInterface`].
///
/// [`Server`]: crate::Server
/// [`Server::playlists_properties_changed`]: crate::Server::playlists_properties_changed
/// [`PlaylistsInterface`]: crate::PlaylistsInterface
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlaylistsProperty {
    PlaylistCount,
    Orderings,
    ActivePlaylist,
}
