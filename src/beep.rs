use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, Sample, SizedSample, Device};
use crate::{Error, Result};

struct Tone {
    hertz: u32, millis: Option<u32>
}

impl Default for Tone {
    fn default() -> Self {
        Self {hertz: 44, millis: Some(1000)}
    }
}

/// Play an indefinite tone of a given hertz.
/// Beeps the PC speaker at the given frequency
#[allow(dead_code)] // TODO: Make this work & make it visible
pub fn beep(hertz: u32) -> Result<()> {
    let tone = Tone {hertz, millis: None};
    play(None, &tone)?;
    Ok(())}

/// Beeps synchronously.
///
/// When called with a frequency in hz, & a duration in milliseconds,
/// produce a sine wave tone. Blocks while beeping. Returns errors
/// if there are issues in the beeping process.
pub fn beep_with_hz_and_millis(hertz: u32, millis: u32) -> Result<()> {
    let tone = Tone {hertz, millis: Some(millis)};
    play(None, &tone)?;
    Ok(())}

fn play(device_name: Option<&str>, tone: &Tone) -> Result<()> {
    let host = cpal::default_host();
    let device_name = device_name.unwrap_or("default");
    let device =if device_name == "default" {
        host.default_output_device()
    } else {
        host.output_devices().map_err(|e|Error::CpalError(Box::new(e)))?
            .find(|x| x.name().map(|y| y == device_name).unwrap_or(false))
    }.ok_or_else(
        || {Error::NoDevice(device_name.to_string())}
    )?;

    play_from_device(&device, tone)?;

    Ok(())
}

fn play_from_device(device: &Device, tone: &Tone) -> Result<()> {

    let config = device.default_output_config().map_err(
        |e| Error::CpalError(Box::new(e))
    )?;

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into(), tone),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), tone),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into(), tone),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into(), tone),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into(), tone),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into(), tone),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into(), tone),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), tone),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into(), tone),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into(), tone),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into(), tone),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into(), tone),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), tone),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into(), tone),
        sample_format => Err(Error::UnsupportedSampleFormat(sample_format)),
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig,
          tone: &Tone) -> Result<()>
    where
        T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;
    let frequency: f32 = tone.hertz as f32;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * frequency * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |_err| {};

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    ).map_err(|e| Error::CpalError(Box::new(e)))?;
    stream.play().map_err(
        |e| Error::CpalError(Box::new(e))
    )?;

    if let Some(millis) = tone.millis {
        std::thread::sleep(std::time::Duration::from_millis(millis as u64));
    } else {
        // TODO: do something about forever tones.
        todo!()
    }

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
    where
        T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    // }
}
