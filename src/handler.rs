use crate::config::Config;
use crate::console::{print_configs, print_config};

use std::collections::HashMap;
use std::{format, panic};
use std::{fs, path::Path};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;

pub fn add_command_handler() {
    let home_dir = match dirs::home_dir() {
        Some(path) => path.as_path().display().to_string(),
        None => String::from("")
    };
    let config_file_path = Path::new(&home_dir).join(".ssh/config");

    let content = match fs::read_to_string(config_file_path) {
        Ok(content) => content,
        Err(err) => panic!("open file fails {:?}", err)
    };
    let configs = parse_file_config(&content);
    let string_content = config_to_string(configs[0].clone());
    let mut file = OpenOptions::new()
        .write(true)
        .append(true).open("./foo.txt").expect("Failed to open the file");
    match write!(file, "{}", string_content) {
        Ok(_) => println!("insert config success"),
        Err(err) => println!("{:?}", err)
    }
}

pub fn get_command_handler(host: String) {
    let home_dir = match dirs::home_dir() {
        Some(path) => path.as_path().display().to_string(),
        None => String::from("")
    };
    let config_file_path = Path::new(&home_dir).join(".ssh/config");
    let content = match fs::read_to_string(config_file_path) {
        Ok(content) => content,
        Err(err) => panic!("open file fails {:?}", err)
    };
    let configs = parse_file_config(&content);
    let config_map = vec_to_map(configs);
    let config = match config_map.get(&host) {
        Some(c) => c,
        None => panic!("not found")
    };
    print_config(config.clone());
}

pub fn test_command_handler(host: String) {
    let mut ssh_command = Command::new("ssh");
    let command_output =ssh_command.arg("-T").arg(host).output();
    println!("{:?}", command_output)
}

pub fn list_command_handler() {
    let home_dir = match dirs::home_dir() {
        Some(path) => path.as_path().display().to_string(),
        None => String::from("")
    };
    let config_file_path = Path::new(&home_dir).join(".ssh/config");
    let content = match fs::read_to_string(config_file_path) {
        Ok(content) => content,
        Err(err) => panic!("open file fails {:?}", err)
    };
    let configs = parse_file_config(&content);
    print_configs(configs);    
}

fn parse_file_config(content: &str)-> Vec<Config>{
    let lines = content.lines().filter(|line| line.len() != 0 && !line.starts_with("#")).map(|line| line.trim());
    let mut configs = Vec::new();
    let mut config = Config{
            host: String::new(),
            host_name: String::new(),
            user: String::new(),
            identity_file: String::new()
    };
    for l in lines {
        if l.starts_with("HostName") {
            config.host_name = match l.split_whitespace().nth(1) {
                Some(str) => str.to_string(),
                None => String::new()
            };
        } else if l.starts_with("Host") {
            if config.host != "" {
                configs.push(config.clone());
            }
            config = Config{
                host: String::new(),
                host_name: String::new(),
                user: String::new(),
                identity_file: String::new()
            };
            config.host = match l.split_whitespace().nth(1) {
                Some(str) => str.to_string(),
                None => String::new()
            };
        } else if l.starts_with("User") {
            config.user = match l.split_whitespace().nth(1) {
                Some(str) => str.to_string(),
                None => String::new()
            };
        } else if l.starts_with("IdentityFile") {
            config.identity_file = match l.split_whitespace().nth(1) {
                Some(str) => str.to_string(),
                None => String::new()
            };
        }
    }
    configs.push(config.clone());
    return configs;
}

fn vec_to_map(configs: Vec<Config>) -> HashMap<String, Config> {
    let mut h = HashMap::new();
    for c in configs {
        h.insert(c.host.clone(), c.clone());
    }
    h
}

fn config_to_string(config: Config) -> String {
    format!(r#"
Host {host}
    HostName {host_name}
    USer {user}
    IdentityFile {identity_file}"#, host = config.host, host_name = config.host_name, user = config.user, identity_file = config.identity_file)
}

