
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub static mut AUDIO_MGR: AudioManager = AudioManager::new();

pub struct AudioManager {
    input_config: Option<cpal::SupportedStreamConfig>,
    input_stream: Option<cpal::Stream>,
    output_config: Option<cpal::SupportedStreamConfig>,
    output_stream: Option<cpal::Stream>
}

impl AudioManager {
    pub const fn new() -> Self {
        Self {
            input_config: None,
            input_stream: None,
            output_config: None,
            output_stream: None
        }
    }

    pub fn start_output<T, D>(&mut self, mut provide_data: D) where
        T: cpal::Sample,
        D: FnMut(&mut [T]) + std::marker::Send + 'static
    {
        if self.output_stream.is_some() {
            return;
        }

        let host = cpal::default_host();
        if let Some(output_device) = host.default_output_device() {
            if let Ok(output_config) = output_device.default_output_config() {
                let result = output_device.build_output_stream(
                    &output_config.config(),
                    move |data, output_callback_info| {
                        provide_data(data);
                        println!("{:?}", output_callback_info);
                    },
                    |stream_error| println!("Received stream error {:?}", stream_error)
                );

                match result {
                    Ok(output_stream) => {
                        match output_stream.play() {
                            Ok(_) => {
                                self.output_config.insert(output_config);
                                self.output_stream.insert(output_stream);
                            },
                            Err(play_stream_error) => {
                                println!("Received {:?}", play_stream_error);
                            }
                        }
                    },
                    Err(build_stream_error) => {
                        println!("Received {:?}", build_stream_error);
                    }
                }
            } else {
                println!("Unable to get the default output config for device {}", output_device.name().unwrap());
            }
        } else {
            println!("Unable to get the default output device for host {}", host.id().name());
        }
    }

    fn stop_input() {
    }

}

/*
pub fn suitable_input_devices(sr: cpal::SampleRate) -> Vec<cpal::Device>
{
    let suitable_sample_rate = |cfg: &cpal::SupportedStreamConfigRange| {
        cfg.min_sample_rate() <= sr && cfg.max_sample_rate() >= sr
    };

    let is_suitable_for_input = |device: &cpal::Device| {
        if let Ok(mut supported_input_configs) = device.supported_input_configs() {
            if supported_input_configs.find(suitable_sample_rate).is_some() {
                return true;
            }
        }

        false
    };

    suitable_devices(is_suitable_for_input)
}

pub fn suitable_output_devices(sr: cpal::SampleRate) -> Vec<cpal::Device>
{
    let suitable_sample_rate = |cfg: &cpal::SupportedStreamConfigRange| {
        cfg.min_sample_rate() <= sr && cfg.max_sample_rate() >= sr
    };

    let is_suitable_for_output = |device: &cpal::Device| {
        if let Ok(mut supported_output_configs) = device.supported_output_configs() {
            if supported_output_configs.find(suitable_sample_rate).is_some() {
                return true;
            }
        }

        false
    };

    suitable_devices(is_suitable_for_output)
}

pub fn suitable_devices<F>(is_suitable: F) -> Vec<cpal::Device>
    where F: Fn(&cpal::Device) -> bool
{
    let host_ids = cpal::available_hosts();
    let hosts =
        host_ids.iter().filter_map(|host_id| cpal::host_from_id(*host_id).ok());
    let devices = hosts.filter_map(|host| host.devices().ok()).flatten();
    let suitable_devices = devices.filter(|device| is_suitable(device));

    suitable_devices.collect()
}
*/

