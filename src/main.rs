
use cpal;

mod audio;

fn on_data_received<T>(data: &[T], input_callback_info: &cpal::InputCallbackInfo) where T: cpal::Sample {
    println!("{:?}", input_callback_info);
}

fn provide_data<T>(data: &mut [T], output_callback_info: &cpal::OutputCallbackInfo) where T: cpal::Sample {
    println!("{:?}", output_callback_info);
}

fn main() {
    let mut audio_manager = audio::AudioManager::new();
    audio_manager.start_input(
        on_data_received::<i16>,
        on_data_received::<u16>,
        on_data_received::<f32>
    );
    audio_manager.start_output(
        provide_data::<i16>,
        provide_data::<u16>,
        provide_data::<f32>
    );

    loop {}
}
