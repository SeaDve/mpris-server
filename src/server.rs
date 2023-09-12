use std::{collections::HashMap, fmt, sync::Arc};

use enumflags2::BitFlags;
use zbus::{
    dbus_interface, fdo,
    names::WellKnownName,
    zvariant::{ObjectPath, Value},
    Connection, ConnectionBuilder, Interface, InterfaceRef, Result, SignalContext,
};

use crate::{
    LoopStatus, MaybePlaylist, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, Playlist,
    PlaylistId, PlaylistOrdering, PlaylistsInterface, PlaylistsProperty, Property, RootInterface,
    TimeInUs, TrackId, TrackListInterface, TrackListProperty, Uri, Volume,
};

const OBJECT_PATH: ObjectPath<'static> =
    ObjectPath::from_static_str_unchecked("/org/mpris/MediaPlayer2");

struct RawRootInterface<T> {
    imp: Arc<T>,
}

#[dbus_interface(name = "org.mpris.MediaPlayer2")]
impl<T> RawRootInterface<T>
where
    T: Send + Sync + RootInterface + 'static,
{
    async fn raise(&self) -> fdo::Result<()> {
        self.imp.raise().await
    }

    async fn quit(&self) -> fdo::Result<()> {
        self.imp.quit().await
    }

    #[dbus_interface(property)]
    async fn can_quit(&self) -> fdo::Result<bool> {
        self.imp.can_quit().await
    }

    #[dbus_interface(property)]
    async fn fullscreen(&self) -> fdo::Result<bool> {
        self.imp.fullscreen().await
    }

    #[dbus_interface(property)]
    async fn set_fullscreen(&self, fullscreen: bool) -> Result<()> {
        self.imp.set_fullscreen(fullscreen).await
    }

    #[dbus_interface(property)]
    async fn can_set_fullscreen(&self) -> fdo::Result<bool> {
        self.imp.can_set_fullscreen().await
    }

    #[dbus_interface(property)]
    async fn can_raise(&self) -> fdo::Result<bool> {
        self.imp.can_raise().await
    }

    #[dbus_interface(property)]
    async fn has_track_list(&self) -> fdo::Result<bool> {
        self.imp.has_track_list().await
    }

    #[dbus_interface(property)]
    async fn identity(&self) -> fdo::Result<String> {
        self.imp.identity().await
    }

    #[dbus_interface(property)]
    async fn desktop_entry(&self) -> fdo::Result<String> {
        self.imp.desktop_entry().await
    }

    #[dbus_interface(property)]
    async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>> {
        self.imp.supported_uri_schemes().await
    }

    #[dbus_interface(property)]
    async fn supported_mime_types(&self) -> fdo::Result<Vec<String>> {
        self.imp.supported_mime_types().await
    }
}

struct RawPlayerInterface<T> {
    imp: Arc<T>,
}

