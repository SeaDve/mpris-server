> **Warning**
The API is still unstable. Expect breaking changes.

# MPRIS Server

[![docs](https://docs.rs/mpris-server/badge.svg)](https://docs.rs/mpris-server/)
[![crates.io](https://img.shields.io/crates/v/mpris-server)](https://crates.io/crates/mpris-server)
[![CI](https://github.com/SeaDve/mpris-server/actions/workflows/ci.yml/badge.svg)](https://github.com/SeaDve/mpris-server/actions/workflows/ci.yml)

Create MPRIS MediaPlayer2 server

To implement a server, this crate provides two flavors: you can either create a custom struct that implements `RootInterface` and `PlayerInterface`, or you can use the premade mutable `Player` struct.

## Player Usage

If you want to create a simple player without having to implement the interfaces, you can use the premade `Player` struct that implements those interfaces internally. This struct is mutable, automatically emits properties changed signal, and allows you to connect to method calls.

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

## Supported Interfaces

This library supports all interfaces defined in the [MPRIS2 specification](https://specifications.freedesktop.org/mpris-spec/2.2/index.html). However, the premade `Player` struct currently only supports the more commonly used `org.mpris.MediaPlayer2` and `org.mpris.MediaPlayer2.Player` interfaces.

### org.mpris.MediaPlayer2 and org.mpris.MediaPlayer2.Player

```rust,ignore
use mpris_server::{export::async_trait, Server};

pub struct MyPlayer;

#[async_trait]
impl RootInterface for MyPlayer {
    ...
}

#[async_trait]
impl PlayerInterface for MyPlayer {
    ...
}

#[async_std::main]
async fn main() {
    let server = Server::new("com.me.Application", MyPlayer).unwrap();
    server.run().await.unwrap();
}
```

### org.mpris.MediaPlayer2.TrackList

```rust,ignore
use mpris_server::{export::async_trait, Server};

pub struct MyPlayer;

#[async_trait]
impl RootInterface for MyPlayer {
    ...
}

#[async_trait]
impl PlayerInterface for MyPlayer {
    ...
}

#[async_trait]
impl TracklistInterface for MyPlayer {
    ...
}

#[async_std::main]
async fn main() {
    let server = Server::new("com.me.Application", MyPlayer).unwrap();
    server.run_with_track_list().await.unwrap();
}
```

### org.mpris.MediaPlayer2.Playlists

```rust,ignore
use mpris_server::{export::async_trait, Server};

pub struct MyPlayer;

#[async_trait]
impl RootInterface for MyPlayer {
    ...
}

#[async_trait]
impl PlayerInterface for MyPlayer {
    ...
}

#[async_trait]
impl PlaylistsInterface for MyPlayer {
    ...
}

#[async_std::main]
async fn main() {
    let server = Server::new("com.me.Application", MyPlayer).unwrap();
    server.run_with_playlists().await.unwrap();
}
```


### org.mpris.MediaPlayer2.TrackList and org.mpris.MediaPlayer2.Playlists

```rust,ignore
use mpris_server::{export::async_trait, Server};

pub struct MyPlayer;

#[async_trait]
impl RootInterface for MyPlayer {
    ...
}

#[async_trait]
impl PlayerInterface for MyPlayer {
    ...
}

#[async_trait]
impl PlaylistsInterface for MyPlayer {
    ...
}

#[async_trait]
impl TracklistInterface for MyPlayer {
    ...
}

#[async_std::main]
async fn main() {
    let server = Server::new("com.me.Application", MyPlayer).unwrap();
    server.run_with_all().await.unwrap();
}
```

For more examples, see the [examples directory](https://github.com/SeaDve/mpris-server/tree/main/examples).

## TODO

* Document public interface
* Replace `DateTime`, and `Uri` with proper types
* Add getter on Metadata
* Profile if inlining is worth it
