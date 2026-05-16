use crate::colors::{BOLD, CYAN, GREEN, PURPLE, RED, RESET, YELLOW, BRIGHT_BLUE, BRIGHT_CYAN};
use std::io::{self, Write};
use std::process::Command;
use crate::user_variable::load_config;


pub fn run_shell() {
    crate::animation::animation_loading();
    crate::animation::shaking_screen();
    
    let user_config = load_config();

    if cfg!(target_os = "windows") {
        if user_config.theme == "ocean"{ 
            let _ = Command::new("cmd").args(&["/C", "color 1A"]).status();
        } 
        else if user_config.theme == "neon"{ 
            let _ = Command::new("cmd").args(&["/C", "color 0E"]).status();
        }
        else if user_config.theme == "forest"{
            let _ = Command::new("cmd").args(&["/C", "color 02"]).status();
        }
        else if user_config.theme == "night"{
            let _ = Command::new("cmd").args(&["/C", "color 08"]).status();
        }
        else if user_config.theme == "sky"{
            let _ = Command::new("cmd").args(&["/C", "color 09"]).status();
        }
        else if user_config.theme == "sun"{
            let _ = Command::new("cmd").args(&["/C", "color 0C"]).status();
        }
        else if user_config.theme == "light"{
            let _ = Command::new("cmd").args(&["/C", "color 0F"]).status();
        }
        else if user_config.theme == "dark"{
            let _ = Command::new("cmd").args(&["/C", "color 00"]).status();
        }
        else if user_config.theme == "smooth"{
            let _ = Command::new("cmd").args(&["/C", "color 07"]).status();
        }
        // Apply the colors to the whole screen
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
    
        println!("{}--- {}Session Info{} ---{}", BOLD, PURPLE, PURPLE, RESET);
        println!("{}[+] Name:   {}{}{}", GREEN, BOLD, user_config.name, RESET);
        println!("{}[+] Age:    {}{}{}", YELLOW, BOLD, user_config.age, RESET);
        println!("{}[+] OS:     {}{}{}", CYAN, BOLD, user_config.os, RESET);
        println!("{}[+] Gender: {}{}{}", RED, BOLD, user_config.gender, RESET);
        println!("{}[+] Theme:  {}{}{}", BRIGHT_BLUE, BOLD, user_config.theme, RESET);
        println!("{}-----------------------{}", PURPLE, RESET);
    

  

        
    loop {
        
        print!("\n");
        // Getting current directory
        let current_dir = std::env::current_dir() // Getting current directory
            .map(|path| path.display().to_string()) // Convert path to string
            .unwrap_or_else(|_| "unknown".to_string()); // If error, return "unknown"

        // Showing prompt for user input
        print!("{}[{}] {}{}RustyShell{} {}${} ", BOLD, current_dir, RESET, BRIGHT_CYAN, RESET, YELLOW, RESET);
        io::stdout().flush().unwrap();

        // Reading input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Remove newline/space
        let input = input.trim();

        // Ignore empty input every time😁
        if input.is_empty() {
            println!("{}Please enter a command! 😁{}", RED, RESET);
            continue;
        }

        // Splitting command into parts (Keep original case for arguments)
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        // Only lowercase the command name for matching
        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        crate::command::execute(&command, args, &user_config);
    }
}