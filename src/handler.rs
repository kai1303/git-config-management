use crate::config::Config;
use crate::console::{print_config, print_configs};

use cliclack::{input, intro, outro};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::panic;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, path::Path};
use tokio::sync::oneshot;

const FILE_CONFIG_PATH: &str = ".ssh/config";

pub fn add_command_handler() {
    let home_dir = match dirs::home_dir() {
        Some(path) => path.as_path().display().to_string(),
        None => String::from(""),
    };
    let mut config = Config {
        host: String::new(),
        host_name: String::new(),
        user: String::new(),
        identity_file: String::new(),
    };
    let config_file_path = Path::new(&home_dir).join(FILE_CONFIG_PATH);
    let _ = intro("Add config file");
    match input("Input your config host")
        .placeholder("github.com")
        .interact()
    {
        Ok(input) => config.host = input,
        Err(err) => panic!("{:?}", err),
    };
    match input("Input your config host name").interact() {
        Ok(input) => config.host_name = input,
        Err(err) => panic!("{:?}", err),
    };
    match input("Input your config user").interact() {
        Ok(input) => config.user = input,
        Err(err) => panic!("{:?}", err),
    };
    match input("Input your identity file")
        .placeholder("~/.ssh/id_rsa")
        .interact()
    {
        Ok(input) => config.identity_file = input,
        Err(err) => panic!("{:?}", err),
    };
    print_config(config.clone());
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&config_file_path)
        .expect("Failed to open the file");
    match write!(file, "{}", config) {
        Ok(_) => {
            let _ = outro("New config added");
        }
        Err(err) => println!("{:?}", err),
    }
}

pub fn get_command_handler(host: String) {
    let home_dir = match dirs::home_dir() {
        Some(path) => path.as_path().display().to_string(),
        None => String::from(""),
    };
    let config_file_path = Path::new(&home_dir).join(FILE_CONFIG_PATH);
    let content = match fs::read_to_string(config_file_path) {
        Ok(content) => content,
        Err(err) => panic!("open file fails {:?}", err),
    };
    let configs = parse_file_config(&content);
    let config_map = vec_to_map(configs);
    let config = match config_map.get(&host) {
        Some(c) => c,
        None => panic!("not found"),
    };
    print_config(config.clone());
}

pub async fn test_command_handler(host: String) {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    tokio::spawn(async {
        let mut ssh_command = Command::new("ssh");
        let command_output = ssh_command.arg("-T").arg(host).output();
        let _ = tx1.send(command_output);
    });
    tokio::spawn(async {
        sleep(Duration::from_secs(10));
        let _ = tx2.send("timeout");
    });
    tokio::select! {
        val = rx1 => {
            println!("{:?}", val) 
        }
        val = rx2 => {
            println!("Command timeout after 10s")
        }
    }
}

pub fn list_command_handler() {
    let home_dir = match dirs::home_dir() {
        Some(path) => path.as_path().display().to_string(),
        None => String::from(""),
    };
    let config_file_path = Path::new(&home_dir).join(FILE_CONFIG_PATH);
    let content = match fs::read_to_string(config_file_path) {
        Ok(content) => content,
        Err(err) => panic!("open file fails {:?}", err),
    };
    let configs = parse_file_config(&content);
    print_configs(configs);
}

fn parse_file_config(content: &str) -> Vec<Config> {
    let lines = content
        .lines()
        .filter(|line| line.len() != 0 && !line.starts_with("#"))
        .map(|line| line.trim());
    let mut configs = Vec::new();
    let mut config = Config {
        host: String::new(),
        host_name: String::new(),
        user: String::new(),
        identity_file: String::new(),
    };
    for l in lines {
        if l.starts_with("HostName") {
            config.host_name = match l.split_whitespace().nth(1) {
                Some(str) => str.to_string(),
                None => String::new(),
            };
        } else if l.starts_with("Host") {
            if config.host != "" {
                configs.push(config.clone());
            }
            config = Config {
                host: String::new(),
                host_name: String::new(),
                user: String::new(),
                identity_file: String::new(),
            };
            config.host = match l.split_whitespace().nth(1) {
                Some(str) => str.to_string(),
                None => String::new(),
            };
        } else if l.starts_with("User") {
            config.user = match l.split_whitespace().nth(1) {
                Some(str) => str.to_string(),
                None => String::new(),
            };
        } else if l.starts_with("IdentityFile") {
            config.identity_file = match l.split_whitespace().nth(1) {
                Some(str) => str.to_string(),
                None => String::new(),
            };
        }
    }
    configs.push(config.clone());
    return configs;
}

pub async fn cur_command_handler() {
    let mut git_command = Command::new("git");
    let command_output = match git_command.arg("remote").arg("-v").output() {
        Ok(output) => output,
        Err(err) => panic!("{:?}", err),
    };
    let command_string_output = match String::from_utf8(command_output.stdout) {
        Ok(i) => i,
        Err(_) => panic!("parse resulve of git remote -v command failed"),
    };
    let lines = command_string_output
        .lines()
        .filter(|&line| line.ends_with("(fetch)"));
    for l in lines {
        let mut splited = l.split_whitespace();
        let remote_url = match splited.nth(1) {
            Some(str) => str.to_string(),
            None => String::new(),
        };
        test_command_handler(format!("{remote_url}", remote_url = remote_url)).await;
    }
}

fn vec_to_map(configs: Vec<Config>) -> HashMap<String, Config> {
    let mut h = HashMap::new();
    for c in configs {
        h.insert(c.host.clone(), c.clone());
    }
    h
}
