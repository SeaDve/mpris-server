use std::{future, rc::Rc};

use mpris_server::{zbus::Result, Player, Time};

#[async_std::main]
async fn main() -> Result<()> {
    let player = Rc::new(
        Player::builder("Test.Application")
            .can_play(true)
            .can_pause(true)
            .can_go_previous(true)
            .can_go_next(true)
            .build()?,
    );

    // Handle `PlayPause` method call
    player.connect_play_pause(|| {
        println!("PlayPause");
    });

    // Handle `Play` method call
    player.connect_previous(|| {
        println!("Previous");
    });

    // Handle `Next` method call
    player.connect_next(|| {
        println!("Next");
    });

    // Init connection and run event handler
    let player_clone = Rc::clone(&player);
    async_std::task::spawn_local(async move {
        player_clone.init_and_run().await.unwrap();
    });

    // Update `CanPlay` property and emit `PropertiesChanged` signal for it
    player.set_can_play(false).await?;

    // Emit `Seeked` signal
    player.seeked(Time::from_millis(1000)).await?;

    // Prevent the program from exiting.
    future::pending::<()>().await;

    Ok(())
}
