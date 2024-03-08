mod converter;
mod grammar;

use std::time::Duration;

use cpal::traits::StreamTrait;
use enigo::{Enigo,KeyboardControllable};
use serde_json::Value;
use winapi::um::winuser::GetAsyncKeyState;

fn main() {

    let mut enigo = Enigo::new();
    let mut stream: Option<cpal::Stream> = None;
    let mut key_active = false;

    loop {

        // Check for our keybinds
        let keystate_one = unsafe { GetAsyncKeyState(0x12) }; // Left Alt
        let keystate_two = unsafe { GetAsyncKeyState(0xA0) }; // Left Shift

        // Keydown
        if keystate_one != 0 && keystate_two != 0 && key_active == false {
            stream = converter::write_audio();
            key_active = true;
        }

        // Keyup
        else if key_active == true {
            
            // If either key has been released
            if keystate_one == 0 || keystate_two == 0 {

                // Pause the stream
                if let Some (stream_ref) = stream.as_ref() {
                    stream_ref.pause().expect("Failed to pause the stream")
                }

                // Send the API request
                match converter::send_request() {
                    Ok(api_response) => {

                        // Parse for only the text value
                        let text_value: Value = serde_json::from_str(api_response.as_str()).expect("Failed to serialise");
                        let mut text = text_value["text"].as_str().unwrap_or_default();

                        // Check the spelling
                        match grammar::check_spelling(text) {
                            Ok(spellchecked_text) => {
                                text = spellchecked_text;
                            },
                            Err(err) => {
                                println!("Error checking the spelling {}", err);
                            }
                        }

                        // Write the &str to the screen
                        enigo.key_sequence(text);
                    }
                    Err(err) => {
                        eprintln!("Error sending request: {}", err);
                        break;
                    }
                }
                key_active = false;
            }
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}