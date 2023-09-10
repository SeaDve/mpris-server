use async_trait::async_trait;
use mpris_server::{
    LoopStatus, MaybePlaylist, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, Playlist,
    PlaylistId, PlaylistOrdering, PlaylistsInterface, RootInterface, Server, TimeInUs, TrackId,
    Volume,
};

pub struct Player;

#[async_trait(?Send)]
impl RootInterface for Player {
    async fn raise(&self) {
        println!("Raise");
    }

    async fn quit(&self) {
        println!("Quit");
    }

    async fn can_quit(&self) -> bool {
        println!("CanQuit");
        true
    }

    async fn fullscreen(&self) -> bool {
        println!("Fullscreen");
        true
    }

    async fn set_fullscreen(&self, fullscreen: bool) {
        println!("SetFullscreen({})", fullscreen);
    }

    async fn can_set_fullscreen(&self) -> bool {
        println!("CanSetFullscreen");
        true
    }

    async fn can_raise(&self) -> bool {
        println!("CanRaise");
        true
    }

    async fn has_track_list(&self) -> bool {
        println!("HasTrackList");
        true
    }

    async fn identity(&self) -> String {
        println!("Identity");
        "TestApplication".to_string()
    }

    async fn desktop_entry(&self) -> String {
        println!("DesktopEntry");
        "Test.Application".to_string()
    }

    async fn supported_uri_schemes(&self) -> Vec<String> {
        println!("SupportedUriSchemes");
        vec!["file".to_string()]
    }

    async fn supported_mime_types(&self) -> Vec<String> {
        println!("SupportedMimeTypes");
        vec!["audio/aac".to_string()]
    }
}

#[async_trait(?Send)]
impl PlayerInterface for Player {
    async fn next(&self) {
        println!("Next");
    }

    async fn previous(&self) {
        println!("Previous");
    }

    async fn pause(&self) {
        println!("Pause");
    }

    async fn play_pause(&self) {
        println!("PlayPause");
    }

    async fn stop(&self) {
        println!("Stop");
    }

    async fn play(&self) {
        println!("Play");
    }

    async fn seek(&self, offset: TimeInUs) {
        println!("Seek({})", offset);
    }

    async fn set_position(&self, track_id: TrackId, position: TimeInUs) {
        println!("SetPosition({}, {})", track_id, position);
    }

    async fn open_uri(&self, uri: String) {
        println!("OpenUri({})", uri);
    }

    async fn playback_status(&self) -> PlaybackStatus {
        println!("PlaybackStatus");
        PlaybackStatus::Playing
    }

    async fn loop_status(&self) -> LoopStatus {
        println!("LoopStatus");
        LoopStatus::None
    }

    async fn set_loop_status(&self, loop_status: LoopStatus) {
        println!("SetLoopStatus({})", loop_status);
    }

    async fn rate(&self) -> PlaybackRate {
        println!("Rate");
        1.0
    }

    async fn set_rate(&self, rate: PlaybackRate) {
        println!("SetRate({})", rate);
    }

    async fn shuffle(&self) -> bool {
        println!("Shuffle");
        false
    }

    async fn set_shuffle(&self, shuffle: bool) {
        println!("SetShuffle({})", shuffle);
    }

    async fn metadata(&self) -> Metadata {
        println!("Metadata");
        Metadata::new()
    }

    async fn volume(&self) -> Volume {
        println!("Volume");
        1.0
    }

    async fn set_volume(&self, volume: Volume) {
        println!("SetVolume({})", volume);
    }

    async fn position(&self) -> TimeInUs {
        println!("Position");
        0
    }

    async fn minimum_rate(&self) -> PlaybackRate {
        println!("MinimumRate");
        0.0
    }

    async fn maximum_rate(&self) -> PlaybackRate {
        println!("MaximumRate");
        1.0
    }

    async fn can_go_next(&self) -> bool {
        println!("CanGoNext");
        true
    }

    async fn can_go_previous(&self) -> bool {
        println!("CanGoPrevious");
        true
    }

    async fn can_play(&self) -> bool {
        println!("CanPlay");
        true
    }

    async fn can_pause(&self) -> bool {
        println!("CanPause");
        true
    }

    async fn can_seek(&self) -> bool {
        println!("CanSeek");
        true
    }

    async fn can_control(&self) -> bool {
        println!("CanControl");
        true
    }
}

#[async_trait(?Send)]
impl PlaylistsInterface for Player {
    async fn activate_playlist(&self, playlist_id: PlaylistId) {
        println!("ActivatePlaylist({})", playlist_id);
    }

    async fn get_playlists(
        &self,
        index: u32,
        max_count: u32,
        order: PlaylistOrdering,
        reverse_order: bool,
    ) -> Vec<Playlist> {
        println!(
            "GetPlaylists({}, {}, {}, {})",
            index, max_count, order, reverse_order
        );
        vec![]
    }

    async fn playlist_count(&self) -> u32 {
        println!("PlaylistCount");
        0
    }

    async fn orderings(&self) -> Vec<PlaylistOrdering> {
        println!("Orderings");
        vec![]
    }

    async fn active_playlist(&self) -> MaybePlaylist {
        println!("ActivePlaylist");
        MaybePlaylist {
            valid: false,
            playlist: Playlist {
                id: "/".try_into().unwrap(),
                name: "Test".into(),
                icon: "file:///home/test.png".into(),
            },
        }
    }
}

#[async_std::main]
async fn main() {
    let server = Server::new("Test.Application", Player).unwrap();
    server.run_with_playlists().await.unwrap();
}
