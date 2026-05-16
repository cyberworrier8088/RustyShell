mod config; // Calling config 
mod shell; // calling shell
mod user_variable; // calling user_variable
mod colors; // calling colors
mod command; // calling command
mod animation;

use std::path::Path; // path input output

fn main() {
    

    let config_path = "config.txt";

    // Check if config exists :}
    if !Path::new(config_path).exists() {

        println!("First time setup!");

        config::configure_shell(); // calling main func in config mod

        println!("Setup complete!");
        shell::run_shell();
    } else {
        println!("Welcome back!");
        shell::run_shell();
    }
}