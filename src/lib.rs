#![warn(rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

mod local_server;
mod loop_status;
mod metadata;
mod playback_status;
mod player;
mod playlist;
mod playlist_ordering;
mod property;
mod server;
mod track_id;

use async_trait::async_trait;
use zbus::{fdo, zvariant::OwnedObjectPath, Result};

pub use crate::{
    local_server::LocalServer,
    loop_status::LoopStatus,
    metadata::{DateTime, Metadata},
    playback_status::PlaybackStatus,
    player::Player,
    playlist::{MaybePlaylist, Playlist},
    playlist_ordering::PlaylistOrdering,
    property::{PlaylistsProperty, Property, TrackListProperty},
    server::Server,
    track_id::TrackId,
};

pub mod export {
    pub use async_trait;
    pub use zbus;
}

pub mod builder {
    pub use crate::{metadata::MetadataBuilder, player::PlayerBuilder};
}

macro_rules! define_iface {
    (#[$attr:meta],
        $root_iface_ident:ident,
        $player_iface_ident:ident,
        $track_list_iface_ident:ident,
        $playlists_iface_ident:ident) => {
        /// Used to implement `org.mpris.MediaPlayer2` interface.
        #[$attr]
        pub trait $root_iface_ident {
            /// Brings the media player's user interface to the front using
            /// any appropriate mechanism available.
            ///
            /// The media player may be unable to control how its user
            /// interface is displayed, or it may not have a graphical user
            /// interface at all. In this case, the `CanRaise` property is
            /// `false` and this method does nothing.
            async fn raise(&self) -> fdo::Result<()>;

            /// Causes the media player to stop running.
            ///
            /// The media player may refuse to allow clients to shut it
            /// down. In this case, the `CanQuit` property is `false` and
            /// this method does nothing.
            ///
            /// Note: Media players which can be D-Bus activated, or for
            ///  which there is no sensibly easy way to terminate a running
            /// instance (via the main interface or a notification area icon
            /// for example) should allow clients to use this method.
            /// Otherwise, it should not be needed.
            ///
            /// If the media player does not have a UI, this should be
            /// implemented.
            async fn quit(&self) -> fdo::Result<()>;

            /// If `false`, calling `Quit` will have no effect, and may raise a
            /// NotSupported error. If `true`, calling `Quit` will cause the media
            /// application to attempt to quit (although it may still be
            /// prevented from quitting by the user, for example).
            async fn can_quit(&self) -> fdo::Result<bool>;

            /// Whether the media player is occupying the fullscreen.
            ///
            /// This is typically used for videos. A value of true indicates
            /// that the media player is taking up the full screen.
            ///
            /// Media centre software may well have this value fixed to true
            ///
            /// If CanSetFullscreen is true, clients may set this property
            /// to true to tell the media player to enter fullscreen mode,
            /// or to false to return to windowed mode.
            ///
            /// If CanSetFullscreen is false, then attempting to set this
            /// property should have no effect, and may raise an error.
            /// However, even if it is true, the media player may still
            /// be unable to fulfil the request, in which case attempting to
            /// set this property will have no effect (but should not raise
            /// an error).
            async fn fullscreen(&self) -> fdo::Result<bool>;

            async fn set_fullscreen(&self, fullscreen: bool) -> Result<()>;

            async fn can_set_fullscreen(&self) -> fdo::Result<bool>;

            async fn can_raise(&self) -> fdo::Result<bool>;

            async fn has_track_list(&self) -> fdo::Result<bool>;

            async fn identity(&self) -> fdo::Result<String>;

            async fn desktop_entry(&self) -> fdo::Result<String>;

            async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>>;

            async fn supported_mime_types(&self) -> fdo::Result<Vec<String>>;
        }

        /// Used to implement `org.mpris.MediaPlayer2.Player` interface.
        #[$attr]
        pub trait $player_iface_ident: $root_iface_ident {
            async fn next(&self) -> fdo::Result<()>;

            async fn previous(&self) -> fdo::Result<()>;

            async fn pause(&self) -> fdo::Result<()>;

            async fn play_pause(&self) -> fdo::Result<()>;

            async fn stop(&self) -> fdo::Result<()>;

            async fn play(&self) -> fdo::Result<()>;

            async fn seek(&self, offset: TimeInUs) -> fdo::Result<()>;

            async fn set_position(&self, track_id: TrackId, position: TimeInUs) -> fdo::Result<()>;

            async fn open_uri(&self, uri: String) -> fdo::Result<()>;

            async fn playback_status(&self) -> fdo::Result<PlaybackStatus>;

            async fn loop_status(&self) -> fdo::Result<LoopStatus>;

            async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()>;

            async fn rate(&self) -> fdo::Result<PlaybackRate>;

            async fn set_rate(&self, rate: PlaybackRate) -> Result<()>;

            async fn shuffle(&self) -> fdo::Result<bool>;

            async fn set_shuffle(&self, shuffle: bool) -> Result<()>;

            async fn metadata(&self) -> fdo::Result<Metadata>;

            async fn volume(&self) -> fdo::Result<Volume>;

            async fn set_volume(&self, volume: Volume) -> Result<()>;

            async fn position(&self) -> fdo::Result<TimeInUs>;

            async fn minimum_rate(&self) -> fdo::Result<PlaybackRate>;

            async fn maximum_rate(&self) -> fdo::Result<PlaybackRate>;

            async fn can_go_next(&self) -> fdo::Result<bool>;

            async fn can_go_previous(&self) -> fdo::Result<bool>;

            async fn can_play(&self) -> fdo::Result<bool>;

            async fn can_pause(&self) -> fdo::Result<bool>;

            async fn can_seek(&self) -> fdo::Result<bool>;

            async fn can_control(&self) -> fdo::Result<bool>;
        }

        /// Used to implement `org.mpris.MediaPlayer2.TrackList` interface.
        #[$attr]
        pub trait $track_list_iface_ident: $player_iface_ident {
            async fn get_tracks_metadata(
                &self,
                track_ids: Vec<TrackId>,
            ) -> fdo::Result<Vec<Metadata>>;

            async fn add_track(
                &self,
                uri: Uri,
                after_track: TrackId,
                set_as_current: bool,
            ) -> fdo::Result<()>;

            async fn remove_track(&self, track_id: TrackId) -> fdo::Result<()>;

            async fn go_to(&self, track_id: TrackId) -> fdo::Result<()>;

            async fn tracks(&self) -> fdo::Result<Vec<TrackId>>;

            async fn can_edit_tracks(&self) -> fdo::Result<bool>;
        }

        /// Used to implement `org.mpris.MediaPlayer2.Playlists` interface.
        #[$attr]
        pub trait $playlists_iface_ident: $player_iface_ident {
            async fn activate_playlist(&self, playlist_id: PlaylistId) -> fdo::Result<()>;

            async fn get_playlists(
                &self,
                index: u32,
                max_count: u32,
                order: PlaylistOrdering,
                reverse_order: bool,
            ) -> fdo::Result<Vec<Playlist>>;

            async fn playlist_count(&self) -> fdo::Result<u32>;

            async fn orderings(&self) -> fdo::Result<Vec<PlaylistOrdering>>;

            async fn active_playlist(&self) -> fdo::Result<MaybePlaylist>;
        }
    };
}

