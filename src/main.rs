extern crate nalgebra as na;

mod bot;
mod graph;
mod components;
mod state;
mod systems;

use amethyst::{
    assets::Processor,
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        types::DefaultBackend,
        RenderingSystem,
        SpriteSheet
    },
    utils::application_root_dir,
    window::WindowBundle,
};

use graph::RustBotsGraph;
use state::RustBots;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("resources").join("display_config.ron");

    let game_data = GameDataBuilder::default()
        // The WindowBundle provides all the scaffolding for opening a window
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with_bundle(TransformBundle::new())?
        .with(systems::MovementSystem, "move_system", &[])
        .with(systems::ScaleSystem, "scale_system", &["move_system"])
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RustBotsGraph::default(),
        )
    );

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, RustBots, game_data)?;
    game.run();

    Ok(())
}
