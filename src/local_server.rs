use std::{
    cell::RefCell,
    fmt,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use async_trait::async_trait;
use enumflags2::BitFlags;
use futures_channel::{mpsc, oneshot};
use futures_util::{FutureExt, StreamExt};
use zbus::{fdo, Connection, Result};

use crate::{
    LocalPlayerInterface, LocalPlaylistsInterface, LocalTrackListInterface, LoopStatus,
    MaybePlaylist, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, Playlist, PlaylistId,
    PlaylistOrdering, PlaylistsInterface, PlaylistsProperty, PlaylistsSignal, Property, Server,
    ServerProxy, Signal, Time, TrackId, TrackListInterface, TrackListProperty, TrackListSignal,
    Uri, Volume,
};

enum PlayerAction {
    // Methods
    Raise(oneshot::Sender<fdo::Result<()>>),
    Quit(oneshot::Sender<fdo::Result<()>>),

    // Properties
    CanQuit(oneshot::Sender<fdo::Result<bool>>),
    Fullscreen(oneshot::Sender<fdo::Result<bool>>),
    SetFullscreen(bool, oneshot::Sender<Result<()>>),
    CanSetFullScreen(oneshot::Sender<fdo::Result<bool>>),
    CanRaise(oneshot::Sender<fdo::Result<bool>>),
    HasTrackList(oneshot::Sender<fdo::Result<bool>>),
    Identity(oneshot::Sender<fdo::Result<String>>),
    DesktopEntry(oneshot::Sender<fdo::Result<String>>),
    SupportedUriSchemes(oneshot::Sender<fdo::Result<Vec<String>>>),
    SupportedMimeTypes(oneshot::Sender<fdo::Result<Vec<String>>>),

    // Methods
    Next(oneshot::Sender<fdo::Result<()>>),
    Previous(oneshot::Sender<fdo::Result<()>>),
    Pause(oneshot::Sender<fdo::Result<()>>),
    PlayPause(oneshot::Sender<fdo::Result<()>>),
    Stop(oneshot::Sender<fdo::Result<()>>),
    Play(oneshot::Sender<fdo::Result<()>>),
    Seek(Time, oneshot::Sender<fdo::Result<()>>),
    SetPosition(TrackId, Time, oneshot::Sender<fdo::Result<()>>),
    OpenUri(String, oneshot::Sender<fdo::Result<()>>),

    // Properties
    PlaybackStatus(oneshot::Sender<fdo::Result<PlaybackStatus>>),
    LoopStatus(oneshot::Sender<fdo::Result<LoopStatus>>),
    SetLoopStatus(LoopStatus, oneshot::Sender<Result<()>>),
    Rate(oneshot::Sender<fdo::Result<PlaybackRate>>),
    SetRate(PlaybackRate, oneshot::Sender<Result<()>>),
    Shuffle(oneshot::Sender<fdo::Result<bool>>),
    SetShuffle(bool, oneshot::Sender<Result<()>>),
    Metadata(oneshot::Sender<fdo::Result<Metadata>>),
    Volume(oneshot::Sender<fdo::Result<Volume>>),
    SetVolume(Volume, oneshot::Sender<Result<()>>),
    Position(oneshot::Sender<fdo::Result<Time>>),
    MinimumRate(oneshot::Sender<fdo::Result<PlaybackRate>>),
    MaximumRate(oneshot::Sender<fdo::Result<PlaybackRate>>),
    CanGoNext(oneshot::Sender<fdo::Result<bool>>),
    CanGoPrevious(oneshot::Sender<fdo::Result<bool>>),
    CanPlay(oneshot::Sender<fdo::Result<bool>>),
    CanPause(oneshot::Sender<fdo::Result<bool>>),
    CanSeek(oneshot::Sender<fdo::Result<bool>>),
    CanControl(oneshot::Sender<fdo::Result<bool>>),
}

enum PlaylistsAction {
    // Methods
    ActivatePlaylist(PlaylistId, oneshot::Sender<fdo::Result<()>>),
    GetPlaylists(
        u32,
        u32,
        PlaylistOrdering,
        bool,
        oneshot::Sender<fdo::Result<Vec<Playlist>>>,
    ),

    // Properties
    PlaylistCount(oneshot::Sender<fdo::Result<u32>>),
    Orderings(oneshot::Sender<fdo::Result<Vec<PlaylistOrdering>>>),
    ActivePlaylist(oneshot::Sender<fdo::Result<MaybePlaylist>>),
}

enum TrackListAction {
    // Methods
    GetTracksMetadata(Vec<TrackId>, oneshot::Sender<fdo::Result<Vec<Metadata>>>),
    AddTrack(Uri, TrackId, bool, oneshot::Sender<fdo::Result<()>>),
    RemoveTrack(TrackId, oneshot::Sender<fdo::Result<()>>),
    GoTo(TrackId, oneshot::Sender<fdo::Result<()>>),

    // Properties
    Tracks(oneshot::Sender<fdo::Result<Vec<TrackId>>>),
    CanEditTracks(oneshot::Sender<fdo::Result<bool>>),
}

enum Action {
    Player(PlayerAction),
    TrackList(TrackListAction),
    Playlists(PlaylistsAction),
}

struct InnerImp<T> {
    tx: mpsc::UnboundedSender<Action>,

    // If we use `PhantomData<T>` and `T` is not `Send` and `Sync`, we get a compile error
    // when using `InnerImp` in the inner non-local `Server` as it requires `T` to be `Send`
    // and `Sync`, which defeats the purpose of `local::Server`. So, we need to use `fn() -> T`
    // in `PhantomData` to preserve the type information without requiring `T` to be `Send`
    // and `Sync` for `InnerImp` to be `Send` and `Sync`.
    imp_ty: PhantomData<fn() -> T>,
}

impl<T> InnerImp<T> {
    fn send_player(&self, action: PlayerAction) {
        self.tx.unbounded_send(Action::Player(action)).unwrap();
    }

    fn send_track_list(&self, action: TrackListAction) {
        self.tx.unbounded_send(Action::TrackList(action)).unwrap();
    }

    fn send_playlists(&self, action: PlaylistsAction) {
        self.tx.unbounded_send(Action::Playlists(action)).unwrap();
    }
}

#[async_trait]
impl<T> PlayerInterface for InnerImp<T> {
    async fn raise(&self, _: &ServerProxy<'_, Self>) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Raise(tx));
        rx.await.unwrap()
    }

    async fn quit(&self, _: &ServerProxy<'_, Self>) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Quit(tx));
        rx.await.unwrap()
    }

    async fn can_quit(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::CanQuit(tx));
        rx.await.unwrap()
    }

    async fn fullscreen(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Fullscreen(tx));
        rx.await.unwrap()
    }

    async fn set_fullscreen(&self, fullscreen: bool) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::SetFullscreen(fullscreen, tx));
        rx.await.unwrap()
    }

    async fn can_set_fullscreen(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::CanSetFullScreen(tx));
        rx.await.unwrap()
    }

    async fn can_raise(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::CanRaise(tx));
        rx.await.unwrap()
    }

    async fn has_track_list(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::HasTrackList(tx));
        rx.await.unwrap()
    }

    async fn identity(&self) -> fdo::Result<String> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Identity(tx));
        rx.await.unwrap()
    }

    async fn desktop_entry(&self) -> fdo::Result<String> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::DesktopEntry(tx));
        rx.await.unwrap()
    }

    async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::SupportedUriSchemes(tx));
        rx.await.unwrap()
    }

    async fn supported_mime_types(&self) -> fdo::Result<Vec<String>> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::SupportedMimeTypes(tx));
        rx.await.unwrap()
    }

    async fn next(&self, _: &ServerProxy<'_, Self>) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Next(tx));
        rx.await.unwrap()
    }

    async fn previous(&self, _: &ServerProxy<'_, Self>) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Previous(tx));
        rx.await.unwrap()
    }

    async fn pause(&self, _: &ServerProxy<'_, Self>) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Pause(tx));
        rx.await.unwrap()
    }

    async fn play_pause(&self, _: &ServerProxy<'_, Self>) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::PlayPause(tx));
        rx.await.unwrap()
    }

    async fn stop(&self, _: &ServerProxy<'_, Self>) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Stop(tx));
        rx.await.unwrap()
    }

    async fn play(&self, _: &ServerProxy<'_, Self>) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Play(tx));
        rx.await.unwrap()
    }

    async fn seek(&self, _: &ServerProxy<'_, Self>, offset: Time) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Seek(offset, tx));
        rx.await.unwrap()
    }

    async fn set_position(
        &self,
        _: &ServerProxy<'_, Self>,
        track_id: TrackId,
        position: Time,
    ) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::SetPosition(track_id, position, tx));
        rx.await.unwrap()
    }

    async fn open_uri(&self, _: &ServerProxy<'_, Self>, uri: String) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::OpenUri(uri, tx));
        rx.await.unwrap()
    }

    async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::PlaybackStatus(tx));
        rx.await.unwrap()
    }

    async fn loop_status(&self) -> fdo::Result<LoopStatus> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::LoopStatus(tx));
        rx.await.unwrap()
    }

    async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::SetLoopStatus(loop_status, tx));
        rx.await.unwrap()
    }

    async fn rate(&self) -> fdo::Result<PlaybackRate> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Rate(tx));
        rx.await.unwrap()
    }

    async fn set_rate(&self, rate: PlaybackRate) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::SetRate(rate, tx));
        rx.await.unwrap()
    }

    async fn shuffle(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Shuffle(tx));
        rx.await.unwrap()
    }

    async fn set_shuffle(&self, shuffle: bool) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::SetShuffle(shuffle, tx));
        rx.await.unwrap()
    }

    async fn metadata(&self) -> fdo::Result<Metadata> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Metadata(tx));
        rx.await.unwrap()
    }

    async fn volume(&self) -> fdo::Result<Volume> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Volume(tx));
        rx.await.unwrap()
    }

    async fn set_volume(&self, volume: Volume) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::SetVolume(volume, tx));
        rx.await.unwrap()
    }

    async fn position(&self) -> fdo::Result<Time> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::Position(tx));
        rx.await.unwrap()
    }

    async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::MinimumRate(tx));
        rx.await.unwrap()
    }

    async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::MaximumRate(tx));
        rx.await.unwrap()
    }

    async fn can_go_next(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::CanGoNext(tx));
        rx.await.unwrap()
    }

    async fn can_go_previous(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::CanGoPrevious(tx));
        rx.await.unwrap()
    }

    async fn can_play(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::CanPlay(tx));
        rx.await.unwrap()
    }

    async fn can_pause(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::CanPause(tx));
        rx.await.unwrap()
    }

    async fn can_seek(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::CanSeek(tx));
        rx.await.unwrap()
    }

    async fn can_control(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_player(PlayerAction::CanControl(tx));
        rx.await.unwrap()
    }
}

