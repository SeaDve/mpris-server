use std::{
    cell::{Cell, Ref, RefCell},
    rc::{Rc, Weak},
};

use zbus::{fdo, Result};

use crate::{
    LocalPlayerInterface, LocalRootInterface, LocalServer, LocalServerRunTask, LoopStatus,
    Metadata, PlaybackRate, PlaybackStatus, Property, Signal, Time, TrackId, Volume,
};

/// Ready-to-use mutable *service*-side object that internally implements
/// [`LocalRootInterface`] and [`LocalPlayerInterface`].
///
/// This has its own internal state, automatically emits properties changed
/// signals, and allows you to connect to method and property setter calls.
///
/// If you need more control to the implementation, you can use [`LocalServer`]
/// or [`Server`] directly instead.
///
/// [`Server`]: crate::Server
#[derive(Debug)]
pub struct Player {
    server: Rc<LocalServer<State>>,
}

#[allow(clippy::type_complexity)]
struct State {
    server: RefCell<Option<Weak<LocalServer<State>>>>,

    raise_cbs: RefCell<Vec<Box<dyn Fn(&Player)>>>,
    quit_cbs: RefCell<Vec<Box<dyn Fn(&Player)>>>,
    set_fullscreen_cbs: RefCell<Vec<Box<dyn Fn(&Player, bool)>>>, // Property
    can_quit: Cell<bool>,
    fullscreen: Cell<bool>,
    can_set_fullscreen: Cell<bool>,
    can_raise: Cell<bool>,
    has_track_list: Cell<bool>,
    identity: RefCell<String>,
    desktop_entry: RefCell<String>,
    supported_uri_schemes: RefCell<Vec<String>>,
    supported_mime_types: RefCell<Vec<String>>,

    next_cbs: RefCell<Vec<Box<dyn Fn(&Player)>>>,
    previous_cbs: RefCell<Vec<Box<dyn Fn(&Player)>>>,
    pause_cbs: RefCell<Vec<Box<dyn Fn(&Player)>>>,
    play_pause_cbs: RefCell<Vec<Box<dyn Fn(&Player)>>>,
    stop_cbs: RefCell<Vec<Box<dyn Fn(&Player)>>>,
    play_cbs: RefCell<Vec<Box<dyn Fn(&Player)>>>,
    seek_cbs: RefCell<Vec<Box<dyn Fn(&Player, Time)>>>,
    set_position_cbs: RefCell<Vec<Box<dyn Fn(&Player, &TrackId, Time)>>>,
    open_uri_cbs: RefCell<Vec<Box<dyn Fn(&Player, &str)>>>,
    set_loop_status_cbs: RefCell<Vec<Box<dyn Fn(&Player, LoopStatus)>>>, // Property
    set_rate_cbs: RefCell<Vec<Box<dyn Fn(&Player, PlaybackRate)>>>,      // Property
    set_shuffle_cbs: RefCell<Vec<Box<dyn Fn(&Player, bool)>>>,           // Property
    set_volume_cbs: RefCell<Vec<Box<dyn Fn(&Player, Volume)>>>,          // Property
    playback_status: Cell<PlaybackStatus>,
    loop_status: Cell<LoopStatus>,
    rate: Cell<PlaybackRate>,
    shuffle: Cell<bool>,
    metadata: RefCell<Metadata>,
    volume: Cell<Volume>,
    position: Cell<Time>,
    minimum_rate: Cell<PlaybackRate>,
    maximum_rate: Cell<PlaybackRate>,
    can_go_next: Cell<bool>,
    can_go_previous: Cell<bool>,
    can_play: Cell<bool>,
    can_pause: Cell<bool>,
    can_seek: Cell<bool>,
    can_control: Cell<bool>,
}

impl State {
    fn player(&self) -> Player {
        Player {
            server: self
                .server
                .borrow()
                .as_ref()
                .expect("server must be set up")
                .upgrade()
                .expect("server must not be dropped"),
        }
    }
}

