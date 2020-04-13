//! A collection of semi-random shape and image drawing examples.

mod field;
mod gui;

use gui::ImGuiWrapper;
use field::Field;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawParam};
use ggez::{Context, GameResult, input};
use std::env;
use std::path;
use graphics::{TextFragment, Text, Scale};
use event::{MouseButton};

const RECT_SIZE: f32 = 50.0;
struct MainState {
    field: Field,
    gui: ImGuiWrapper,
    hidpi_factor: f32
}

impl MainState {
    /// Load images and create meshes.
    fn new(ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainState> {
        let field = Field::new(5, 5);
        let color_palette = &field.color_palette;
        let gui = ImGuiWrapper::new(ctx, color_palette);
        let s = MainState {
            field,
            gui,
            hidpi_factor
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
                        let text = TextFragment::new(i.to_string()).scale(Scale::uniform(50.0));
                        graphics::queue_text(ctx, &Text::new(text), rect.point() , Some(graphics::WHITE));
                        let rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.0), rect, graphics::WHITE)?;
                        graphics::draw(ctx, &rect, DrawParam::default())?
                    }
                }
            }
        }
        graphics::draw_queued_text(ctx, DrawParam::default(), None, graphics::FilterMode::Linear)?;
        self.gui.render(ctx, self.hidpi_factor);
        // Finished drawing, show it all on the screen!
        graphics::present(ctx)?;
        Ok(())
    }
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.gui.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.gui.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.gui.update_mouse_down((false, false, false));
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
    let hidpi_factor = events_loop.get_primary_monitor().get_hidpi_factor() as f32;

    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx, hidpi_factor).unwrap();
    event::run(ctx, events_loop, state)
}
