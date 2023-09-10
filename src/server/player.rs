use futures_channel::{mpsc, oneshot};
use futures_util::StreamExt;
use zbus::{dbus_interface, ConnectionBuilder, Result, SignalContext};

use super::{
    utils::{changed_delegate, signal_delegate},
    Action, Server, OBJECT_PATH,
};
use crate::{
    LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, TimeInUs, TrackId, Volume,
};

/// `org.mpris.MediaPlayer2` Actions
pub(super) enum RootAction {
    // Methods
    Raise,
    Quit,

    // Properties
    CanQuit(oneshot::Sender<bool>),
    Fullscreen(oneshot::Sender<bool>),
    SetFullscreen(bool),
    CanSetFullScreen(oneshot::Sender<bool>),
    CanRaise(oneshot::Sender<bool>),
    HasTrackList(oneshot::Sender<bool>),
    Identity(oneshot::Sender<String>),
    DesktopEntry(oneshot::Sender<String>),
    SupportedUriSchemes(oneshot::Sender<Vec<String>>),
    SupportedMimeTypes(oneshot::Sender<Vec<String>>),
}

pub(super) struct RawRootInterface {
    pub(super) tx: mpsc::UnboundedSender<Action>,
}

impl RawRootInterface {
    fn send(&self, action: RootAction) {
        self.tx.unbounded_send(Action::Root(action)).unwrap();
    }
}

#[dbus_interface(name = "org.mpris.MediaPlayer2")]
impl RawRootInterface {
    fn raise(&self) {
        self.send(RootAction::Raise);
    }

    fn quit(&self) {
        self.send(RootAction::Quit);
    }

