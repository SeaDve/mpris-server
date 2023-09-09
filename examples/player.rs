use mpris_server::Player;

#[async_std::main]
async fn main() {
    let player = Player::builder("Test.Application").build().unwrap();

    player.connect_play_pause(|| {
        println!("PlayPause");
    });

    player.run().await.unwrap();
}
