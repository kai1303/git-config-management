use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// command to trigger
    #[clap(value_enum)]
    pub command: Command,

    pub key: Option<String>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Command {
    Get,
    List,
    Add,
    Test,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Entity {
    Config,
}


