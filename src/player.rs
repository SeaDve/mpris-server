use std::cell::{Cell, Ref, RefCell};

use async_trait::async_trait;
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
    set_fullscreen_cbs: RefCell<Vec<Box<dyn Fn(bool)>>>, // Property
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
    set_loop_status_cbs: RefCell<Vec<Box<dyn Fn(LoopStatus)>>>, // Property
    set_rate_cbs: RefCell<Vec<Box<dyn Fn(PlaybackRate)>>>,      // Property
    set_shuffle_cbs: RefCell<Vec<Box<dyn Fn(bool)>>>,           // Property
    set_volume_cbs: RefCell<Vec<Box<dyn Fn(Volume)>>>,          // Property
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

#[async_trait(?Send)]
impl RootInterface for Inner {
    async fn raise(&self) {
        for cb in self.raise_cbs.borrow().iter() {
            cb();
        }
    }

    async fn quit(&self) {
        for cb in self.quit_cbs.borrow().iter() {
            cb();
        }
    }

    async fn can_quit(&self) -> bool {
        self.can_quit.get()
    }

    async fn fullscreen(&self) -> bool {
        self.fullscreen.get()
    }

    async fn set_fullscreen(&self, fullscreen: bool) {
        for cb in self.set_fullscreen_cbs.borrow().iter() {
            cb(fullscreen);
        }
    }

    async fn can_set_fullscreen(&self) -> bool {
        self.can_set_fullscreen.get()
    }

    async fn can_raise(&self) -> bool {
        self.can_raise.get()
    }

    async fn has_track_list(&self) -> bool {
        self.has_track_list.get()
    }

    async fn identity(&self) -> String {
        self.identity.borrow().clone()
    }

    async fn desktop_entry(&self) -> String {
        self.desktop_entry.borrow().clone()
    }

    async fn supported_uri_schemes(&self) -> Vec<String> {
        self.supported_uri_schemes.borrow().clone()
    }

    async fn supported_mime_types(&self) -> Vec<String> {
        self.supported_mime_types.borrow().clone()
    }
}

#[async_trait(?Send)]
impl PlayerInterface for Inner {
    async fn next(&self) {
        for cb in self.next_cbs.borrow().iter() {
            cb();
        }
    }

    async fn previous(&self) {
        for cb in self.previous_cbs.borrow().iter() {
            cb();
        }
    }

    async fn pause(&self) {
        for cb in self.pause_cbs.borrow().iter() {
            cb();
        }
    }

    async fn play_pause(&self) {
        for cb in self.play_pause_cbs.borrow().iter() {
            cb();
        }
    }

    async fn stop(&self) {
        for cb in self.stop_cbs.borrow().iter() {
            cb();
        }
    }

    async fn play(&self) {
        for cb in self.play_cbs.borrow().iter() {
            cb();
        }
    }

    async fn seek(&self, offset: TimeInUs) {
        for cb in self.seek_cbs.borrow().iter() {
            cb(offset);
        }
    }

    async fn set_position(&self, track_id: TrackId, position: TimeInUs) {
        for cb in self.set_position_cbs.borrow().iter() {
            cb(&track_id, position);
        }
    }

    async fn open_uri(&self, uri: String) {
        for cb in self.open_uri_cbs.borrow().iter() {
            cb(&uri);
        }
    }

    async fn playback_status(&self) -> PlaybackStatus {
        self.playback_status.get()
    }

    async fn loop_status(&self) -> LoopStatus {
        self.loop_status.get()
    }

    async fn set_loop_status(&self, loop_status: LoopStatus) {
        for cb in self.set_loop_status_cbs.borrow().iter() {
            cb(loop_status);
        }
    }

    async fn rate(&self) -> PlaybackRate {
        self.rate.get()
    }

    async fn set_rate(&self, rate: PlaybackRate) {
        for cb in self.set_rate_cbs.borrow().iter() {
            cb(rate);
        }
    }

    async fn shuffle(&self) -> bool {
        self.shuffle.get()
    }

    async fn set_shuffle(&self, shuffle: bool) {
        for cb in self.set_shuffle_cbs.borrow().iter() {
            cb(shuffle);
        }
    }