// define_iface!(
//     #[async_trait],
//     RootInterface,
//     PlayerInterface,
//     TrackListInterface,
//     PlaylistsInterface
// );

/// Used to implement `org.mpris.MediaPlayer2` interface.
#[async_trait]
pub trait RootInterface: Send + Sync {
    /// Brings the media player's user interface to the front using any
    /// appropriate mechanism available.
    ///
    /// The media player may be unable to control how its user interface is
    /// displayed, or it may not have a graphical user interface at all. In this
    /// case, the `CanRaise` property is `false` and this method does nothing.
    #[doc(alias = "Raise")]
    async fn raise(&self) -> fdo::Result<()>;

    /// Causes the media player to stop running.
    ///
    /// The media player may refuse to allow clients to shut it down. In this
    /// case, the `CanQuit` property is `false` and this method does nothing.
    ///
    /// Note: Media players which can be D-Bus activated, or for which there is
    /// no sensibly easy way to terminate a running instance (via the main
    /// interface or a notification area icon for example) should allow clients
    /// to use this method. Otherwise, it should not be needed.
    ///
    /// If the media player does not have a UI, this should be implemented.
    #[doc(alias = "Quit")]
    async fn quit(&self) -> fdo::Result<()>;

    #[doc(alias = "CanQuit")]
    async fn can_quit(&self) -> fdo::Result<bool>;

