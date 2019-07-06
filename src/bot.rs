use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Component, VecStorage, Builder},
    prelude::World,
    renderer::{SpriteRender, SpriteSheet},
};
use amethyst_renderer::Rgba;
use rand::Rng;

use crate::state::{ARENA_HEIGHT, ARENA_WIDTH};

const START_BOTS: u32 = 5;
const RANDOM_BOTS: bool = false;

#[derive(Debug)]
pub struct Bot {
}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

impl Component for Bot {
    type Storage = VecStorage<Self>;
}

pub fn initialise_bots(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {

    // Assign the sprites for the bots
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    let mut rng = rand::thread_rng();

    if RANDOM_BOTS {
        for _ in 0..START_BOTS {
            let x_lim = ARENA_WIDTH as f64;
            let y_lim = ARENA_HEIGHT as f64;
            let x = rng.gen_range(0.0, x_lim);
            let y = rng.gen_range(0.0, y_lim);
            let rot: f64 = rng.gen_range(0f64, 360f64).to_radians();

            let r = rng.gen_range(0.05, 1.0);
            let g = rng.gen_range(0.05, 1.0);
            let b = rng.gen_range(0.05, 1.0);
            let col = Rgba(r, g, b, 1.0);

            println!("Generated rgba {:?}", col);

            let mut transform = Transform::default();
            transform.set_translation_xyz(x, y, 0.0);
            transform.set_rotation_2d(rot);
            world.
                create_entity()
                .with(Bot::new())
                .with(transform)
                .with(col)
                .with(sprite_render.clone())
                .build();
        }
    } else {
        for idx in 0..5 {
            let idx_f64 = idx as f64;
            let x = ARENA_WIDTH as f64 / 2.0 + idx_f64 * 20f64;
            let y = ARENA_HEIGHT as f64 / 2.0 + idx_f64 * 20f64;
            let rot = (idx_f64 * -30f64).to_radians();
            let mut transform = Transform::default();
            transform.set_translation_xyz(x, y, 0.0);
            transform.set_rotation_2d(rot);

            let col = match idx {
                0 => Rgba(1.0, 0.0, 0.0, 1.0),  // Red
                1 => Rgba(0.0, 1.0, 0.0, 1.0),  // Green
                2 => Rgba(0.0, 0.0, 1.0, 1.0),  // Blue
                3 => Rgba(1.0, 1.0, 0.0, 1.0),  // Yellow
                4 => Rgba(0.727, 0.0, 1.0, 1.0),  // Purple
                _ => Rgba(1.0, 1.0, 1.0, 1.0),  // White
            };

            println!("Generated rgba {:?}", col);

            world.
                create_entity()
                .with(Bot::new())
                .with(transform)
                .with(col)
                .with(sprite_render.clone())
                .build();
        }
    }
}