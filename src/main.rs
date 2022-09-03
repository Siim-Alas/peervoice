
use gtk::prelude::*;
use gtk::Application;

mod ui;

const APP_ID: &str = "com.github.Siim-Alas.peervoice";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build);
    app.run();
}
