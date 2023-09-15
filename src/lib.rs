#![warn(
    rust_2018_idioms,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

// TODO:
// * Document public interface
// * Replace `DateTime`, and `Uri` with proper types
// * Profile if inlining is worth it
// * Add public `test` method to check if interface is implemented correctly
// * Avoid clones in `Metadata` getters

mod local_server;
mod loop_status;
mod metadata;
mod playback_status;
mod player;
mod playlist;
mod playlist_ordering;
mod property;
mod server;
mod time;
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
    time::Time,
    track_id::TrackId,
};

/// This contains libraries that are used alongside with this crate.
pub mod export {
    pub use async_trait;
    pub use zbus;
}

/// This contains the definitions of builder-pattern structs.
///
/// The `builder` methods on the objects must be used instead to construct
/// these builder-pattern structs.
pub mod builder {
    pub use crate::{metadata::MetadataBuilder, player::PlayerBuilder};
}

macro_rules! define_iface {
    (#[$attr:meta],
        $root_iface_ident:ident$(: $bound:tt $(+ $other_bounds:tt)* )? extra_docs $extra_root_docs:literal,
        $player_iface_ident:ident extra_docs $extra_player_docs:literal,
        $track_list_iface_ident:ident extra_docs $extra_track_list_docs:literal,
        $playlists_iface_ident:ident extra_docs $extra_playlists_docs:literal) => {
        #[doc = $extra_root_docs]
        #[doc = ""]
        /// Used to implement [org.mpris.MediaPlayer2] interface.
        ///
        /// [org.mpris.MediaPlayer2]: https://specifications.freedesktop.org/mpris-spec/2.2/Media_Player.html
        #[$attr]
        pub trait $root_iface_ident$(: $bound $(+ $other_bounds)* )?  {
            /// Brings the media player's user interface to the front using any
            /// appropriate mechanism available.
            ///
            /// The media player may be unable to control how its user interface is
            /// displayed, or it may not have a graphical user interface at all. In this
            /// case, the [`CanRaise`] property is **false** and this method does
            /// nothing.
            ///
            /// [`CanRaise`]: Self::can_raise
            #[doc(alias = "Raise")]
            async fn raise(&self) -> fdo::Result<()>;

            /// Causes the media player to stop running.
            ///
            /// The media player may refuse to allow clients to shut it down. In this
            /// case, the [`CanQuit`] property is **false** and this method does
            /// nothing.
            ///
            /// **Note:** Media players which can be D-Bus activated, or for which there
            /// is no sensibly easy way to terminate a running instance (via the
            /// main interface or a notification area icon for example) should allow
            /// clients to use this method. Otherwise, it should not be needed.
            ///
            /// If the media player does not have a UI, this should be implemented.
            ///
            /// [`CanQuit`]: Self::can_quit
            #[doc(alias = "Quit")]
            async fn quit(&self) -> fdo::Result<()>;

            /// Whether the player may be asked to quit.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// If **false**, calling [`Quit`] will have no effect, and may raise a
            /// `NotSupported` error. If **true**, calling [`Quit`] will cause the media
            /// application to attempt to quit (although it may still be prevented from
            /// quitting by the user, for example).
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`Quit`]: Self::quit
            #[doc(alias = "CanQuit")]
            async fn can_quit(&self) -> fdo::Result<bool>;

            /// Whether the media player is occupying the fullscreen.
            ///
            /// This property is *optional*. Clients should handle its absence
            /// gracefully.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// This is typically used for videos. A value of **true** indicates that
            /// the media player is taking up the full screen.
            ///
            /// Media centre software may well have this value fixed to **true**
            ///
            /// If [`CanSetFullscreen`] is **true**, clients may set this property to
            /// **true** to tell the media player to enter fullscreen mode, or to
            /// **false** to return to windowed mode.
            ///
            /// If [`CanSetFullscreen`] is **false**, then attempting to set this
            /// property should have no effect, and may raise an error. However,
            /// even if it is **true**, the media player may still be unable to
            /// fulfil the request, in which case attempting to set this property
            /// will have no effect (but should not raise an error).
            ///
            /// ## Rationale
            ///
            /// This allows remote control interfaces, such as LIRC or mobile devices
            /// like phones, to control whether a video is shown in fullscreen.
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`CanSetFullscreen`]: Self::can_set_fullscreen
            #[doc(alias = "Fullscreen")]
            async fn fullscreen(&self) -> fdo::Result<bool>;

            /// Sets whether the media player is occupying the fullscreen.
            ///
            /// See [`Fullscreen`] for more details.
            ///
            /// [`Fullscreen`]: Self::fullscreen
            #[doc(alias = "Fullscreen")]
            async fn set_fullscreen(&self, fullscreen: bool) -> Result<()>;

            /// Whether the player may be asked to enter or leave fullscreen.
            ///
            /// This property is *optional*. Clients should handle its absence
            /// gracefully.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// If **false**, attempting to set [`Fullscreen`] will have no effect, and
            /// may raise an error. If **true**, attempting to set [`Fullscreen`]
            /// will not raise an error, and (if it is different from the current
            /// value) will cause the media player to attempt to enter or exit
            /// fullscreen mode.
            ///
            /// Note that the media player may be unable to fulfil the request. In this
            /// case, the value will not change. If the media player knows in advance
            /// that it will not be able to fulfil the request, however, this property
            /// should be **false**.
            ///
            /// ## Rationale
            ///
            /// This allows clients to choose whether to display controls for entering
            /// or exiting fullscreen mode.
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`Fullscreen`]: Self::fullscreen
            #[doc(alias = "CanSetFullscreen")]
            async fn can_set_fullscreen(&self) -> fdo::Result<bool>;

            /// Whether the media player may be asked to be raised.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// If **false**, calling [`Raise`] will have no effect, and may raise a
            /// `NotSupported` error. If **true**, calling [`Raise`] will cause the
            /// media application to attempt to bring its user interface to the
            /// front, although it may be prevented from doing so (by the window
            /// manager, for example).
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`Raise`]: Self::raise
            #[doc(alias = "CanRaise")]
            async fn can_raise(&self) -> fdo::Result<bool>;

            /// Indicates whether the `/org/mpris/MediaPlayer2` object implements the
            /// [`TrackList interface`].
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// [`TrackList interface`]: TrackListInterface
            /// [`properties_changed`]: Server::properties_changed
            #[doc(alias = "HasTrackList")]
            async fn has_track_list(&self) -> fdo::Result<bool>;

            /// A friendly name to identify the media player to users (eg: "VLC media
            /// player").
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// This should usually match the name found in .desktop files
            ///
            /// [`properties_changed`]: Server::properties_changed
            #[doc(alias = "Identity")]
            async fn identity(&self) -> fdo::Result<String>;

            /// The basename of an installed .desktop file which complies with the
            /// [Desktop entry specification], with the ".desktop" extension stripped.
            ///
            /// This property is *optional*. Clients should handle its absence
            /// gracefully.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// Example: The desktop entry file is
            /// "/usr/share/applications/vlc.desktop", and this property contains "vlc"
            ///
            /// [Desktop entry specification]: https://specifications.freedesktop.org/desktop-entry-spec/latest/
            /// [`properties_changed`]: Server::properties_changed
            #[doc(alias = "DesktopEntry")]
            async fn desktop_entry(&self) -> fdo::Result<String>;

            /// The URI schemes supported by the media player.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// This can be viewed as protocols supported by the player in almost all
            /// cases. Almost every media player will include support for the "file"
            /// scheme. Other common schemes are "http" and "rtsp".
            ///
            /// Note that URI schemes should be lower-case.
            ///
            /// ## Rationale
            ///
            /// This is important for clients to know when using the editing
            /// capabilities of the [`Playlists interface`], for example.
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`Playlists interface`]: PlaylistsInterface
            #[doc(alias = "SupportedUriSchemes")]
            async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>>;

            /// The mime-types supported by the media player.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// Mime-types should be in the standard format (eg: audio/mpeg or
            /// application/ogg).
            ///
            /// ## Rationale
            ///
            /// This is important for clients to know when using the editing
            /// capabilities of the [`Playlists interface`], for example.
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`Playlists interface`]: PlaylistsInterface
            #[doc(alias = "SupportedMimeTypes")]
            async fn supported_mime_types(&self) -> fdo::Result<Vec<String>>;
        }

        #[doc = $extra_player_docs]
        #[doc = ""]
        /// Used to implement [org.mpris.MediaPlayer2.Player] interface, which
        /// implements the methods for querying and providing basic control over what is
        /// currently playing.
        ///
        /// [org.mpris.MediaPlayer2.Player]: https://specifications.freedesktop.org/mpris-spec/2.2/Player_Interface.html
        #[$attr]
        #[doc(alias = "org.mpris.MediaPlayer2.Player")]
        pub trait $player_iface_ident: $root_iface_ident {
            /// Skips to the next track in the tracklist.
            ///
            /// If there is no next track (and endless playback and track repeat are
            /// both off), stop playback.
            ///
            /// If playback is paused or stopped, it remains that way.
            ///
            /// If [`CanGoNext`] is **false**, attempting to call this method should
            /// have no effect.
            ///
            /// [`CanGoNext`]: Self::can_go_next
            #[doc(alias = "Next")]
            async fn next(&self) -> fdo::Result<()>;

            /// Skips to the previous track in the tracklist.
            ///
            /// If there is no previous track (and endless playback and track repeat are
            /// both off), stop playback.
            ///
            /// If playback is paused or stopped, it remains that way.
            ///
            /// If [`CanGoPrevious`] is **false**, attempting to call this method should
            /// have no effect.
            ///
            /// [`CanGoPrevious`]: Self::can_go_previous
            #[doc(alias = "Previous")]
            async fn previous(&self) -> fdo::Result<()>;

            /// Pauses playback.
            ///
            /// If playback is already paused, this has no effect.
            ///
            /// Calling [`Play`] after this should cause playback to start again from
            /// the same position.
            ///
            /// If [`CanPause`] is **false**, attempting to call this method should have
            /// no effect.
            ///
            /// [`Play`]: Self::play
            /// [`CanPause`]: Self::can_pause
            #[doc(alias = "Pause")]
            async fn pause(&self) -> fdo::Result<()>;

            /// Pauses playback.
            ///
            /// If playback is already paused, resumes playback.
            ///
            /// If playback is stopped, starts playback.
            ///
            /// If [`CanPause`] is **false**, attempting to call this method should have
            /// no effect and raise an error.
            ///
            /// [`CanPause`]: Self::can_pause
            #[doc(alias = "PlayPause")]
            async fn play_pause(&self) -> fdo::Result<()>;

            /// Stops playback.
            ///
            /// If playback is already stopped, this has no effect.
            ///
            /// Calling Play after this should cause playback to start again from the
            /// beginning of the track.
            ///
            /// If [`CanControl`] is **false**, attempting to call this method should
            /// have no effect and raise an error.
            ///
            /// [`CanControl`]: Self::can_control
            #[doc(alias = "Stop")]
            async fn stop(&self) -> fdo::Result<()>;

            /// Starts or resumes playback.
            ///
            /// If already playing, this has no effect.
            ///
            /// If paused, playback resumes from the current position.
            ///
            /// If there is no track to play, this has no effect.
            ///
            /// If [`CanPlay`] is **false**, attempting to call this method should have
            /// no effect.
            ///
            /// [`CanPlay`]: Self::can_play
            #[doc(alias = "Play")]
            async fn play(&self) -> fdo::Result<()>;

            /// Seeks forward in the current track by the specified offset in time.
            ///
            /// ## Parameters
            ///
            /// * `offset` - The offset in time to seek forward.
            ///
            /// A negative value seeks back. If this would mean seeking back further
            /// than the start of the track, the position is set to 0.
            ///
            /// If the value passed in would mean seeking beyond the end of the track,
            /// acts like a call to Next.
            ///
            /// If the [`CanSeek`] property is **false**, this has no effect.
            ///
            /// [`CanSeek`]: Self::can_seek
            #[doc(alias = "Seek")]
            async fn seek(&self, offset: Time) -> fdo::Result<()>;

            /// Sets the current track position.
            ///
            /// ## Parameters
            ///
            /// * `track_id` - The currently playing track's identifier. If this does
            ///   not match the id of the currently-playing track, the call is ignored
            ///   as "stale". [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] is not a
            ///   valid value for this argument.
            /// * `position` - The track position. This must be between 0 and
            ///   <track_length>.
            ///
            /// If the Position argument is less than 0, do nothing.
            ///
            /// If the Position argument is greater than the track length, do nothing.
            ///
            /// If the [`CanSeek`] property is **false**, this has no effect.
            ///
            /// ## Rationale
            ///
            /// The reason for having this method, rather than making [`Position`]
            /// writable, is to include the `track_id` argument to avoid race
            /// conditions where a client tries to seek to a position when the track
            /// has already changed.
            ///
            /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: TrackId::NO_TRACK
            /// [`CanSeek`]: Self::can_seek
            /// [`Position`]: Self::position
            #[doc(alias = "SetPosition")]
            async fn set_position(&self, track_id: TrackId, position: Time) -> fdo::Result<()>;

            /// Opens the `uri` given as an argument
            ///
            /// ## Parameters
            ///
            /// * `uri` - Uri of the track to load. Its uri scheme should be an element
            ///   of the [`SupportedUriSchemes`] property and the mime-type should match
            ///   one of the elements of the [`SupportedMimeTypes`].
            ///
            /// If the playback is stopped, starts playing
            ///
            /// If the uri scheme or the mime-type of the uri to open is not supported,
            /// this method does nothing and may raise an error. In particular, if the
            /// list of available uri schemes is empty, this method may not be
            /// implemented.
            ///
            /// Clients should not assume that the `uri` has been opened as soon as this
            /// method returns. They should wait until the [`mpris:trackid`] field in
            /// the [`Metadata`] property changes.
            ///
            /// If the media player implements the [`TrackList interface`], then the
            /// opened track should be made part of the tracklist, the [`TrackAdded`] or
            /// [`TrackListReplaced`] signal should be fired, as well as the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal on the
            /// [`TrackList interface`].
            ///
            /// [`SupportedUriSchemes`]: RootInterface::supported_uri_schemes
            /// [`SupportedMimeTypes`]: RootInterface::supported_mime_types
            /// [`mpris:trackid`]: Metadata::set_trackid
            /// [`Metadata`]: Self::metadata
            /// [`TrackList interface`]: TrackListInterface
            /// [`TrackAdded`]: Server::track_added
            /// [`TrackListReplaced`]: Server::track_list_replaced
            #[doc(alias = "OpenUri")]
            async fn open_uri(&self, uri: String) -> fdo::Result<()>;

            /// The current playback status.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// May be [`Playing`], [`Paused`] or [`Stopped`].
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`Playing`]: PlaybackStatus::Playing
            /// [`Paused`]: PlaybackStatus::Paused
            /// [`Stopped`]: PlaybackStatus::Stopped
            #[doc(alias = "PlaybackStatus")]
            async fn playback_status(&self) -> fdo::Result<PlaybackStatus>;

            /// The current loop / repeat status
            ///
            /// This property is *optional*. Clients should handle its absence
            /// gracefully.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// May be:
            ///
            /// * [`None`] if the playback will stop when there are no more tracks to
            ///   play
            /// * [`Track`] if the current track will start again from the beginning
            ///   once it has finished playing
            /// * [`Playlist`] if the playback loops through a list of tracks
            ///
            /// If [`CanControl`] is **false**, attempting to set this property should
            /// have no effect and raise an error.
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`None`]: LoopStatus::None
            /// [`Track`]: LoopStatus::Track
            /// [`Playlist`]: LoopStatus::Playlist
            /// [`CanControl`]: Self::can_control
            #[doc(alias = "LoopStatus")]
            async fn loop_status(&self) -> fdo::Result<LoopStatus>;

            /// Sets the current loop / repeat status
            ///
            /// See [`LoopStatus`] for more details.
            ///
            /// [`LoopStatus`]: Self::loop_status
            #[doc(alias = "LoopStatus")]
            async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()>;

            /// The current playback rate.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// The value must fall in the range described by [`MinimumRate`] and
            /// [`MaximumRate`], and must not be 0.0. If playback is paused, the
            /// [`PlaybackStatus`] property should be used to indicate this. A value of
            /// 0.0 should not be set by the client. If it is, the media player
            /// should act as though [`Pause`] was called.
            ///
            /// If the media player has no ability to play at speeds other than the
            /// normal playback rate, this must still be implemented, and must return
            /// 1.0. The [`MinimumRate`] and [`MaximumRate`] properties must also be set
            /// to 1.0.
            ///
            /// Not all values may be accepted by the media player. It is left to media
            /// player implementations to decide how to deal with values they cannot
            /// use; they may either ignore them or pick a "best fit" value. Clients are
            /// recommended to only use sensible fractions or multiples of 1 (eg: 0.5,
            /// 0.25, 1.5, 2.0, etc).
            ///
            /// ## Rationale
            ///
            /// This allows clients to display (reasonably) accurate progress bars
            /// without having to regularly query the media player for the current
            /// position.
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`MinimumRate`]: Self::minimum_rate
            /// [`MaximumRate`]: Self::maximum_rate
            /// [`PlaybackStatus`]: Self::playback_status
            /// [`Pause`]: Self::pause
            #[doc(alias = "Rate")]
            async fn rate(&self) -> fdo::Result<PlaybackRate>;

            /// Sets the current playback rate.
            ///
            /// See [`Rate`] for more details.
            ///
            /// [`Rate`]: Self::rate
            #[doc(alias = "Rate")]
            async fn set_rate(&self, rate: PlaybackRate) -> Result<()>;

            /// Whether playback is shuffled.
            ///
            /// This property is *optional*. Clients should handle its absence
            /// gracefully.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// A value of **false** indicates that playback is progressing linearly
            /// through a playlist, while **true** means playback is progressing through
            /// a playlist in some other order.
            ///
            /// If [`CanControl`] is **false**, attempting to set this property should
            /// have no effect and raise an error.
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`CanControl`]: Self::can_control
            #[doc(alias = "Shuffle")]
            async fn shuffle(&self) -> fdo::Result<bool>;

            /// Sets whether playback is shuffled.
            ///
            /// See [`Shuffle`] for more details.
            ///
            /// [`Shuffle`]: Self::shuffle
            #[doc(alias = "Shuffle")]
            async fn set_shuffle(&self, shuffle: bool) -> Result<()>;

            /// The metadata of the current element.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// If there is a current track, this must have a [`mpris:trackid`] entry at
            /// the very least, which contains a D-Bus path that uniquely identifies
            /// this track.
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`mpris:trackid`]: Metadata::set_trackid
            #[doc(alias = "Metadata")]
            async fn metadata(&self) -> fdo::Result<Metadata>;

            /// The volume level.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// When setting, if a negative value is passed, the volume should be set to
            /// 0.0.
            ///
            /// If [`CanControl`] is **false**, attempting to set this property should
            /// have no effect and raise an error.
            ///
            /// [`properties_changed`]: Server::properties_changed
            /// [`CanControl`]: Self::can_control
            #[doc(alias = "Volume")]
            async fn volume(&self) -> fdo::Result<Volume>;

            /// Sets the volume level.
            ///
            /// See [`Volume`] for more details.
            ///
            /// [`Volume`]: Self::volume
            #[doc(alias = "Volume")]
            async fn set_volume(&self, volume: Volume) -> Result<()>;

            /// The current track position, between 0 and the [`mpris:length`]
            /// metadata entry.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must *not* be emitted.
            ///
            /// **Note:** If the media player allows it, the current playback position
            /// can be changed either the [`SetPosition`] method or the [`Seek`]
            /// method on this interface. If this is not the case, the [`CanSeek`]
            /// property is **false**, and setting this property has no effect and
            /// can raise an error.
            ///
            /// If the playback progresses in a way that is inconstistent with the
            /// [`Rate`] property, the [`Seeked`] signal is emitted.
            ///
            /// [`mpris:length`]: Metadata::set_length
            /// [`properties_changed`]: Server::properties_changed
            /// [`SetPosition`]: Self::set_position
            /// [`Seek`]: Self::seek
            /// [`CanSeek`]: Self::can_seek
            /// [`Rate`]: Self::rate
            /// [`Seeked`]: Server::seeked
            #[doc(alias = "Position")]
            async fn position(&self) -> fdo::Result<Time>;

            /// The minimum value which the [`Rate`] property can take. Clients should
            /// not attempt to set the [`Rate`] property below this value.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// Note that even if this value is 0.0 or negative, clients should not
            /// attempt to set the [`Rate`] property to 0.0.
            ///
            /// This value should always be 1.0 or less.
            ///
            /// [`Rate`]: Self::rate
            /// [`properties_changed`]: Server::properties_changed
            #[doc(alias = "MinimumRate")]
            async fn minimum_rate(&self) -> fdo::Result<PlaybackRate>;

            /// The maximum value which the [`Rate`] property can take. Clients should
            /// not attempt to set the [`Rate`] property above this value.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// This value should always be 1.0 or greater.
            ///
            /// [`Rate`]: Self::rate
            /// [`properties_changed`]: Server::properties_changed
            #[doc(alias = "MaximumRate")]
            async fn maximum_rate(&self) -> fdo::Result<PlaybackRate>;

            /// Whether the client can call the [`Next`] method on this interface and
            /// expect the current track to change.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// If it is unknown whether a call to [`Next`] will be successful (for
            /// example, when streaming tracks), this property should be set to
            /// **true**.
            ///
            /// If [`CanControl`] is **false**, this property should also be **false**.
            ///
            /// ## Rationale
            ///
            /// Even when playback can generally be controlled, there may not always be
            /// a next track to move to.
            ///
            /// [`Next`]: Self::next
            /// [`properties_changed`]: Server::properties_changed
            /// [`CanControl`]: Self::can_control
            #[doc(alias = "CanGoNext")]
            async fn can_go_next(&self) -> fdo::Result<bool>;

            /// Whether the client can call the [`Previous`] method on this interface
            /// and expect the current track to change.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// If it is unknown whether a call to [`Previous`] will be successful (for
            /// example, when streaming tracks), this property should be set to
            /// **true**.
            ///
            /// If [`CanControl`] is **false**, this property should also be **false**.
            ///
            /// ## Rationale
            ///
            /// Even when playback can generally be controlled, there may not always be
            /// a next previous to move to.
            ///
            /// [`Previous`]: Self::previous
            /// [`properties_changed`]: Server::properties_changed
            /// [`CanControl`]: Self::can_control
            #[doc(alias = "CanGoPrevious")]
            async fn can_go_previous(&self) -> fdo::Result<bool>;

            /// Whether playback can be started using [`Play`] or [`PlayPause`].
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// Note that this is related to whether there is a "current track": the
            /// value should not depend on whether the track is currently paused or
            /// playing. In fact, if a track is currently playing (and [`CanControl`] is
            /// **true**), this should be **true**.
            ///
            /// If [`CanControl`] is **false**, this property should also be **false**.
            ///
            /// ## Rationale
            ///
            /// Even when playback can generally be controlled, it may not be possible
            /// to enter a "playing" state, for example if there is no "current track".
            ///
            /// [`Play`]: Self::play
            /// [`PlayPause`]: Self::play_pause
            /// [`properties_changed`]: Server::properties_changed
            /// [`CanControl`]: Self::can_control
            #[doc(alias = "CanPlay")]
            async fn can_play(&self) -> fdo::Result<bool>;

            /// Whether playback can be paused using [`Pause`] or [`PlayPause`].
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// Note that this is an intrinsic property of the current track: its value
            /// should not depend on whether the track is currently paused or playing.
            /// In fact, if playback is currently paused (and [`CanControl`] is
            /// **true**), this should be **true**.
            ///
            /// If [`CanControl`] is **false**, this property should also be **false**.
            ///
            /// ## Rationale
            ///
            /// Not all media is pausable: it may not be possible to pause some streamed
            /// media, for example.
            ///
            /// [`Pause`]: Self::pause
            /// [`PlayPause`]: Self::play_pause
            /// [`properties_changed`]: Server::properties_changed
            /// [`CanControl`]: Self::can_control
            #[doc(alias = "CanPause")]
            async fn can_pause(&self) -> fdo::Result<bool>;

            /// Whether the client can control the playback position using [`Seek`] and
            /// [`SetPosition`]. This may be different for different tracks.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must be emitted with the new value.
            ///
            /// If [`CanControl`] is **false**, this property should also be **false**.
            ///
            /// ## Rationale
            ///
            /// Not all media is seekable: it may not be possible to seek when playing
            /// some streamed media, for example.
            ///
            /// [`Seek`]: Self::seek
            /// [`SetPosition`]: Self::set_position
            /// [`properties_changed`]: Server::properties_changed
            /// [`CanControl`]: Self::can_control
            #[doc(alias = "CanSeek")]
            async fn can_seek(&self) -> fdo::Result<bool>;

            /// Whether the media player may be controlled over this interface.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`properties_changed`] must *not* be emitted.
            ///
            /// This property is not expected to change, as it describes an intrinsic
            /// capability of the implementation.
            ///
            /// If this is **false**, clients should assume that all properties on this
            /// interface are read-only (and will raise errors if writing to them is
            /// attempted), no methods are implemented and all other properties starting
            /// with `Can` are also **false**.
            ///
            /// ## Rationale
            ///
            /// This allows clients to determine whether to present and enable controls
            /// to the user in advance of attempting to call methods and write to
            /// properties.
            ///
            /// [`properties_changed`]: Server::properties_changed
            #[doc(alias = "CanControl")]
            async fn can_control(&self) -> fdo::Result<bool>;
        }

        #[doc = $extra_track_list_docs]
        #[doc = ""]
        /// Used to implement [org.mpris.MediaPlayer2.TrackList] interface, which
        /// provides access to a short list of tracks which were recently played or will
        /// be played shortly. This is intended to provide context to the
        /// currently-playing track, rather than giving complete access to the media
        /// player's playlist.
        ///
        /// Example use cases are the list of tracks from the same album as the
        /// currently playing song or the [Rhythmbox] play queue.
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
        /// Note that the (memory and processing) burden of implementing this interface
        /// and maintaining unique track ids for the playlist can be mitigated by only
        /// exposing a subset of the playlist when it is very long (the 20 or so tracks
        /// around the currently playing track, for example). This is a recommended
        /// practice as the tracklist interface is not designed to enable browsing
        /// through a large list of tracks, but rather to provide clients with context
        /// about the currently playing track.
        ///
        /// [org.mpris.MediaPlayer2.TrackList]: https://specifications.freedesktop.org/mpris-spec/2.2/Track_List_Interface.html
        /// [Rhythmbox]: https://wiki.gnome.org/Apps/Rhythmbox
        /// [`TrackList interface`]: TrackListInterface
        #[$attr]
        #[doc(alias = "org.mpris.MediaPlayer2.TrackList")]
        pub trait $track_list_iface_ident: $player_iface_ident {
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
            async fn get_tracks_metadata(
                &self,
                track_ids: Vec<TrackId>,
            ) -> fdo::Result<Vec<Metadata>>;

            /// Adds a URI in the tracklist.
            ///
            /// ## Parameters
            ///
            /// * `uri` - The uri of the item to add. Its uri scheme should be an
            ///   element of the [`SupportedUriSchemes`] property and the mime-type
            ///   should match one of the elements of the [`SupportedMimeTypes`]
            ///   property.
            /// * `after_track` - The identifier of the track after which the new item
            ///   should be inserted. The path
            ///   [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] indicates that the track
            ///   should be inserted at the start of the track list.
            /// * `set_as_current` - Whether the newly inserted track should be
            ///   considered as the current track. Setting this to **true** has the same
            ///   effect as calling [`GoTo`] afterwards.
            ///
            /// If the [`CanEditTracks`] property is **false**, this has no effect.
            ///
            /// **Note:** Clients should not assume that the track has been added at the
            /// time when this method returns. They should wait for a [`TrackAdded`] (or
            /// [`TrackListReplaced`]) signal.
            ///
            /// [`SupportedUriSchemes`]: RootInterface::supported_uri_schemes
            /// [`SupportedMimeTypes`]: RootInterface::supported_mime_types
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

            /// Removes an item from the tracklist.
            ///
            /// ## Parameters
            ///
            /// * `track_id` - Identifier of the track to be removed.
            ///   [`/org/mpris/MediaPlayer2/TrackList/NoTrack`] is *not* a valid value
            ///   for this argument.
            ///
            /// If the track is not part of this tracklist, this has no effect.
            ///
            /// If the [`CanEditTracks`] property is **false**, this has no effect.
            ///
            /// **Note:** Clients should not assume that the track has been removed at
            /// the time when this method returns. They should wait for a
            /// [`TrackRemoved`] (or [`TrackListReplaced`]) signal.
            ///
            /// [`/org/mpris/MediaPlayer2/TrackList/NoTrack`]: TrackId::NO_TRACK
            /// [`CanEditTracks`]: Self::can_edit_tracks
            /// [`TrackRemoved`]: Server::track_removed
            /// [`TrackListReplaced`]: Server::track_list_replaced
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
            /// If this object is not `/org/mpris/MediaPlayer2`, the current tracklist's
            /// tracks should be replaced with the contents of this tracklist, and the
            /// [`TrackListReplaced`] signal should be fired from
            /// `/org/mpris/MediaPlayer2`.
            ///
            /// [`TrackListReplaced`]: Server::track_list_replaced
            #[doc(alias = "GoTo")]
            async fn go_to(&self, track_id: TrackId) -> fdo::Result<()>;

            /// An array which contains the identifier of each track in the tracklist,
            /// in order.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`track_list_properties_changed`] must be emitted *without* the new
            /// value.
            ///
            /// The `org.freedesktop.DBus.Properties.PropertiesChanged` signal is
            /// emitted every time this property changes, but the signal message
            /// does not contain the new value. Client implementations should rather
            /// rely on the [`TrackAdded`], [`TrackRemoved`] and
            /// [`TrackListReplaced`] signals to keep their representation of the
            /// tracklist up to date.
            ///
            /// [`track_list_properties_changed`]: Server::track_list_properties_changed
            /// [`TrackAdded`]: Server::track_added
            /// [`TrackRemoved`]: Server::track_removed
            /// [`TrackListReplaced`]: Server::track_list_replaced
            #[doc(alias = "Tracks")]
            async fn tracks(&self) -> fdo::Result<Vec<TrackId>>;

            /// Whether tracks can be added to and removed from the tracklist.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`track_list_properties_changed`] must be emitted with the new value.
            ///
            /// If **false**, calling [`AddTrack`] or [`RemoveTrack`] will have no
            /// effect, and may raise a `NotSupported` error.
            ///
            /// [`track_list_properties_changed`]: Server::track_list_properties_changed
            /// [`AddTrack`]: Self::add_track
            /// [`RemoveTrack`]: Self::remove_track
            #[doc(alias = "CanEditTracks")]
            async fn can_edit_tracks(&self) -> fdo::Result<bool>;
        }

        #[doc = $extra_playlists_docs]
        #[doc = ""]
        /// Used to implement [org.mpris.MediaPlayer2.Playlists] interface, which
        /// provides access to the media player's playlists.
        ///
        /// Since D-Bus does not provide an easy way to check for what interfaces are
        /// exported on an object, clients should attempt to get one of the properties
        /// on this interface to see if it is implemented.
        ///
        /// [org.mpris.MediaPlayer2.Playlists]: https://specifications.freedesktop.org/mpris-spec/2.2/Playlists_Interface.html
        #[$attr]
        #[doc(alias = "org.mpris.MediaPlayer2.Playlists")]
        pub trait $playlists_iface_ident: $player_iface_ident {
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
            #[doc(alias = "GetPlaylists")]
            async fn get_playlists(
                &self,
                index: u32,
                max_count: u32,
                order: PlaylistOrdering,
                reverse_order: bool,
            ) -> fdo::Result<Vec<Playlist>>;

            /// The number of playlists available.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`playlists_properties_changed`] must be emitted with the new value.
            ///
            /// [`playlists_properties_changed`]: Server::playlists_properties_changed
            #[doc(alias = "PlaylistCount")]
            async fn playlist_count(&self) -> fdo::Result<u32>;

            /// The available orderings. At least one must be offered.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`playlists_properties_changed`] must be emitted with the new value.
            ///
            /// ## Rationale
            ///
            /// Media players may not have access to all the data required for some
            /// orderings. For example, creation times are not available on UNIX
            /// filesystems (don't let the ctime fool you!). On the other hand, clients
            /// should have some way to get the "most recent" playlists.
            ///
            /// [`playlists_properties_changed`]: Server::playlists_properties_changed
            #[doc(alias = "Orderings")]
            async fn orderings(&self) -> fdo::Result<Vec<PlaylistOrdering>>;

            /// The currently-active playlist.
            ///
            /// When this property changes, the
            /// `org.freedesktop.DBus.Properties.PropertiesChanged` signal via
            /// [`playlists_properties_changed`] must be emitted with the new value.
            ///
            /// If there is no currently-active playlist, the structure's Valid field
            /// will be **false**, and the playlist details are undefined.
            ///
            /// Note that this may not have a value even after [`ActivatePlaylist`] is
            /// called with a valid playlist id as [`ActivatePlaylist`] implementations
            /// have the option of simply inserting the contents of the playlist
            /// into the current tracklist.
            ///
            /// [`playlists_properties_changed`]: Server::playlists_properties_changed
            /// [`ActivatePlaylist`]: Self::activate_playlist
            #[doc(alias = "ActivePlaylist")]
            async fn active_playlist(&self) -> fdo::Result<MaybePlaylist>;
        }
    };
}

define_iface!(
    #[async_trait],
    RootInterface: Send + Sync extra_docs "",
    PlayerInterface extra_docs "",
    TrackListInterface extra_docs "",
    PlaylistsInterface extra_docs ""
);

define_iface!(
    #[async_trait(?Send)],
    LocalRootInterface extra_docs "Local version of [`RootInterface`] to be used with [`LocalServer`].",
    LocalPlayerInterface extra_docs "Local version of [`PlayerInterface`] to be used with [`LocalServer`].",
    LocalTrackListInterface extra_docs "Local version of [`TrackListInterface`] to be used with [`LocalServer`].",
    LocalPlaylistsInterface  extra_docs "Local version of [`PlaylistsInterface`] to be used with [`LocalServer`]."
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
