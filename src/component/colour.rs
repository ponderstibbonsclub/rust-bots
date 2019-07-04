use specs::{Component, VecStorage};
use conrod::color;
use conrod::color::Color;

#[derive(Clone, Debug)]
pub struct Colour {
    pub colour: Color
}

impl Component for Colour {
    type Storage = VecStorage<Self>;
}

impl Colour {
    pub fn new(r: f32, g: f32, b: f32) -> Colour {
        Colour { colour: Color::from(color::Rgba(r, g, b, 1.0)) }
    }

    pub fn from_color(colour: Color) -> Colour {
        Colour { colour}
    }
}
