
use cpal::traits::{DeviceTrait, HostTrait};

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

    pub fn start_input<FI16, FU16, FF32>(&mut self, cb_i16: FI16, cb_u16: FU16, cb_f32: FF32) where
        FI16: FnMut(&[i16], &cpal::InputCallbackInfo) + std::marker::Send + 'static,
        FU16: FnMut(&[u16], &cpal::InputCallbackInfo) + std::marker::Send + 'static,
        FF32: FnMut(&[f32], &cpal::InputCallbackInfo) + std::marker::Send + 'static,
    {
        if self.input_stream.is_some() {
            return;
        }

        if let Some((input_config, input_stream)) = start_input_internal(cb_i16, cb_u16, cb_f32) {
            self.input_config = Some(input_config);
            self.input_stream = Some(input_stream);
        }
    }

    pub fn start_output<FI16, FU16, FF32>(&mut self, cb_i16: FI16, cb_u16: FU16, cb_f32: FF32) where
        FI16: FnMut(&mut [i16], &cpal::OutputCallbackInfo) + std::marker::Send + 'static,
        FU16: FnMut(&mut [u16], &cpal::OutputCallbackInfo) + std::marker::Send + 'static,
        FF32: FnMut(&mut [f32], &cpal::OutputCallbackInfo) + std::marker::Send + 'static,
    {
        if self.output_stream.is_some() {
            return;
        }

        if let Some((output_config, output_stream)) = start_output_internal(cb_i16, cb_u16, cb_f32) {
            self.output_config = Some(output_config);
            self.output_stream = Some(output_stream);
        }
    }
}

fn start_input_internal<FI16, FU16, FF32>(cb_i16: FI16, cb_u16: FU16, cb_f32: FF32) ->
    Option<(cpal::SupportedStreamConfig, cpal::Stream)>
where
    FI16: FnMut(&[i16], &cpal::InputCallbackInfo) + std::marker::Send + 'static,
    FU16: FnMut(&[u16], &cpal::InputCallbackInfo) + std::marker::Send + 'static,
    FF32: FnMut(&[f32], &cpal::InputCallbackInfo) + std::marker::Send + 'static,
{
    let host = cpal::default_host();
    let input_device = host.default_input_device()?;
    let input_config = input_device.default_input_config().ok()?;
    let onerr = |stream_error| eprintln!("{:?}", stream_error);
    let input_stream = match input_config.sample_format() {
        cpal::SampleFormat::I16 => input_device.build_input_stream(&input_config.config(), cb_i16, onerr),
        cpal::SampleFormat::U16 => input_device.build_input_stream(&input_config.config(), cb_u16, onerr),
        cpal::SampleFormat::F32 => input_device.build_input_stream(&input_config.config(), cb_f32, onerr),
    }.ok()?;

    Some((input_config, input_stream))
}

fn start_output_internal<FI16, FU16, FF32>(cb_i16: FI16, cb_u16: FU16, cb_f32: FF32) ->
    Option<(cpal::SupportedStreamConfig, cpal::Stream)>
where
    FI16: FnMut(&mut [i16], &cpal::OutputCallbackInfo) + std::marker::Send + 'static,
    FU16: FnMut(&mut [u16], &cpal::OutputCallbackInfo) + std::marker::Send + 'static,
    FF32: FnMut(&mut [f32], &cpal::OutputCallbackInfo) + std::marker::Send + 'static,
{
    let host = cpal::default_host();
    let output_device = host.default_output_device()?;
    let output_config = output_device.default_output_config().ok()?;
    let onerr = |stream_error| eprintln!("{:?}", stream_error);
    let output_stream = match output_config.sample_format() {
        cpal::SampleFormat::I16 => output_device.build_output_stream(&output_config.config(), cb_i16, onerr),
        cpal::SampleFormat::U16 => output_device.build_output_stream(&output_config.config(), cb_u16, onerr),
        cpal::SampleFormat::F32 => output_device.build_output_stream(&output_config.config(), cb_f32, onerr),
    }.ok()?;

    Some((output_config, output_stream))
}

