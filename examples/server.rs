use std::future;

use mpris_server::{
    async_trait,
    zbus::{fdo, Result},
    LoopStatus, MaybePlaylist, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, Playlist,
    PlaylistId, PlaylistOrdering, PlaylistsInterface, Property, RootInterface, Server, Signal,
    Time, TrackId, TrackListInterface, Uri, Volume,
};

pub struct Player;

#[async_trait]
impl RootInterface for Player {
    async fn raise(&self) -> fdo::Result<()> {
        println!("Raise");
        Ok(())
    }

    async fn quit(&self) -> fdo::Result<()> {
        println!("Quit");
        Ok(())
    }

    async fn can_quit(&self) -> fdo::Result<bool> {
        println!("CanQuit");
        Ok(true)
    }

    async fn fullscreen(&self) -> fdo::Result<bool> {
        println!("Fullscreen");
        Ok(false)
    }

    async fn set_fullscreen(&self, fullscreen: bool) -> Result<()> {
        println!("SetFullscreen({})", fullscreen);
        Ok(())
    }

    async fn can_set_fullscreen(&self) -> fdo::Result<bool> {
        println!("CanSetFullscreen");
        Ok(false)
    }

    async fn can_raise(&self) -> fdo::Result<bool> {
        println!("CanRaise");
        Ok(true)
    }

    async fn has_track_list(&self) -> fdo::Result<bool> {
        println!("HasTrackList");
        Ok(false)
    }

    async fn identity(&self) -> fdo::Result<String> {
        println!("Identity");
        Ok("TestApplication".to_string())
    }

    async fn desktop_entry(&self) -> fdo::Result<String> {
        println!("DesktopEntry");
        Ok("Test.Application".to_string())
    }

    async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>> {
        println!("SupportedUriSchemes");
        Ok(vec![])
    }

    async fn supported_mime_types(&self) -> fdo::Result<Vec<String>> {
        println!("SupportedMimeTypes");
        Ok(vec![])
    }
}

#[async_trait]
impl PlayerInterface for Player {
    async fn next(&self) -> fdo::Result<()> {
        println!("Next");
        Ok(())
    }

    async fn previous(&self) -> fdo::Result<()> {
        println!("Previous");
        Ok(())
    }

    async fn pause(&self) -> fdo::Result<()> {
        println!("Pause");
        Ok(())
    }

    async fn play_pause(&self) -> fdo::Result<()> {
        println!("PlayPause");
        Ok(())
    }

    async fn stop(&self) -> fdo::Result<()> {
        println!("Stop");
        Ok(())
    }

    async fn play(&self) -> fdo::Result<()> {
        println!("Play");
        Ok(())
    }

    async fn seek(&self, offset: Time) -> fdo::Result<()> {
        println!("Seek({:?})", offset);
        Ok(())
    }

    async fn set_position(&self, track_id: TrackId, position: Time) -> fdo::Result<()> {
        println!("SetPosition({}, {:?})", track_id, position);
        Ok(())
    }

    async fn open_uri(&self, uri: String) -> fdo::Result<()> {
        println!("OpenUri({})", uri);
        Ok(())
    }

    async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
        println!("PlaybackStatus");
        Ok(PlaybackStatus::Playing)
    }

    async fn loop_status(&self) -> fdo::Result<LoopStatus> {
        println!("LoopStatus");
        Ok(LoopStatus::None)
    }

    async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()> {
        println!("SetLoopStatus({})", loop_status);
        Ok(())
    }

    async fn rate(&self) -> fdo::Result<PlaybackRate> {
        println!("Rate");
        Ok(PlaybackRate::default())
    }

    async fn set_rate(&self, rate: PlaybackRate) -> Result<()> {
        println!("SetRate({})", rate);
        Ok(())
    }

    async fn shuffle(&self) -> fdo::Result<bool> {
        println!("Shuffle");
        Ok(false)
    }

    async fn set_shuffle(&self, shuffle: bool) -> Result<()> {
        println!("SetShuffle({})", shuffle);
        Ok(())
    }

    async fn metadata(&self) -> fdo::Result<Metadata> {
        println!("Metadata");
        Ok(Metadata::default())
    }

    async fn volume(&self) -> fdo::Result<Volume> {
        println!("Volume");
        Ok(Volume::default())
    }

    async fn set_volume(&self, volume: Volume) -> Result<()> {
        println!("SetVolume({})", volume);
        Ok(())
    }

    async fn position(&self) -> fdo::Result<Time> {
        println!("Position");
        Ok(Time::ZERO)
    }

    async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
        println!("MinimumRate");
        Ok(PlaybackRate::default())
    }

    async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
        println!("MaximumRate");
        Ok(PlaybackRate::default())
    }

    async fn can_go_next(&self) -> fdo::Result<bool> {
        println!("CanGoNext");
        Ok(false)
    }

    async fn can_go_previous(&self) -> fdo::Result<bool> {
        println!("CanGoPrevious");
        Ok(false)
    }

    async fn can_play(&self) -> fdo::Result<bool> {
        println!("CanPlay");
        Ok(true)
    }

    async fn can_pause(&self) -> fdo::Result<bool> {
        println!("CanPause");
        Ok(true)
    }

    async fn can_seek(&self) -> fdo::Result<bool> {
        println!("CanSeek");
        Ok(false)
    }

    async fn can_control(&self) -> fdo::Result<bool> {
        println!("CanControl");
        Ok(true)
    }
}

