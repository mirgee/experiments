use clap::{Parser, command, arg};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct AppConfig {
    #[arg(short, long, default)]
    agent_name: String,
    #[arg(short, long)]
    seed: Option<String>,
    genesis_path: String,
    port: u32,
    log_level: String,
    accept_taa: bool
}
