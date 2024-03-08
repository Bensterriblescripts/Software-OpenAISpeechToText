use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::env;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

pub fn audio_return() {

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
    let mut audio_data: Vec<i16> = Vec::new();

    // Input Stream
    let timeout = Some(Duration::from_secs(3));
    let stream = device
        .build_input_stream(
            &config,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    file.write_sample(sample).expect("Failed to write audio data to file");
                    audio_data.push(sample);
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
    std::thread::sleep(Duration::from_secs(3));
    stream.pause().expect("Failed to pause the stream.");

    // Send the request
    let _result = send_request();

    std::thread::sleep(std::time::Duration::from_secs(100));
}

fn send_request() -> Result<(), Box<dyn std::error::Error>> {

    let apikey = env::var("openai")?;
    let client = Client::new();

    // Request Headers
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", apikey))?);


    // Audio File
    let filepath = "output.wav";
    let mut file = File::open(filepath)?;
    println!("File: {:?}", file);
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Set up the form with the file
    let form = reqwest::blocking::multipart::Form::new()
        .part("file", reqwest::blocking::multipart::Part::bytes(buffer).file_name("output.wav"))
        .part("model", reqwest::blocking::multipart::Part::text("whisper-1"));

    // Make the HTTP POST request to the OpenAI API
    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .headers(headers)
        .multipart(form)
        .send()?;

    // Handle the API response here (e.g., print or process the result)
    println!("API Response: {}", response.text()?);

    Ok(())
}