extern crate gtk;
extern crate gio;
extern crate gdk;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{
    Application, ApplicationWindow,
    Button, Label, DrawingArea, Entry,
    Box,
    MenuBar,
};

fn main() {
    let application = Application::new(
        Some("com.github.jasmaa.paint-rs"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
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
        //window.add_events(gdk::EventMask::all());
        drawing_area.add_events(
            gdk::EventMask::BUTTON_PRESS_MASK |
            gdk::EventMask::BUTTON_RELEASE_MASK |
            gdk::EventMask::POINTER_MOTION_MASK
        );

        drawing_area.connect_draw(|w, c|{
            // Print dimensions for now
            println!("w:{} h:{}", w.get_allocated_width(), w.get_allocated_height());
            
            c.rectangle(1.0, 1.0, 100.0, 200.0);
            c.set_source_rgb(1.0, 0.0, 0.0);
            c.fill();

            gtk::Inhibit(true)
        });

        drawing_area.connect_button_press_event(|_, e| {
            println!("{} down", e.get_button());
            gtk::Inhibit(true)
        });
        drawing_area.connect_button_release_event(|_, e| {
            println!("{} up", e.get_button());
            gtk::Inhibit(true)
        });
        drawing_area.connect_motion_notify_event(|_, e| {
            println!("motion!");
            gtk::Inhibit(true)
        });

        container.add(&drawing_area);
        container.set_child_packing(&drawing_area, true, true, 0, gtk::PackType::Start);

        window.show_all();
    });

    application.run(&[]);
}