#[dbus_interface(name = "org.mpris.MediaPlayer2.Player")]
impl<T> RawPlayerInterface<T>
where
    T: Send + Sync + PlayerInterface + 'static,
{
    async fn next(&self) -> fdo::Result<()> {
        self.imp.next().await
    }

    async fn previous(&self) -> fdo::Result<()> {
        self.imp.previous().await
    }

    async fn pause(&self) -> fdo::Result<()> {
        self.imp.pause().await
    }

    async fn play_pause(&self) -> fdo::Result<()> {
        self.imp.play_pause().await
    }

    async fn stop(&self) -> fdo::Result<()> {
        self.imp.stop().await
    }

    async fn play(&self) -> fdo::Result<()> {
        self.imp.play().await
    }

    async fn seek(&self, offset: TimeInUs) -> fdo::Result<()> {
        self.imp.seek(offset).await
    }

    async fn set_position(&self, track_id: TrackId, position: TimeInUs) -> fdo::Result<()> {
        self.imp.set_position(track_id, position).await
    }

    async fn open_uri(&self, uri: String) -> fdo::Result<()> {
        self.imp.open_uri(uri).await
    }

    #[dbus_interface(signal)]
    async fn seeked(ctxt: &SignalContext<'_>, position: TimeInUs) -> Result<()>;

    #[dbus_interface(property)]
    async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
        self.imp.playback_status().await
    }

    #[dbus_interface(property)]
    async fn loop_status(&self) -> fdo::Result<LoopStatus> {
        self.imp.loop_status().await
    }

    #[dbus_interface(property)]
    async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()> {
        self.imp.set_loop_status(loop_status).await
    }

    #[dbus_interface(property)]
    async fn rate(&self) -> fdo::Result<PlaybackRate> {
        self.imp.rate().await
    }

    #[dbus_interface(property)]
    async fn set_rate(&self, rate: PlaybackRate) -> Result<()> {
        self.imp.set_rate(rate).await
    }

    #[dbus_interface(property)]
    async fn shuffle(&self) -> fdo::Result<bool> {
        self.imp.shuffle().await
    }

    #[dbus_interface(property)]
    async fn set_shuffle(&self, shuffle: bool) -> Result<()> {
        self.imp.set_shuffle(shuffle).await
    }

    #[dbus_interface(property)]
    async fn metadata(&self) -> fdo::Result<Metadata> {
        self.imp.metadata().await
    }

    #[dbus_interface(property)]
    async fn volume(&self) -> fdo::Result<Volume> {
        self.imp.volume().await
    }

    #[dbus_interface(property)]
    async fn set_volume(&self, volume: Volume) -> Result<()> {
        self.imp.set_volume(volume).await
    }

    #[dbus_interface(property)]
    async fn position(&self) -> fdo::Result<TimeInUs> {
        self.imp.position().await
    }

    #[dbus_interface(property)]
    async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
        self.imp.minimum_rate().await
    }

    #[dbus_interface(property)]
    async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
        self.imp.maximum_rate().await
    }

    #[dbus_interface(property)]
    async fn can_go_next(&self) -> fdo::Result<bool> {
        self.imp.can_go_next().await
    }

    #[dbus_interface(property)]
    async fn can_go_previous(&self) -> fdo::Result<bool> {
        self.imp.can_go_previous().await
    }

    #[dbus_interface(property)]
    async fn can_play(&self) -> fdo::Result<bool> {
        self.imp.can_play().await
    }

    #[dbus_interface(property)]
    async fn can_pause(&self) -> fdo::Result<bool> {
        self.imp.can_pause().await
    }

    #[dbus_interface(property)]
    async fn can_seek(&self) -> fdo::Result<bool> {
        self.imp.can_seek().await
    }

    #[dbus_interface(property)]
    async fn can_control(&self) -> fdo::Result<bool> {
        self.imp.can_control().await
    }
}

struct RawTrackListInterface<T> {
    imp: Arc<T>,
}

