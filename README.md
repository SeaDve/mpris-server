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

For more detailed examples, see also the [examples directory](https://github.com/SeaDve/mpris-server/tree/main/examples).

### Manual Implementation (via `Server` or `LocalServer`)

It is recommended to manually create your own implementation of the interfaces if you want to have more control. You can do this by creating your own struct and implementing the required interfaces, then passing your struct as implementation in `Server`. You can also use `LocalServer` and the local version of the interfaces if your struct can't be sent and shared across threads.

```rust,ignore
use std::future;

use mpris_server::{
    export::{
        async_trait::async_trait,
        zbus::{fdo, Result},
    },
    Metadata, PlayerInterface, Property, RootInterface, Server, Time, Volume,
};

pub struct MyPlayer;

#[async_trait]
impl RootInterface for MyPlayer {
    async fn identity(&self) -> fdo::Result<String> {
        Ok("MyPlayer".into())
    }

    // Other methods...
}

#[async_trait]
impl PlayerInterface for MyPlayer {
    async fn set_volume(&self, volume: Volume) -> Result<()> {
        self.volume.set(volume);
        Ok(())
    }

    async fn metadata(&self) -> fdo::Result<Metadata> {
        let metadata = Metadata::builder()
            .title("My Song")
            .artist(["My Artist"])
            .album("My Album")
            .length(Time::from_micros(123))
            .build();
        Ok(metadata)
    }

    // Other methods...
}

#[async_std::main]
async fn main() {
    let server = Server::new("com.my.Application", MyPlayer).unwrap();

    // Initialize server's connection to the session bus
    server.init().await.unwrap();

    // Emit `PropertiesChanged` signal for `CanSeek` and `Metadata` properties
    server.properties_changed(Property::CanSeek | Property::Metadata).await.unwrap();

    // Emit `Seeked` signal
    server.seeked(Time::from_micros(124)).await.unwrap();

    // Prevent the program from exiting.
    future::pending::<()>().await;
}
```

### Ready-to-use Implementation (via `Player`)

If you want to create a simple player without having to implement the interfaces, you can use the ready-to-use `Player` struct that implements those interfaces internally. This struct is mutable, automatically emits properties changed signal, and allows you to connect to method and property setter calls.

However, `Player` currently only supports the more commonly used `org.mpris.MediaPlayer2` and `org.mpris.MediaPlayer2.Player` interfaces.

```rust,ignore
use mpris_server::{Player, Time};

#[async_std::main]
async fn main() {
    let player = Player::builder("com.my.Application")
        .can_play(true)
        .can_pause(true)
        .build()
        .unwrap();

    // Handle `PlayPause` method call
    player.connect_play_pause(|| {
        println!("PlayPause");
    });

    // Update `CanPlay` property and emit `PropertiesChanged` signal for it
    player.set_can_play(false).await.unwrap();

    // Emit `Seeked` signal
    player.emit_seeked(Time::from_millis(1000)).await.unwrap();

    player.init_and_run().await.unwrap();
}
```

## License

Copyright 2023 Dave Patrick Caberto

This software is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at [this site](http://mozilla.org/MPL/2.0/).
