use mpris_server::Player;

#[async_std::main]
async fn main() {
    let player = Player::builder("Test.Application")
        .can_play(true)
        .can_pause(true)
        .build()
        .unwrap();

    player.connect_play_pause(|| {
        println!("PlayPause");
    });

    player.run().await.unwrap();
}
