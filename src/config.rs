use std::fs; 
use std::io; 
use std::io::Write; 
use crate::colors::{BOLD, CYAN, GREEN, RED, RESET, YELLOW, PURPLE, BRIGHT_CYAN, BRIGHT_GREEN, BRIGHT_YELLOW, BRIGHT_BLACK, WHITE};

pub fn configure_shell() {
    print!("\x1B[2J\x1B[1;1H"); // Clear screen
    io::stdout().flush().unwrap();

    println!("{}=== {}RustyShell Setup (Meme Shell){} ==={}", BOLD, BRIGHT_CYAN, BRIGHT_CYAN, RESET);
    println!("{}Please fill in your details to get started!{}\n", YELLOW, RESET);

    // Name Input
    let mut name = String::new();
    print!("{}[?] Enter your Name >{} ", GREEN, RESET);
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut name).unwrap();

    // Age Input
    let mut age = String::new();
    print!("{}[?] Enter your Age  >{} ", GREEN, RESET);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age).unwrap();

    // Gender Selection
    let mut gender_input = String::new();
    println!("\n{}Available Genders:{} Male, Female, Other", PURPLE, RESET);
    print!("{}[?] Enter Gender    >{} ", GREEN, RESET);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut gender_input).unwrap();

    let gender_trimmed = gender_input.trim().to_lowercase();
    let gender = match gender_trimmed.as_str() {
        "boy" | "male" | "man" | "m" => "Boy", // boy
        "girl" | "female" | "woman" | "f" => "Girl", // girl
        "nonbinary" | "non-binary" | "nb" => "Non-Binary", // non-binary
        "trans" | "transgender" => "Transgender", // transgender
        "other" | "custom" => "Other", // other
        _ => {
            println!("{}Invalid gender! Defaulting to Other.{}", RED, RESET); // Invalid gender
            "Other"
        }
    };

    // Theme Input
    let mut theme = String::new();
    println!("\n{}Available Themes:{} \n {}Ocean{} , {}Neon{} , {}Forest{} , {}Night{} , {}Sky{} , {}Sun{} , {}Light{} , {}Dark{} , {}Smooth{}", 
        PURPLE, RESET, CYAN, RESET, GREEN, RESET, BRIGHT_GREEN, RESET, BRIGHT_BLACK, RESET, BRIGHT_CYAN, RESET, BRIGHT_YELLOW, RESET, WHITE, RESET, BRIGHT_BLACK, RESET, BOLD, RESET);
    print!("{}[?] Enter Theme     >{} ", GREEN, RESET);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut theme).unwrap();
    theme = theme.trim().to_lowercase();
    
    // OS Detection
    let os = std::env::consts::OS;
    println!("\n{}[*] System Detected: {}{}{}", CYAN, BOLD, os.to_uppercase(), RESET);

    // Create config text
    let data = format!(
        "name={}\nage={}\ntheme={}\nos={}\ngender={}",
        name.trim(),
        age.trim(),
        theme.trim(),
        os.trim(),
        gender.trim(),
    );

    // Save to file
    if let Err(e) = fs::write("config.txt", data) {
        println!("{}Error saving config: {}{}", RED, e, RESET);
    } else {
        println!("\n{}✓ Configuration Saved Successfully!{}", GREEN, RESET);
        println!("{}Launching RustyShell...{}\n", CYAN, RESET);
    }
}