use mpris_server::{Player, Time};

#[async_std::main]
async fn main() {
    let player = Player::builder("Test.Application")
        .can_play(true)
        .can_pause(true)
        .can_go_previous(true)
        .can_go_next(true)
        .build()
        .unwrap();

    player.connect_play_pause(|| {
        println!("PlayPause");
    });

    player.connect_previous(|| {
        println!("Previous");
    });

    player.connect_next(|| {
        println!("Next");
    });

    player.set_can_play(false).await.unwrap();
    player.emit_seeked(Time::from_millis(1000)).await.unwrap();

    player.run().await.unwrap();
}
