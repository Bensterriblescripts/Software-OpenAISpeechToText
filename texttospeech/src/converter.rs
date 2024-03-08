use std::fs;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::env;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

pub fn write_audio() -> Option<cpal::Stream> {

    // Device
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");
    let supported_config = device.default_input_config().expect("Failed to get default input format");
    let config: cpal:: StreamConfig = supported_config.into();

    // Wav file
    let mut file = hound::WavWriter::create("output.wav", hound::WavSpec {
        channels: config.channels as u16,
        sample_rate: config.sample_rate.0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,

    }).expect("Failed to create the wav file.");

    // Input Stream
    let timeout = Some(Duration::from_secs(3));
    let stream = device
        .build_input_stream(
            &config,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    file.write_sample(sample).expect("Failed to write audio data to file");
                }
            },
            move |err| eprintln!("Error in stream: {}", err),
            timeout,
        )
        .expect("Failed to build input stream");

    println!("Device: {:?}", device.name().unwrap());
    println!("Supported Config: {:?}", config);

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
    match fs::remove_file("output.wav") {
        Ok(_) => (),
        Err(e) => eprintln!("Error deleting the wav file: {}", e)
    }

    Ok(response_text)
}