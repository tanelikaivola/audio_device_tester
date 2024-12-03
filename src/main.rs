#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]

mod display;
use display::CPALString;

use itertools::Itertools;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample,
};
use std::collections::HashMap;
use std::time::Instant;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> Result<(), anyhow::Error> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    println!("Supported hosts:\n  {:?}", cpal::ALL_HOSTS);
    let available_hosts = cpal::available_hosts();
    println!("Available hosts:\n  {available_hosts:?}");

    for host_id in available_hosts {
        let mut input_rates: HashMap<_, Vec<_>> = HashMap::new();
        let mut output_rates: HashMap<_, Vec<_>> = HashMap::new();
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        println!("{}", host_id.name());
        stdout.reset()?;
        let host = cpal::host_from_id(host_id)?;

        if let Some(default_in) = host.default_input_device().map(|e| e.name().unwrap()) {
            println!("  Default Input Device: {default_in}");
        }
        if let Some(default_out) = host.default_output_device().map(|e| e.name().unwrap()) {
            println!("  Default Output Device: {default_out}");
        }

        let devices = host.devices()?;
        println!("  Devices: ");
        for (device_index, device) in devices.enumerate() {
            let start = Instant::now();
            println!("  {}. \"{}\"", device_index + 1, device.name()?);

            // Input configs
            if let Ok(conf) = device.default_input_config() {
                println!(
                    "    Default input stream config:\n      {}",
                    conf.to_string()
                );
                input_rates
                    .entry(conf.sample_rate().0)
                    .or_default()
                    .push(device.name()?);
            }
            let input_configs = match device.supported_input_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    println!("    Error getting supported input configs: {e:?}");
                    Vec::new()
                }
            };
            if !input_configs.is_empty() {
                println!("    All supported input stream configs:");
                for (config_index, config) in input_configs.into_iter().enumerate() {
                    println!(
                        "      {}.{}. {}",
                        device_index + 1,
                        config_index + 1,
                        config.to_string()
                    );
                }
            }

            // Output configs
            if let Ok(conf) = device.default_output_config() {
                println!(
                    "    Default output stream config:\n      {}",
                    conf.to_string()
                );
                output_rates
                    .entry(conf.sample_rate().0)
                    .or_default()
                    .push(device.name()?);
            }
            let output_configs = match device.supported_output_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    println!("    Error getting supported output configs: {e:?}");
                    Vec::new()
                }
            };
            if !output_configs.is_empty() {
                println!("    All supported output stream configs:");
                for (config_index, config) in output_configs.into_iter().enumerate() {
                    println!(
                        "      {}.{}. {}",
                        device_index + 1,
                        config_index + 1,
                        config.to_string()
                    );
                }
            }

            let elapsed = start.elapsed();
            if elapsed.as_millis() > 2000 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                println!("    Slow device! Time taken: {elapsed:?}");
                stdout.reset()?;
            }
        }
        println!();

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        println!("Default sample rates:");
        stdout.reset()?;

        let rates = input_rates
            .keys()
            .chain(output_rates.keys())
            .sorted()
            .dedup();
        for rate in rates {
            println!("  {rate} Hz");
            input_rates
                .iter()
                .filter(|(r, _)| r == &rate)
                .for_each(|(_, names)| {
                    for name in names {
                        println!("    Input: {name}");
                    }
                });
            output_rates
                .iter()
                .filter(|(r, _)| r == &rate)
                .for_each(|(_, names)| {
                    for name in names {
                        println!("    Output: {name}");
                    }
                });
        }
        println!();

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        println!("Playing test sound on all devices...");
        stdout.reset()?;

        let devices = host.devices()?;
        for (device_index, device) in devices
            .filter(|d| d.default_output_config().is_ok())
            .enumerate()
        {
            let start = Instant::now();
            println!("  {}. \"{}\"", device_index + 1, device.name()?);

            // Output configs
            if let Ok(conf) = device.default_output_config() {
                println!("    {}", conf.to_string());
                match match conf.sample_format() {
                    cpal::SampleFormat::I16 => run::<i16>(&device, &conf.clone().into()),
                    cpal::SampleFormat::U16 => run::<u16>(&device, &conf.clone().into()),
                    cpal::SampleFormat::F32 => run::<f32>(&device, &conf.clone().into()),
                } {
                    Ok(_) => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                        println!("     Device opened successfully");
                        stdout.reset()?;
                    }
                    Err(e) => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                        println!("     Error opening device: {e}");
                        println!("     Attempted format: {:?}", conf);
                        stdout.reset()?;
                    }
                }
            }

            let elapsed = start.elapsed();
            if elapsed.as_millis() > 2000 {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                println!("    Slow device! Time taken: {elapsed:?}");
                stdout.reset()?;
            }
        }
    }

    Ok(())
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let err_fn = |err| eprintln!("an error occurred on stream: {err}");

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = Sample::from(&0.0);
            }
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}