#[async_trait]
impl<T> TrackListInterface for InnerImp<T>
where
    T: LocalTrackListInterface,
{
    async fn get_tracks_metadata(
        &self,
        _: &ServerProxy<'_, Self>,
        track_ids: Vec<TrackId>,
    ) -> fdo::Result<Vec<Metadata>> {
        let (tx, rx) = oneshot::channel();
        self.send_track_list(TrackListAction::GetTracksMetadata(track_ids, tx));
        rx.await.unwrap()
    }

    async fn add_track(
        &self,
        _: &ServerProxy<'_, Self>,
        uri: Uri,
        after_track: TrackId,
        set_as_current: bool,
    ) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_track_list(TrackListAction::AddTrack(
            uri,
            after_track,
            set_as_current,
            tx,
        ));
        rx.await.unwrap()
    }

    async fn remove_track(&self, _: &ServerProxy<'_, Self>, track_id: TrackId) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_track_list(TrackListAction::RemoveTrack(track_id, tx));
        rx.await.unwrap()
    }

    async fn go_to(&self, _: &ServerProxy<'_, Self>, track_id: TrackId) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_track_list(TrackListAction::GoTo(track_id, tx));
        rx.await.unwrap()
    }

    async fn tracks(&self) -> fdo::Result<Vec<TrackId>> {
        let (tx, rx) = oneshot::channel();
        self.send_track_list(TrackListAction::Tracks(tx));
        rx.await.unwrap()
    }

    async fn can_edit_tracks(&self) -> fdo::Result<bool> {
        let (tx, rx) = oneshot::channel();
        self.send_track_list(TrackListAction::CanEditTracks(tx));
        rx.await.unwrap()
    }
}

