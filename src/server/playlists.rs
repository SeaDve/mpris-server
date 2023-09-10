use futures_channel::{mpsc, oneshot};
use futures_util::StreamExt;
use zbus::{dbus_interface, ConnectionBuilder, Result, SignalContext};

use super::{
    player::{RawPlayerInterface, RawRootInterface},
    utils::iface_delegate,
    Action, Server, OBJECT_PATH,
};
use crate::{MaybePlaylist, Playlist, PlaylistId, PlaylistOrdering, PlaylistsInterface};

pub(super) enum PlaylistsAction {
    // Methods
    ActivatePlaylist(PlaylistId),
    GetPlaylists(
        u32,
        u32,
        PlaylistOrdering,
        bool,
        oneshot::Sender<Vec<Playlist>>,
    ),

    // Properties
    PlaylistCount(oneshot::Sender<u32>),
    Orderings(oneshot::Sender<Vec<PlaylistOrdering>>),
    ActivePlaylist(oneshot::Sender<MaybePlaylist>),
}

pub(super) struct RawPlaylistsInterface {
    pub(super) tx: mpsc::UnboundedSender<Action>,
}

impl RawPlaylistsInterface {
    fn send(&self, action: PlaylistsAction) {
        self.tx.unbounded_send(Action::Playlists(action)).unwrap();
    }
}

#[dbus_interface(name = "org.mpris.MediaPlayer2.Playlists")]
impl RawPlaylistsInterface {
    fn activate_playlist(&self, playlist_id: PlaylistId) {
        self.send(PlaylistsAction::ActivatePlaylist(playlist_id));
    }

    async fn get_playlists(
        &self,
        index: u32,
        max_count: u32,
        order: PlaylistOrdering,
        reverse_order: bool,
    ) -> Vec<Playlist> {
        let (tx, rx) = oneshot::channel();
        self.send(PlaylistsAction::GetPlaylists(
            index,
            max_count,
            order,
            reverse_order,
            tx,
        ));
        rx.await.unwrap()
    }

    #[dbus_interface(signal)]
    async fn playlist_changed(ctxt: &SignalContext<'_>, playlist: Playlist) -> Result<()>;

    #[dbus_interface(property)]
    async fn playlist_count(&self) -> u32 {
        let (tx, rx) = oneshot::channel();
        self.send(PlaylistsAction::PlaylistCount(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn orderings(&self) -> Vec<PlaylistOrdering> {
        let (tx, rx) = oneshot::channel();
        self.send(PlaylistsAction::Orderings(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn active_playlist(&self) -> MaybePlaylist {
        let (tx, rx) = oneshot::channel();
        self.send(PlaylistsAction::ActivePlaylist(tx));
        rx.await.unwrap()
    }
}

impl<T> Server<T>
where
    T: PlaylistsInterface + 'static,
{
    pub async fn run_with_playlists(&self) -> Result<()> {
        let (tx, mut rx) = mpsc::unbounded::<Action>();

        let connection = ConnectionBuilder::session()?
            .name(&self.bus_name)?
            .serve_at(OBJECT_PATH, RawRootInterface { tx: tx.clone() })?
            .serve_at(OBJECT_PATH, RawPlayerInterface { tx: tx.clone() })?
            .serve_at(OBJECT_PATH, RawPlaylistsInterface { tx })?
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
                Action::Playlists(action) => self.handle_playlists_interface_action(action).await,
                Action::TrackList(_) => unreachable!(),
            }
        }

        Ok(())
    }

    pub(super) async fn handle_playlists_interface_action(&self, action: PlaylistsAction) {
        match action {
            PlaylistsAction::ActivatePlaylist(_) => todo!(),
            PlaylistsAction::GetPlaylists(_, _, _, _, _) => todo!(),
            PlaylistsAction::PlaylistCount(_) => todo!(),
            PlaylistsAction::Orderings(_) => todo!(),
            PlaylistsAction::ActivePlaylist(_) => todo!(),
        }
    }

    // org.mpris.MediaPlayer2.Playlists
    pub async fn playlist_changed(&self, playlist: Playlist) -> Result<()> {
        let iface_ref = self.interface_ref::<RawPlaylistsInterface>().await?;
        RawPlaylistsInterface::playlist_changed(iface_ref.signal_context(), playlist).await
    }
    iface_delegate!(RawPlaylistsInterface, playlist_count_changed);
    iface_delegate!(RawPlaylistsInterface, orderings_changed);
    iface_delegate!(RawPlaylistsInterface, active_playlist_changed);
}
