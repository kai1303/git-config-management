#[macro_use]
extern crate prettytable;
extern crate dirs;

mod cli;
mod config;
mod console;
mod handler;

use crate::cli::{Cli, Command};
use crate::handler::{
    add_command_handler, get_command_handler, list_command_handler, test_command_handler, cur_command_handler
};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match &args.command {
        Command::Add => add_command_handler(),
        Command::Get => {
            if args.key.is_none() {
                return;
            }
            let k = match args.key {
                Some(s) => s,
                None => String::from(""),
            };
            get_command_handler(k);
        }
        Command::List => list_command_handler(),
        Command::Test => {
            if args.key.is_none() {
                return;
            }
            let k = match args.key {
                Some(s) => s,
                None => String::from(""),
            };
            test_command_handler(k).await;
        }
        Command::Cur => cur_command_handler().await
    }
}
