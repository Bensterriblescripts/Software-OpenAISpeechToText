mod converter;

use winapi::um::winuser::GetAsyncKeyState;

fn main() {

    let mut key_active = false;

    loop {
        // Check for our keybind
        let keystate = unsafe { GetAsyncKeyState(0x12) }; // Left Alt

        // Run based on keystate
        if keystate != 0 && key_active == false {
            // converter::audio_return();
            println!("Key down");
            key_active = true;
        }
        else if keystate == 0 && key_active == true {
            println!("Key up");
            key_active = false;
        }
    }
}