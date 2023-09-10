# MPRIS Server

Create MPRIS2 media player server

To implement a server, this crate provides two flavors: you can either create a custom struct that implements `RootInterface` and `PlayerInterface`, or you can use the premade mutable `Player` struct that already implements the interfaces internally.

## Supported Interfaces

This library supports all interfaces defined in the [MPRIS2 specification](https://specifications.freedesktop.org/mpris-spec/2.2/index.html). However, the premade `Player` struct currently only supports the more commonly used `org.mpris.MediaPlayer2` and `org.mpris.MediaPlayer2.Player` interfaces.

### org.mpris.MediaPlayer2 and org.mpris.MediaPlayer2.Player

```rust
pub struct MyPlayer;

#[async_trait(?Send)]
impl RootInterface for MyPlayer {
    ...
}

#[async_trait(?Send)]
impl PlayerInterface for MyPlayer {
    ...
}

#[async_std::main]
async fn main() {
    let server = Server::new("Test.Application", MyPlayer).unwrap();
    server.run().await.unwrap();
}
```

### org.mpris.MediaPlayer2.TrackList

```rust
pub struct MyPlayer;

#[async_trait(?Send)]
impl RootInterface for MyPlayer {
    ...
}

#[async_trait(?Send)]
impl PlayerInterface for MyPlayer {
    ...
}

#[async_trait(?Send)]
impl TracklistInterface for MyPlayer {
    ...
}

#[async_std::main]
async fn main() {
    let server = Server::new("Test.Application", Player).unwrap();
    server.run_with_track_list().await.unwrap();
}
```

### org.mpris.MediaPlayer2.Playlists

```rust
pub struct MyPlayer;

#[async_trait(?Send)]
impl RootInterface for MyPlayer {
    ...
}

#[async_trait(?Send)]
impl PlayerInterface for MyPlayer {
    ...
}

#[async_trait(?Send)]
impl PlaylistsInterface for MyPlayer {
    ...
}

#[async_std::main]
async fn main() {
    let server = Server::new("Test.Application", Player).unwrap();
    server.run_with_playlists().await.unwrap();
}
```


### org.mpris.MediaPlayer2.TrackList and org.mpris.MediaPlayer2.Playlists

```rust
pub struct MyPlayer;

#[async_trait(?Send)]
impl RootInterface for MyPlayer {
    ...
}

#[async_trait(?Send)]
impl PlayerInterface for MyPlayer {
    ...
}

#[async_trait(?Send)]
impl PlaylistsInterface for MyPlayer {
    ...
}

#[async_trait(?Send)]
impl TracklistInterface for MyPlayer {
    ...
}

#[async_std::main]
async fn main() {
    let server = Server::new("Test.Application", Player).unwrap();
    server.run_with_all().await.unwrap();
}
```




For more examples, see the [examples directory](examples).

## TODO

* Document public interface
* Replace `TimeInUs`, `DateTime`, and `Uri` with proper types
* Add getter on Metadata
