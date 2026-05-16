use std::io::{self, Write};
use std::time::Duration;
use std::thread::sleep;

pub fn animation_loading() { // animation for loading
    let frames = ["|", "/", "-", "\\"]; // frames for loading
    for i in 0..20 { // loop for loading
        print!("\r[+] Loading {} ", frames[i % 4]);
        io::stdout().flush().unwrap(); // flush output
        std::thread::sleep(std::time::Duration::from_millis(100)); // sleep for 100ms
    }
    println!("\r[+] Meme started          ");
}


pub fn shaking_screen() {
    // Move cursor left and right rapidly
    for _ in 0..5 {
        print!("\x1b[1D"); // Move left hahha
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(50));
        print!("\x1b[1C"); // Move right hahha
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(50));
    }

}