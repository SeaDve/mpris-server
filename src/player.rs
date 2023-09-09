use std::cell::{Cell, RefCell};

use zbus::Result;

use crate::{
    LoopStatus, Metadata, PlaybackRate, PlaybackStatus, PlayerInterface, RootInterface, Server,
    TimeInUs, TrackId, Volume,
};

/// Premade mutable object that internally implements [`RootInterface`] and [`PlayerInterface`].
#[derive(Debug)]
pub struct Player {
    server: Server<Inner>,
}

#[allow(clippy::type_complexity)]
struct Inner {
    raise_cbs: RefCell<Vec<Box<dyn Fn()>>>,
    quit_cbs: RefCell<Vec<Box<dyn Fn()>>>,
    can_quit: Cell<bool>,
    fullscreen: Cell<bool>,
    can_set_fullscreen: Cell<bool>,
    can_raise: Cell<bool>,
    has_track_list: Cell<bool>,
    identity: RefCell<String>,
    desktop_entry: RefCell<String>,
    supported_uri_schemes: RefCell<Vec<String>>,
    supported_mime_types: RefCell<Vec<String>>,

    next_cbs: RefCell<Vec<Box<dyn Fn()>>>,
    previous_cbs: RefCell<Vec<Box<dyn Fn()>>>,
    pause_cbs: RefCell<Vec<Box<dyn Fn()>>>,
    play_pause_cbs: RefCell<Vec<Box<dyn Fn()>>>,
    stop_cbs: RefCell<Vec<Box<dyn Fn()>>>,
    play_cbs: RefCell<Vec<Box<dyn Fn()>>>,
    seek_cbs: RefCell<Vec<Box<dyn Fn(TimeInUs)>>>,
    set_position_cbs: RefCell<Vec<Box<dyn Fn(&TrackId, TimeInUs)>>>,
    open_uri_cbs: RefCell<Vec<Box<dyn Fn(&str)>>>,
    playback_status: Cell<PlaybackStatus>,
    loop_status: Cell<LoopStatus>,
    rate: Cell<PlaybackRate>,
    shuffle: Cell<bool>,
    metadata: RefCell<Metadata>,
    volume: Cell<Volume>,
    position: Cell<TimeInUs>,
    minimum_rate: Cell<PlaybackRate>,
    maximum_rate: Cell<PlaybackRate>,
    can_go_next: Cell<bool>,
    can_go_previous: Cell<bool>,
    can_play: Cell<bool>,
    can_pause: Cell<bool>,
    can_seek: Cell<bool>,
    can_control: Cell<bool>,
}

impl Player {
    pub fn new(bus_name_suffix: &str) -> Result<Self> {
        let server = Server::new(
            bus_name_suffix,
            Inner {
                raise_cbs: RefCell::new(Vec::new()),
                quit_cbs: RefCell::new(Vec::new()),
                can_quit: Cell::new(false),
                fullscreen: Cell::new(false),
                can_set_fullscreen: Cell::new(false),
                can_raise: Cell::new(false),
                has_track_list: Cell::new(false),
                identity: RefCell::new(String::new()),
                desktop_entry: RefCell::new(String::new()),
                supported_uri_schemes: RefCell::new(Vec::new()),
                supported_mime_types: RefCell::new(Vec::new()),
                next_cbs: RefCell::new(Vec::new()),
                previous_cbs: RefCell::new(Vec::new()),
                pause_cbs: RefCell::new(Vec::new()),
                play_pause_cbs: RefCell::new(Vec::new()),
                stop_cbs: RefCell::new(Vec::new()),
                play_cbs: RefCell::new(Vec::new()),
                seek_cbs: RefCell::new(Vec::new()),
                set_position_cbs: RefCell::new(Vec::new()),
                open_uri_cbs: RefCell::new(Vec::new()),
                playback_status: Cell::new(PlaybackStatus::Stopped),
                loop_status: Cell::new(LoopStatus::None),
                rate: Cell::new(1.0),
                shuffle: Cell::new(false),
                metadata: RefCell::new(Metadata::new()),
                volume: Cell::new(1.0),
                position: Cell::new(0),
                minimum_rate: Cell::new(1.0),
                maximum_rate: Cell::new(1.0),
                can_go_next: Cell::new(false),
                can_go_previous: Cell::new(false),
                can_play: Cell::new(false),
                can_pause: Cell::new(false),
                can_seek: Cell::new(false),
                can_control: Cell::new(false),
            },
        )?;

        Ok(Self { server })
    }

