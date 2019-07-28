extern crate gtk;
extern crate gio;

use std::env::args;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label,};

use std::sync::atomic::{AtomicIsize, Ordering};

struct Counter(AtomicIsize);

impl Counter {
    fn new(init: isize) -> Counter {
        Counter(AtomicIsize::new(init))
    }

    fn increment(&self) -> isize {
        let new = self.0.load(Ordering::SeqCst) + 1;
        self.0.store(new, Ordering::SeqCst);
        new
    }

    fn decrement(&self) -> isize {
        let new = self.0.load(Ordering::SeqCst) - 1;
        self.0.store(new, Ordering::SeqCst);
        new
    }
}

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

        // State
        use std::sync::Arc;
        let counter = Arc::new(Counter::new(0));

        // Header
        let header = gtk::HeaderBar::new();
        let label = Label::new(Some("Starting at 0"));

        // Content
        let inc_btn = Button::new_with_label("Increment");
        {
            let label_clone = label.clone();
            let counter_clone = counter.clone();
            inc_btn.connect_clicked(move |_| {
                let val = counter_clone.increment();
                label_clone.set_label(&format!("Incremented to {}", val));
            });
        }
        let dec_btn = Button::new_with_label("Decrement");
        {
            let label_clone = label.clone();
            let counter_clone = counter.clone();
            dec_btn.connect_clicked(move |_| {
                let val = counter_clone.decrement();
                label_clone.set_label(&format!("Decremented to {}", val));
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
