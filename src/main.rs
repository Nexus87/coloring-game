//! A collection of semi-random shape and image drawing examples.

mod field;

use field::Field;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawParam};
use ggez::{Context, GameResult, input};
use std::env;
use std::path;
use graphics::Text;

const RECT_SIZE: f32 = 50.0;
struct MainState {
    field: Field
}

impl MainState {
    /// Load images and create meshes.
    fn new(_: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            field: Field::new(5, 5)
        };
        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let button = input::mouse::button_pressed(ctx, input::mouse::MouseButton::Left);
        if button {
            let pos = input::mouse::position(ctx);
            let column = (pos.x / RECT_SIZE).floor() as usize;
            let row = (pos.y / RECT_SIZE).floor() as usize;

            self.field.color(row, column);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.33, 0.39, 0.41, 1.0].into());
        let field = &self.field;
        for i in 0.. field.width {
            for j in 0 .. field.height {
                let rect = graphics::Rect::new(i as f32 * RECT_SIZE, j as f32 * RECT_SIZE, RECT_SIZE, RECT_SIZE);
                match field.point(j, i) {
                    field::Cell::Colored(c) => { 
                        let rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, c.clone())?;
                        graphics::draw(ctx, &rect, DrawParam::default())?;
                    }
                    field::Cell::Empty(i) => {
                        graphics::queue_text(ctx, &Text::new(i.to_string()), rect.point() , Some(graphics::WHITE));
                        let rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.0), rect, graphics::WHITE)?;
                        graphics::draw(ctx, &rect, DrawParam::default())?
                    }
                }
            }
        }
        graphics::draw_queued_text(ctx, DrawParam::default(), None, graphics::FilterMode::Linear)?;
        // Finished drawing, show it all on the screen!
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, events_loop, state)
}
