use specs::{Builder, World};
use specs::WorldExt;

use crate::component::Pose;
use crate::component::Colour;
use crate::bot::Bot;
// use crate::{WIDTH, HEIGHT, START_BOTS};

pub struct BotWorld;

impl BotWorld {

    pub fn create() -> World {
        // Create and register components to world
        let mut world = World::new();
        world.register::<Pose>();
        world.register::<Colour>();

        // Construct a list of random bots
        // let bots : Vec<Bot> = (0..START_BOTS).map(|_| Bot::random(WIDTH, HEIGHT)).collect();
        let bots : Vec<Bot> = (0..5).map(|index| Bot::modified(index)).collect();
        println!("Generated following bots:");
        for bot in &bots {
            println!("{:?}", bot);
        }

        // Create entities within world
        for bot in bots {
            world
                .create_entity()
                .with(Pose::new(bot.pos, bot.rot))
                .with(Colour::from_color(bot.colour))
                .build();
        }

        world
    }

}
