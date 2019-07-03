use nalgebra::{Isometry2, Vector2, Point2};
use conrod::color;
use conrod::color::Color;
use conrod::widget::triangles::{Triangle, ColoredPoint};
use rand::Rng;

#[derive(Debug)]
pub struct Bot {
    pub pos: Vector2<f64>,
    pub rot: f64,  // Degrees anti-clockwise
    pub colour: Color,
}

impl Bot {
    pub fn new(pos: Vector2<f64>, rot: f64, colour: Color) -> Bot {
        Bot { pos, rot, colour }
    }

    pub fn random(x_max: u32, y_max: u32) -> Bot {
        let mut rng = rand::thread_rng();
        let x_lim = x_max as f64 / 2.0;
        let y_lim = y_max as f64 / 2.0;
        let x = rng.gen_range(-x_lim, x_lim);
        let y = rng.gen_range(-y_lim, y_lim);

        let rot: f64 = rng.gen_range(0.0, 360.0);
        let col_r = rng.gen_range(0.0, 1.0);
        let col_g = rng.gen_range(0.0, 1.0);
        let col_b = rng.gen_range(0.0, 1.0);
        let col = Color::from(color::Rgba(col_r, col_g, col_b, 1.0));
        Bot::new(Vector2::new(x, y), rot, col)
    }

    pub fn modified(index: u32) -> Bot {
        let x = index as f64 * 20.0;
        let y = index as f64 * 20.0;
        let rot: f64 = index as f64 * -30.0;
        let col = match index {
            0 => color::RED,
            1 => color::GREEN,
            2 => color::BLUE,
            3 => color::YELLOW,
            4 => color::PURPLE,
            _ => color::WHITE
        };
        Bot::new(Vector2::new(x, y), rot, col)
    }

    fn to_triangle_point(&self, x: f64, y: f64) -> Point2<f64> {
        let source = Point2::new( x, y );
        let transform = Isometry2::new(self.pos, self.rot.to_radians());

        transform * source
    }

    fn l_b(&self) -> ColoredPoint {
        let p = self.to_triangle_point( -5.0, -5.0);
        ([p.x, p.y], self.colour.into())
    }

    fn r_b(&self) -> ColoredPoint {
        let p = self.to_triangle_point( 5.0, -5.0);
        ([p.x, p.y], self.colour.into())
    }

    fn top(&self) -> ColoredPoint {
        let p = self.to_triangle_point( 0.0, 10.0);
        ([p.x, p.y], self.colour.into())
    }

    pub fn to_triangle(&self) -> Triangle<ColoredPoint> {
        Triangle([ self.l_b(), self.r_b(), self.top() ])
    }

    pub fn to_triangle_raw(pos: Vector2<f64>, rot: f64, colour: Color) -> Triangle<ColoredPoint> {
        let bot = Bot::new(pos, rot, colour);

        bot.to_triangle()
    }
}
