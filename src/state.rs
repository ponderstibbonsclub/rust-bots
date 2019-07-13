use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::*,
    renderer::{
        Camera,
        ImageFormat,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
        resources::Tint,
    },
};

use crate::bot::{Bot, initialise_bots};
use crate::components::Velocity;

pub const ARENA_WIDTH: f32 = 800.0;
pub const ARENA_HEIGHT: f32 = 600.0;

pub struct RustBots;

impl SimpleState for RustBots {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Bot>();
        world.register::<Velocity>();
        world.register::<Tint>();

        initialise_bots(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

pub fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/bot_sprite.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/bot_sprite.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}