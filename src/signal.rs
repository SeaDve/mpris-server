use crate::{Metadata, Playlist, Time, TrackId};

/// Used for emitting signals on [`Server::emit`] and [`LocalServer::emit`].
///
/// [`Server::emit`]: crate::Server::emit
/// [`LocalServer::emit`]: crate::LocalServer::emit
#[derive(Debug)]
pub enum Signal {
    /// Indicates that the track position has changed in a way that is
    /// inconsistent with the current playing state.
    ///
    /// When this signal is not received, clients should assume that:
    ///
    /// * When playing, the position progresses according to the rate property.
    /// * When paused, it remains constant.
    ///
    /// This signal does not need to be emitted when playback starts or when the
    /// track changes, unless the track is starting at an unexpected position.
    /// An expected position would be the last known one when going from Paused
    /// to Playing, and 0 when going from Stopped to Playing.
    Seeked {
        /// The new position.
        position: Time,
    },
}

/// Used for emitting signals on [`Server::track_list_emit`] and
/// [`LocalServer::track_list_emit`], if `T` implements [`TrackListInterface`]
/// or [`LocalTrackListInterface`].
///
/// [`Server::track_list_emit`]: crate::Server::track_list_emit
/// [`LocalServer::track_list_emit`]: crate::LocalServer::track_list_emit
/// [`TrackListInterface`]: crate::TrackListInterface
/// [`LocalTrackListInterface`]: crate::LocalTrackListInterface
#[derive(Debug)]
pub enum TrackListSignal {
    /// Indicates that the entire tracklist has been replaced.
    ///
    /// It is left up to the implementation to decide when a change to the track
    /// list is invasive enough that this signal should be emitted instead of a
    /// series of [`TrackAdded`] and [`TrackRemoved`] signals.
    ///
    /// [`TrackAdded`]: TrackListSignal::TrackAdded
    /// [`TrackRemoved`]: TrackListSignal::TrackRemoved
    TrackListReplaced {
        /// The new content of the tracklist.
        tracks: Vec<TrackId>,
        /// The identifier of the track to be considered as current.
        ///
        /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] indicates that there
        /// is no current track.
        ///
        /// This should correspond to the [`mpris:trackid`] field of the
        /// [`Metadata`] property of the [`Player interface`].
        ///
        /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: crate::TrackId::NO_TRACK
        /// [`mpris:trackid`]: crate::Metadata::trackid
        /// [`Metadata`]: crate::PlayerInterface::metadata
        /// [`Player interface`]: crate::PlayerInterface
        current_track: TrackId,
    },
    /// Indicates that a track has been added to the track list.
    TrackAdded {
        /// The metadata of the newly added item.
        ///
        /// This must include a [`mpris:trackid`] entry.
        ///
        /// [`mpris:trackid`]: crate::Metadata::trackid
        metadata: Metadata,
        /// The identifier of the track after which the new track was inserted.
        /// The path [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] indicates
        /// that the track was inserted at the start of the track list.
        ///
        /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: crate::TrackId::NO_TRACK
        after_track: TrackId,
    },
    /// Indicates that a track has been removed from the track list.
    TrackRemoved {
        /// The identifier of the track being removed.
        ///
        /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] is *not* a valid value
        /// for this argument.
        ///
        /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: crate::TrackId::NO_TRACK
        track_id: TrackId,
    },
    /// Indicates that the metadata of a track in the tracklist has changed.
    ///
    /// This may indicate that a track has been replaced, in which case the
    /// [`mpris:trackid`] metadata entry is different from the `track_id`
    /// argument.
    ///
    /// [`mpris:trackid`]: crate::Metadata::trackid
    TrackMetadataChanged {
        /// The id of the track which metadata has changed.
        ///
        /// If the track id has changed, this will be the old value.
        ///
        /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] is *not* a valid value
        /// for this argument.
        ///
        /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: crate::TrackId::NO_TRACK
        track_id: TrackId,
        /// The new track metadata.
        ///
        /// This must include a [`mpris:trackid`] entry. If the track id has
        /// changed, this will be the new value.
        ///
        /// [`mpris:trackid`]: crate::Metadata::trackid
        metadata: Metadata,
    },
}

/// Used for emitting signals on [`Server::playlists_emit`] and
/// [`LocalServer::playlists_emit`], if `T` implements
/// [`PlaylistsInterface`] or [`LocalPlaylistsInterface`].
///
/// [`Server::playlists_emit`]: crate::Server::playlists_emit
/// [`LocalServer::playlists_emit`]: crate::LocalServer::playlists_emit
/// [`PlaylistsInterface`]: crate::PlaylistsInterface
/// [`LocalPlaylistsInterface`]: crate::LocalPlaylistsInterface
#[derive(Debug)]
pub enum PlaylistsSignal {
    /// Indicates that either the Name or Icon attribute of a playlist has
    /// changed.
    ///
    /// Client implementations should be aware that this signal may not be
    /// implemented.
    ///
    /// ## Rationale
    ///
    /// Without this signal, media players have no way to notify clients of a
    /// change in the attributes of a playlist other than the active one.
    PlaylistChanged {
        /// The playlist which details have changed.
        playlist: Playlist,
    },
}