impl LocalRootInterface for State {
    async fn raise(&self) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.raise_cbs.borrow().iter() {
            cb(&player);
        }
        Ok(())
    }

    async fn quit(&self) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.quit_cbs.borrow().iter() {
            cb(&player);
        }
        Ok(())
    }

    async fn can_quit(&self) -> fdo::Result<bool> {
        Ok(self.can_quit.get())
    }

    async fn fullscreen(&self) -> fdo::Result<bool> {
        Ok(self.fullscreen.get())
    }

    async fn set_fullscreen(&self, fullscreen: bool) -> Result<()> {
        let player = self.player();
        for cb in self.set_fullscreen_cbs.borrow().iter() {
            cb(&player, fullscreen);
        }
        Ok(())
    }

    async fn can_set_fullscreen(&self) -> fdo::Result<bool> {
        Ok(self.can_set_fullscreen.get())
    }

    async fn can_raise(&self) -> fdo::Result<bool> {
        Ok(self.can_raise.get())
    }

    async fn has_track_list(&self) -> fdo::Result<bool> {
        Ok(self.has_track_list.get())
    }

    async fn identity(&self) -> fdo::Result<String> {
        Ok(self.identity.borrow().clone())
    }

    async fn desktop_entry(&self) -> fdo::Result<String> {
        Ok(self.desktop_entry.borrow().clone())
    }

    async fn supported_uri_schemes(&self) -> fdo::Result<Vec<String>> {
        Ok(self.supported_uri_schemes.borrow().clone())
    }

    async fn supported_mime_types(&self) -> fdo::Result<Vec<String>> {
        Ok(self.supported_mime_types.borrow().clone())
    }
}

