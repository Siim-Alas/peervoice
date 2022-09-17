
use cpal::traits::DeviceTrait;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ComboBoxText};

#[path = "audio.rs"]
mod audio;

const TITLE: &str = "PeerVoice";
const DEFAULT_WIDTH: i32 = 800;
const DEFAULT_HEIGHT: i32 = 500;

const SAMPLE_RATE: cpal::SampleRate = cpal::SampleRate(48000);

pub fn build(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .build();

    /*
    unsafe {
        audio::AUDIO_MGR.start_output(|data: &mut [i16]| {
            let mut ctr = 0i16;

            for data_point in data.iter_mut() {
                *data_point = ctr;
                ctr = ((ctr + 5000 + 100) % 10000) - 5000;
            }
        });
    }
    */

    window.present();
}