#[async_trait]
impl<T> PlaylistsInterface for InnerImp<T>
where
    T: LocalPlaylistsInterface,
{
    async fn activate_playlist(
        &self,
        _: &ServerProxy<'_, Self>,
        playlist_id: PlaylistId,
    ) -> fdo::Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_playlists(PlaylistsAction::ActivatePlaylist(playlist_id, tx));
        rx.await.unwrap()
    }

    async fn get_playlists(
        &self,
        _: &ServerProxy<'_, Self>,
        index: u32,
        max_count: u32,
        order: PlaylistOrdering,
        reverse_order: bool,
    ) -> fdo::Result<Vec<Playlist>> {
        let (tx, rx) = oneshot::channel();
        self.send_playlists(PlaylistsAction::GetPlaylists(
            index,
            max_count,
            order,
            reverse_order,
            tx,
        ));
        rx.await.unwrap()
    }

    async fn playlist_count(&self) -> fdo::Result<u32> {
        let (tx, rx) = oneshot::channel();
        self.send_playlists(PlaylistsAction::PlaylistCount(tx));
        rx.await.unwrap()
    }

    async fn orderings(&self) -> fdo::Result<Vec<PlaylistOrdering>> {
        let (tx, rx) = oneshot::channel();
        self.send_playlists(PlaylistsAction::Orderings(tx));
        rx.await.unwrap()
    }

    async fn active_playlist(&self) -> fdo::Result<MaybePlaylist> {
        let (tx, rx) = oneshot::channel();
        self.send_playlists(PlaylistsAction::ActivePlaylist(tx));
        rx.await.unwrap()
    }
}

