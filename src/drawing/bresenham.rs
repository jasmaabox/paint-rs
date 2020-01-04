// bresenham.rs
// Plotting for Bresenham line algo

extern crate gdk_pixbuf;

use gdk_pixbuf::Pixbuf;

// TODO: Add a closure for brushes
pub fn plot(buffer: &Pixbuf, x0: i32, y0: i32, x1: i32, y1: i32) {
    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            plot_low(buffer, x1, y1, x0, y0);
        }
        else {
            plot_low(buffer, x0, y0, x1, y1);
        }
    }
    else {
        if y0 > y1 {
            plot_high(buffer, x1, y1, x0, y0);
        }
        else {
            plot_high(buffer, x0, y0, x1, y1);
        }
    }
}

fn plot_low(buffer: &Pixbuf, x0: i32, y0: i32, x1: i32, y1: i32) {
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }

    let mut D = 2*dy - dx;
    let mut y = y0;
    for x in x0..x1 {
        buffer.put_pixel(x, y, 255, 0, 0, 0);
        if D > 0 {
            y += yi;
            D -= 2*dx;
        }
        D += 2*dy;
    }
}

fn plot_high(buffer: &Pixbuf, x0: i32, y0: i32, x1: i32, y1: i32) {
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }

    let mut D = 2*dx - dy;
    let mut x = x0;
    for y in y0..y1 {
        buffer.put_pixel(x, y, 255, 0, 0, 0);
        if D > 0 {
            x += xi;
            D -= 2*dy;
        }
        D += 2*dx;
    }
}