    #[doc(alias = "Fullscreen")]
    async fn fullscreen(&self) -> fdo::Result<bool>;

    #[doc(alias = "Fullscreen")]
    async fn set_fullscreen(&self, fullscreen: bool) -> Result<()>;

    #[doc(alias = "CanSetFullscreen")]
    async fn can_set_fullscreen(&self) -> fdo::Result<bool>;

    #[doc(alias = "CanRaise")]
    async fn can_raise(&self) -> fdo::Result<bool>;

    #[doc(alias = "HasTrackList")]
    async fn has_track_list(&self) -> fdo::Result<bool>;

    #[doc(alias = "Identity")]
    async fn identity(&self) -> fdo::Result<String>;

    #[doc(alias = "DesktopEntry")]
    async fn desktop_entry(&self) -> fdo::Result<String>;

    #[doc(alias = "SupportedUriSchemes")]
    async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>>;

    #[doc(alias = "SupportedMimeTypes")]
    async fn supported_mime_types(&self) -> fdo::Result<Vec<String>>;
}

#[async_trait]
#[doc(alias = "org.mpris.MediaPlayer2.Player")]
pub trait PlayerInterface: RootInterface {
    #[doc(alias = "Next")]
    async fn next(&self) -> fdo::Result<()>;

    #[doc(alias = "Previous")]
    async fn previous(&self) -> fdo::Result<()>;

    #[doc(alias = "Pause")]
    async fn pause(&self) -> fdo::Result<()>;

    #[doc(alias = "PlayPause")]
    async fn play_pause(&self) -> fdo::Result<()>;

    #[doc(alias = "Stop")]
    async fn stop(&self) -> fdo::Result<()>;

    #[doc(alias = "Play")]
    async fn play(&self) -> fdo::Result<()>;

    #[doc(alias = "Seek")]
    async fn seek(&self, offset: TimeInUs) -> fdo::Result<()>;

    #[doc(alias = "SetPosition")]
    async fn set_position(&self, track_id: TrackId, position: TimeInUs) -> fdo::Result<()>;

    #[doc(alias = "OpenUri")]
    async fn open_uri(&self, uri: String) -> fdo::Result<()>;

    #[doc(alias = "PlaybackStatus")]
    async fn playback_status(&self) -> fdo::Result<PlaybackStatus>;

    #[doc(alias = "LoopStatus")]
    async fn loop_status(&self) -> fdo::Result<LoopStatus>;

    #[doc(alias = "LoopStatus")]
    async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()>;

    #[doc(alias = "Rate")]
    async fn rate(&self) -> fdo::Result<PlaybackRate>;

    #[doc(alias = "Rate")]
    async fn set_rate(&self, rate: PlaybackRate) -> Result<()>;

    #[doc(alias = "Shuffle")]
    async fn shuffle(&self) -> fdo::Result<bool>;

    #[doc(alias = "Shuffle")]
    async fn set_shuffle(&self, shuffle: bool) -> Result<()>;

    #[doc(alias = "Metadata")]
    async fn metadata(&self) -> fdo::Result<Metadata>;

    #[doc(alias = "Volume")]
    async fn volume(&self) -> fdo::Result<Volume>;

    #[doc(alias = "Volume")]
    async fn set_volume(&self, volume: Volume) -> Result<()>;

