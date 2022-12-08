mod command;

use std::sync::{RwLock, Arc};

use anyhow::Context;
use aries_vcx_agent::{Agent, aries_vcx::messages::connection::invite::Invitation};
use inquire::{Select, Text};

use self::command::{ConnectionCommand, get_options};

use super::LoopStatus;

async fn process_connection_command(agent: Arc<RwLock<Agent>>) -> anyhow::Result<LoopStatus> {
    match Select::new("Select command:", get_options()).prompt()?.clone() {
        ConnectionCommand::CreateInvite => {
            let invite = agent
                .read().unwrap()
                .connections()
                .create_invitation()
                .await
                .map_err(|err| anyhow!("Error creating invitation: {}", err))?;
            info!("Invite: {}", json!(invite).to_string());
            Ok(LoopStatus::Continue)
        }
        ConnectionCommand::ReceiveInvite => {
            let s = Text::new("Enter invite:\n").prompt()?;
            let invite: Invitation = serde_json::from_str(&s).context("Failed to deserialize invite")?;
            let tid = agent
                .read().unwrap()
                .connections()
                .receive_invitation(invite)
                .await
                .map_err(|err| anyhow!("Error receiving invitation: {}", err))?;
            agent
                .read().unwrap()
                .connections()
                .send_request(&tid)
                .await
                .map_err(|err| anyhow!("Error sending request: {}", err))?;
            Ok(LoopStatus::Continue)
        }
        ConnectionCommand::GoBack => {
            Ok(LoopStatus::GoBack)
        }
        m @ _ => {
            info!("Selected command {}", m);
            Ok(LoopStatus::Continue)
        }
    }
}

pub async fn connection_command_loop(agent: Arc<RwLock<Agent>>) -> anyhow::Result<LoopStatus> {
    loop {
        match process_connection_command(agent.clone()).await {
            Ok(LoopStatus::Continue) => break,
            Ok(LoopStatus::GoBack) => continue,
            Ok(LoopStatus::Exit)  => continue,
            Err(err) => {
                error!("Error processing connection command: {}", err);
                break;
            }
        }
    }
    Ok(LoopStatus::Continue)
}