    async fn metadata(&self) -> Metadata {
        self.metadata.borrow().clone()
    }

    async fn volume(&self) -> Volume {
        self.volume.get()
    }

    async fn set_volume(&self, volume: Volume) {
        for cb in self.set_volume_cbs.borrow().iter() {
            cb(volume);
        }
    }

    async fn position(&self) -> TimeInUs {
        self.position.get()
    }

    async fn minimum_rate(&self) -> PlaybackRate {
        self.minimum_rate.get()
    }

    async fn maximum_rate(&self) -> PlaybackRate {
        self.maximum_rate.get()
    }

    async fn can_go_next(&self) -> bool {
        self.can_go_next.get()
    }

    async fn can_go_previous(&self) -> bool {
        self.can_go_previous.get()
    }

    async fn can_play(&self) -> bool {
        self.can_play.get()
    }

    async fn can_pause(&self) -> bool {
        self.can_pause.get()
    }

    async fn can_seek(&self) -> bool {
        self.can_seek.get()
    }

    async fn can_control(&self) -> bool {
        self.can_control.get()
    }
}

impl Player {
    pub fn builder(bus_name_suffix: &str) -> PlayerBuilder {
        PlayerBuilder {
            bus_name_suffix: bus_name_suffix.to_string(),
            can_quit: false,
            fullscreen: false,
            can_set_fullscreen: false,
            can_raise: false,
            has_track_list: false,
            identity: String::new(),
            desktop_entry: String::new(),
            supported_uri_schemes: Vec::new(),
            supported_mime_types: Vec::new(),
            playback_status: PlaybackStatus::Stopped,
            loop_status: LoopStatus::None,
            rate: 1.0,
            shuffle: false,
            metadata: Metadata::new(),
            volume: 1.0,
            position: 0,
            minimum_rate: 1.0,
            maximum_rate: 1.0,
            can_go_next: false,
            can_go_previous: false,
            can_play: false,
            can_pause: false,
            can_seek: false,
            can_control: true,
        }
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
        if self.can_quit() == can_quit {
            return Ok(());
        }

        self.server.imp().can_quit.set(can_quit);
        self.server.can_quit_changed().await
    }

    pub fn fullscreen(&self) -> bool {
        self.server.imp().fullscreen.get()
    }

    pub async fn set_fullscreen(&self, fullscreen: bool) -> Result<()> {
        if self.fullscreen() == fullscreen {
            return Ok(());
        }

        self.server.imp().fullscreen.set(fullscreen);
        self.server.fullscreen_changed().await
    }

    pub fn can_set_fullscreen(&self) -> bool {
        self.server.imp().can_set_fullscreen.get()
    }

    pub async fn set_can_set_fullscreen(&self, can_set_fullscreen: bool) -> Result<()> {
        if self.can_set_fullscreen() == can_set_fullscreen {
            return Ok(());
        }

        self.server.imp().can_set_fullscreen.set(can_set_fullscreen);
        self.server.can_set_fullscreen_changed().await
    }

    pub fn can_raise(&self) -> bool {
        self.server.imp().can_raise.get()
    }

    pub async fn set_can_raise(&self, can_raise: bool) -> Result<()> {
        if self.can_raise() == can_raise {
            return Ok(());
        }

        self.server.imp().can_raise.set(can_raise);
        self.server.can_raise_changed().await
    }

    pub fn has_track_list(&self) -> bool {
        self.server.imp().has_track_list.get()
    }

    pub async fn set_has_track_list(&self, has_track_list: bool) -> Result<()> {
        if self.has_track_list() == has_track_list {
            return Ok(());
        }

        self.server.imp().has_track_list.set(has_track_list);
        self.server.has_track_list_changed().await
    }

