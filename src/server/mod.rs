mod player;
mod track_list;
mod utils;

use std::{fmt, sync::OnceLock};

use zbus::{names::WellKnownName, zvariant::ObjectPath, Connection, InterfaceRef, Result};

use self::{
    player::{PlayerAction, RootAction},
    track_list::TrackListAction,
};
use crate::PlayerInterface;

const OBJECT_PATH: ObjectPath<'static> =
    ObjectPath::from_static_str_unchecked("/org/mpris/MediaPlayer2");

enum Action {
    Root(RootAction),
    Player(PlayerAction),
    TrackList(TrackListAction),
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
