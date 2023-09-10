use mpris_server::Player;

fn main() {
    let m = glib::MainLoop::new(None, false);

    let player = Player::builder("Test.Application")
        .can_play(true)
        .can_pause(true)
        .build()
        .unwrap();

    player.connect_play_pause(|| {
        println!("PlayPause");
    });

    let ctx = glib::MainContext::default();
    ctx.spawn_local(async move {
        player.run().await.unwrap();
    });

    m.run();
}
