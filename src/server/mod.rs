mod player;
mod playlists;
mod track_list;
mod utils;

use std::{fmt, sync::OnceLock};

use futures_channel::mpsc;
use futures_util::StreamExt;
use zbus::{
    names::WellKnownName, zvariant::ObjectPath, Connection, ConnectionBuilder, InterfaceRef, Result,
};

use self::{
    player::{PlayerAction, RawPlayerInterface, RawRootInterface, RootAction},
    playlists::{PlaylistsAction, RawPlaylistsInterface},
    track_list::{RawTrackListInterface, TrackListAction},
};
use crate::{PlayerInterface, PlaylistsInterface, TrackListInterface};

const OBJECT_PATH: ObjectPath<'static> =
    ObjectPath::from_static_str_unchecked("/org/mpris/MediaPlayer2");

enum Action {
    Root(RootAction),
    Player(PlayerAction),
    TrackList(TrackListAction),
    Playlists(PlaylistsAction),
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

    async fn interface_ref<I: zbus::Interface>(&self) -> Result<InterfaceRef<I>> {
        self.connection
            .get()
            .expect("server must be ran first")
            .object_server()
            .interface::<_, I>(OBJECT_PATH)
            .await
    }
}

impl<T> Server<T>
where
    T: TrackListInterface + PlaylistsInterface + 'static,
{
    // FIXME Improve this API. Have only one `run` method that serves interfaces, depending on T impls.
    pub async fn run_with_all(&self) -> Result<()> {
        let (tx, mut rx) = mpsc::unbounded::<Action>();

        let connection = ConnectionBuilder::session()?
            .name(&self.bus_name)?
            .serve_at(OBJECT_PATH, RawRootInterface { tx: tx.clone() })?
            .serve_at(OBJECT_PATH, RawPlayerInterface { tx: tx.clone() })?
            .serve_at(OBJECT_PATH, RawTrackListInterface { tx: tx.clone() })?
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
                Action::TrackList(action) => self.handle_track_list_interface_action(action).await,
                Action::Playlists(action) => self.handle_playlists_interface_action(action).await,
            }
        }

        Ok(())
    }
}
