use async_trait::async_trait;
use mpris_server::{
    LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, RootInterface, Server,
    TimeInUs, TrackId, TrackListInterface, Uri, Volume,
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
impl TrackListInterface for Player {
    async fn get_tracks_metadata(&self, track_ids: Vec<TrackId>) -> Vec<Metadata> {
        println!("GetTracksMetadata({:?})", track_ids);
        vec![]
    }

    async fn add_track(&self, uri: Uri, after_track: TrackId, set_as_current: bool) {
        println!("AddTrack({}, {}, {})", uri, after_track, set_as_current);
    }

    async fn remove_track(&self, track_id: TrackId) {
        println!("RemoveTrack({})", track_id);
    }

    async fn go_to(&self, track_id: TrackId) {
        println!("GoTo({})", track_id);
    }

    async fn tracks(&self) -> Vec<TrackId> {
        println!("Tracks");
        vec![]
    }

    async fn can_edit_tracks(&self) -> bool {
        println!("CanEditTracks");
        true
    }
}

#[async_std::main]
async fn main() {
    let server = Server::new("Test.Application", Player).unwrap();
    server.run_with_track_list().await.unwrap();
}