type TaskInner = Pin<Box<dyn Future<Output = Result<()>>>>;

/// A task that initializes the connection and runs [`LocalServer`]'s event
/// handler until the server and this task is dropped.
///
/// This must be awaited as soon as possible after creating the server.
///
/// See [`LocalServer::init_and_run`] for more information.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct LocalServerRunTask {
    inner: Option<TaskInner>,
}

impl fmt::Debug for LocalServerRunTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalServerRunTask")
            .field("has_inner", &self.inner.is_some())
            .finish()
    }
}

impl Future for LocalServerRunTask {
    type Output = Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.as_mut() {
            Some(inner) => inner.poll_unpin(cx),
            None => Poll::Ready(Ok(())),
        }
    }
}

/// Local version of [`Server`] that doesn't require `T` to be `Send` and
/// `Sync`.
///
/// If your type is already `Send` and `Sync`, consider using [`Server`] instead
/// because [`LocalServer`] has an overhead of sending messages across threads.
///
/// For more information, see [`Server`] documentations.
pub struct LocalServer<T>
where
    T: LocalPlayerInterface + 'static,
{
    inner: Rc<Server<InnerImp<T>>>,
    imp: Rc<T>,
    runner: RefCell<Option<TaskInner>>,
}

impl<T> fmt::Debug for LocalServer<T>
where
    T: LocalPlayerInterface + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalServer").finish()
    }
}