#[dbus_interface(name = "org.mpris.MediaPlayer2.TrackList")]
impl<T> RawTrackListInterface<T>
where
    T: TrackListInterface + Send + Sync + 'static,
{
    async fn get_tracks_metadata(&self, track_ids: Vec<TrackId>) -> fdo::Result<Vec<Metadata>> {
        self.imp.get_tracks_metadata(track_ids).await
    }

    async fn add_track(
        &self,
        uri: Uri,
        after_track: TrackId,
        set_as_current: bool,
    ) -> fdo::Result<()> {
        self.imp.add_track(uri, after_track, set_as_current).await
    }

    async fn remove_track(&self, track_id: TrackId) -> fdo::Result<()> {
        self.imp.remove_track(track_id).await
    }

    async fn go_to(&self, track_id: TrackId) -> fdo::Result<()> {
        self.imp.go_to(track_id).await
    }

    #[dbus_interface(signal)]
    async fn track_list_replaced(
        ctxt: &SignalContext<'_>,
        tracks: Vec<TrackId>,
        current_track: TrackId,
    ) -> Result<()>;

    #[dbus_interface(signal)]
    async fn track_added(
        ctxt: &SignalContext<'_>,
        metadata: Metadata,
        after_track: TrackId,
    ) -> Result<()>;

    #[dbus_interface(signal)]
    async fn track_removed(ctxt: &SignalContext<'_>, track_id: TrackId) -> Result<()>;

    #[dbus_interface(signal)]
    async fn track_metadata_changed(
        ctxt: &SignalContext<'_>,
        track_id: TrackId,
        metadata: Metadata,
    ) -> Result<()>;

    #[dbus_interface(property)]
    async fn tracks(&self) -> fdo::Result<Vec<TrackId>> {
        self.imp.tracks().await
    }

    #[dbus_interface(property)]
    async fn can_edit_tracks(&self) -> fdo::Result<bool> {
        self.imp.can_edit_tracks().await
    }
}

struct RawPlaylistsInterface<T> {
    imp: Arc<T>,
}

#[dbus_interface(name = "org.mpris.MediaPlayer2.Playlists")]
impl<T> RawPlaylistsInterface<T>
where
    T: PlaylistsInterface + Send + Sync + 'static,
{
    async fn activate_playlist(&self, playlist_id: PlaylistId) -> fdo::Result<()> {
        self.imp.activate_playlist(playlist_id).await
    }

    async fn get_playlists(
        &self,
        index: u32,
        max_count: u32,
        order: PlaylistOrdering,
        reverse_order: bool,
    ) -> fdo::Result<Vec<Playlist>> {
        self.imp
            .get_playlists(index, max_count, order, reverse_order)
            .await
    }

    #[dbus_interface(signal)]
    async fn playlist_changed(ctxt: &SignalContext<'_>, playlist: Playlist) -> Result<()>;

    #[dbus_interface(property)]
    async fn playlist_count(&self) -> fdo::Result<u32> {
        self.imp.playlist_count().await
    }

    #[dbus_interface(property)]
    async fn orderings(&self) -> fdo::Result<Vec<PlaylistOrdering>> {
        self.imp.orderings().await
    }

    #[dbus_interface(property)]
    async fn active_playlist(&self) -> fdo::Result<MaybePlaylist> {
        self.imp.active_playlist().await
    }
}

/// Thin wrapper around [`zbus::Connection`] that serves the MPRIS interfaces.
pub struct Server<T>
where
    T: PlayerInterface + Send + Sync + 'static,
{
    connection: Connection,
    imp: Arc<T>,
}

impl<T> fmt::Debug for Server<T>
where
    T: PlayerInterface + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Server").finish()
    }
}

macro_rules! signal_delegate {
    ($iface:ty, $name:ident($($arg_name:ident: $arg_ty:ty),*)) => {
        pub async fn $name(&self, $($arg_name: $arg_ty),*) -> Result<()> {
            let iface_ref = self.interface_ref::<$iface>().await?;
            <$iface>::$name(iface_ref.signal_context(), $($arg_name),*).await
        }
    };
}

macro_rules! handle_property {
    ($property_ident:ident, $property_type:ident, $source:ident => $($map:ident, $property:ident, $getter:ident);*) => {
        match $property_ident {
            $(
                $property_type::$property => {
                    let value = Value::new($source.imp.$getter().await?);
                    $map.insert(stringify!($property), value);
                }
            )*
        }
    };
}