    #[doc(alias = "Position")]
    async fn position(&self) -> fdo::Result<TimeInUs>;

    #[doc(alias = "MinimumRate")]
    async fn minimum_rate(&self) -> fdo::Result<PlaybackRate>;

    #[doc(alias = "MaximumRate")]
    async fn maximum_rate(&self) -> fdo::Result<PlaybackRate>;

    #[doc(alias = "CanGoNext")]
    async fn can_go_next(&self) -> fdo::Result<bool>;

    #[doc(alias = "CanGoPrevious")]
    async fn can_go_previous(&self) -> fdo::Result<bool>;

    #[doc(alias = "CanPlay")]
    async fn can_play(&self) -> fdo::Result<bool>;

    #[doc(alias = "CanPause")]
    async fn can_pause(&self) -> fdo::Result<bool>;

    #[doc(alias = "CanSeek")]
    async fn can_seek(&self) -> fdo::Result<bool>;

    #[doc(alias = "CanControl")]
    async fn can_control(&self) -> fdo::Result<bool>;
}

/// Used to implement `org.mpris.MediaPlayer2.Playlists` interface, which
/// provides access to a short list of tracks which were recently played or will
/// be played shortly. This is intended to provide context to the
/// currently-playing track, rather than giving complete access to the media
/// player's playlist.
///
/// Example use cases are the list of tracks from the same album as the
/// currently playing song or the Rhythmbox play queue.
///
/// Each track in the tracklist has a unique identifier. The intention is that
/// this uniquely identifies the track within the scope of the tracklist. In
/// particular, if a media item (a particular music file, say) occurs twice in
/// the track list, each occurrence should have a different identifier. If a
/// track is removed from the middle of the playlist, it should not affect the
/// track ids of any other tracks in the tracklist.
///
/// As a result, the traditional track identifiers of URLs and position in the
/// playlist cannot be used. Any scheme which satisfies the uniqueness
/// requirements is valid, as clients should not make any assumptions about the
/// value of the track id beyond the fact that it is a unique identifier.
///
/// Note that the (memory and processing) burden of implementing the TrackList
/// interface and maintaining unique track ids for the playlist can be mitigated
/// by only exposing a subset of the playlist when it is very long (the 20 or so
/// tracks around the currently playing track, for example). This is a
/// recommended practice as the tracklist interface is not designed to enable
/// browsing through a large list of tracks, but rather to provide clients with
/// context about the currently playing track.

#[async_trait]
#[doc(alias = "org.mpris.MediaPlayer2.TrackList")]
pub trait TrackListInterface: PlayerInterface {
    /// Gets all the metadata available for a set of tracks.
    ///
    /// ## Parameters
    ///
    /// * `track_ids` - The list of track ids for which metadata is requested.
    ///
    /// ## Returns
    ///
    /// * `metadata` - Metadata of the set of tracks given as input.
    ///
    /// Each set of metadata must have a [`mpris:trackid`] entry at the very
    /// least, which contains a string that uniquely identifies this track
    /// within the scope of the tracklist.
    ///
    /// [`mpris:trackid`]: Metadata::set_trackid
    #[doc(alias = "GetTracksMetadata")]
    async fn get_tracks_metadata(&self, track_ids: Vec<TrackId>) -> fdo::Result<Vec<Metadata>>;

    /// Adds a URI in the TrackList.
    ///
    /// ## Parameters
    ///
    /// * `uri` - The uri of the item to add. Its uri scheme should be an
    ///   element of the `org.mpris.MediaPlayer2.SupportedUriSchemes` property
    ///   and the mime-type should match one of the elements of the
    ///   `org.mpris.MediaPlayer2.SupportedMimeTypes`
    /// * `after_track` - The identifier of the track after which the new item
    ///   should be inserted. The path
    ///   [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] indicates that the track
    ///   should be inserted at the start of the track list.
    /// * `set_as_current` - Whether the newly inserted track should be
    ///   considered as the current track. Setting this to true has the same
    ///   effect as calling [`GoTo`] afterwards.
    ///
    /// If the [`CanEditTracks`] property is false, this has no effect.
    ///
    /// Note: Clients should not assume that the track has been added at the
    /// time when this method returns. They should wait for a [`TrackAdded`] (or
    /// [`TrackListReplaced`]) signal.
    ///
    /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: TrackId::NO_TRACK
    /// [`GoTo`]: Self::go_to
    /// [`CanEditTracks`]: Self::can_edit_tracks
    /// [`TrackAdded`]: Server::track_added
    /// [`TrackListReplaced`]: Server::track_list_replaced
    #[doc(alias = "AddTrack")]
    async fn add_track(
        &self,
        uri: Uri,
        after_track: TrackId,
        set_as_current: bool,
    ) -> fdo::Result<()>;

