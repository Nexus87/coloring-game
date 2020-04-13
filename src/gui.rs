use ggez::graphics;
use ggez::Context;

use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;

use imgui::*;
use imgui_gfx_renderer::*;

use std::time::Instant;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
  pos: (i32, i32),
  pressed: (bool, bool, bool),
  wheel: f32,
}

pub struct ImGuiWrapper {
  pub imgui: imgui::Context,
  pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
  last_frame: Instant,
  mouse_state: MouseState,
}

impl ImGuiWrapper {
  pub fn new(ctx: &mut Context) -> Self {
    // Create the imgui object
    let mut imgui = imgui::Context::create();
    let (factory, gfx_device, _, _, _) = graphics::gfx_objects(ctx);

    // Shaders
    let shaders = {
      let version = gfx_device.get_info().shading_language;
      if version.is_embedded {
        if version.major >= 3 {
          Shaders::GlSlEs300
        } else {
          Shaders::GlSlEs100
        }
      } else if version.major >= 4 {
        Shaders::GlSl400
      } else if version.major >= 3 {
        Shaders::GlSl130
      } else {
        Shaders::GlSl110
      }
    };

    // Renderer
    let renderer = Renderer::init(&mut imgui, &mut *factory, shaders).unwrap();


    // Register the texture with the gfx renderer
    // Create instace
    Self {
      imgui,
      renderer,
      last_frame: Instant::now(),
      mouse_state: MouseState::default(),
    }
  }

  pub fn render(&mut self, ctx: &mut Context, hidpi_factor: f32) {
    // Update mouse
    self.update_mouse();

    // Create new frame
    let now = Instant::now();
    let delta = now - self.last_frame;
    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
    self.last_frame = now;

    let (draw_width, draw_height) = graphics::drawable_size(ctx);
    self.imgui.io_mut().display_size = [draw_width, draw_height];
    self.imgui.io_mut().display_framebuffer_scale = [hidpi_factor, hidpi_factor];
    self.imgui.io_mut().delta_time = delta_s;

    let ui = self.imgui.frame();
    // Various ui things
    {
      // Window
      Window::new(im_str!("Color Picker"))
        .movable(false)
        .resizable(false)
        .title_bar(false)
        .size([draw_width - 10.0, 128.0], imgui::Condition::Once)
        .position([5.0, draw_height - 133.0], imgui::Condition::Always)
        .build(&ui, || {
          let (c1, c2, c3, c4) =graphics::BLACK.to_rgba();
          let color = ColorButton::new(im_str!("01"), [c1 as f32, c2 as f32, c3 as f32, c4 as f32])
          .size([96.0, 96.0])
          .build(&ui);
          ui.same_line(0.0);

          if color {
            println!("color button clicked");
          }
          if ui.button(im_str!("small button"), [32.0, 32.0]) {
            println!("Small button clicked");
          }
        });

    }

    // Render
    let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
    let draw_data = ui.render();
    self
      .renderer
      .render(
        &mut *factory,
        encoder,
        &mut RenderTargetView::new(render_target.clone()),
        draw_data,
      )
      .unwrap();
  }

  fn update_mouse(&mut self) {
    self.imgui.io_mut().mouse_pos = [self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32];

    self.imgui.io_mut().mouse_down = [
      self.mouse_state.pressed.0,
      self.mouse_state.pressed.1,
      self.mouse_state.pressed.2,
      false,
      false,
    ];

    self.imgui.io_mut().mouse_wheel = self.mouse_state.wheel;
    self.mouse_state.wheel = 0.0;
  }

  pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
    self.mouse_state.pos = (x as i32, y as i32);
  }

  pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
    self.mouse_state.pressed = pressed;
  }
}