    pub fn identity(&self) -> Ref<'_, String> {
        self.server.imp().identity.borrow()
    }

    pub async fn set_identity(&self, identity: impl Into<String>) -> Result<()> {
        let identity = identity.into();

        if *self.identity() == identity {
            return Ok(());
        }

        self.server.imp().identity.replace(identity);
        self.server.identity_changed().await
    }

    pub fn desktop_entry(&self) -> Ref<'_, String> {
        self.server.imp().desktop_entry.borrow()
    }

    pub async fn set_desktop_entry(&self, desktop_entry: impl Into<String>) -> Result<()> {
        let desktop_entry = desktop_entry.into();

        if *self.desktop_entry() == desktop_entry {
            return Ok(());
        }

        self.server.imp().desktop_entry.replace(desktop_entry);
        self.server.desktop_entry_changed().await
    }

    pub fn supported_uri_schemes(&self) -> Ref<'_, Vec<String>> {
        self.server.imp().supported_uri_schemes.borrow()
    }

    pub async fn set_supported_uri_schemes(
        &self,
        supported_uri_schemes: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<()> {
        let supported_uri_schemes = supported_uri_schemes
            .into_iter()
            .map(|i| i.into())
            .collect();

        if *self.supported_uri_schemes() == supported_uri_schemes {
            return Ok(());
        }

        self.server
            .imp()
            .supported_uri_schemes
            .replace(supported_uri_schemes);
        self.server.supported_uri_schemes_changed().await
    }

    pub fn supported_mime_types(&self) -> Ref<'_, Vec<String>> {
        self.server.imp().supported_mime_types.borrow()
    }

    pub async fn set_supported_mime_types(
        &self,
        supported_mime_types: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<()> {
        let supported_mime_types = supported_mime_types.into_iter().map(|i| i.into()).collect();

        if *self.supported_mime_types() == supported_mime_types {
            return Ok(());
        }

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
        if self.playback_status() == playback_status {
            return Ok(());
        }

        self.server.imp().playback_status.set(playback_status);
        self.server.playback_status_changed().await
    }

    pub fn loop_status(&self) -> LoopStatus {
        self.server.imp().loop_status.get()
    }

    pub async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()> {
        if self.loop_status() == loop_status {
            return Ok(());
        }

        self.server.imp().loop_status.set(loop_status);
        self.server.loop_status_changed().await
    }

    pub fn rate(&self) -> PlaybackRate {
        self.server.imp().rate.get()
    }

    pub async fn set_rate(&self, rate: PlaybackRate) -> Result<()> {
        if self.rate() == rate {
            return Ok(());
        }

        self.server.imp().rate.set(rate);
        self.server.rate_changed().await
    }

    pub fn shuffle(&self) -> bool {
        self.server.imp().shuffle.get()
    }

    pub async fn set_shuffle(&self, shuffle: bool) -> Result<()> {
        if self.shuffle() == shuffle {
            return Ok(());
        }

        self.server.imp().shuffle.set(shuffle);
        self.server.shuffle_changed().await
    }

    pub fn metadata(&self) -> Ref<'_, Metadata> {
        self.server.imp().metadata.borrow()
    }

    pub async fn set_metadata(&self, metadata: Metadata) -> Result<()> {
        if *self.metadata() == metadata {
            return Ok(());
        }

        self.server.imp().metadata.replace(metadata);
        self.server.metadata_changed().await
    }

    pub fn volume(&self) -> Volume {
        self.server.imp().volume.get()
    }

    pub async fn set_volume(&self, volume: Volume) -> Result<()> {
        if self.volume() == volume {
            return Ok(());
        }

        self.server.imp().volume.set(volume);
        self.server.volume_changed().await
    }

    pub fn position(&self) -> TimeInUs {
        self.server.imp().position.get()
    }

    pub async fn set_position(&self, position: TimeInUs) -> Result<()> {
        if self.position() == position {
            return Ok(());
        }

        self.server.imp().position.set(position);
        self.server.position_changed().await
    }

    pub fn minimum_rate(&self) -> PlaybackRate {
        self.server.imp().minimum_rate.get()
    }

    pub async fn set_minimum_rate(&self, minimum_rate: PlaybackRate) -> Result<()> {
        if self.minimum_rate() == minimum_rate {
            return Ok(());
        }

        self.server.imp().minimum_rate.set(minimum_rate);
        self.server.minimum_rate_changed().await
    }

    pub fn maximum_rate(&self) -> PlaybackRate {
        self.server.imp().maximum_rate.get()
    }

    pub async fn set_maximum_rate(&self, maximum_rate: PlaybackRate) -> Result<()> {
        if self.maximum_rate() == maximum_rate {
            return Ok(());
        }

        self.server.imp().maximum_rate.set(maximum_rate);
        self.server.maximum_rate_changed().await
    }

    pub fn can_go_next(&self) -> bool {
        self.server.imp().can_go_next.get()
    }

    pub async fn set_can_go_next(&self, can_go_next: bool) -> Result<()> {
        if self.can_go_next() == can_go_next {
            return Ok(());
        }

        self.server.imp().can_go_next.set(can_go_next);
        self.server.can_go_next_changed().await
    }

    pub fn can_go_previous(&self) -> bool {
        self.server.imp().can_go_previous.get()
    }

    pub async fn set_can_go_previous(&self, can_go_previous: bool) -> Result<()> {
        if self.can_go_previous() == can_go_previous {
            return Ok(());
        }

        self.server.imp().can_go_previous.set(can_go_previous);
        self.server.can_go_previous_changed().await
    }

    pub fn can_play(&self) -> bool {
        self.server.imp().can_play.get()
    }

    pub async fn set_can_play(&self, can_play: bool) -> Result<()> {
        if self.can_play() == can_play {
            return Ok(());
        }

        self.server.imp().can_play.set(can_play);
        self.server.can_play_changed().await
    }

    pub fn can_pause(&self) -> bool {
        self.server.imp().can_pause.get()
    }

    pub async fn set_can_pause(&self, can_pause: bool) -> Result<()> {
        if self.can_pause() == can_pause {
            return Ok(());
        }

        self.server.imp().can_pause.set(can_pause);
        self.server.can_pause_changed().await
    }

    pub fn can_seek(&self) -> bool {
        self.server.imp().can_seek.get()
    }

    pub async fn set_can_seek(&self, can_seek: bool) -> Result<()> {
        if self.can_seek() == can_seek {
            return Ok(());
        }

        self.server.imp().can_seek.set(can_seek);
        self.server.can_seek_changed().await
    }

    pub fn can_control(&self) -> bool {
        self.server.imp().can_control.get()
    }

    pub async fn set_can_control(&self, can_control: bool) -> Result<()> {
        if self.can_control() == can_control {
            return Ok(());
        }

        self.server.imp().can_control.set(can_control);
        self.server.can_control_changed().await
    }
}

