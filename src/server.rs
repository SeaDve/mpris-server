use std::{fmt, sync::OnceLock};

use futures_channel::{mpsc, oneshot};
use zbus::{
    dbus_interface, export::futures_util::StreamExt, names::WellKnownName, zvariant::ObjectPath,
    Connection, ConnectionBuilder, InterfaceRef, Result, SignalContext,
};

use crate::{
    LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, TimeInUs, TrackId, Volume,
};

const OBJECT_PATH: ObjectPath<'static> =
    ObjectPath::from_static_str_unchecked("/org/mpris/MediaPlayer2");

enum Action {
    RootInterface(RootInterfaceAction),
    PlayerInterface(PlayerInterfaceAction),
}

/// `org.mpris.MediaPlayer2` Actions
enum RootInterfaceAction {
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

/// `org.mpris.MediaPlayer2.Player` Actions
enum PlayerInterfaceAction {
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

struct RawRootInterface {
    tx: mpsc::UnboundedSender<Action>,
}

impl RawRootInterface {
    fn send(&self, action: RootInterfaceAction) {
        self.tx
            .unbounded_send(Action::RootInterface(action))
            .unwrap();
    }
}

#[dbus_interface(name = "org.mpris.MediaPlayer2")]
impl RawRootInterface {
    fn raise(&self) {
        self.send(RootInterfaceAction::Raise);
    }

    fn quit(&self) {
        self.send(RootInterfaceAction::Quit);
    }

