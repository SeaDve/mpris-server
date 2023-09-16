use crate::{Metadata, Playlist, Time, TrackId};

#[derive(Debug)]
pub enum Signal {
    /// Indicates that the track position has changed in a way that is
    /// inconsistent with the current playing state.
    ///
    /// ## Parameters
    ///
    /// * `position` - The new position.
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
    Seeked { position: Time },
}

#[derive(Debug)]
pub enum TrackListSignal {
    /// Indicates that the entire tracklist has been replaced.
    ///
    /// ## Parameters
    ///
    /// * `tracks` - The new content of the tracklist.
    /// * `current_track` - The identifier of the track to be considered as
    ///   current. [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] indicates that
    ///   there is no current track.  This should correspond to the
    ///   [`mpris:trackid`] field of the [`Metadata`] property of the [`Player
    ///   interface`].
    ///
    /// It is left up to the implementation to decide when a change to the track
    /// list is invasive enough that this signal should be emitted instead of a
    /// series of [`TrackAdded`] and [`TrackRemoved`] signals.
    ///
    /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: crate::TrackId::NO_TRACK
    /// [`mpris:trackid`]: crate::Metadata::trackid
    /// [`Metadata`]: crate::PlayerInterface::metadata
    /// [`Player interface`]: crate::PlayerInterface
    /// [`TrackAdded`]: TrackListSignal::TrackAdded
    /// [`TrackRemoved`]: TrackListSignal::TrackRemoved
    TrackListReplaced {
        tracks: Vec<TrackId>,
        current_track: TrackId,
    },
    /// Indicates that a track has been added to the track list.
    ///
    /// ## Parameters
    ///
    /// * `metadata` - The metadata of the newly added item. This must include a
    ///   [`mpris:trackid`] entry.
    /// * `after_track` - The identifier of the track after which the new track
    ///   was inserted. The path [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]
    ///   indicates that the track was inserted at the start of the track list.
    ///
    /// [`mpris:trackid`]: crate::Metadata::trackid
    /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: crate::TrackId::NO_TRACK
    TrackAdded {
        metadata: Metadata,
        after_track: TrackId,
    },
    /// Indicates that a track has been removed from the track list.
    ///
    /// ## Parameters
    ///
    /// * `track_id` - The identifier of the track being removed.
    ///   [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] is not a valid value for
    ///   this argument.
    ///
    /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: crate::TrackId::NO_TRACK
    TrackRemoved { track_id: TrackId },

    /// Indicates that the metadata of a track in the tracklist has changed.
    ///
    /// ## Parameters
    ///
    /// * `track_id` - The id of the track which metadata has changed. If the
    ///   track id has changed, this will be the old value.
    ///   [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] is not a valid value for
    ///   this argument.
    /// * `metadata` - The new track metadata. This must include a
    ///   [`mpris:trackid`] entry. If the track id has changed, this will be the
    ///   new value.
    ///
    /// This may indicate that a track has been replaced, in which case the
    /// [`mpris:trackid`] metadata entry is different from the `track_id`
    /// argument.
    ///
    /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: crate::TrackId::NO_TRACK
    /// [`mpris:trackid`]: crate::Metadata::trackid
    TrackMetadataChanged {
        track_id: TrackId,
        metadata: Metadata,
    },
}

#[derive(Debug)]
pub enum PlaylistsSignal {
    /// Indicates that either the Name or Icon attribute of a playlist has
    /// changed.
    ///
    /// ## Parameters
    ///
    /// * `playlist` - The playlist which details have changed.
    ///
    /// Client implementations should be aware that this signal may not be
    /// implemented.
    ///
    /// ## Rationale
    ///
    /// Without this signal, media players have no way to notify clients of a
    /// change in the attributes of a playlist other than the active one.
    PlaylistChanged { playlist: Playlist },
}
