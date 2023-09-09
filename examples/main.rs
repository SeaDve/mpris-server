use mpris_server::{
    LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, RootInterface, Server,
    TimeInUs, TrackId, Volume,
};

pub struct Player;

impl RootInterface for Player {
    fn raise(&self) {
        println!("Raise");
    }

    fn quit(&self) {
        println!("Quit");
    }

    fn can_quit(&self) -> bool {
        println!("CanQuit");
        true
    }

    fn fullscreen(&self) -> bool {
        println!("Fullscreen");
        true
    }

    fn set_fullscreen(&self, fullscreen: bool) {
        println!("SetFullscreen({})", fullscreen);
    }

    fn can_set_fullscreen(&self) -> bool {
        println!("CanSetFullscreen");
        true
    }

    fn can_raise(&self) -> bool {
        println!("CanRaise");
        true
    }

    fn has_track_list(&self) -> bool {
        println!("HasTrackList");
        true
    }

    fn identity(&self) -> String {
        println!("Identity");
        "TestApplication".to_string()
    }

    fn desktop_entry(&self) -> String {
        println!("DesktopEntry");
        "Test.Application".to_string()
    }

    fn supported_uri_schemes(&self) -> Vec<String> {
        println!("SupportedUriSchemes");
        vec!["file".to_string()]
    }

    fn supported_mime_types(&self) -> Vec<String> {
        println!("SupportedMimeTypes");
        vec!["audio/aac".to_string()]
    }
}

impl PlayerInterface for Player {
    fn next(&self) {
        println!("Next");
    }

    fn previous(&self) {
        println!("Previous");
    }

    fn pause(&self) {
        println!("Pause");
    }

    fn play_pause(&self) {
        println!("PlayPause");
    }

    fn stop(&self) {
        println!("Stop");
    }

    fn play(&self) {
        println!("Play");
    }

    fn seek(&self, offset: TimeInUs) {
        println!("Seek({})", offset);
    }

    fn set_position(&self, track_id: TrackId, position: TimeInUs) {
        println!("SetPosition({}, {})", track_id, position);
    }

    fn open_uri(&self, uri: String) {
        println!("OpenUri({})", uri);
    }

    fn playback_status(&self) -> PlaybackStatus {
        println!("PlaybackStatus");
        PlaybackStatus::Playing
    }

    fn loop_status(&self) -> LoopStatus {
        println!("LoopStatus");
        LoopStatus::None
    }

    fn set_loop_status(&self, loop_status: LoopStatus) {
        println!("SetLoopStatus({})", loop_status);
    }

    fn rate(&self) -> PlaybackRate {
        println!("Rate");
        1.0
    }

    fn set_rate(&self, rate: PlaybackRate) {
        println!("SetRate({})", rate);
    }

    fn shuffle(&self) -> bool {
        println!("Shuffle");
        false
    }

    fn set_shuffle(&self, shuffle: bool) {
        println!("SetShuffle({})", shuffle);
    }

    fn metadata(&self) -> Metadata {
        println!("Metadata");
        Metadata::new()
    }

    fn volume(&self) -> Volume {
        println!("Volume");
        1.0
    }

    fn set_volume(&self, volume: Volume) {
        println!("SetVolume({})", volume);
    }

    fn position(&self) -> TimeInUs {
        println!("Position");
        0
    }

    fn minimum_rate(&self) -> PlaybackRate {
        println!("MinimumRate");
        0.0
    }

    fn maximum_rate(&self) -> PlaybackRate {
        println!("MaximumRate");
        1.0
    }

    fn can_go_next(&self) -> bool {
        println!("CanGoNext");
        true
    }

    fn can_go_previous(&self) -> bool {
        println!("CanGoPrevious");
        true
    }

    fn can_play(&self) -> bool {
        println!("CanPlay");
        true
    }

    fn can_pause(&self) -> bool {
        println!("CanPause");
        true
    }

    fn can_seek(&self) -> bool {
        println!("CanSeek");
        true
    }

    fn can_control(&self) -> bool {
        println!("CanControl");
        true
    }
}

#[async_std::main]
async fn main() {
    let server = Server::new("Test.Application", Player).unwrap();
    server.run().await.unwrap();
}
