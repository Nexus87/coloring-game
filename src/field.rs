use rand::Rng;
use ggez::graphics::{self, Color};
use std::{rc::Rc, collections::HashMap};

pub enum Cell {
    Colored (Color),
    Empty (i32)
}
pub struct Field {
    pub width: usize,
    pub height: usize,
    content: Vec<Cell>,
    pub color_palette: Rc<HashMap<i32, Color>>
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        let len = width * height;
        let mut content = Vec::with_capacity(len);
        let mut rng = rand::thread_rng();
        for _ in 0..len {
            content.push(Cell::Empty(rng.gen_range(0, 5)));
        }
        let mut color_palette = HashMap::new();
        color_palette.insert(0, graphics::WHITE);
        color_palette.insert(1, graphics::BLACK);
        color_palette.insert(2, Color::from_rgb(66, 135, 245));
        color_palette.insert(3, Color::from_rgb(217, 53, 24));
        color_palette.insert(4, Color::from_rgb(13, 219, 58));
        color_palette.insert(5, Color::from_rgb(252, 252, 38));
        Field {
            width,
            height,
            content,
            color_palette: Rc::new(color_palette)
        }
    }

    pub fn point<'a>(&'a self, row: usize, column: usize) -> &'a Cell {
        let idx = (row * self.width) + column;
        &self.content[idx]
    }

    pub fn color(&mut self, row: usize, column: usize) {
        if row >= self.height || column >= self.width {
            return;
        }
        let idx = (row * self.width) + column;
        let cell = &self.content[idx];
        if let Cell::Empty(i) = cell{
            let color = self.color_palette.get(i).unwrap_or(&graphics::BLACK);
            self.content[idx] = Cell::Colored(color.clone());
        }
    }
}