pub struct PlayerBuilder {
    bus_name_suffix: String,
    can_quit: bool,
    fullscreen: bool,
    can_set_fullscreen: bool,
    can_raise: bool,
    has_track_list: bool,
    identity: String,
    desktop_entry: String,
    supported_uri_schemes: Vec<String>,
    supported_mime_types: Vec<String>,
    playback_status: PlaybackStatus,
    loop_status: LoopStatus,
    rate: PlaybackRate,
    shuffle: bool,
    metadata: Metadata,
    volume: Volume,
    position: TimeInUs,
    minimum_rate: PlaybackRate,
    maximum_rate: PlaybackRate,
    can_go_next: bool,
    can_go_previous: bool,
    can_play: bool,
    can_pause: bool,
    can_seek: bool,
    can_control: bool,
}

impl PlayerBuilder {
    pub fn can_quit(mut self, can_quit: bool) -> Self {
        self.can_quit = can_quit;
        self
    }

    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn can_set_fullscreen(mut self, can_set_fullscreen: bool) -> Self {
        self.can_set_fullscreen = can_set_fullscreen;
        self
    }

    pub fn can_raise(mut self, can_raise: bool) -> Self {
        self.can_raise = can_raise;
        self
    }

    pub fn has_track_list(mut self, has_track_list: bool) -> Self {
        self.has_track_list = has_track_list;
        self
    }

    pub fn identity(mut self, identity: impl Into<String>) -> Self {
        self.identity = identity.into();
        self
    }

    pub fn desktop_entry(mut self, desktop_entry: impl Into<String>) -> Self {
        self.desktop_entry = desktop_entry.into();
        self
    }

    pub fn supported_uri_schemes(
        mut self,
        supported_uri_schemes: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.supported_uri_schemes = supported_uri_schemes
            .into_iter()
            .map(|i| i.into())
            .collect();
        self
    }

    pub fn supported_mime_types(
        mut self,
        supported_mime_types: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.supported_mime_types = supported_mime_types.into_iter().map(|i| i.into()).collect();
        self
    }

