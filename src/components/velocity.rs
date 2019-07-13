use na::Vector2;
use amethyst::{
    ecs::prelude::{Component, VecStorage},
};

#[derive(Clone, Debug)]
pub struct Velocity {
    pub vel: Vector2<f32>,
}

impl Velocity {
    pub fn new(vel_x: f32, vel_y: f32) -> Velocity {
        Velocity {
            vel: Vector2::new(vel_x, vel_y)
        }
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
