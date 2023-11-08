# MPRIS Server

[![github](https://img.shields.io/badge/github-seadve/mpris-server)](https://github.com/SeaDve/mpris-server)
[![crates.io](https://img.shields.io/crates/v/mpris-server)](https://crates.io/crates/mpris-server)
[![docs](https://docs.rs/mpris-server/badge.svg)](https://docs.rs/mpris-server/)
[![CI](https://github.com/SeaDve/mpris-server/actions/workflows/ci.yml/badge.svg)](https://github.com/SeaDve/mpris-server/actions/workflows/ci.yml)

Implement MPRIS D-Bus interface in your application.

This library provides the essential functionalities for implementing the [MPRIS D-Bus interface](https://specifications.freedesktop.org/mpris-spec/2.2/) on the *service* side. This enables your application to become discoverable and controllable by other MPRIS-compatible media controllers, including but not limited to GNOME Shell, KDE Plasma, and other libraries such as [`mpris`](https://github.com/Mange/mpris-rs).

This library supports all the following interfaces as defined in the specification:

* [org.mpris.MediaPlayer2](https://specifications.freedesktop.org/mpris-spec/2.2/Media_Player.html)
* [org.mpris.MediaPlayer2.Player](https://specifications.freedesktop.org/mpris-spec/2.2/Player_Interface.html)
* [org.mpris.MediaPlayer2.TrackList](https://specifications.freedesktop.org/mpris-spec/2.2/Track_List_Interface.html)
* [org.mpris.MediaPlayer2.Playlists](https://specifications.freedesktop.org/mpris-spec/2.2/Playlists_Interface.html)

To implement these interfaces, this crate offers two flavors: you can either create your own struct and implement `RootInterface` and `PlayerInterface` (or with optional `TrackListInterface` and `PlaylistsInterface`), or you can use the ready-to-use `Player` struct.

## Examples

For more detailed examples, see also the [examples directory](https://github.com/SeaDve/mpris-server/tree/main/examples).

There is also a real-word example of this library being used in [Mousai](https://github.com/SeaDve/Mousai), a music recognizer application for Linux.

### Manual Implementation (via `Server` or `LocalServer`)

It is recommended to manually create your own implementation of the interfaces if you want to have more control. You can do this by creating your own struct and implementing the required interfaces, then passing your struct as implementation in `Server`. You can also use `LocalServer` and the local version of the interfaces if your struct can't be sent and shared across threads.

```rust,ignore
use std::future;

use mpris_server::{
    async_trait,
    zbus::{fdo, Result},
    Metadata, PlayerInterface, Property, RootInterface, Server, Signal, Time, Volume,
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
async fn main() -> Result<()> {
    let server = Server::new("com.my.Application", MyPlayer).await?;

    // Emit `PropertiesChanged` signal for `CanSeek` and `Metadata` properties
    server.properties_changed(Property::CanSeek | Property::Metadata).await?;

    // Emit `Seeked` signal
    server
        .emit(Signal::Seeked {
            position: Time::from_micros(124),
        })
        .await?;

    // Prevent the program from exiting.
    future::pending::<()>().await;

    Ok(())
}
```

### Ready-to-use Implementation (via `Player`)

If you want to create a simple player without having to implement the interfaces, you can use the ready-to-use `Player` struct that implements those interfaces internally. This struct has its own internal state, automatically emits properties changed signals, and allows you to connect to method and property setter calls.

However, `Player` currently only supports the more commonly used `org.mpris.MediaPlayer2` and `org.mpris.MediaPlayer2.Player` interfaces.

```rust,ignore
use std::future;

use mpris_server::{zbus::Result, Player, Time};

#[async_std::main]
async fn main() -> Result<()> {
    let player = Player::builder("com.my.Application")
        .can_play(true)
        .can_pause(true)
        .build()
        .await?;

    // Handle `PlayPause` method call
    player.connect_play_pause(|_player| {
        println!("PlayPause");
    });

    // Run event handler task
    let task = player.run();
    async_std::task::spawn_local(async move {
        task.await.unwrap();
    });

    // Update `CanPlay` property and emit `PropertiesChanged` signal for it
    player.set_can_play(false).await?;

    // Emit `Seeked` signal
    player.seeked(Time::from_millis(1000)).await?;

    // Prevent the program from exiting.
    future::pending::<()>().await;

    Ok(())
}
```

## License

Copyright 2023 Dave Patrick Caberto

This software is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at [this site](http://mozilla.org/MPL/2.0/).
