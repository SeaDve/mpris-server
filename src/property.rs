use enumflags2::bitflags;

/// Used for emitting `PropertiesChanged` signals on [`Server<T>`] via [`Server::properties_changed`].
///
/// [`Server<T>`]: crate::Server
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
    Position,
    MinimumRate,
    MaximumRate,
    CanGoNext,
    CanGoPrevious,
    CanPlay,
    CanPause,
    CanSeek,
    CanControl,
}

/// Used for emitting `PropertiesChanged` signals on [`Server<T>`] via [`Server::track_list_properties_changed`],
/// if `T` implements [`TrackListInterface`].
///
/// [`Server<T>`]: crate::Server
/// [`Server::track_list_properties_changed`]: crate::Server::track_list_properties_changed
/// [`TrackListInterface`]: crate::TrackListInterface
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TrackListProperty {
    Tracks,
    CanEditTracks,
}

/// Used for emitting `PropertiesChanged` signals on [`Server<T>`] via [`Server::playlists_properties_changed`],
/// if `T` implements [`PlaylistsInterface`].
///
/// [`Server<T>`]: crate::Server
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
