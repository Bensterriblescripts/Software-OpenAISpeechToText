use std::{fs, mem, u32};
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::env;

use cpal::traits::*;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

pub fn write_input_audio() -> Option<cpal::Stream> {

    // Device - Input
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");
    let supported_config = device
        .default_input_config()
        .expect("Failed to get default input format");
    let config = cpal::StreamConfig {
        channels: supported_config.channels(),
        sample_rate: supported_config.sample_rate(),
        buffer_size: cpal::BufferSize::Default
    };


    // Wav file
    let mut file = hound::WavWriter::create("output.wav", hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,

    }).expect("Failed to create the wav file.");

    let timeout = Duration::from_secs(3);

    // Input Stream
    let stream = device
        .build_input_stream(
            &config,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    file.write_sample(sample).expect("Failed to write audio data to file");
                }
            },
            move |err| eprintln!("Error in stream: {}", err),
            Some(timeout)
        )
        .expect("Failed to build input stream");

        println!("Device: {:?}", device.name().unwrap());

    // Record the audio
    stream.play().expect("Failed to start stream");

    return Some(stream);
}
pub fn write_output_audio() -> Option<cpal::Stream> {

    let available_hosts = cpal::OutputDevices();
    println!("Available hosts: {:?}", available_hosts);

    // Device - Output
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Failed to get default output device");
    let supported_config = device
        .default_output_config()
        .expect("Failed to get default output format");
    let config = cpal::StreamConfig {
        channels: supported_config.channels(),
        sample_rate: supported_config.sample_rate(),
        buffer_size: cpal::BufferSize::Default
    };

    // Wav file
    let mut file = hound::WavWriter::create("output.wav", hound::WavSpec {
        channels: supported_config.channels(),
        sample_rate: supported_config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,

    }).expect("Failed to create the wav file.");

    let timeout = Duration::from_secs(3);

    // Output Stream
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                for sample in data {
                    file.write_sample(*sample).expect("Failed to write audio data to file");
                }
            },
            move |err| eprintln!("Error in stream: {}", err),
            Some(timeout)
        )
        .expect("Failed to build output stream");

    println!("Device: {:?}", device.name().unwrap());

    // Record the audio
    stream.play().expect("Failed to start stream");

    return Some(stream);
}

// Send the OpenAI request
pub fn send_request() -> Result<String, Box<dyn std::error::Error>> {

    let apikey = env::var("openai")?;

    // Request Headers
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", apikey))?);

    // Audio File
    let filepath = "output.wav";
    let mut file = File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Set up the form with the file
    let form = reqwest::blocking::multipart::Form::new()
        .part("file", reqwest::blocking::multipart::Part::bytes(buffer).file_name("output.wav"))
        .part("model", reqwest::blocking::multipart::Part::text("whisper-1"));

    // Make the HTTP POST request to the OpenAI API
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .headers(headers)
        .multipart(form)
        .send()?;

    let response_text = response.text().unwrap();

    // Delete the wav file
    // match fs::remove_file("output.wav") {
    //     Ok(_) => (),
    //     Err(e) => eprintln!("Error deleting the wav file: {}", e)
    // }

    Ok(response_text)
}