use std::process::Command;
use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};


use crate::colors::{CYAN, GREEN, RED, RESET, YELLOW, BG_BLUE, BG_PURPLE, BG_GREEN, BG_CYAN, BG_YELLOW, WHITE, BLACK};
use crate::user_variable::Config;

// Simple rando number generator without using crates
fn pseudo_rand(seed: &mut u32) -> u32 {
    *seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
    (*seed / 65536) % 32768
}

pub fn execute(command: &str, args: &[&str], config: &Config) {
    match command {
        "clear" | "vanish" | "cls" => {
            if cfg!(target_os = "windows") {
                // OS-used clear for Windows
                let _ = Command::new("cmd").args(&["/C", "cls"]).status();
            } else {
                // OS-used clear for Linux/macOS
                let _ = Command::new("clear").status();
            }
        }
        "echo" => {
            if args.is_empty() {
                println!("{}Nothing to echo!. bro add something after echo!{}", RED, RESET);
            } else {
                for arg in args {
                    print!("{} ", arg);
                }
                println!(); // Add newline
            }
        }
        

        "help" | "list" | "brohelp" => {
            println!("\n{}{} --- Available Commands --- {}{}", BG_BLUE, WHITE, RESET, RESET);
            println!("{}help/list/brohelp{}     - show this menu", GREEN, RESET);
            println!("{}vanish/clear/cls{} - clear terminal", YELLOW, RESET);
            println!("{}whoami{}   - show your name", CYAN, RESET);
            println!("{}config{}   - show your profile", crate::colors::PURPLE, RESET);
            println!("{}time{}     - show system time", RED, RESET);
            println!("{}pwd{}      - show current folder", GREEN, RESET);
            println!("{}teleport{} - change folder (alias: cd)", YELLOW, RESET);
            println!("{}matrix{}   - enter the matrix", crate::colors::BRIGHT_GREEN, RESET);
            println!("{}exit{}     - leave RustyShell", RED, RESET);
            println!("{}{} ---------------------------- {}{}\n", BG_BLUE, WHITE, RESET, RESET);
        }

        "whoami" => {
            println!("{}{}  {}  {}{}", BG_GREEN, WHITE, config.name, RESET, RESET);
        }

        "config" => {
            println!("\n{}{} --- Current Config --- {}{}", BG_PURPLE, WHITE, RESET, RESET);
            println!("{}Name:{}   {}", GREEN, RESET, config.name);
            println!("{}Age:{}    {}", YELLOW, RESET, config.age);
            println!("{}OS:{}     {}", CYAN, RESET, config.os);
            println!("{}Gender:{} {}", RED, RESET, config.gender);
            println!("{}Theme:{}  {}", crate::colors::BRIGHT_BLUE, RESET, config.theme);
            println!("{}{} ----------------------- {}{}\n", BG_PURPLE, WHITE, RESET, RESET);
        }

        "pwd" => {
            match std::env::current_dir() {
                Ok(path) => println!("{}{} Path: {} {}{}", BG_CYAN, WHITE, path.display(), RESET, RESET),
                Err(e) => println!("Error: {}", e),
            }
        }

        "teleport" | "cd" => {
            let path = if args.is_empty() {
                println!(" where are you going? Please add a path, {}.", config.gender);
                "."
            } else {
                args[0]
            };

            if let Err(e) = std::env::set_current_dir(path) {
                println!("{}Error:{} {}", RED, RESET, e);
            }
        }

        "time" => {
            println!("{}{} Time: {:?} {}{}", BG_YELLOW, BLACK, std::time::SystemTime::now(), RESET, RESET);
        }
        "sleep" => {
            if args.is_empty() {
                println!("{}Please add a duration, my {}.{}", RED, config.gender, RESET);
            } else {
                match args[0].parse::<u64>() {
                    Ok(duration) => sleep(Duration::from_secs(duration)),
                    Err(_) => println!("{}Error: '{}' is not a valid number!{}", RED, args[0], RESET),
                }
            }
        }

        "matrix" => {
            print!("\x1B[2J\x1B[1;1H"); // Initial clear
            let mut seed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;

            let width = 80;
            let mut column_pos = vec![0; width];

            println!("Press Ctrl+C to exit the Matrix...");
            sleep(Duration::from_secs(2));

            loop {
                for i in 0..width {
                    if column_pos[i] > 0 {
                        // Move cursor to row column_pos[i], column i
                        print!("\x1B[{};{}H\x1B[32m{}\x1B[0m", column_pos[i], i, (pseudo_rand(&mut seed) % 94 + 33) as u8 as char);
                        
                        column_pos[i] += 1;
                        if column_pos[i] > 25 {
                            column_pos[i] = 0;
                        }
                    } else if pseudo_rand(&mut seed) % 100 < 5 {
                        column_pos[i] = 1;
                    }
                }
                io::stdout().flush().unwrap();
                sleep(Duration::from_millis(50));
                
                // Dim effect: we don't clear the whole screen, just let it scroll or overwrite
                // For a true matrix feel, we'd need to track and erase old characters, 
                // but this is a simple version.
            }
        }

        "exit" => {
            println!("Exiting shell...");
            sleep(Duration::from_secs(2));
            println!("Bye my {} friend!", config.gender);
            std::process::exit(0);
        }

        _ => {
            // Handle aliases (e.g., 'ls' on Windows)
            let (final_cmd, final_args) = if command == "ls" && cfg!(target_os = "windows") {
                ("cmd", vec!["/C", "dir"])
            } else if command == "dir" && cfg!(target_os = "linux") {
                ("ls", args.to_vec())
            } else {
                (command, args.to_vec())
            };

            // external commands
            let spawn_result = if final_cmd == "cmd" {
                let mut all_args = final_args;
                all_args.extend_from_slice(args);
                Command::new("cmd").args(all_args).spawn()
            } else {
                Command::new(final_cmd).args(args).spawn()
            };

            match spawn_result {
                Ok(mut child) => {
                    if let Err(e) = child.wait() {
                        println!("{}Error:{} {}", RED, RESET, e);
                    }
                }
                Err(e) => {
                    println!("{}Error:{} {}", RED, RESET, e);
                }
            }
        }
    }
}