impl<T> LocalServer<T>
where
    T: LocalPlayerInterface + 'static,
{
    /// Creates a new [`LocalServer`] with the given bus name suffix and
    /// implementation, `imp`, which must implement [`LocalRootInterface`] and
    /// [`LocalPlayerInterface`].
    ///
    /// To start the connection, [`LocalServer::init_and_run`] must be called.
    ///
    /// The resulting bus name will be
    /// `org.mpris.MediaPlayer2.<bus_name_suffix>`, where
    /// `<bus_name_suffix>`must be a unique identifier, such as one based on a
    /// UNIX process id. For example, this could be:
    ///
    /// * `org.mpris.MediaPlayer2.vlc.instance7389`
    ///
    /// **Note:** According to the [`D-Bus specification`], the unique
    /// identifier "must only contain  the ASCII characters
    /// `[A-Z][a-z][0-9]_-`" and "must not begin with a digit".
    ///
    /// [`LocalRootInterface`]: crate::LocalRootInterface
    /// [`D-Bus specification`]: dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
    pub fn new(bus_name_suffix: &str, imp: T) -> Self {
        Self::new_inner(
            bus_name_suffix,
            imp,
            Server::new,
            |mut rx, connection, imp| async move {
                while let Some(action) = rx.next().await {
                    match action {
                        Action::Player(action) => {
                            Self::handle_player_action(&connection, &imp, action).await
                        }
                        Action::TrackList(_) | Action::Playlists(_) => unreachable!(),
                    }
                }
            },
        )
    }

    /// Returns a task that initializes the connection and run the server until
    /// the server and the task is dropped.
    ///
    /// The task must be awaited as soon as possible after creating the server.
    ///
    /// The returned task is no-op if the server has been ran before.
    pub fn init_and_run(&self) -> LocalServerRunTask {
        LocalServerRunTask {
            inner: self.runner.take(),
        }
    }

    /// Returns a reference to the underlying implementation.
    #[inline]
    pub fn imp(&self) -> &T {
        &self.imp
    }

    /// Returns a reference to the inner [`Connection`].
    ///
    /// If you needed to call this, consider filing an issue.
    pub async fn connection(&self) -> Result<&Connection> {
        self.inner.connection().await
    }

    /// Emits the given signal.
    #[inline]
    pub async fn emit(&self, signal: Signal) -> Result<()> {
        self.inner.emit(signal).await
    }

    /// Emits the `PropertiesChanged` signal for the given properties.
    ///
    /// This categorizes the property in the `changed` or `invalidated`
    /// properties as defined by the spec.
    ///
    /// [`LocalServer::track_list_properties_changed`] or
    /// [`LocalServer::playlists_properties_changed`] are used
    /// to emit `PropertiesChanged` for the `TrackList` or `Playlists`
    /// interfaces respectively.
    #[inline]
    pub async fn properties_changed(
        &self,
        properties: impl Into<BitFlags<Property>>,
    ) -> Result<()> {
        self.inner.properties_changed(properties).await
    }

    async fn handle_player_action(connection: &Connection, imp: &T, action: PlayerAction) {
        let proxy = ServerProxy::new(connection, imp);
        match action {
            // Methods
            PlayerAction::Raise(sender) => {
                let ret = imp.raise(&proxy).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Quit(sender) => {
                let ret = imp.quit(&proxy).await;
                sender.send(ret).unwrap();
            }
            // Properties
            PlayerAction::CanQuit(sender) => {
                let ret = imp.can_quit().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Fullscreen(sender) => {
                let ret = imp.fullscreen().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::SetFullscreen(fullscreen, sender) => {
                let ret = imp.set_fullscreen(fullscreen).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::CanSetFullScreen(sender) => {
                let ret = imp.can_set_fullscreen().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::CanRaise(sender) => {
                let ret = imp.can_raise().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::HasTrackList(sender) => {
                let ret = imp.has_track_list().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Identity(sender) => {
                let ret = imp.identity().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::DesktopEntry(sender) => {
                let ret = imp.desktop_entry().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::SupportedUriSchemes(sender) => {
                let ret = imp.supported_uri_schemes().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::SupportedMimeTypes(sender) => {
                let ret = imp.supported_mime_types().await;
                sender.send(ret).unwrap();
            }
            // Methods
            PlayerAction::Next(sender) => {
                let ret = imp.next(&proxy).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Previous(sender) => {
                let ret = imp.previous(&proxy).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Pause(sender) => {
                let ret = imp.pause(&proxy).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::PlayPause(sender) => {
                let ret = imp.play_pause(&proxy).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Stop(sender) => {
                let ret = imp.stop(&proxy).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Play(sender) => {
                let ret = imp.play(&proxy).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Seek(offset, sender) => {
                let ret = imp.seek(&proxy, offset).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::SetPosition(track_id, position, sender) => {
                let ret = imp.set_position(&proxy, track_id, position).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::OpenUri(uri, sender) => {
                let ret = imp.open_uri(&proxy, uri).await;
                sender.send(ret).unwrap();
            }
            // Properties
            PlayerAction::PlaybackStatus(sender) => {
                let ret = imp.playback_status().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::LoopStatus(sender) => {
                let ret = imp.loop_status().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::SetLoopStatus(loop_status, sender) => {
                let ret = imp.set_loop_status(loop_status).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Rate(sender) => {
                let ret = imp.rate().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::SetRate(rate, sender) => {
                let ret = imp.set_rate(rate).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Shuffle(sender) => {
                let ret = imp.shuffle().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::SetShuffle(shuffle, sender) => {
                let ret = imp.set_shuffle(shuffle).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Metadata(sender) => {
                let ret = imp.metadata().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Volume(sender) => {
                let ret = imp.volume().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::SetVolume(volume, sender) => {
                let ret = imp.set_volume(volume).await;
                sender.send(ret).unwrap();
            }
            PlayerAction::Position(sender) => {
                let ret = imp.position().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::MinimumRate(sender) => {
                let ret = imp.minimum_rate().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::MaximumRate(sender) => {
                let ret = imp.maximum_rate().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::CanGoNext(sender) => {
                let ret = imp.can_go_next().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::CanGoPrevious(sender) => {
                let ret = imp.can_go_previous().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::CanPlay(sender) => {
                let ret = imp.can_play().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::CanPause(sender) => {
                let ret = imp.can_pause().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::CanSeek(sender) => {
                let ret = imp.can_seek().await;
                sender.send(ret).unwrap();
            }
            PlayerAction::CanControl(sender) => {
                let ret = imp.can_control().await;
                sender.send(ret).unwrap();
            }
        }
    }

    fn new_inner<R>(
        bus_name_suffix: &str,
        imp: T,
        server_func: impl FnOnce(&str, InnerImp<T>) -> Server<InnerImp<T>>,
        runner_func: impl FnOnce(mpsc::UnboundedReceiver<Action>, Connection, Rc<T>) -> R + 'static,
    ) -> Self
    where
        R: Future<Output = ()> + 'static,
    {
        let (tx, rx) = mpsc::unbounded::<Action>();

        let inner = Rc::new(server_func(
            bus_name_suffix,
            InnerImp {
                tx,
                imp_ty: PhantomData,
            },
        ));

        let imp = Rc::new(imp);

        let inner_clone = Rc::clone(&inner);
        let imp_clone = Rc::clone(&imp);
        let runner = Box::pin(async move {
            inner_clone.init().await?;
            let connection = inner_clone.connection().await?;
            runner_func(rx, connection.clone(), imp_clone).await;
            Ok(())
        });

        Self {
            inner,
            imp,
            runner: RefCell::new(Some(runner)),
        }
    }
}

impl<T> LocalServer<T>
where
    T: LocalTrackListInterface + 'static,
{
    /// Creates a new [`LocalServer`] with the given bus name suffix and
    /// implementation, which must implement [`TrackListInterface`] in addition
    /// to [`LocalRootInterface`] and [`LocalPlayerInterface`].
    ///
    /// See also [`LocalServer::new`].
    ///
    /// [`LocalRootInterface`]: crate::LocalRootInterface
    pub fn new_with_track_list(bus_name_suffix: &str, imp: T) -> Self {
        Self::new_inner(
            bus_name_suffix,
            imp,
            Server::new_with_track_list,
            |mut rx, connection, imp| async move {
                while let Some(action) = rx.next().await {
                    match action {
                        Action::Player(action) => {
                            Self::handle_player_action(&connection, &imp, action).await
                        }
                        Action::TrackList(action) => {
                            Self::handle_track_list_action(&connection, &imp, action).await
                        }
                        Action::Playlists(_) => unreachable!(),
                    }
                }
            },
        )
    }

    /// Emits the given signal on the `TrackList` interface.
    #[inline]
    pub async fn track_list_emit(&self, signal: TrackListSignal) -> Result<()> {
        self.inner.track_list_emit(signal).await
    }

    /// Emits the `PropertiesChanged` signal for the given properties.
    ///
    /// This categorizes the property in the `changed` or `invalidated`
    /// properties as defined by the spec.
    #[inline]
    pub async fn track_list_properties_changed(
        &self,
        properties: impl Into<BitFlags<TrackListProperty>>,
    ) -> Result<()> {
        self.inner.track_list_properties_changed(properties).await
    }

    async fn handle_track_list_action(connection: &Connection, imp: &T, action: TrackListAction) {
        let proxy = ServerProxy::new(connection, imp);
        match action {
            // Methods
            TrackListAction::GetTracksMetadata(track_ids, sender) => {
                let ret = imp.get_tracks_metadata(&proxy, track_ids).await;
                sender.send(ret).unwrap();
            }
            TrackListAction::AddTrack(uri, after_track, set_as_current, sender) => {
                let ret = imp
                    .add_track(&proxy, uri, after_track, set_as_current)
                    .await;
                sender.send(ret).unwrap();
            }
            TrackListAction::RemoveTrack(track_id, sender) => {
                let ret = imp.remove_track(&proxy, track_id).await;
                sender.send(ret).unwrap();
            }
            TrackListAction::GoTo(track_id, sender) => {
                let ret = imp.go_to(&proxy, track_id).await;
                sender.send(ret).unwrap();
            }
            // Properties
            TrackListAction::Tracks(sender) => {
                let ret = imp.tracks().await;
                sender.send(ret).unwrap();
            }
            TrackListAction::CanEditTracks(sender) => {
                let ret = imp.can_edit_tracks().await;
                sender.send(ret).unwrap();
            }
        }
    }
}

impl<T> LocalServer<T>
where
    T: LocalPlaylistsInterface + 'static,
{
    /// Creates a new [`LocalServer`] with the given bus name suffix and
    /// implementation, which must implement [`LocalPlaylistsInterface`] in
    /// addition to [`LocalRootInterface`] and [`LocalPlayerInterface`].
    ///
    /// See also [`LocalServer::new`].
    ///
    /// [`LocalRootInterface`]: crate::LocalRootInterface
    pub fn new_with_playlists(bus_name_suffix: &str, imp: T) -> Self {
        Self::new_inner(
            bus_name_suffix,
            imp,
            Server::new_with_playlists,
            |mut rx, connection, imp| async move {
                while let Some(action) = rx.next().await {
                    match action {
                        Action::Player(action) => {
                            Self::handle_player_action(&connection, &imp, action).await
                        }
                        Action::Playlists(action) => {
                            Self::handle_playlists_actions(&connection, &imp, action).await
                        }
                        Action::TrackList(_) => unreachable!(),
                    }
                }
            },
        )
    }

    /// Emits the given signal on the `Playlists` interface.
    #[inline]
    pub async fn playlists_emit(&self, signal: PlaylistsSignal) -> Result<()> {
        self.inner.playlists_emit(signal).await
    }

    /// Emits the `PropertiesChanged` signal for the given properties.
    ///
    /// This categorizes the property in the `changed` or `invalidated`
    /// properties as defined by the spec.
    #[inline]
    pub async fn playlists_properties_changed(
        &self,
        properties: impl Into<BitFlags<PlaylistsProperty>>,
    ) -> Result<()> {
        self.inner.playlists_properties_changed(properties).await
    }

    async fn handle_playlists_actions(connection: &Connection, imp: &T, action: PlaylistsAction) {
        let proxy = ServerProxy::new(connection, imp);
        match action {
            PlaylistsAction::ActivatePlaylist(playlist_id, sender) => {
                let ret = imp.activate_playlist(&proxy, playlist_id).await;
                sender.send(ret).unwrap();
            }
            PlaylistsAction::GetPlaylists(index, max_count, order, reverse_order, sender) => {
                let ret = imp
                    .get_playlists(&proxy, index, max_count, order, reverse_order)
                    .await;
                sender.send(ret).unwrap();
            }
            PlaylistsAction::PlaylistCount(sender) => {
                let ret = imp.playlist_count().await;
                sender.send(ret).unwrap();
            }
            PlaylistsAction::Orderings(sender) => {
                let ret = imp.orderings().await;
                sender.send(ret).unwrap();
            }
            PlaylistsAction::ActivePlaylist(sender) => {
                let ret = imp.active_playlist().await;
                sender.send(ret).unwrap();
            }
        }
    }
}

impl<T> LocalServer<T>
where
    T: LocalTrackListInterface + LocalPlaylistsInterface + 'static,
{
    /// Creates a new [`LocalServer`] with the given bus name suffix and
    /// implementation, which must implement [`LocalTrackListInterface`] and
    /// [`LocalPlaylistsInterface`] in addition to [`LocalRootInterface`] and
    /// [`LocalPlayerInterface`].
    ///
    /// See also [`LocalServer::new`].
    ///
    /// [`LocalRootInterface`]: crate::LocalRootInterface
    pub fn new_with_all(bus_name_suffix: &str, imp: T) -> Self {
        Self::new_inner(
            bus_name_suffix,
            imp,
            Server::new_with_all,
            |mut rx, connection, imp| async move {
                while let Some(action) = rx.next().await {
                    match action {
                        Action::Player(action) => {
                            Self::handle_player_action(&connection, &imp, action).await
                        }
                        Action::Playlists(action) => {
                            Self::handle_playlists_actions(&connection, &imp, action).await
                        }
                        Action::TrackList(action) => {
                            Self::handle_track_list_action(&connection, &imp, action).await
                        }
                    }
                }
            },
        )
    }
}
