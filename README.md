# MPRIS Server

[![docs](https://docs.rs/mpris-server/badge.svg)](https://docs.rs/mpris-server/)
[![crates.io](https://img.shields.io/crates/v/mpris-server)](https://crates.io/crates/mpris-server)
[![CI](https://github.com/SeaDve/mpris-server/actions/workflows/ci.yml/badge.svg)](https://github.com/SeaDve/mpris-server/actions/workflows/ci.yml)

Implement MPRIS D-Bus interface in your application.

This library provides the essential functionalities for implementing the [MPRIS D-Bus interface](https://specifications.freedesktop.org/mpris-spec/2.2/). This enables your application to become discoverable and controllable by other MPRIS-compatible media controllers, including but not limited to GNOME Shell, KDE Plasma, and other libraries such as [`mpris`](https://github.com/Mange/mpris-rs).

This library supports all the following interfaces as defined in the specification:

* [org.mpris.MediaPlayer2](https://specifications.freedesktop.org/mpris-spec/2.2/Media_Player.html)
* [org.mpris.MediaPlayer2.Player](https://specifications.freedesktop.org/mpris-spec/2.2/Player_Interface.html)
* [org.mpris.MediaPlayer2.TrackList](https://specifications.freedesktop.org/mpris-spec/2.2/Track_List_Interface.html)
* [org.mpris.MediaPlayer2.Playlists](https://specifications.freedesktop.org/mpris-spec/2.2/Playlists_Interface.html)

To implement these interfaces, this crate offers two flavors: you can either create your own struct and implement `RootInterface` and `PlayerInterface` (or with optional `TrackListInterface` and `PlaylistsInterface`), or you can use the ready-to-use mutable `Player` struct.

## Examples

### Manual Implementation (via `Server` or `LocalServer`)

```rust,ignore
use mpris_server::{
    export::{async_trait::async_trait, zbus::fdo},
    Metadata, PlayerInterface, Property, RootInterface, Server, Time, TrackId,
};

pub struct MyPlayer;

#[async_trait]
impl RootInterface for MyPlayer {
    async fn identity(&self) -> fdo::Result<String> {
        unimplemented!()
    }

    ...
}

#[async_trait]
impl PlayerInterface for MyPlayer {
    async fn set_position(&self, track_id: TrackId, position: Time) -> fdo::Result<()> {
        unimplemented!()
    }

    async fn metadata(&self) -> fdo::Result<Metadata> {
        unimplemented!()
    }

    ...
}

#[async_std::main]
async fn main() {
    let server = Server::new("com.me.Application", MyPlayer).unwrap();

    // Initialize server's connection to the session bus
    server.init().await.unwrap();

    // Emit `PropertiesChanged` signal for `Position` and `Metadata` properties
    server.properties_changed(Property::Position | Property::Metadata).await.unwrap();

    // Emit `Seeked` signal
    server.seeked(Time::from_micros(124)).await.unwrap();
}
```

For more examples, see the [examples directory](https://github.com/SeaDve/mpris-server/tree/main/examples).

### Ready-to-use Implementation (via `Player`)

If you want to create a simple player without having to implement the interfaces, you can use the ready-to-use `Player` struct that implements those interfaces internally. This struct is mutable, automatically emits properties changed signal, and allows you to connect to method and property setter calls.

However, `Player` currently only supports the more commonly used `org.mpris.MediaPlayer2` and `org.mpris.MediaPlayer2.Player` interfaces.

```rust,ignore
use mpris_server::Player;

#[async_std::main]
async fn main() {
    let player = Player::builder("com.me.Application")
        .can_play(true)
        .can_pause(true)
        .build()
        .unwrap();

    player.connect_play_pause(|| {
        println!("PlayPause");
    });

    player.run().await.unwrap();
}
```

## License

Copyright 2023 Dave Patrick Caberto

This software is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at [this site](http://mozilla.org/MPL/2.0/).