impl LocalPlayerInterface for State {
    async fn next(&self) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.next_cbs.borrow().iter() {
            cb(&player);
        }
        Ok(())
    }

    async fn previous(&self) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.previous_cbs.borrow().iter() {
            cb(&player);
        }
        Ok(())
    }

    async fn pause(&self) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.pause_cbs.borrow().iter() {
            cb(&player);
        }
        Ok(())
    }

    async fn play_pause(&self) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.play_pause_cbs.borrow().iter() {
            cb(&player);
        }
        Ok(())
    }

    async fn stop(&self) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.stop_cbs.borrow().iter() {
            cb(&player);
        }
        Ok(())
    }

    async fn play(&self) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.play_cbs.borrow().iter() {
            cb(&player);
        }
        Ok(())
    }

    async fn seek(&self, offset: Time) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.seek_cbs.borrow().iter() {
            cb(&player, offset);
        }
        Ok(())
    }

    async fn set_position(&self, track_id: TrackId, position: Time) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.set_position_cbs.borrow().iter() {
            cb(&player, &track_id, position);
        }
        Ok(())
    }

    async fn open_uri(&self, uri: String) -> fdo::Result<()> {
        let player = self.player();
        for cb in self.open_uri_cbs.borrow().iter() {
            cb(&player, &uri);
        }
        Ok(())
    }

    async fn playback_status(&self) -> fdo::Result<PlaybackStatus> {
        Ok(self.playback_status.get())
    }

    async fn loop_status(&self) -> fdo::Result<LoopStatus> {
        Ok(self.loop_status.get())
    }

    async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()> {
        let player = self.player();
        for cb in self.set_loop_status_cbs.borrow().iter() {
            cb(&player, loop_status);
        }
        Ok(())
    }

    async fn rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(self.rate.get())
    }

    async fn set_rate(&self, rate: PlaybackRate) -> Result<()> {
        let player = self.player();
        for cb in self.set_rate_cbs.borrow().iter() {
            cb(&player, rate);
        }
        Ok(())
    }

    async fn shuffle(&self) -> fdo::Result<bool> {
        Ok(self.shuffle.get())
    }

    async fn set_shuffle(&self, shuffle: bool) -> Result<()> {
        let player = self.player();
        for cb in self.set_shuffle_cbs.borrow().iter() {
            cb(&player, shuffle);
        }
        Ok(())
    }

    async fn metadata(&self) -> fdo::Result<Metadata> {
        Ok(self.metadata.borrow().clone())
    }

    async fn volume(&self) -> fdo::Result<Volume> {
        Ok(self.volume.get())
    }

    async fn set_volume(&self, volume: Volume) -> Result<()> {
        let player = self.player();
        for cb in self.set_volume_cbs.borrow().iter() {
            cb(&player, volume);
        }
        Ok(())
    }

    async fn position(&self) -> fdo::Result<Time> {
        Ok(self.position.get())
    }

    async fn minimum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(self.minimum_rate.get())
    }

    async fn maximum_rate(&self) -> fdo::Result<PlaybackRate> {
        Ok(self.maximum_rate.get())
    }

    async fn can_go_next(&self) -> fdo::Result<bool> {
        Ok(self.can_go_next.get())
    }

    async fn can_go_previous(&self) -> fdo::Result<bool> {
        Ok(self.can_go_previous.get())
    }

    async fn can_play(&self) -> fdo::Result<bool> {
        Ok(self.can_play.get())
    }

    async fn can_pause(&self) -> fdo::Result<bool> {
        Ok(self.can_pause.get())
    }

    async fn can_seek(&self) -> fdo::Result<bool> {
        Ok(self.can_seek.get())
    }

    async fn can_control(&self) -> fdo::Result<bool> {
        Ok(self.can_control.get())
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
            position: Time::ZERO,
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

    /// Returns a task that runs the player's server until the player and the
    /// task is dropped.
    ///
    /// The task must be awaited as soon as possible after creating the player.
    ///
    /// The returned task is no-op if the player's server has been ran before.
    pub fn run(&self) -> LocalServerRunTask {
        self.server.run()
    }

    pub fn connect_raise(&self, cb: impl Fn(&Self) + 'static) {
        self.server.imp().raise_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_quit(&self, cb: impl Fn(&Self) + 'static) {
        self.server.imp().quit_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_set_fullscreen(&self, cb: impl Fn(&Self, bool) + 'static) {
        self.server
            .imp()
            .set_fullscreen_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn can_quit(&self) -> bool {
        self.server.imp().can_quit.get()
    }

    pub async fn set_can_quit(&self, can_quit: bool) -> Result<()> {
        if self.can_quit() == can_quit {
            return Ok(());
        }

        self.server.imp().can_quit.set(can_quit);
        self.server
            .properties_changed([Property::CanQuit(can_quit)])
            .await
    }

    pub fn fullscreen(&self) -> bool {
        self.server.imp().fullscreen.get()
    }

    pub async fn set_fullscreen(&self, fullscreen: bool) -> Result<()> {
        if self.fullscreen() == fullscreen {
            return Ok(());
        }

        self.server.imp().fullscreen.set(fullscreen);
        self.server
            .properties_changed([Property::Fullscreen(fullscreen)])
            .await
    }

    pub fn can_set_fullscreen(&self) -> bool {
        self.server.imp().can_set_fullscreen.get()
    }

    pub async fn set_can_set_fullscreen(&self, can_set_fullscreen: bool) -> Result<()> {
        if self.can_set_fullscreen() == can_set_fullscreen {
            return Ok(());
        }

        self.server.imp().can_set_fullscreen.set(can_set_fullscreen);
        self.server
            .properties_changed([Property::CanSetFullscreen(can_set_fullscreen)])
            .await
    }

    pub fn can_raise(&self) -> bool {
        self.server.imp().can_raise.get()
    }

    pub async fn set_can_raise(&self, can_raise: bool) -> Result<()> {
        if self.can_raise() == can_raise {
            return Ok(());
        }

        self.server.imp().can_raise.set(can_raise);
        self.server
            .properties_changed([Property::CanRaise(can_raise)])
            .await
    }

    pub fn has_track_list(&self) -> bool {
        self.server.imp().has_track_list.get()
    }

    pub async fn set_has_track_list(&self, has_track_list: bool) -> Result<()> {
        if self.has_track_list() == has_track_list {
            return Ok(());
        }

        self.server.imp().has_track_list.set(has_track_list);
        self.server
            .properties_changed([Property::HasTrackList(has_track_list)])
            .await
    }

    pub fn identity(&self) -> Ref<'_, String> {
        self.server.imp().identity.borrow()
    }

    pub async fn set_identity(&self, identity: impl Into<String>) -> Result<()> {
        let identity = identity.into();

        if *self.identity() == identity {
            return Ok(());
        }

        self.server.imp().identity.replace(identity.clone());
        self.server
            .properties_changed([Property::Identity(identity)])
            .await
    }

    pub fn desktop_entry(&self) -> Ref<'_, String> {
        self.server.imp().desktop_entry.borrow()
    }

    pub async fn set_desktop_entry(&self, desktop_entry: impl Into<String>) -> Result<()> {
        let desktop_entry = desktop_entry.into();

        if *self.desktop_entry() == desktop_entry {
            return Ok(());
        }

        self.server
            .imp()
            .desktop_entry
            .replace(desktop_entry.clone());
        self.server
            .properties_changed([Property::DesktopEntry(desktop_entry)])
            .await
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
            .collect::<Vec<_>>();

        if *self.supported_uri_schemes() == supported_uri_schemes {
            return Ok(());
        }

        self.server
            .imp()
            .supported_uri_schemes
            .replace(supported_uri_schemes.clone());
        self.server
            .properties_changed([Property::SupportedUriSchemes(supported_uri_schemes)])
            .await
    }

    pub fn supported_mime_types(&self) -> Ref<'_, Vec<String>> {
        self.server.imp().supported_mime_types.borrow()
    }

    pub async fn set_supported_mime_types(
        &self,
        supported_mime_types: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<()> {
        let supported_mime_types = supported_mime_types
            .into_iter()
            .map(|i| i.into())
            .collect::<Vec<_>>();

        if *self.supported_mime_types() == supported_mime_types {
            return Ok(());
        }

        self.server
            .imp()
            .supported_mime_types
            .replace(supported_mime_types.clone());
        self.server
            .properties_changed([Property::SupportedMimeTypes(supported_mime_types)])
            .await
    }

    pub fn connect_next(&self, cb: impl Fn(&Self) + 'static) {
        self.server.imp().next_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_previous(&self, cb: impl Fn(&Self) + 'static) {
        self.server
            .imp()
            .previous_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn connect_pause(&self, cb: impl Fn(&Self) + 'static) {
        self.server.imp().pause_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_play_pause(&self, cb: impl Fn(&Self) + 'static) {
        self.server
            .imp()
            .play_pause_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn connect_stop(&self, cb: impl Fn(&Self) + 'static) {
        self.server.imp().stop_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_play(&self, cb: impl Fn(&Self) + 'static) {
        self.server.imp().play_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_seek(&self, cb: impl Fn(&Self, Time) + 'static) {
        self.server.imp().seek_cbs.borrow_mut().push(Box::new(cb));
    }

    pub fn connect_set_position(&self, cb: impl Fn(&Self, &TrackId, Time) + 'static) {
        self.server
            .imp()
            .set_position_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn connect_open_uri(&self, cb: impl Fn(&Self, &str) + 'static) {
        self.server
            .imp()
            .open_uri_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn connect_set_loop_status(&self, cb: impl Fn(&Self, LoopStatus) + 'static) {
        self.server
            .imp()
            .set_loop_status_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn connect_set_rate(&self, cb: impl Fn(&Self, PlaybackRate) + 'static) {
        self.server
            .imp()
            .set_rate_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn connect_set_shuffle(&self, cb: impl Fn(&Self, bool) + 'static) {
        self.server
            .imp()
            .set_shuffle_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub fn connect_set_volume(&self, cb: impl Fn(&Self, Volume) + 'static) {
        self.server
            .imp()
            .set_volume_cbs
            .borrow_mut()
            .push(Box::new(cb));
    }

    pub async fn seeked(&self, position: Time) -> Result<()> {
        self.server.emit(Signal::Seeked { position }).await
    }

    pub fn playback_status(&self) -> PlaybackStatus {
        self.server.imp().playback_status.get()
    }

    pub async fn set_playback_status(&self, playback_status: PlaybackStatus) -> Result<()> {
        if self.playback_status() == playback_status {
            return Ok(());
        }

        self.server.imp().playback_status.set(playback_status);
        self.server
            .properties_changed([Property::PlaybackStatus(playback_status)])
            .await
    }

    pub fn loop_status(&self) -> LoopStatus {
        self.server.imp().loop_status.get()
    }

    pub async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<()> {
        if self.loop_status() == loop_status {
            return Ok(());
        }

        self.server.imp().loop_status.set(loop_status);
        self.server
            .properties_changed([Property::LoopStatus(loop_status)])
            .await
    }

    pub fn rate(&self) -> PlaybackRate {
        self.server.imp().rate.get()
    }

    pub async fn set_rate(&self, rate: PlaybackRate) -> Result<()> {
        if self.rate() == rate {
            return Ok(());
        }

        self.server.imp().rate.set(rate);
        self.server.properties_changed([Property::Rate(rate)]).await
    }

    pub fn shuffle(&self) -> bool {
        self.server.imp().shuffle.get()
    }

    pub async fn set_shuffle(&self, shuffle: bool) -> Result<()> {
        if self.shuffle() == shuffle {
            return Ok(());
        }

        self.server.imp().shuffle.set(shuffle);
        self.server
            .properties_changed([Property::Shuffle(shuffle)])
            .await
    }

    pub fn metadata(&self) -> Ref<'_, Metadata> {
        self.server.imp().metadata.borrow()
    }

    pub async fn set_metadata(&self, metadata: Metadata) -> Result<()> {
        if *self.metadata() == metadata {
            return Ok(());
        }

        self.server.imp().metadata.replace(metadata.clone());
        self.server
            .properties_changed([Property::Metadata(metadata)])
            .await
    }

    pub fn volume(&self) -> Volume {
        self.server.imp().volume.get()
    }

    pub async fn set_volume(&self, volume: Volume) -> Result<()> {
        if self.volume() == volume {
            return Ok(());
        }

        self.server.imp().volume.set(volume);
        self.server
            .properties_changed([Property::Volume(volume)])
            .await
    }

    pub fn position(&self) -> Time {
        self.server.imp().position.get()
    }

    /// This does not emit `PropertiesChanged` signal.
    pub fn set_position(&self, position: Time) {
        self.server.imp().position.set(position);
    }

    pub fn minimum_rate(&self) -> PlaybackRate {
        self.server.imp().minimum_rate.get()
    }

    pub async fn set_minimum_rate(&self, minimum_rate: PlaybackRate) -> Result<()> {
        if self.minimum_rate() == minimum_rate {
            return Ok(());
        }

        self.server.imp().minimum_rate.set(minimum_rate);
        self.server
            .properties_changed([Property::MinimumRate(minimum_rate)])
            .await
    }

    pub fn maximum_rate(&self) -> PlaybackRate {
        self.server.imp().maximum_rate.get()
    }

    pub async fn set_maximum_rate(&self, maximum_rate: PlaybackRate) -> Result<()> {
        if self.maximum_rate() == maximum_rate {
            return Ok(());
        }

        self.server.imp().maximum_rate.set(maximum_rate);
        self.server
            .properties_changed([Property::MaximumRate(maximum_rate)])
            .await
    }

    pub fn can_go_next(&self) -> bool {
        self.server.imp().can_go_next.get()
    }

    pub async fn set_can_go_next(&self, can_go_next: bool) -> Result<()> {
        if self.can_go_next() == can_go_next {
            return Ok(());
        }

        self.server.imp().can_go_next.set(can_go_next);
        self.server
            .properties_changed([Property::CanGoNext(can_go_next)])
            .await
    }

    pub fn can_go_previous(&self) -> bool {
        self.server.imp().can_go_previous.get()
    }

    pub async fn set_can_go_previous(&self, can_go_previous: bool) -> Result<()> {
        if self.can_go_previous() == can_go_previous {
            return Ok(());
        }

        self.server.imp().can_go_previous.set(can_go_previous);
        self.server
            .properties_changed([Property::CanGoPrevious(can_go_previous)])
            .await
    }

    pub fn can_play(&self) -> bool {
        self.server.imp().can_play.get()
    }

    pub async fn set_can_play(&self, can_play: bool) -> Result<()> {
        if self.can_play() == can_play {
            return Ok(());
        }

        self.server.imp().can_play.set(can_play);
        self.server
            .properties_changed([Property::CanPlay(can_play)])
            .await
    }

    pub fn can_pause(&self) -> bool {
        self.server.imp().can_pause.get()
    }

    pub async fn set_can_pause(&self, can_pause: bool) -> Result<()> {
        if self.can_pause() == can_pause {
            return Ok(());
        }

        self.server.imp().can_pause.set(can_pause);
        self.server
            .properties_changed([Property::CanPause(can_pause)])
            .await
    }

    pub fn can_seek(&self) -> bool {
        self.server.imp().can_seek.get()
    }

    pub async fn set_can_seek(&self, can_seek: bool) -> Result<()> {
        if self.can_seek() == can_seek {
            return Ok(());
        }

        self.server.imp().can_seek.set(can_seek);
        self.server
            .properties_changed([Property::CanSeek(can_seek)])
            .await
    }

    /// This can only be set on construct.
    pub fn can_control(&self) -> bool {
        self.server.imp().can_control.get()
    }
}

/// A builder used to create [`Player`].
#[derive(Debug)]
#[must_use = "must call `build()` to finish building the player"]
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
    position: Time,
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

    pub fn position(mut self, position: Time) -> Self {
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

    #[must_use = "building player is usually expensive and is not expected to have side effects"]
    pub async fn build(self) -> Result<Player> {
        let server = Rc::new(
            LocalServer::new(
                &self.bus_name_suffix,
                State {
                    server: RefCell::new(None),
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
            )
            .await?,
        );

        server.imp().server.replace(Some(Rc::downgrade(&server)));

        Ok(Player { server })
    }
}