#[async_trait]
impl TrackListInterface for Player {
    async fn get_tracks_metadata(&self, track_ids: Vec<TrackId>) -> fdo::Result<Vec<Metadata>> {
        println!("GetTracksMetadata({:?})", track_ids);
        Ok(vec![])
    }

    async fn add_track(
        &self,
        uri: Uri,
        after_track: TrackId,
        set_as_current: bool,
    ) -> fdo::Result<()> {
        println!("AddTrack({}, {}, {})", uri, after_track, set_as_current);
        Ok(())
    }

    async fn remove_track(&self, track_id: TrackId) -> fdo::Result<()> {
        println!("RemoveTrack({})", track_id);
        Ok(())
    }

    async fn go_to(&self, track_id: TrackId) -> fdo::Result<()> {
        println!("GoTo({})", track_id);
        Ok(())
    }

    async fn tracks(&self) -> fdo::Result<Vec<TrackId>> {
        println!("Tracks");
        Ok(vec![])
    }

    async fn can_edit_tracks(&self) -> fdo::Result<bool> {
        println!("CanEditTracks");
        Ok(false)
    }
}

#[async_trait]
impl PlaylistsInterface for Player {
    async fn activate_playlist(&self, playlist_id: PlaylistId) -> fdo::Result<()> {
        println!("ActivatePlaylist({})", playlist_id);
        Ok(())
    }

    async fn get_playlists(
        &self,
        index: u32,
        max_count: u32,
        order: PlaylistOrdering,
        reverse_order: bool,
    ) -> fdo::Result<Vec<Playlist>> {
        println!(
            "GetPlaylists({}, {}, {}, {})",
            index, max_count, order, reverse_order
        );
        Ok(vec![])
    }

    async fn playlist_count(&self) -> fdo::Result<u32> {
        println!("PlaylistCount");
        Ok(0)
    }

    async fn orderings(&self) -> fdo::Result<Vec<PlaylistOrdering>> {
        println!("Orderings");
        Ok(vec![])
    }

    async fn active_playlist(&self) -> fdo::Result<MaybePlaylist> {
        println!("ActivePlaylist");
        Ok(None.into())
    }
}

#[async_std::main]
async fn main() {
    // Create a server that exports `org.mpris.MediaPlayer2` and
    // `org.mpris.MediaPlayer2.Player` interfaces.
    let server = Server::new("Test.Application", Player).unwrap();

    // We only just need to call `init` here as the server is ran in the background,
    // unlike in `LocalServer`.
    server.init().await.unwrap();

    // Create a server that exports `org.mpris.MediaPlayer2.TrackList`
    // interface, in addition to the previous interfaces.
    let server = Server::new_with_track_list("Test.ApplicationWithTrackList", Player).unwrap();
    server.init().await.unwrap();

    // Create a server that exports `org.mpris.MediaPlayer2.Playlists`
    // interface, in addition to the previous interfaces.
    let server = Server::new_with_playlists("Test.ApplicationWithPlaylists", Player).unwrap();
    server.init().await.unwrap();

    // Create a server that exports all interfaces.
    let server = Server::new_with_all("Test.ApplicationWithTrackListAndPlaylists", Player).unwrap();
    server.init().await.unwrap();

    // Emit `PropertiesChanged` signal for `CanSeek` and `Metadata` properties
    server
        .properties_changed(Property::CanSeek | Property::Metadata)
        .await
        .unwrap();

    // Emit `Seeked` signal
    server
        .emit(Signal::Seeked {
            position: Time::from_micros(124),
        })
        .await
        .unwrap();

    // Prevent the program from exiting.
    future::pending::<()>().await;
}
