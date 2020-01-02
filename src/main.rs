extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{
    Application, ApplicationWindow,
    Button, Box, Orientation
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

        let toolbar = Box::new(Orientation::Horizontal, 5);
        window.add(&toolbar);

        let brush_button = Button::new_with_label("Brush");
        brush_button.connect_clicked(|_| {
            println!("Clicked!");
        });
        toolbar.add(&brush_button);

        let eraser_button = Button::new_with_label("Eraser");
        eraser_button.connect_clicked(|_| {
            println!("Clicked!");
        });
        toolbar.add(&eraser_button);

        let color1_button = Button::new_with_label("Color 1");
        color1_button.connect_clicked(|_| {
            println!("Clicked!");
        });
        toolbar.add(&color1_button);

        let color2_button = Button::new_with_label("Color 2");
        color2_button.connect_clicked(|_| {
            println!("Clicked!");
        });
        toolbar.add(&color2_button);

        window.show_all();
    });

    application.run(&[]);
}
