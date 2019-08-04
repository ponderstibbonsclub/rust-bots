use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};
use na::Vector3;

use crate::components::Size;

pub struct ScaleSystem;

impl<'s> System<'s> for ScaleSystem {
    type SystemData = (
        ReadStorage<'s, Size>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (sizes, mut transforms, _time): Self::SystemData) {
        // Scale every entity according to its size.
        for (size, transform) in (&sizes, &mut transforms).join() {
            transform.set_scale(Vector3::new(size.size, size.size, 1.0));
        }
    }
}
