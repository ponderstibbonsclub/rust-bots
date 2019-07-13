use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use std::f32::consts::PI;
use crate::components::Velocity;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (vels, mut positions, time): Self::SystemData) {
        // Move every entity according to its speed, and the time passed.
        for (vel, position) in (&vels, &mut positions).join() {
            let mov = vel.vel;
            let rot = mov.x.atan2(mov.y);

            position.prepend_translation_x(mov.x * time.delta_seconds());
            position.prepend_translation_y(mov.y * time.delta_seconds());

            // Face in direction of travel. Amethyst rotates from y axis, for some reason
            position.set_rotation_2d(rot);
            position.rotate_2d(PI/2.0);
        }
    }
}