impl<T> Server<T>
where
    T: PlayerInterface + Send + Sync + 'static,
{
    pub async fn new(bus_name_suffix: &str, imp: T) -> Result<Self> {
        Self::new_inner(bus_name_suffix, imp, |builder, _| Ok(builder)).await
    }

    pub fn imp(&self) -> &T {
        &self.imp
    }

    signal_delegate!(RawPlayerInterface<T>, seeked(position: TimeInUs));

    /// Emits the `PropertiesChanged` signal for the given properties.
    pub async fn properties_changed(
        &self,
        properties: impl Into<BitFlags<Property>>,
    ) -> Result<()> {
        let mut root_changed = HashMap::new();
        let mut player_changed = HashMap::new();
        for property in properties.into().iter() {
            handle_property!(
                property, Property, self =>
                root_changed, CanQuit, can_quit;
                root_changed, Fullscreen, fullscreen;
                root_changed, CanSetFullscreen, can_set_fullscreen;
                root_changed, CanRaise, can_raise;
                root_changed, HasTrackList, has_track_list;
                root_changed, Identity, identity;
                root_changed, DesktopEntry, desktop_entry;
                root_changed, SupportedUriSchemes, supported_uri_schemes;
                root_changed, SupportedMimeTypes, supported_mime_types;
                player_changed, PlaybackStatus, playback_status;
                player_changed, LoopStatus, loop_status;
                player_changed, Rate, rate;
                player_changed, Shuffle, shuffle;
                player_changed, Metadata, metadata;
                player_changed, Volume, volume;
                player_changed, Position, position;
                player_changed, MinimumRate, minimum_rate;
                player_changed, MaximumRate, maximum_rate;
                player_changed, CanGoNext, can_go_next;
                player_changed, CanGoPrevious, can_go_previous;
                player_changed, CanPlay, can_play;
                player_changed, CanPause, can_pause;
                player_changed, CanSeek, can_seek;
                player_changed, CanControl, can_control
            );
        }

        if !root_changed.is_empty() {
            self.properties_changed_inner::<RawRootInterface<T>>(root_changed)
                .await?;
        }

        if !player_changed.is_empty() {
            self.properties_changed_inner::<RawPlayerInterface<T>>(player_changed)
                .await?;
        }

        Ok(())
    }

    async fn new_inner(
        bus_name_suffix: &str,
        imp: T,
        builder_ext_func: impl FnOnce(ConnectionBuilder<'_>, Arc<T>) -> Result<ConnectionBuilder<'_>>,
    ) -> Result<Self> {
        let bus_name =
            WellKnownName::try_from(format!("org.mpris.MediaPlayer2.{}", bus_name_suffix))?;
        let imp = Arc::new(imp);

        let connection_builder = ConnectionBuilder::session()?
            .name(bus_name)?
            .serve_at(
                OBJECT_PATH,
                RawRootInterface {
                    imp: Arc::clone(&imp),
                },
            )?
            .serve_at(
                OBJECT_PATH,
                RawPlayerInterface {
                    imp: Arc::clone(&imp),
                },
            )?;
        let connection = builder_ext_func(connection_builder, Arc::clone(&imp))?
            .build()
            .await?;

        Ok(Self { connection, imp })
    }

    async fn interface_ref<I: zbus::Interface>(&self) -> Result<InterfaceRef<I>> {
        self.connection
            .object_server()
            .interface::<_, I>(OBJECT_PATH)
            .await
    }

    async fn properties_changed_inner<I>(
        &self,
        changed_properties: HashMap<&str, Value<'_>>,
    ) -> Result<()>
    where
        I: Interface,
    {
        let iface_ref = self.interface_ref::<I>().await?;

        let ctxt = iface_ref.signal_context();
        ctxt.connection()
            .emit_signal(
                ctxt.destination(),
                ctxt.path(),
                fdo::Properties::name(),
                "PropertiesChanged",
                &(I::name(), changed_properties, &[] as &[&str]),
            )
            .await?;

        Ok(())
    }
}

impl<T> Server<T>
where
    T: TrackListInterface + Send + Sync + 'static,
{
    pub async fn new_with_track_list(bus_name_suffix: &str, imp: T) -> Result<Self> {
        Self::new_inner(bus_name_suffix, imp, |builder, imp| {
            builder.serve_at(
                OBJECT_PATH,
                RawTrackListInterface {
                    imp: Arc::clone(&imp),
                },
            )
        })
        .await
    }

    signal_delegate!(RawTrackListInterface<T>, track_list_replaced(tracks: Vec<TrackId>, current_track: TrackId));
    signal_delegate!(RawTrackListInterface<T>, track_added(metadata: Metadata, after_track: TrackId));
    signal_delegate!(RawTrackListInterface<T>, track_removed(track_id: TrackId));
    signal_delegate!(RawTrackListInterface<T>, track_metadata_changed(track_id: TrackId, metadata: Metadata));

    pub async fn track_list_properties_changed(
        &self,
        properties: impl Into<BitFlags<TrackListProperty>>,
    ) -> Result<()> {
        let mut changed = HashMap::new();
        for property in properties.into().iter() {
            handle_property!(
                property, TrackListProperty, self =>
                changed, Tracks, tracks;
                changed, CanEditTracks, can_edit_tracks
            );
        }

        if !changed.is_empty() {
            self.properties_changed_inner::<RawTrackListInterface<T>>(changed)
                .await?;
        }

        Ok(())
    }
}

impl<T> Server<T>
where
    T: PlaylistsInterface + Send + Sync + 'static,
{
    pub async fn new_with_playlists(bus_name_suffix: &str, imp: T) -> Result<Self> {
        Self::new_inner(bus_name_suffix, imp, |builder, imp| {
            builder.serve_at(
                OBJECT_PATH,
                RawPlaylistsInterface {
                    imp: Arc::clone(&imp),
                },
            )
        })
        .await
    }

    signal_delegate!(RawPlaylistsInterface<T>, playlist_changed(playlist: Playlist));

    pub async fn playlists_properties_changed(
        &self,
        properties: impl Into<BitFlags<PlaylistsProperty>>,
    ) -> Result<()> {
        let mut changed = HashMap::new();
        for property in properties.into().iter() {
            handle_property!(
                property, PlaylistsProperty, self =>
                changed, PlaylistCount, playlist_count;
                changed, Orderings, orderings;
                changed, ActivePlaylist, active_playlist
            );
        }

        if !changed.is_empty() {
            self.properties_changed_inner::<RawPlaylistsInterface<T>>(changed)
                .await?;
        }

        Ok(())
    }
}

impl<T> Server<T>
where
    T: TrackListInterface + PlaylistsInterface + Send + Sync + 'static,
{
    pub async fn new_with_all(bus_name_suffix: &str, imp: T) -> Result<Self> {
        Self::new_inner(bus_name_suffix, imp, |builder, imp| {
            builder
                .serve_at(
                    OBJECT_PATH,
                    RawTrackListInterface {
                        imp: Arc::clone(&imp),
                    },
                )?
                .serve_at(
                    OBJECT_PATH,
                    RawPlaylistsInterface {
                        imp: Arc::clone(&imp),
                    },
                )
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use static_assertions::assert_impl_all;

    use super::*;

    struct TestPlayer;

    #[async_trait]
    impl RootInterface for TestPlayer {
        async fn raise(&self) -> fdo::Result<()> {
            unreachable!()
        }

        async fn quit(&self) -> fdo::Result<()> {
            unreachable!()
        }

        async fn can_quit(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn fullscreen(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn set_fullscreen(&self, _fullscreen: bool) -> Result<()> {
            unreachable!()
        }

        async fn can_set_fullscreen(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn can_raise(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn has_track_list(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn identity(&self) -> fdo::Result<String> {
            unreachable!()
        }

        async fn desktop_entry(&self) -> fdo::Result<String> {
            unreachable!()
        }

        async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>> {
            unreachable!()
        }

        async fn supported_mime_types(&self) -> fdo::Result<Vec<String>> {
            unreachable!()
        }
    }

    #[async_trait]
    impl PlayerInterface for TestPlayer {
        async fn next(&self) -> fdo::Result<()> {
            unreachable!()
        }

        async fn previous(&self) -> fdo::Result<()> {
            unreachable!()
        }

        async fn pause(&self) -> fdo::Result<()> {
            unreachable!()
        }

        async fn play_pause(&self) -> fdo::Result<()> {
            unreachable!()
        }

        async fn stop(&self) -> fdo::Result<()> {
            unreachable!()
        }

        async fn play(&self) -> fdo::Result<()> {
            unreachable!()
        }

        async fn seek(&self, _offset: TimeInUs) -> fdo::Result<()> {
            unreachable!()
        }

        async fn set_position(&self, _track_id: TrackId, _position: TimeInUs) -> fdo::Result<()> {
            unreachable!()
        }

        async fn open_uri(&self, _uri: String) -> fdo::Result<()> {
            unreachable!()
        }

        async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
            unreachable!()
        }

        async fn loop_status(&self) -> fdo::Result<LoopStatus> {
            unreachable!()
        }

        async fn set_loop_status(&self, _loop_status: LoopStatus) -> Result<()> {
            unreachable!()
        }

        async fn rate(&self) -> fdo::Result<PlaybackRate> {
            unreachable!()
        }

        async fn set_rate(&self, _rate: PlaybackRate) -> Result<()> {
            unreachable!()
        }

        async fn shuffle(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn set_shuffle(&self, _shuffle: bool) -> Result<()> {
            unreachable!()
        }

        async fn metadata(&self) -> fdo::Result<Metadata> {
            unreachable!()
        }

        async fn volume(&self) -> fdo::Result<Volume> {
            unreachable!()
        }

        async fn set_volume(&self, _volume: Volume) -> Result<()> {
            unreachable!()
        }

        async fn position(&self) -> fdo::Result<TimeInUs> {
            unreachable!()
        }

        async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
            unreachable!()
        }

        async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
            unreachable!()
        }

        async fn can_go_next(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn can_go_previous(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn can_play(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn can_pause(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn can_seek(&self) -> fdo::Result<bool> {
            unreachable!()
        }

        async fn can_control(&self) -> fdo::Result<bool> {
            unreachable!()
        }
    }

    #[async_trait]
    impl TrackListInterface for TestPlayer {
        async fn get_tracks_metadata(
            &self,
            _track_ids: Vec<TrackId>,
        ) -> fdo::Result<Vec<Metadata>> {
            unreachable!()
        }

        async fn add_track(
            &self,
            _uri: Uri,
            _after_track: TrackId,
            _set_as_current: bool,
        ) -> fdo::Result<()> {
            unreachable!()
        }

        async fn remove_track(&self, _track_id: TrackId) -> fdo::Result<()> {
            unreachable!()
        }

        async fn go_to(&self, _track_id: TrackId) -> fdo::Result<()> {
            unreachable!()
        }

        async fn tracks(&self) -> fdo::Result<Vec<TrackId>> {
            unreachable!()
        }

        async fn can_edit_tracks(&self) -> fdo::Result<bool> {
            unreachable!()
        }
    }

    #[async_trait]
    impl PlaylistsInterface for TestPlayer {
        async fn activate_playlist(&self, _playlist_id: PlaylistId) -> fdo::Result<()> {
            unreachable!()
        }

        async fn get_playlists(
            &self,
            _index: u32,
            _max_count: u32,
            _order: PlaylistOrdering,
            _reverse_order: bool,
        ) -> fdo::Result<Vec<Playlist>> {
            unreachable!()
        }

        async fn playlist_count(&self) -> fdo::Result<u32> {
            unreachable!()
        }

        async fn orderings(&self) -> fdo::Result<Vec<PlaylistOrdering>> {
            unreachable!()
        }

        async fn active_playlist(&self) -> fdo::Result<MaybePlaylist> {
            unreachable!()
        }
    }

    assert_impl_all!(Server<TestPlayer>: Send, Sync, Unpin);
}