    /// Removes an item from the TrackList.
    ///
    /// ## Parameters
    ///
    /// * `track_id` - Identifier of the track to be removed.
    ///   [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] is *not* a valid value
    ///   for this argument.
    ///
    /// If the track is not part of this tracklist, this has no effect.
    ///
    /// If the [`CanEditTracks`] property is false, this has no effect.
    ///
    /// Note: Clients should not assume that the track has been removed at the
    /// time when this method returns. They should wait for a [`TrackRemoved`]
    /// (or TrackListReplaced) signal.
    ///
    /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: TrackId::NO_TRACK
    /// [`CanEditTracks`]: Self::can_edit_tracks
    /// [`TrackRemoved`]: Server::track_removed
    #[doc(alias = "RemoveTrack")]
    async fn remove_track(&self, track_id: TrackId) -> fdo::Result<()>;

    /// Skip to the specified TrackId.
    ///
    /// ## Parameters
    ///
    /// * `track_id` - Identifier of the track to skip to.
    ///   [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] is *not* a valid value
    ///   for this argument.
    ///
    /// If the track is not part of this tracklist, this has no effect.
    ///
    /// If this object is not `/org/mpris/MediaPlayer2`, the current TrackList's
    /// tracks should be replaced with the contents of this TrackList, and the
    /// [`TrackListReplaced`] signal should be fired from
    /// `/org/mpris/MediaPlayer2`.
    ///
    /// [`TrackListReplaced`]: Server::track_list_replaced
    #[doc(alias = "GoTo")]
    async fn go_to(&self, track_id: TrackId) -> fdo::Result<()>;

    /// An array which contains the identifier of each track in the tracklist,
    /// in order.
    ///
    /// The `org.freedesktop.DBus.Properties.PropertiesChanged` signal is
    /// emitted every time this property changes, but the signal message
    /// does not contain the new value. Client implementations should rather
    /// rely on the [`TrackAdded`], [`TrackRemoved`] and
    /// [`TrackListReplaced`] signals to keep their representation of the
    /// tracklist up to date.
    ///
    /// [`TrackAdded`]: Server::track_added
    /// [`TrackRemoved`]: Server::track_removed
    /// [`TrackListReplaced`]: Server::track_list_replaced
    #[doc(alias = "Tracks")]
    async fn tracks(&self) -> fdo::Result<Vec<TrackId>>;

    /// If false, calling [`AddTrack`] or [`RemoveTrack`] will have no effect,
    /// and may raise a NotSupported error.
    ///
    /// [`AddTrack`]: Self::add_track
    /// [`RemoveTrack`]: Self::remove_track
    #[doc(alias = "CanEditTracks")]
    async fn can_edit_tracks(&self) -> fdo::Result<bool>;
}

