// drawing.rs

extern crate gdk_pixbuf;
mod bresenham;

use gdk_pixbuf::Pixbuf;

enum PointerOption {
    Brush,
    Pencil,
    Eraser,
}

pub struct Drawing {
    pointer_option: PointerOption,
    previous_point: Option<(f64, f64)>,
}

impl Drawing {
    pub fn new() -> Self {
        Self {
            pointer_option: PointerOption::Pencil,
            previous_point: None,
        }
    }

    pub fn clear_previous(&mut self) {
        self.previous_point = None;
    }

    pub fn draw(&mut self, current_point: (f64, f64), buffer: &Pixbuf) {
        match self.previous_point {
            Some(previous_point) => {
                // Bresenham
                bresenham::plot(
                    buffer,
                    previous_point.0 as i32, previous_point.1 as i32,
                    current_point.0 as i32, current_point.1 as i32,
                );
                
            },
            None => {
                let (x, y) = current_point;
                buffer.put_pixel(x as i32, y as i32, 255, 0, 0, 0);
            }
        }

        self.previous_point = Some(current_point);
    }
}