    #[dbus_interface(property)]
    async fn can_quit(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(RootAction::CanQuit(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn fullscreen(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(RootAction::Fullscreen(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    fn set_fullscreen(&self, fullscreen: bool) {
        self.send(RootAction::SetFullscreen(fullscreen));
    }

    #[dbus_interface(property)]
    async fn can_set_fullscreen(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(RootAction::CanSetFullScreen(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_raise(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(RootAction::CanRaise(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn has_track_list(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(RootAction::HasTrackList(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn identity(&self) -> String {
        let (tx, rx) = oneshot::channel();
        self.send(RootAction::Identity(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn desktop_entry(&self) -> String {
        let (tx, rx) = oneshot::channel();
        self.send(RootAction::DesktopEntry(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn supported_uri_schemes(&self) -> Vec<String> {
        let (tx, rx) = oneshot::channel();
        self.send(RootAction::SupportedUriSchemes(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn supported_mime_types(&self) -> Vec<String> {
        let (tx, rx) = oneshot::channel();
        self.send(RootAction::SupportedMimeTypes(tx));
        rx.await.unwrap()
    }
}

/// `org.mpris.MediaPlayer2.Player` Actions
pub(super) enum PlayerAction {
    // Methods
    Next,
    Previous,
    Pause,
    PlayPause,
    Stop,
    Play,
    Seek(TimeInUs),
    SetPosition(TrackId, TimeInUs),
    OpenUri(String),

    // Properties
    PlaybackStatus(oneshot::Sender<PlaybackStatus>),
    LoopStatus(oneshot::Sender<LoopStatus>),
    SetLoopStatus(LoopStatus),
    Rate(oneshot::Sender<PlaybackRate>),
    SetRate(PlaybackRate),
    Shuffle(oneshot::Sender<bool>),
    SetShuffle(bool),
    Metadata(oneshot::Sender<Metadata>),
    Volume(oneshot::Sender<Volume>),
    SetVolume(Volume),
    Position(oneshot::Sender<TimeInUs>),
    MinimumRate(oneshot::Sender<PlaybackRate>),
    MaximumRate(oneshot::Sender<PlaybackRate>),
    CanGoNext(oneshot::Sender<bool>),
    CanGoPrevious(oneshot::Sender<bool>),
    CanPlay(oneshot::Sender<bool>),
    CanPause(oneshot::Sender<bool>),
    CanSeek(oneshot::Sender<bool>),
    CanControl(oneshot::Sender<bool>),
}

pub(super) struct RawPlayerInterface {
    pub(super) tx: mpsc::UnboundedSender<Action>,
}

impl RawPlayerInterface {
    fn send(&self, action: PlayerAction) {
        self.tx.unbounded_send(Action::Player(action)).unwrap();
    }
}

#[dbus_interface(name = "org.mpris.MediaPlayer2.Player")]
impl RawPlayerInterface {
    fn next(&self) {
        self.send(PlayerAction::Next);
    }

    fn previous(&self) {
        self.send(PlayerAction::Previous);
    }

    fn pause(&self) {
        self.send(PlayerAction::Pause);
    }

    fn play_pause(&self) {
        self.send(PlayerAction::PlayPause);
    }

    fn stop(&self) {
        self.send(PlayerAction::Stop);
    }

    fn play(&self) {
        self.send(PlayerAction::Play);
    }

    fn seek(&self, offset: TimeInUs) {
        self.send(PlayerAction::Seek(offset));
    }

    fn set_position(&self, track_id: TrackId, position: TimeInUs) {
        self.send(PlayerAction::SetPosition(track_id, position));
    }

    fn open_uri(&self, uri: String) {
        self.send(PlayerAction::OpenUri(uri));
    }

    #[dbus_interface(signal)]
    async fn seeked(ctxt: &SignalContext<'_>, position: TimeInUs) -> Result<()>;

    #[dbus_interface(property)]
    async fn playback_status(&self) -> PlaybackStatus {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::PlaybackStatus(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn loop_status(&self) -> LoopStatus {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::LoopStatus(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()> {
        self.send(PlayerAction::SetLoopStatus(loop_status));
        Ok(())
    }

    #[dbus_interface(property)]
    async fn rate(&self) -> PlaybackRate {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::Rate(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    fn set_rate(&self, rate: PlaybackRate) {
        self.send(PlayerAction::SetRate(rate));
    }

    #[dbus_interface(property)]
    async fn shuffle(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::Shuffle(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    fn set_shuffle(&self, shuffle: bool) {
        self.send(PlayerAction::SetShuffle(shuffle));
    }

    #[dbus_interface(property)]
    async fn metadata(&self) -> Metadata {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::Metadata(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn volume(&self) -> Volume {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::Volume(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    fn set_volume(&self, volume: Volume) {
        self.send(PlayerAction::SetVolume(volume));
    }

    #[dbus_interface(property)]
    async fn position(&self) -> TimeInUs {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::Position(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn minimum_rate(&self) -> PlaybackRate {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::MinimumRate(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn maximum_rate(&self) -> PlaybackRate {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::MaximumRate(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_go_next(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::CanGoNext(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_go_previous(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::CanGoPrevious(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_play(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::CanPlay(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_pause(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::CanPause(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_seek(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::CanSeek(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_control(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerAction::CanControl(tx));
        rx.await.unwrap()
    }
}

impl<T> Server<T>
where
    T: PlayerInterface + 'static,
{
    pub async fn run(&self) -> Result<()> {
        let (tx, mut rx) = mpsc::unbounded::<Action>();

        let connection = ConnectionBuilder::session()?
            .name(&self.bus_name)?
            .serve_at(OBJECT_PATH, RawRootInterface { tx: tx.clone() })?
            .serve_at(OBJECT_PATH, RawPlayerInterface { tx })?
            .build()
            .await?;

        self.connection
            .set(connection)
            .expect("server must only be ran once");

        // FIXME Spawn tasks so we can handle calls concurrently
        while let Some(action) = rx.next().await {
            match action {
                Action::Root(action) => self.handle_interface_action(action).await,
                Action::Player(action) => self.handle_player_interface_action(action).await,
                Action::TrackList(_) | Action::Playlists(_) => unreachable!(),
            }
        }

        Ok(())
    }

    pub(super) async fn handle_interface_action(&self, action: RootAction) {
        match action {
            // Methods
            RootAction::Raise => self.imp.raise().await,
            RootAction::Quit => self.imp.quit().await,
            // Properties
            RootAction::CanQuit(sender) => sender.send(self.imp.can_quit().await).unwrap(),
            RootAction::Fullscreen(sender) => sender.send(self.imp.fullscreen().await).unwrap(),
            RootAction::SetFullscreen(fullscreen) => self.imp.set_fullscreen(fullscreen).await,
            RootAction::CanSetFullScreen(sender) => {
                sender.send(self.imp.can_set_fullscreen().await).unwrap()
            }
            RootAction::CanRaise(sender) => sender.send(self.imp.can_raise().await).unwrap(),
            RootAction::HasTrackList(sender) => {
                sender.send(self.imp.has_track_list().await).unwrap()
            }
            RootAction::Identity(sender) => sender.send(self.imp.identity().await).unwrap(),
            RootAction::DesktopEntry(sender) => {
                sender.send(self.imp.desktop_entry().await).unwrap()
            }
            RootAction::SupportedUriSchemes(sender) => {
                sender.send(self.imp.supported_uri_schemes().await).unwrap()
            }
            RootAction::SupportedMimeTypes(sender) => {
                sender.send(self.imp.supported_mime_types().await).unwrap()
            }
        }
    }

    pub(super) async fn handle_player_interface_action(&self, action: PlayerAction) {
        match action {
            // Methods
            PlayerAction::Next => self.imp.next().await,
            PlayerAction::Previous => self.imp.previous().await,
            PlayerAction::Pause => self.imp.pause().await,
            PlayerAction::PlayPause => self.imp.play_pause().await,
            PlayerAction::Stop => self.imp.stop().await,
            PlayerAction::Play => self.imp.play().await,
            PlayerAction::Seek(offset) => self.imp.seek(offset).await,
            PlayerAction::SetPosition(track_id, position) => {
                self.imp.set_position(track_id, position).await
            }
            PlayerAction::OpenUri(uri) => self.imp.open_uri(uri).await,
            // Properties
            PlayerAction::PlaybackStatus(sender) => {
                sender.send(self.imp.playback_status().await).unwrap()
            }
            PlayerAction::LoopStatus(sender) => sender.send(self.imp.loop_status().await).unwrap(),
            PlayerAction::SetLoopStatus(loop_status) => self.imp.set_loop_status(loop_status).await,
            PlayerAction::Rate(sender) => sender.send(self.imp.rate().await).unwrap(),
            PlayerAction::SetRate(rate) => self.imp.set_rate(rate).await,
            PlayerAction::Shuffle(sender) => sender.send(self.imp.shuffle().await).unwrap(),
            PlayerAction::SetShuffle(shuffle) => self.imp.set_shuffle(shuffle).await,
            PlayerAction::Metadata(sender) => sender.send(self.imp.metadata().await).unwrap(),
            PlayerAction::Volume(sender) => sender.send(self.imp.volume().await).unwrap(),
            PlayerAction::SetVolume(volume) => self.imp.set_volume(volume).await,
            PlayerAction::Position(sender) => sender.send(self.imp.position().await).unwrap(),
            PlayerAction::MinimumRate(sender) => {
                sender.send(self.imp.minimum_rate().await).unwrap()
            }
            PlayerAction::MaximumRate(sender) => {
                sender.send(self.imp.maximum_rate().await).unwrap()
            }
            PlayerAction::CanGoNext(sender) => sender.send(self.imp.can_go_next().await).unwrap(),
            PlayerAction::CanGoPrevious(sender) => {
                sender.send(self.imp.can_go_previous().await).unwrap()
            }
            PlayerAction::CanPlay(sender) => sender.send(self.imp.can_play().await).unwrap(),
            PlayerAction::CanPause(sender) => sender.send(self.imp.can_pause().await).unwrap(),
            PlayerAction::CanSeek(sender) => sender.send(self.imp.can_seek().await).unwrap(),
            PlayerAction::CanControl(sender) => sender.send(self.imp.can_control().await).unwrap(),
        }
    }

    // org.mpris.MediaPlayer2
    changed_delegate!(RawRootInterface, can_quit_changed);
    changed_delegate!(RawRootInterface, fullscreen_changed);
    changed_delegate!(RawRootInterface, can_set_fullscreen_changed);
    changed_delegate!(RawRootInterface, can_raise_changed);
    changed_delegate!(RawRootInterface, has_track_list_changed);
    changed_delegate!(RawRootInterface, identity_changed);
    changed_delegate!(RawRootInterface, desktop_entry_changed);
    changed_delegate!(RawRootInterface, supported_uri_schemes_changed);
    changed_delegate!(RawRootInterface, supported_mime_types_changed);

    // org.mpris.MediaPlayer2.Player
    signal_delegate!(RawPlayerInterface, seeked(position: TimeInUs));
    changed_delegate!(RawPlayerInterface, playback_status_changed);
    changed_delegate!(RawPlayerInterface, loop_status_changed);
    changed_delegate!(RawPlayerInterface, rate_changed);
    changed_delegate!(RawPlayerInterface, shuffle_changed);
    changed_delegate!(RawPlayerInterface, metadata_changed);
    changed_delegate!(RawPlayerInterface, volume_changed);
    changed_delegate!(RawPlayerInterface, position_changed);
    changed_delegate!(RawPlayerInterface, minimum_rate_changed);
    changed_delegate!(RawPlayerInterface, maximum_rate_changed);
    changed_delegate!(RawPlayerInterface, can_go_next_changed);
    changed_delegate!(RawPlayerInterface, can_go_previous_changed);
    changed_delegate!(RawPlayerInterface, can_play_changed);
    changed_delegate!(RawPlayerInterface, can_pause_changed);
    changed_delegate!(RawPlayerInterface, can_seek_changed);
    changed_delegate!(RawPlayerInterface, can_control_changed);
}
