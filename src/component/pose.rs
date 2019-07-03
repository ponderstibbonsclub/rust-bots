use nalgebra::Vector2;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Pose {
    pos: Vector2<f64>,
    rot: f64,  // Degrees anti-clockwise
}

impl Component for Pose {
    type Storage = VecStorage<Self>;
}

impl Pose {
    pub fn new(pos: Vector2<f64>, rot: f64) -> Pose {
        Pose { pos, rot }
    }
}