    #[dbus_interface(property)]
    async fn can_quit(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(RootInterfaceAction::CanQuit(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn fullscreen(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(RootInterfaceAction::Fullscreen(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    fn set_fullscreen(&self, fullscreen: bool) {
        self.send(RootInterfaceAction::SetFullscreen(fullscreen));
    }

    #[dbus_interface(property)]
    async fn can_set_fullscreen(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(RootInterfaceAction::CanSetFullScreen(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_raise(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(RootInterfaceAction::CanRaise(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn has_track_list(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(RootInterfaceAction::HasTrackList(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn identity(&self) -> String {
        let (tx, rx) = oneshot::channel();
        self.send(RootInterfaceAction::Identity(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn desktop_entry(&self) -> String {
        let (tx, rx) = oneshot::channel();
        self.send(RootInterfaceAction::DesktopEntry(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn supported_uri_schemes(&self) -> Vec<String> {
        let (tx, rx) = oneshot::channel();
        self.send(RootInterfaceAction::SupportedUriSchemes(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn supported_mime_types(&self) -> Vec<String> {
        let (tx, rx) = oneshot::channel();
        self.send(RootInterfaceAction::SupportedMimeTypes(tx));
        rx.await.unwrap()
    }
}

struct RawPlayerInterface {
    tx: mpsc::UnboundedSender<Action>,
}

impl RawPlayerInterface {
    fn send(&self, action: PlayerInterfaceAction) {
        self.tx
            .unbounded_send(Action::PlayerInterface(action))
            .unwrap();
    }
}

#[dbus_interface(name = "org.mpris.MediaPlayer2.Player")]
impl RawPlayerInterface {
    fn next(&self) {
        self.send(PlayerInterfaceAction::Next);
    }

    fn previous(&self) {
        self.send(PlayerInterfaceAction::Previous);
    }

    fn pause(&self) {
        self.send(PlayerInterfaceAction::Pause);
    }

    fn play_pause(&self) {
        self.send(PlayerInterfaceAction::PlayPause);
    }

    fn stop(&self) {
        self.send(PlayerInterfaceAction::Stop);
    }

    fn play(&self) {
        self.send(PlayerInterfaceAction::Play);
    }

    fn seek(&self, offset: TimeInUs) {
        self.send(PlayerInterfaceAction::Seek(offset));
    }

    fn set_position(&self, track_id: TrackId, position: TimeInUs) {
        self.send(PlayerInterfaceAction::SetPosition(track_id, position));
    }

    fn open_uri(&self, uri: String) {
        self.send(PlayerInterfaceAction::OpenUri(uri));
    }

    #[dbus_interface(signal)]
    async fn seeked(ctxt: &SignalContext<'_>, position: TimeInUs) -> Result<()>;

    #[dbus_interface(property)]
    async fn playback_status(&self) -> PlaybackStatus {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::PlaybackStatus(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn loop_status(&self) -> LoopStatus {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::LoopStatus(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()> {
        self.send(PlayerInterfaceAction::SetLoopStatus(loop_status));
        Ok(())
    }

    #[dbus_interface(property)]
    async fn rate(&self) -> PlaybackRate {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::Rate(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    fn set_rate(&self, rate: PlaybackRate) {
        self.send(PlayerInterfaceAction::SetRate(rate));
    }

    #[dbus_interface(property)]
    async fn shuffle(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::Shuffle(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    fn set_shuffle(&self, shuffle: bool) {
        self.send(PlayerInterfaceAction::SetShuffle(shuffle));
    }

    #[dbus_interface(property)]
    async fn metadata(&self) -> Metadata {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::Metadata(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn volume(&self) -> Volume {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::Volume(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    fn set_volume(&self, volume: Volume) {
        self.send(PlayerInterfaceAction::SetVolume(volume));
    }

    #[dbus_interface(property)]
    async fn position(&self) -> TimeInUs {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::Position(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn minimum_rate(&self) -> PlaybackRate {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::MinimumRate(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn maximum_rate(&self) -> PlaybackRate {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::MaximumRate(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_go_next(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::CanGoNext(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_go_previous(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::CanGoPrevious(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_play(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::CanPlay(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_pause(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::CanPause(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_seek(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::CanSeek(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_control(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(PlayerInterfaceAction::CanControl(tx));
        rx.await.unwrap()
    }
}

pub struct Server<T>
where
    T: PlayerInterface + 'static,
{
    bus_name: WellKnownName<'static>,
    connection: OnceLock<Connection>,
    imp: T,
}

impl<T> fmt::Debug for Server<T>
where
    T: PlayerInterface + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Server")
            .field("bus_name", &self.bus_name)
            .finish()
    }
}

impl<T> Server<T>
where
    T: PlayerInterface + 'static,
{
    /// ## Error
    /// Returns `Err` if the resulting bus name is invalid.
    pub fn new(bus_name_suffix: &str, imp: T) -> Result<Self> {
        Ok(Self {
            bus_name: WellKnownName::try_from(format!(
                "org.mpris.MediaPlayer2.{}",
                bus_name_suffix
            ))?,
            imp,
            connection: OnceLock::new(),
        })
    }

    pub fn imp(&self) -> &T {
        &self.imp
    }

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

        while let Some(action) = rx.next().await {
            match action {
                Action::RootInterface(action) => self.handle_interface_action(action),
                Action::PlayerInterface(action) => self.handle_player_interface_action(action),
            }
        }

        Ok(())
    }

    async fn interface_ref<I: zbus::Interface>(&self) -> Result<InterfaceRef<I>> {
        self.connection
            .get()
            .expect("server must be ran first")
            .object_server()
            .interface::<_, I>(OBJECT_PATH)
            .await
    }

    fn handle_interface_action(&self, action: RootInterfaceAction) {
        match action {
            // Methods
            RootInterfaceAction::Raise => self.imp.raise(),
            RootInterfaceAction::Quit => self.imp.quit(),
            // Properties
            RootInterfaceAction::CanQuit(sender) => sender.send(self.imp.can_quit()).unwrap(),
            RootInterfaceAction::Fullscreen(sender) => sender.send(self.imp.fullscreen()).unwrap(),
            RootInterfaceAction::SetFullscreen(fullscreen) => self.imp.set_fullscreen(fullscreen),
            RootInterfaceAction::CanSetFullScreen(sender) => {
                sender.send(self.imp.can_set_fullscreen()).unwrap()
            }
            RootInterfaceAction::CanRaise(sender) => sender.send(self.imp.can_raise()).unwrap(),
            RootInterfaceAction::HasTrackList(sender) => {
                sender.send(self.imp.has_track_list()).unwrap()
            }
            RootInterfaceAction::Identity(sender) => sender.send(self.imp.identity()).unwrap(),
            RootInterfaceAction::DesktopEntry(sender) => {
                sender.send(self.imp.desktop_entry()).unwrap()
            }
            RootInterfaceAction::SupportedUriSchemes(sender) => {
                sender.send(self.imp.supported_uri_schemes()).unwrap()
            }
            RootInterfaceAction::SupportedMimeTypes(sender) => {
                sender.send(self.imp.supported_mime_types()).unwrap()
            }
        }
    }

    fn handle_player_interface_action(&self, action: PlayerInterfaceAction) {
        match action {
            // Methods
            PlayerInterfaceAction::Next => self.imp.next(),
            PlayerInterfaceAction::Previous => self.imp.previous(),
            PlayerInterfaceAction::Pause => self.imp.pause(),
            PlayerInterfaceAction::PlayPause => self.imp.play_pause(),
            PlayerInterfaceAction::Stop => self.imp.stop(),
            PlayerInterfaceAction::Play => self.imp.play(),
            PlayerInterfaceAction::Seek(offset) => self.imp.seek(offset),
            PlayerInterfaceAction::SetPosition(track_id, position) => {
                self.imp.set_position(track_id, position)
            }
            PlayerInterfaceAction::OpenUri(uri) => self.imp.open_uri(uri),
            // Properties
            PlayerInterfaceAction::PlaybackStatus(sender) => {
                sender.send(self.imp.playback_status()).unwrap()
            }
            PlayerInterfaceAction::LoopStatus(sender) => {
                sender.send(self.imp.loop_status()).unwrap()
            }
            PlayerInterfaceAction::SetLoopStatus(loop_status) => {
                self.imp.set_loop_status(loop_status)
            }
            PlayerInterfaceAction::Rate(sender) => sender.send(self.imp.rate()).unwrap(),
            PlayerInterfaceAction::SetRate(rate) => self.imp.set_rate(rate),
            PlayerInterfaceAction::Shuffle(sender) => sender.send(self.imp.shuffle()).unwrap(),
            PlayerInterfaceAction::SetShuffle(shuffle) => self.imp.set_shuffle(shuffle),
            PlayerInterfaceAction::Metadata(sender) => sender.send(self.imp.metadata()).unwrap(),
            PlayerInterfaceAction::Volume(sender) => sender.send(self.imp.volume()).unwrap(),
            PlayerInterfaceAction::SetVolume(volume) => self.imp.set_volume(volume),
            PlayerInterfaceAction::Position(sender) => sender.send(self.imp.position()).unwrap(),
            PlayerInterfaceAction::MinimumRate(sender) => {
                sender.send(self.imp.minimum_rate()).unwrap()
            }
            PlayerInterfaceAction::MaximumRate(sender) => {
                sender.send(self.imp.maximum_rate()).unwrap()
            }
            PlayerInterfaceAction::CanGoNext(sender) => {
                sender.send(self.imp.can_go_next()).unwrap()
            }
            PlayerInterfaceAction::CanGoPrevious(sender) => {
                sender.send(self.imp.can_go_previous()).unwrap()
            }
            PlayerInterfaceAction::CanPlay(sender) => sender.send(self.imp.can_play()).unwrap(),
            PlayerInterfaceAction::CanPause(sender) => sender.send(self.imp.can_pause()).unwrap(),
            PlayerInterfaceAction::CanSeek(sender) => sender.send(self.imp.can_seek()).unwrap(),
            PlayerInterfaceAction::CanControl(sender) => {
                sender.send(self.imp.can_control()).unwrap()
            }
        }
    }
}

macro_rules! delegate_changed {
    ($iface:ty, $name:ident) => {
        pub async fn $name(&self) -> Result<()> {
            let iface_ref = self.interface_ref::<$iface>().await?;
            let iface = iface_ref.get().await;
            iface.$name(iface_ref.signal_context()).await
        }
    };
}

impl<T> Server<T>
where
    T: PlayerInterface + 'static,
{
    // org.mpris.MediaPlayer2
    delegate_changed!(RawRootInterface, can_quit_changed);
    delegate_changed!(RawRootInterface, fullscreen_changed);
    delegate_changed!(RawRootInterface, can_set_fullscreen_changed);
    delegate_changed!(RawRootInterface, can_raise_changed);
    delegate_changed!(RawRootInterface, has_track_list_changed);
    delegate_changed!(RawRootInterface, identity_changed);
    delegate_changed!(RawRootInterface, desktop_entry_changed);
    delegate_changed!(RawRootInterface, supported_uri_schemes_changed);
    delegate_changed!(RawRootInterface, supported_mime_types_changed);

    // org.mpris.MediaPlayer2.Player
    pub async fn seeked(&self, position: TimeInUs) -> Result<()> {
        let iface_ref = self.interface_ref::<RawPlayerInterface>().await?;
        RawPlayerInterface::seeked(iface_ref.signal_context(), position).await
    }
    delegate_changed!(RawPlayerInterface, playback_status_changed);
    delegate_changed!(RawPlayerInterface, loop_status_changed);
    delegate_changed!(RawPlayerInterface, rate_changed);
    delegate_changed!(RawPlayerInterface, shuffle_changed);
    delegate_changed!(RawPlayerInterface, metadata_changed);
    delegate_changed!(RawPlayerInterface, volume_changed);
    delegate_changed!(RawPlayerInterface, position_changed);
    delegate_changed!(RawPlayerInterface, minimum_rate_changed);
    delegate_changed!(RawPlayerInterface, maximum_rate_changed);
    delegate_changed!(RawPlayerInterface, can_go_next_changed);
    delegate_changed!(RawPlayerInterface, can_go_previous_changed);
    delegate_changed!(RawPlayerInterface, can_play_changed);
    delegate_changed!(RawPlayerInterface, can_pause_changed);
    delegate_changed!(RawPlayerInterface, can_seek_changed);
    delegate_changed!(RawPlayerInterface, can_control_changed);
}