    pub async fn run(&self) -> Result<()> {
        self.server.run().await
    }

    pub fn connect_raise(&self, cb: impl Fn() + 'static) {
        self.server.imp().raise_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_quit(&self, cb: impl Fn() + 'static) {
        self.server.imp().quit_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn can_quit(&self) -> bool {
        self.server.imp().can_quit.get()
    }

    pub async fn set_can_quit(&self, can_quit: bool) -> Result<()> {
        self.server.imp().can_quit.set(can_quit);
        self.server.can_quit_changed().await
    }

    pub fn fullscreen(&self) -> bool {
        self.server.imp().fullscreen.get()
    }

    pub async fn set_fullscreen(&self, fullscreen: bool) -> Result<()> {
        self.server.imp().fullscreen.set(fullscreen);
        self.server.fullscreen_changed().await
    }

    pub fn can_set_fullscreen(&self) -> bool {
        self.server.imp().can_set_fullscreen.get()
    }

    pub async fn set_can_set_fullscreen(&self, can_set_fullscreen: bool) -> Result<()> {
        self.server.imp().can_set_fullscreen.set(can_set_fullscreen);
        self.server.can_set_fullscreen_changed().await
    }

    pub fn can_raise(&self) -> bool {
        self.server.imp().can_raise.get()
    }

    pub async fn set_can_raise(&self, can_raise: bool) -> Result<()> {
        self.server.imp().can_raise.set(can_raise);
        self.server.can_raise_changed().await
    }

    pub fn has_track_list(&self) -> bool {
        self.server.imp().has_track_list.get()
    }

    pub async fn set_has_track_list(&self, has_track_list: bool) -> Result<()> {
        self.server.imp().has_track_list.set(has_track_list);
        self.server.has_track_list_changed().await
    }

    pub fn identity(&self) -> String {
        self.server.imp().identity.borrow().clone()
    }

    pub async fn set_identity(&self, identity: String) -> Result<()> {
        self.server.imp().identity.replace(identity);
        self.server.identity_changed().await
    }

    pub fn desktop_entry(&self) -> String {
        self.server.imp().desktop_entry.borrow().clone()
    }

    pub async fn set_desktop_entry(&self, desktop_entry: String) -> Result<()> {
        self.server.imp().desktop_entry.replace(desktop_entry);
        self.server.desktop_entry_changed().await
    }

    pub fn supported_uri_schemes(&self) -> Vec<String> {
        self.server.imp().supported_uri_schemes.borrow().clone()
    }

    pub async fn set_supported_uri_schemes(
        &self,
        supported_uri_schemes: Vec<String>,
    ) -> Result<()> {
        self.server
            .imp()
            .supported_uri_schemes
            .replace(supported_uri_schemes);
        self.server.supported_uri_schemes_changed().await
    }

    pub fn supported_mime_types(&self) -> Vec<String> {
        self.server.imp().supported_mime_types.borrow().clone()
    }

    pub async fn set_supported_mime_types(&self, supported_mime_types: Vec<String>) -> Result<()> {
        self.server
            .imp()
            .supported_mime_types
            .replace(supported_mime_types);
        self.server.supported_mime_types_changed().await
    }

    pub fn connect_next(&self, cb: impl Fn() + 'static) {
        self.server.imp().next_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_previous(&self, cb: impl Fn() + 'static) {
        self.server
            .imp()
            .previous_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn connect_pause(&self, cb: impl Fn() + 'static) {
        self.server.imp().pause_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_play_pause(&self, cb: impl Fn() + 'static) {
        self.server
            .imp()
            .play_pause_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn connect_stop(&self, cb: impl Fn() + 'static) {
        self.server.imp().stop_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_play(&self, cb: impl Fn() + 'static) {
        self.server.imp().play_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_seek(&self, cb: impl Fn(TimeInUs) + 'static) {
        self.server.imp().seek_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_set_position(&self, cb: impl Fn(&TrackId, TimeInUs) + 'static) {
        self.server
            .imp()
            .set_position_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn connect_open_uri(&self, cb: impl Fn(&str) + 'static) {
        self.server
            .imp()
            .open_uri_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub async fn emit_seeked(&self, position: TimeInUs) -> Result<()> {
        self.server.seeked(position).await
    }

    pub fn playback_status(&self) -> PlaybackStatus {
        self.server.imp().playback_status.get()
    }

    pub async fn set_playback_status(&self, playback_status: PlaybackStatus) -> Result<()> {
        self.server.imp().playback_status.set(playback_status);
        self.server.playback_status_changed().await
    }

    pub fn loop_status(&self) -> LoopStatus {
        self.server.imp().loop_status.get()
    }

    pub async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()> {
        self.server.imp().loop_status.set(loop_status);
        self.server.loop_status_changed().await
    }

    pub fn rate(&self) -> PlaybackRate {
        self.server.imp().rate.get()
    }

    pub async fn set_rate(&self, rate: PlaybackRate) -> Result<()> {
        self.server.imp().rate.set(rate);
        self.server.rate_changed().await
    }

    pub fn shuffle(&self) -> bool {
        self.server.imp().shuffle.get()
    }

    pub async fn set_shuffle(&self, shuffle: bool) -> Result<()> {
        self.server.imp().shuffle.set(shuffle);
        self.server.shuffle_changed().await
    }

    pub fn metadata(&self) -> Metadata {
        self.server.imp().metadata.borrow().clone()
    }

    pub async fn set_metadata(&self, metadata: Metadata) -> Result<()> {
        self.server.imp().metadata.replace(metadata);
        self.server.metadata_changed().await
    }

    pub fn volume(&self) -> Volume {
        self.server.imp().volume.get()
    }

    pub async fn set_volume(&self, volume: Volume) -> Result<()> {
        self.server.imp().volume.set(volume);
        self.server.volume_changed().await
    }

    pub fn position(&self) -> TimeInUs {
        self.server.imp().position.get()
    }

    pub async fn set_position(&self, position: TimeInUs) -> Result<()> {
        self.server.imp().position.set(position);
        self.server.position_changed().await
    }

    pub fn minimum_rate(&self) -> PlaybackRate {
        self.server.imp().minimum_rate.get()
    }

    pub async fn set_minimum_rate(&self, minimum_rate: PlaybackRate) -> Result<()> {
        self.server.imp().minimum_rate.set(minimum_rate);
        self.server.minimum_rate_changed().await
    }

    pub fn maximum_rate(&self) -> PlaybackRate {
        self.server.imp().maximum_rate.get()
    }

    pub async fn set_maximum_rate(&self, maximum_rate: PlaybackRate) -> Result<()> {
        self.server.imp().maximum_rate.set(maximum_rate);
        self.server.maximum_rate_changed().await
    }

    pub fn can_go_next(&self) -> bool {
        self.server.imp().can_go_next.get()
    }

    pub async fn set_can_go_next(&self, can_go_next: bool) -> Result<()> {
        self.server.imp().can_go_next.set(can_go_next);
        self.server.can_go_next_changed().await
    }

    pub fn can_go_previous(&self) -> bool {
        self.server.imp().can_go_previous.get()
    }

    pub async fn set_can_go_previous(&self, can_go_previous: bool) -> Result<()> {
        self.server.imp().can_go_previous.set(can_go_previous);
        self.server.can_go_previous_changed().await
    }

    pub fn can_play(&self) -> bool {
        self.server.imp().can_play.get()
    }

    pub async fn set_can_play(&self, can_play: bool) -> Result<()> {
        self.server.imp().can_play.set(can_play);
        self.server.can_play_changed().await
    }

    pub fn can_pause(&self) -> bool {
        self.server.imp().can_pause.get()
    }

    pub async fn set_can_pause(&self, can_pause: bool) -> Result<()> {
        self.server.imp().can_pause.set(can_pause);
        self.server.can_pause_changed().await
    }

    pub fn can_seek(&self) -> bool {
        self.server.imp().can_seek.get()
    }

    pub async fn set_can_seek(&self, can_seek: bool) -> Result<()> {
        self.server.imp().can_seek.set(can_seek);
        self.server.can_seek_changed().await
    }

    pub fn can_control(&self) -> bool {
        self.server.imp().can_control.get()
    }

    pub async fn set_can_control(&self, can_control: bool) -> Result<()> {
        self.server.imp().can_control.set(can_control);
        self.server.can_control_changed().await
    }
}

impl RootInterface for Inner {
    fn raise(&self) {
        for cb in self.raise_cbs.borrow().iter() {
            cb();
        }
    }

    fn quit(&self) {
        for cb in self.quit_cbs.borrow().iter() {
            cb();
        }
    }

    fn can_quit(&self) -> bool {
        self.can_quit.get()
    }

    fn fullscreen(&self) -> bool {
        self.fullscreen.get()
    }

    fn set_fullscreen(&self, fullscreen: bool) {
        self.fullscreen.set(fullscreen);
    }

    fn can_set_fullscreen(&self) -> bool {
        self.can_set_fullscreen.get()
    }

    fn can_raise(&self) -> bool {
        self.can_raise.get()
    }

    fn has_track_list(&self) -> bool {
        self.has_track_list.get()
    }

    fn identity(&self) -> String {
        self.identity.borrow().clone()
    }

    fn desktop_entry(&self) -> String {
        self.desktop_entry.borrow().clone()
    }

    fn supported_uri_schemes(&self) -> Vec<String> {
        self.supported_uri_schemes.borrow().clone()
    }

    fn supported_mime_types(&self) -> Vec<String> {
        self.supported_mime_types.borrow().clone()
    }
}

impl PlayerInterface for Inner {
    fn next(&self) {
        for cb in self.next_cbs.borrow().iter() {
            cb();
        }
    }

    fn previous(&self) {
        for cb in self.previous_cbs.borrow().iter() {
            cb();
        }
    }

    fn pause(&self) {
        for cb in self.pause_cbs.borrow().iter() {
            cb();
        }
    }

    fn play_pause(&self) {
        for cb in self.play_pause_cbs.borrow().iter() {
            cb();
        }
    }

    fn stop(&self) {
        for cb in self.stop_cbs.borrow().iter() {
            cb();
        }
    }

    fn play(&self) {
        for cb in self.play_cbs.borrow().iter() {
            cb();
        }
    }

    fn seek(&self, offset: TimeInUs) {
        for cb in self.seek_cbs.borrow().iter() {
            cb(offset);
        }
    }

    fn set_position(&self, track_id: TrackId, position: TimeInUs) {
        for cb in self.set_position_cbs.borrow().iter() {
            cb(&track_id, position);
        }
    }

    fn open_uri(&self, uri: String) {
        for cb in self.open_uri_cbs.borrow().iter() {
            cb(&uri);
        }
    }

    fn playback_status(&self) -> PlaybackStatus {
        self.playback_status.get()
    }

    fn loop_status(&self) -> LoopStatus {
        self.loop_status.get()
    }

    fn set_loop_status(&self, loop_status: LoopStatus) {
        self.loop_status.set(loop_status);
    }

    fn rate(&self) -> PlaybackRate {
        self.rate.get()
    }

    fn set_rate(&self, rate: PlaybackRate) {
        self.rate.set(rate);
    }

    fn shuffle(&self) -> bool {
        self.shuffle.get()
    }

    fn set_shuffle(&self, shuffle: bool) {
        self.shuffle.set(shuffle);
    }

    fn metadata(&self) -> Metadata {
        self.metadata.borrow().clone()
    }

    fn volume(&self) -> Volume {
        self.volume.get()
    }

    fn set_volume(&self, volume: Volume) {
        self.volume.set(volume);
    }

    fn position(&self) -> TimeInUs {
        self.position.get()
    }

    fn minimum_rate(&self) -> PlaybackRate {
        self.minimum_rate.get()
    }

    fn maximum_rate(&self) -> PlaybackRate {
        self.maximum_rate.get()
    }

    fn can_go_next(&self) -> bool {
        self.can_go_next.get()
    }

    fn can_go_previous(&self) -> bool {
        self.can_go_previous.get()
    }

    fn can_play(&self) -> bool {
        self.can_play.get()
    }

    fn can_pause(&self) -> bool {
        self.can_pause.get()
    }

    fn can_seek(&self) -> bool {
        self.can_seek.get()
    }

    fn can_control(&self) -> bool {
        self.can_control.get()
    }
}
