// main.rs

extern crate gtk;
extern crate gio;
extern crate gdk;
extern crate gdk_pixbuf;
mod drawing;

use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gio::prelude::*;
use gdk::prelude::*;
use gtk::{
    Application, ApplicationWindow,
    Button, DrawingArea,
    Box,
    MenuBar,
};
use gdk_pixbuf::Pixbuf;

use crate::drawing::Drawing;


fn main() {
    let application = Application::new(
        Some("com.github.jasmaa.paint-rs"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {

        // Data
        let is_pointer_down = Rc::new(RefCell::new(false));
        let is_pointer_down_up = is_pointer_down.clone();
        let is_pointer_down_down = is_pointer_down.clone();
        let is_pointer_down_draw = is_pointer_down.clone();

        let buffer = Rc::new(RefCell::new(
            {
                // Init buffer
                let buf = Pixbuf::new(
                    gdk_pixbuf::Colorspace::Rgb,
                    false,
                    8,
                    400, 300,
                ).unwrap();
                buf.fill(0xffffff00);
                buf
            }
        ));
        let buffer_draw = buffer.clone();
        let buffer_move = buffer.clone();
        let buffer_press = buffer.clone();

        let drawing = Rc::new(RefCell::new(
            Drawing::new()
        ));
        let drawing_move = drawing.clone();
        let drawing_press = drawing.clone();
        let drawing_release = drawing.clone();

        // Window
        let window = ApplicationWindow::new(app);
        window.set_title("Paint");
        window.set_default_size(400, 300);

        let container = Box::new(gtk::Orientation::Vertical, 5);
        window.add(&container);

        let drawing_bar = Box::new(gtk::Orientation::Horizontal, 5);
        container.add(&drawing_bar);

        // Drawing bar
        let brush_button = Button::new_with_label("Brush");
        brush_button.connect_clicked(|_| {
            println!("Clicked!");
        });
        drawing_bar.add(&brush_button);

        let eraser_button = Button::new_with_label("Eraser");
        eraser_button.connect_clicked(|_| {
            println!("Clicked!");
        });
        drawing_bar.add(&eraser_button);

        let color1_button = Button::new_with_label("Color 1");
        color1_button.connect_clicked(|_| {
            println!("Clicked!");
        });
        drawing_bar.add(&color1_button);

        let color2_button = Button::new_with_label("Color 2");
        color2_button.connect_clicked(|_| {
            println!("Clicked!");
        });
        drawing_bar.add(&color2_button);

        // Drawing area
        let drawing_area = DrawingArea::new();
        drawing_area.add_events(
            gdk::EventMask::BUTTON_PRESS_MASK |
            gdk::EventMask::BUTTON_RELEASE_MASK |
            gdk::EventMask::POINTER_MOTION_MASK
        );

        drawing_area.connect_draw(move |_, c|{
            // Update buffer
            let buffer = buffer_draw.borrow();
            c.set_source_pixbuf(&buffer, 0.0, 0.0);
            c.paint();
            c.stroke();
            gtk::Inhibit(false)
        });
        drawing_area.connect_button_press_event(move |w, e| {
            *is_pointer_down_up.borrow_mut() = true;
            let (x, y) = e.get_position();
            println!("pointer at: {} {}", x, y);

            // Draw
            let buffer = buffer_press.borrow();
            let mut drawing = drawing_press.borrow_mut();
            drawing.draw(e.get_position(), &buffer);
            w.queue_draw();

            gtk::Inhibit(false)
        });
        drawing_area.connect_button_release_event(move |w, _| {
            *is_pointer_down_down.borrow_mut() = false;

            let mut drawing = drawing_release.borrow_mut();
            drawing.clear_previous();
            w.queue_draw();

            gtk::Inhibit(false)
        });
        drawing_area.connect_motion_notify_event(move |w, e| {
            if *is_pointer_down_draw.borrow() {
                let (x, y) = e.get_position();
                println!("pointer at: {} {}", x, y);

                // Draw
                let buffer = buffer_move.borrow();
                let mut drawing = drawing_move.borrow_mut();
                drawing.draw(e.get_position(), &buffer);
                w.queue_draw();
            }
            gtk::Inhibit(false)
        });

        container.add(&drawing_area);
        container.set_child_packing(&drawing_area, true, true, 0, gtk::PackType::Start);

        window.show_all();
    });

    application.run(&[]);
}
