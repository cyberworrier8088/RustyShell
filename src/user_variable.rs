use std::collections::HashMap; // hash map is a data structure that stores key-value pairs.
use std::fs; // file input output evary file hace {:}

pub struct Config { // struct use for config data storing
    pub name: String,
    pub age: String,
    pub os: String,
    pub gender: String,
    pub theme: String,
}

pub fn load_config() -> Config { // loading config from file

    let content = fs::read_to_string("config.txt").unwrap(); // reading file

    let mut vars = HashMap::new(); // creating hash map for big things

    for line in content.lines() { // iterating through each line

        let parts: Vec<&str> = line.split('=').collect(); // splitting line by "=" to seperate name and age and os and etc.

        if parts.len() == 2 { // checking if line has "=" in it 

            vars.insert(
                parts[0].trim().to_string(), // adding name and age and os in hash map
                parts[1].trim().to_string() // adding name and age and os in hash map
            );
        }
    }

    Config {

    name: vars.get("name")
        .unwrap_or(&"Unknown".to_string())
        .clone(),

    age: vars.get("age")
        .unwrap_or(&"0".to_string())
        .clone(),

    os: vars.get("os")
        .unwrap_or(&"unknown".to_string())
        .clone(),

    gender: vars.get("gender")
        .unwrap_or(&"unknown".to_string())
        .clone(),

    theme: vars.get("theme")
        .unwrap_or(&"default".to_string())
        .clone(),
}
}