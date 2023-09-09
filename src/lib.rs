#![warn(rust_2018_idioms)]

mod loop_status;
mod metadata;
mod playback_status;
mod player;
mod server;

use zbus::zvariant::OwnedObjectPath;

pub use crate::{
    loop_status::{LoopStatus, ParseLoopStatusError},
    metadata::{DateTime, Metadata, MetadataBuilder, Uri},
    playback_status::{ParsePlaybackStatusError, PlaybackStatus},
    player::Player,
    server::Server,
};

pub trait RootInterface {
    /// Brings the media player's user interface to the front using
    /// any appropriate mechanism available.
    ///
    /// The media player may be unable to control how its user
    /// interface is displayed, or it may not have a graphical user
    /// interface at all. In this case, the `CanRaise` property is
    /// `false` and this method does nothing.
    fn raise(&self);

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
    fn quit(&self);

    /// If `false`, calling `Quit` will have no effect, and may raise a
    /// NotSupported error. If `true`, calling `Quit` will cause the media
    /// application to attempt to quit (although it may still be
    /// prevented from quitting by the user, for example).
    fn can_quit(&self) -> bool;

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
    fn fullscreen(&self) -> bool {
        false
    }

    fn set_fullscreen(&self, _fullscreen: bool) {}

    fn can_set_fullscreen(&self) -> bool {
        false
    }

    fn can_raise(&self) -> bool;

    fn has_track_list(&self) -> bool;

    fn identity(&self) -> String;

    fn desktop_entry(&self) -> String {
        String::new()
    }

    fn supported_uri_schemes(&self) -> Vec<String>;

    fn supported_mime_types(&self) -> Vec<String>;
}

pub trait PlayerInterface: RootInterface {
    fn next(&self);

    fn previous(&self);

    fn pause(&self);

    fn play_pause(&self);

    fn stop(&self);

    fn play(&self);

    fn seek(&self, offset: TimeInUs);

    fn set_position(&self, track_id: TrackId, position: TimeInUs);

    fn open_uri(&self, uri: String);

    fn playback_status(&self) -> PlaybackStatus;

    fn loop_status(&self) -> LoopStatus {
        LoopStatus::None
    }

    fn set_loop_status(&self, _loop_status: LoopStatus) {}

    fn rate(&self) -> PlaybackRate;

    fn set_rate(&self, rate: PlaybackRate);

    fn shuffle(&self) -> bool {
        false
    }

    fn set_shuffle(&self, _shuffle: bool) {}

    fn metadata(&self) -> Metadata;

    fn volume(&self) -> Volume;

    fn set_volume(&self, volume: Volume);

    fn position(&self) -> TimeInUs;

    fn minimum_rate(&self) -> PlaybackRate;

    fn maximum_rate(&self) -> PlaybackRate;

    fn can_go_next(&self) -> bool;

    fn can_go_previous(&self) -> bool;

    fn can_play(&self) -> bool;

    fn can_pause(&self) -> bool;

    fn can_seek(&self) -> bool;

    fn can_control(&self) -> bool;
}

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

/// A playback rate
///
/// This is a multiplier, so a value of 0.5 indicates that playback
/// is happening at half speed, while 1.5 means that 1.5 seconds of
/// "track time" is consumed every second.
pub type PlaybackRate = f64;

/// Audio volume level
///
/// * 0.0 means mute.
/// * 1.0 is a sensible maximum volume level (ex: 0dB).
///
/// Note that the volume may be higher than 1.0, although generally
/// clients should not attempt to set it above 1.0.
pub type Volume = f64;

/// Time in microseconds.
pub type TimeInUs = i64;
