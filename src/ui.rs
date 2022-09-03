
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const TITLE: &str = "PeerVoice";
const DEFAULT_WIDTH: i32 = 800;
const DEFAULT_HEIGHT: i32 = 500;

pub fn build(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .build();

    window.present();
}

