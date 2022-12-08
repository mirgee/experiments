mod connection;
mod messages;
mod ledger;

use std::{fmt::Display, sync::{Arc, RwLock}};

use aries_vcx_agent::Agent;
use inquire::Select;

use self::{connection::connection_command_loop, ledger::ledger_command_loop, messages::messages_command_loop};

#[derive(Clone)]
enum Command {
    Ledger,
    Connections,
    Messages,
    Exit,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ledger => f.write_str("Connections"),
            Self::Connections => f.write_str("Connections"),
            Self::Messages => f.write_str("Messages"),
            Self::Exit => f.write_str("Exit"),
        }
    }
}

impl Command {
    pub fn iter() -> impl Iterator<Item = &'static Command> {
        [
            Self::Ledger,
            Self::Connections,
            Self::Messages,
            Self::Exit,
        ]
        .iter()
    }
}

fn get_options() -> Vec<&'static Command> {
    Command::iter().collect()
}

pub enum LoopStatus {
    Continue,
    Exit,
    GoBack
}

async fn process_root_command(agent: Arc<RwLock<Agent>>) -> anyhow::Result<LoopStatus> {
    match Select::new("Select command:", get_options()).prompt()?.clone() {
        Command::Ledger => ledger_command_loop(agent).await,
        Command::Connections => connection_command_loop(agent).await,
        Command::Messages => messages_command_loop(agent).await,
        Command::Exit => Ok(LoopStatus::Exit)
    }
}

pub async fn root_command_loop(agent: Arc<RwLock<Agent>>) -> Result<(), std::io::Error> {
    loop {
        match process_root_command(agent.clone()).await {
            Ok(LoopStatus::Continue) => continue,
            Ok(LoopStatus::Exit) | Ok(LoopStatus::GoBack) => { return Ok(()) },
            Err(err) => {
                error!("An error occurred inside user input loop: {}", err);
                continue
            }
        }
    }
}
