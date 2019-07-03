use specs::{ReadStorage, System};

use crate::component::Pose;

pub struct UpdatePose;

impl<'a> System<'a> for UpdatePose {
    type SystemData = ReadStorage<'a, Pose>;

    fn run(&mut self, pose: Self::SystemData) {
        use specs::Join;

        for pose in pose.join() {
            println!("Hello, {:?}", &pose);
        }
    }
}
