use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;

fn main() {
    let app = Application::builder()
        .application_id("org.plok.sdr")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(1920)
            .default_height(1080)
            .title("Hello, World!")
            .build();

        // Show the window.
        window.show();
    });

    app.run();
}
