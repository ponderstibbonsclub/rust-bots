#[macro_use]
extern crate conrod;

use conrod::{widget, Widget};
use conrod::widget::triangles::{Triangle, ColoredPoint};
use conrod::backend::glium::glium::{self, Surface};

use specs::{DispatcherBuilder, WorldExt, Join};

mod bot;
mod component;
mod system;
mod world;

use bot::Bot;
use system::UpdatePose;
use world::BotWorld;
use component::{Colour, Pose};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const START_BOTS: u32 = 400;

pub fn main() {

    // Construct world containing bots
    let mut world = BotWorld::create();
    let mut dispatcher = DispatcherBuilder::new()
        .with(UpdatePose, "update_pose", &[])
        .build();
    dispatcher.dispatch(&mut world);
    world.maintain();

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

            // TODO
            // Get ECS to draw directly to screen through a system to avoid read_storage

            let pos_storage = world.read_storage::<Pose>();
            let col_storage = world.read_storage::<Colour>();

            let triangles: Vec<Triangle<ColoredPoint>> = (&pos_storage, &col_storage)
                .join()
                .map(|(pose, col)| Bot::to_triangle_raw(pose.pos, pose.rot, col.colour))
                .collect();

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
