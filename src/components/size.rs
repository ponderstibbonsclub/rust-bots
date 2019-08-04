use amethyst::{
    ecs::prelude::{Component, VecStorage},
};

#[derive(Clone, Debug)]
pub struct Size {
    pub size: f32,
}

impl Size {
    pub fn new(size: f32) -> Size {
        Size {
            size: size
        }
    }
}

impl Component for Size {
    type Storage = VecStorage<Self>;
}