/// Used to implement `org.mpris.MediaPlayer2.Playlists` interface, which
/// provides access to the media player's playlists.
///
/// Since D-Bus does not provide an easy way to check for what interfaces are
/// exported on an object, clients should attempt to get one of the properties
/// on this interface to see if it is implemented.
#[async_trait]
#[doc(alias = "org.mpris.MediaPlayer2.Playlists")]
pub trait PlaylistsInterface: PlayerInterface {
    /// Starts playing the given playlist.
    ///
    /// ## Parameters
    ///
    /// * `playlist_id` - The id of the playlist to activate.
    ///
    /// Note that this must be implemented. If the media player does not allow
    /// clients to change the playlist, it should not implement this interface
    /// at all.
    ///
    /// It is up to the media player whether this completely replaces the
    /// current tracklist, or whether it is merely inserted into the tracklist
    /// and the first track starts. For example, if the media player is
    /// operating in a "jukebox" mode, it may just append the playlist to the
    /// list of upcoming tracks, and skip to the first track in the playlist.
    #[doc(alias = "ActivatePlaylist")]
    async fn activate_playlist(&self, playlist_id: PlaylistId) -> fdo::Result<()>;

    /// Gets a set of playlists.
    ///
    /// ## Parameters
    ///
    /// * `index` - The index of the first playlist to be fetched (according to
    ///   the ordering).
    /// * `max_count` - The maximum number of playlists to fetch.
    /// * `order` - The ordering that should be used.
    /// * `reverse_order` - Whether the order should be reversed.
    ///
    /// ## Returns
    ///
    /// * `playlists` - A list of (at most `max_count`) playlists.
    ///
    /// ## Rationale
    ///
    /// Media players may not have access to all the data required for some
    /// orderings. For example, creation times are not available on UNIX
    /// filesystems (don't let the ctime fool you!). On the other hand, clients
    /// should have some way to get the "most recent" playlists.
    #[doc(alias = "GetPlaylists")]
    async fn get_playlists(
        &self,
        index: u32,
        max_count: u32,
        order: PlaylistOrdering,
        reverse_order: bool,
    ) -> fdo::Result<Vec<Playlist>>;

    /// The number of playlists available.
    #[doc(alias = "PlaylistCount")]
    async fn playlist_count(&self) -> fdo::Result<u32>;

    /// The available orderings. At least one must be offered.
    #[doc(alias = "Orderings")]
    async fn orderings(&self) -> fdo::Result<Vec<PlaylistOrdering>>;

    /// The currently-active playlist.
    ///
    /// If there is no currently-active playlist, the structure's Valid field
    /// will be false, and the Playlist details are undefined.
    ///
    /// Note that this may not have a value even after [`ActivatePlaylist`] is
    /// called with a valid playlist id as [`ActivatePlaylist`] implementations
    /// have the option of simply inserting the contents of the playlist
    /// into the current tracklist.
    ///
    /// [`ActivatePlaylist`]: Self::activate_playlist
    #[doc(alias = "ActivePlaylist")]
    async fn active_playlist(&self) -> fdo::Result<MaybePlaylist>;
}

define_iface!(
    #[async_trait(?Send)],
    LocalRootInterface,
    LocalPlayerInterface,
    LocalTrackListInterface,
    LocalPlaylistsInterface
);

/// A playback rate.
///
/// This is a multiplier, so a value of 0.5 indicates that playback
/// is happening at half speed, while 1.5 means that 1.5 seconds of
/// "track time" is consumed every second.
#[doc(alias = "Playback_Rate")]
pub type PlaybackRate = f64;

/// Audio volume level.
///
/// * 0.0 means mute.
/// * 1.0 is a sensible maximum volume level (ex: 0dB).
///
/// Note that the volume may be higher than 1.0, although generally
/// clients should not attempt to set it above 1.0.
pub type Volume = f64;

/// Time in microseconds.
#[doc(alias = "Time_In_Us")]
pub type TimeInUs = i64;

/// Unique playlist identifier.
///
/// ## Rationale
///
/// Multiple playlists may have the same name.
///
/// This is a D-Bus object id as that is the definitive way to have unique
/// identifiers on D-Bus. It also allows for future optional expansions to
/// the specification where tracks are exported to D-Bus with an interface
/// similar to org.gnome.UPnP.MediaItem2.
#[doc(alias = "Playlist_Id")]
pub type PlaylistId = OwnedObjectPath;

/// A unique resource identifier.
///
/// URIs should be sent as (UTF-8) strings. Local files should use the
/// "file://" schema.
pub type Uri = String;
