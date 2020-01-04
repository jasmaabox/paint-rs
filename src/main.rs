extern crate gtk;
extern crate gio;
extern crate gdk;

use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{
    Application, ApplicationWindow,
    Button, DrawingArea,
    Box,
    MenuBar,
};

fn main() {
    let application = Application::new(
        Some("com.github.jasmaa.paint-rs"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {

        let is_pointer_down = Rc::new(RefCell::new(false));
        let is_pointer_down_up = is_pointer_down.clone();
        let is_pointer_down_down = is_pointer_down.clone();
        let is_pointer_down_draw = is_pointer_down.clone();

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
            c.rectangle(1.0, 1.0, 100.0, 200.0);
            c.set_source_rgb(1.0, 0.0, 0.0);
            c.fill();

            gtk::Inhibit(false)
        });
        drawing_area.connect_button_press_event(move |_, e| {
            *is_pointer_down_up.borrow_mut() = true;
            let (x, y) = e.get_position();
            println!("pointer at: {} {}", x, y);
            gtk::Inhibit(false)
        });
        drawing_area.connect_button_release_event(move |_, _| {
            *is_pointer_down_down.borrow_mut() = false;
            gtk::Inhibit(false)
        });
        drawing_area.connect_motion_notify_event(move |_, e| {
            if *is_pointer_down_draw.borrow_mut() {
                let (x, y) = e.get_position();
                println!("pointer at: {} {}", x, y);
            }
            gtk::Inhibit(false)
        });

        container.add(&drawing_area);
        container.set_child_packing(&drawing_area, true, true, 0, gtk::PackType::Start);

        window.show_all();
    });

    application.run(&[]);
}
