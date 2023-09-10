use futures_channel::{mpsc, oneshot};
use futures_util::StreamExt;
use zbus::{dbus_interface, ConnectionBuilder, Result, SignalContext};

use super::{
    player::{RawPlayerInterface, RawRootInterface},
    utils::{changed_delegate, signal_delegate},
    Action, Server, OBJECT_PATH,
};
use crate::{Metadata, TrackId, TrackListInterface, Uri};

/// `org.mpris.MediaPlayer2.TrackList` Actions
pub(super) enum TrackListAction {
    // Methods
    GetTracksMetadata(Vec<TrackId>, oneshot::Sender<Vec<Metadata>>),
    AddTrack(Uri, TrackId, bool),
    RemoveTrack(TrackId),
    GoTo(TrackId),

    // Properties
    Tracks(oneshot::Sender<Vec<TrackId>>),
    CanEditTracks(oneshot::Sender<bool>),
}

pub(super) struct RawTrackListInterface {
    pub(super) tx: mpsc::UnboundedSender<Action>,
}

impl RawTrackListInterface {
    fn send(&self, action: TrackListAction) {
        self.tx.unbounded_send(Action::TrackList(action)).unwrap();
    }
}

#[dbus_interface(name = "org.mpris.MediaPlayer2.TrackList")]
impl RawTrackListInterface {
    async fn get_tracks_metadata(&self, track_ids: Vec<TrackId>) -> Vec<Metadata> {
        let (tx, rx) = oneshot::channel();
        self.send(TrackListAction::GetTracksMetadata(track_ids, tx));
        rx.await.unwrap()
    }

    fn add_track(&self, uri: Uri, after_track: TrackId, set_as_current: bool) {
        self.send(TrackListAction::AddTrack(uri, after_track, set_as_current));
    }

    fn remove_track(&self, track_id: TrackId) {
        self.send(TrackListAction::RemoveTrack(track_id));
    }

    fn go_to(&self, track_id: TrackId) {
        self.send(TrackListAction::GoTo(track_id));
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
    async fn tracks(&self) -> Vec<TrackId> {
        let (tx, rx) = oneshot::channel();
        self.send(TrackListAction::Tracks(tx));
        rx.await.unwrap()
    }

    #[dbus_interface(property)]
    async fn can_edit_tracks(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.send(TrackListAction::CanEditTracks(tx));
        rx.await.unwrap()
    }
}

impl<T> Server<T>
where
    T: TrackListInterface + 'static,
{
    pub async fn run_with_track_list(&self) -> Result<()> {
        let (tx, mut rx) = mpsc::unbounded::<Action>();

        let connection = ConnectionBuilder::session()?
            .name(&self.bus_name)?
            .serve_at(OBJECT_PATH, RawRootInterface { tx: tx.clone() })?
            .serve_at(OBJECT_PATH, RawPlayerInterface { tx: tx.clone() })?
            .serve_at(OBJECT_PATH, RawTrackListInterface { tx })?
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
                Action::Playlists(_) => unreachable!(),
            }
        }

        Ok(())
    }

    pub(super) async fn handle_track_list_interface_action(&self, action: TrackListAction) {
        match action {
            TrackListAction::GetTracksMetadata(track_ids, sender) => sender
                .send(self.imp.get_tracks_metadata(track_ids).await)
                .unwrap(),
            TrackListAction::AddTrack(uri, after_track, set_as_current) => {
                self.imp.add_track(uri, after_track, set_as_current).await
            }
            TrackListAction::RemoveTrack(track_id) => self.imp.remove_track(track_id).await,
            TrackListAction::GoTo(track_id) => self.imp.go_to(track_id).await,
            TrackListAction::Tracks(sender) => sender.send(self.imp.tracks().await).unwrap(),
            TrackListAction::CanEditTracks(sender) => {
                sender.send(self.imp.can_edit_tracks().await).unwrap()
            }
        }
    }

    // org.mpris.MediaPlayer2.TrackList
    signal_delegate!(RawTrackListInterface, track_list_replaced(tracks: Vec<TrackId>, current_track: TrackId));
    signal_delegate!(RawTrackListInterface, track_added(metadata: Metadata, after_track: TrackId));
    signal_delegate!(RawTrackListInterface, track_removed(track_id: TrackId));
    signal_delegate!(RawTrackListInterface, track_metadata_changed(track_id: TrackId, metadata: Metadata));
    changed_delegate!(RawTrackListInterface, tracks_changed);
    changed_delegate!(RawTrackListInterface, can_edit_tracks_changed);
}