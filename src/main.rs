#[macro_use]
extern crate conrod;

use conrod::{color, widget, Widget};
use conrod::widget::triangles::{Triangle, ColoredPoint};
use conrod::backend::glium::glium::{self, Surface};

use color::Color;

use rand::Rng;

#[derive(Debug)]
struct Bot {
    x: f64,
    y: f64,
    rot: f64,  // Degrees
    colour: color::Color,
}

impl Bot {
    fn new(x: f64, y: f64, rot: f64, colour: Color) -> Bot {
        Bot { x, y, rot, colour }
    }

    fn random(x_max: u32, y_max: u32) -> Bot {
        let mut rng = rand::thread_rng();
        let x_lim = x_max as f64 / 2.0;
        let y_lim = y_max as f64 / 2.0;
        let x = rng.gen_range(-x_lim, x_lim);
        let y = rng.gen_range(-y_lim, y_lim);
        let rot = rng.gen_range(0.0, 360.0);
        let col_r = rng.gen_range(0.0, 1.0);
        let col_g = rng.gen_range(0.0, 1.0);
        let col_b = rng.gen_range(0.0, 1.0);
        let col = Color::from(color::Rgba(col_r, col_g, col_b, 1.0));
        Bot::new(x, y, rot, col)
    }

    fn modified(index: u32) -> Bot {
        let x = index as f64 * 100.0;
        let y = index as f64 * 100.0;
        let rot = index as f64 * 30.0;
        let col = match index {
            0 => color::RED,
            1 => color::GREEN,
            2 => color::BLUE,
            3 => color::YELLOW,
            4 => color::PURPLE,
            _ => color::WHITE
        };
        Bot::new(x, y, rot, col)
    }

    fn l_b(&self) -> ColoredPoint {
        ([self.x - 5.0, self.y - 5.0], self.colour.into())
    }

    fn r_b(&self) -> ColoredPoint {
        ([self.x + 5.0, self.y - 5.0], self.colour.into())
    }

    fn top(&self) -> ColoredPoint {
        ([self.x, self.y + 5.0], self.colour.into())
    }

    fn to_triangle(&self) -> Triangle<ColoredPoint> {
        Triangle([ self.l_b(), self.r_b(), self.top() ])
    }
}

pub fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    const START_BOTS: u32 = 5;

    // Construct a list of random bots
    let bots : Vec<Bot> = (0..START_BOTS).map(|_| Bot::random(WIDTH, HEIGHT)).collect();
    // let bots : Vec<Bot> = (0..5).map(|index| Bot::modified(index)).collect();
    println!("Generated following bots:");
    for bot in &bots {
        println!("{:?}", bot);
    }

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Triangles!")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Generate the widget identifiers.
    widget_ids!(struct Ids { triangles });
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    const FONT_PATH: &'static str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(FONT_PATH).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    events_loop.run_forever(|event| {

        match event.clone() {
            glium::glutin::Event::WindowEvent { event, .. } => match event {

                // Break from the loop upon `Escape` or closed window.
                glium::glutin::WindowEvent::CloseRequested |
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => return glium::glutin::ControlFlow::Break,

                _ => (),
            },
            _ => (),
        }

        // Use the `winit` backend feature to convert the winit event to a conrod one.
        let input = match conrod::backend::winit::convert_event(event, &display) {
            None => return glium::glutin::ControlFlow::Continue,
            Some(input) => input,
        };

        // Handle the input with the `Ui`.
        ui.handle_event(input);

        // Set the triangle widget.
        {
            let ui = &mut ui.set_widgets();
            let rect = ui.rect_of(ui.window).unwrap();
            // Disabled as triangles no longer fill screen
            // let (l, r, b, t) = rect.l_r_b_t();
            // let (c1, c2, c3) = (color::RED.to_rgb(), color::GREEN.to_rgb(), color::BLUE.to_rgb());

            let triangles: Vec<Triangle<ColoredPoint>> = bots.iter().map(|bot| bot.to_triangle()).collect();

            widget::Triangles::multi_color(triangles.iter().cloned())
                .with_bounding_rect(rect)
                .set(ids.triangles, ui);
        }

        // Draw the `Ui` if it has changed.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }

        glium::glutin::ControlFlow::Continue
    });
}
