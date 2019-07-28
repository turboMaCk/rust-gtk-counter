extern crate gtk;
extern crate gio;

use std::env::args;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label,};

fn main() {
    // The simplest way to initalize new application
    let application = Application::new(None, Default::default())
        .expect("failed to initialize GTK application");

    // UI initialization
    application.connect_activate(|app| {

        // Window
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        // Containers
        let container = Box::new(gtk::Orientation::Vertical, 3);

        // Header
        let header = gtk::HeaderBar::new();
        let label = Label::new(Some("Starting at 0"));

        // Content
        let inc_btn = Button::new_with_label("Increment");
        {
            let label_clone = label.clone();
            inc_btn.connect_clicked(move |_| {
                label_clone.set_label("Incremented");
            });
        }
        let dec_btn = Button::new_with_label("Decrement");
        {
            let label_clone = label.clone();
            dec_btn.connect_clicked(move |_| {
                label_clone.set_label("Decremented");
            });
        }

        // Compose piece together
        header.add(&label);
        container.add(&header);
        container.add(&inc_btn);
        container.add(&dec_btn);
        window.add(&container);

        // Display all widgets
        window.show_all();
    });

    // Passing arguments to the app
    application.run(&args().collect::<Vec<_>>());
}