    pub fn playback_status(mut self, playback_status: PlaybackStatus) -> Self {
        self.playback_status = playback_status;
        self
    }

    pub fn loop_status(mut self, loop_status: LoopStatus) -> Self {
        self.loop_status = loop_status;
        self
    }

    pub fn rate(mut self, rate: PlaybackRate) -> Self {
        self.rate = rate;
        self
    }

    pub fn shuffle(mut self, shuffle: bool) -> Self {
        self.shuffle = shuffle;
        self
    }

    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn volume(mut self, volume: Volume) -> Self {
        self.volume = volume;
        self
    }

    pub fn position(mut self, position: TimeInUs) -> Self {
        self.position = position;
        self
    }

    pub fn minimum_rate(mut self, minimum_rate: PlaybackRate) -> Self {
        self.minimum_rate = minimum_rate;
        self
    }

    pub fn maximum_rate(mut self, maximum_rate: PlaybackRate) -> Self {
        self.maximum_rate = maximum_rate;
        self
    }

    pub fn can_go_next(mut self, can_go_next: bool) -> Self {
        self.can_go_next = can_go_next;
        self
    }

    pub fn can_go_previous(mut self, can_go_previous: bool) -> Self {
        self.can_go_previous = can_go_previous;
        self
    }

    pub fn can_play(mut self, can_play: bool) -> Self {
        self.can_play = can_play;
        self
    }

    pub fn can_pause(mut self, can_pause: bool) -> Self {
        self.can_pause = can_pause;
        self
    }

    pub fn can_seek(mut self, can_seek: bool) -> Self {
        self.can_seek = can_seek;
        self
    }

    pub fn can_control(mut self, can_control: bool) -> Self {
        self.can_control = can_control;
        self
    }

    pub fn build(self) -> Result<Player> {
        let server = Server::new(
            &self.bus_name_suffix,
            Inner {
                raise_cbs: RefCell::new(Vec::new()),
                quit_cbs: RefCell::new(Vec::new()),
                set_fullscreen_cbs: RefCell::new(Vec::new()),
                can_quit: Cell::new(self.can_quit),
                fullscreen: Cell::new(self.fullscreen),
                can_set_fullscreen: Cell::new(self.can_set_fullscreen),
                can_raise: Cell::new(self.can_raise),
                has_track_list: Cell::new(self.has_track_list),
                identity: RefCell::new(self.identity),
                desktop_entry: RefCell::new(self.desktop_entry),
                supported_uri_schemes: RefCell::new(self.supported_uri_schemes),
                supported_mime_types: RefCell::new(self.supported_mime_types),
                next_cbs: RefCell::new(Vec::new()),
                previous_cbs: RefCell::new(Vec::new()),
                pause_cbs: RefCell::new(Vec::new()),
                play_pause_cbs: RefCell::new(Vec::new()),
                stop_cbs: RefCell::new(Vec::new()),
                play_cbs: RefCell::new(Vec::new()),
                seek_cbs: RefCell::new(Vec::new()),
                set_position_cbs: RefCell::new(Vec::new()),
                open_uri_cbs: RefCell::new(Vec::new()),
                set_loop_status_cbs: RefCell::new(Vec::new()),
                set_rate_cbs: RefCell::new(Vec::new()),
                set_shuffle_cbs: RefCell::new(Vec::new()),
                set_volume_cbs: RefCell::new(Vec::new()),
                playback_status: Cell::new(self.playback_status),
                loop_status: Cell::new(self.loop_status),
                rate: Cell::new(self.rate),
                shuffle: Cell::new(self.shuffle),
                metadata: RefCell::new(self.metadata),
                volume: Cell::new(self.volume),
                position: Cell::new(self.position),
                minimum_rate: Cell::new(self.minimum_rate),
                maximum_rate: Cell::new(self.maximum_rate),
                can_go_next: Cell::new(self.can_go_next),
                can_go_previous: Cell::new(self.can_go_previous),
                can_play: Cell::new(self.can_play),
                can_pause: Cell::new(self.can_pause),
                can_seek: Cell::new(self.can_seek),
                can_control: Cell::new(self.can_control),
            },
        )?;
        Ok(Player { server })
    }
}