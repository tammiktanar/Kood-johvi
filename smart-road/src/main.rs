use std::{env, path};
use ggez::{ContextBuilder, event};
use ggez::conf::{WindowMode, WindowSetup};
use crate::game::Game;

mod game;

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup{
            title: "road_intersection".to_string(),
            vsync: true,
            ..Default::default()
        })
        .window_mode(WindowMode {
            width: 1008.0,
            height: 1008.0,

            ..Default::default()
        })
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = Game::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

