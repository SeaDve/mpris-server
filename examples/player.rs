use std::future;

use mpris_server::{zbus::Result, Player, Time};

#[async_std::main]
async fn main() -> Result<()> {
    let player = Player::builder("Test.Application")
        .can_play(true)
        .can_pause(true)
        .can_go_previous(true)
        .can_go_next(true)
        .build()
        .await?;

    // Handle `PlayPause` method call
    player.connect_play_pause(|_player| {
        println!("PlayPause");
    });

    // Handle `Play` method call
    player.connect_previous(|_player| {
        println!("Previous");
    });

    // Handle `Next` method call
    player.connect_next(|_player| {
        println!("Next");
    });

    // Run event handler task
    async_std::task::spawn_local(player.run());

    // Update `CanPlay` property and emit `PropertiesChanged` signal for it
    player.set_can_play(false).await?;

    // Emit `Seeked` signal
    player.seeked(Time::from_millis(1000)).await?;

    // Prevent the program from exiting.
    future::pending::<()>().await;

    Ok(())
}
