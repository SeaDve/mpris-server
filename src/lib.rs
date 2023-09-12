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

use async_trait::async_trait;
use zbus::{fdo, zvariant::OwnedObjectPath, Result};

pub use crate::{
    local_server::LocalServer,
    loop_status::LoopStatus,
    metadata::{DateTime, Metadata, Uri},
    playback_status::PlaybackStatus,
    player::Player,
    playlist::{MaybePlaylist, Playlist},
    playlist_ordering::PlaylistOrdering,
    property::{PlaylistsProperty, Property, TrackListProperty},
    server::Server,
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
            ///  set this property will have no effect (but should not raise
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

define_iface!(
    #[async_trait],
    RootInterface,
    PlayerInterface,
    TrackListInterface,
    PlaylistsInterface
);

define_iface!(
    #[async_trait(?Send)],
    LocalRootInterface,
    LocalPlayerInterface,
    LocalTrackListInterface,
    LocalPlaylistsInterface
);

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
/// ### Rationale
///
/// This is a D-Bus object id as that is the definitive way to have unique
/// identifiers on D-Bus. It also allows for future optional expansions
/// to the specification where tracks are exported to D-Bus with an
/// interface similar to org.gnome.UPnP.MediaItem2.
pub type TrackId = OwnedObjectPath;

/// A playback rate.
///
/// This is a multiplier, so a value of 0.5 indicates that playback
/// is happening at half speed, while 1.5 means that 1.5 seconds of
/// "track time" is consumed every second.
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
pub type TimeInUs = i64;

/// Unique playlist identifier.
///
/// # Rationale
///
/// Multiple playlists may have the same name.
///
/// This is a D-Bus object id as that is the definitive way to have unique
/// identifiers on D-Bus. It also allows for future optional expansions to
/// the specification where tracks are exported to D-Bus with an interface
/// similar to org.gnome.UPnP.MediaItem2.
pub type PlaylistId = OwnedObjectPath;
