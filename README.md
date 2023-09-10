# MPRIS Server

Implement an MPRIS server.

To implement a server, this crate provides two flavors: you can either create a custom struct that implements `RootInterface` and `PlayerInterface`, or you can use the premade mutable `Player` struct that already implements the interfaces internally.

## Examples

* [Custom struct](examples/server.rs)
* [Player usage](examples/player.rs)
* [Running player with glib](examples/glib.rs)


## TODO

* Document public interface
* Replace `TimeInUs`, `DateTime`, and `Uri` with proper types
* Add getter on Metadata
