mod command;
mod connection;

use std::sync::{Arc, RwLock};

use aries_vcx_agent::Agent;
use inquire::Select;

use self::{
    command::{get_messages, MessagesCommand},
    connection::{
        request::process_connection_request_message_command, response::process_connection_response_message_command,
    },
};

use super::LoopStatus;

async fn process_messages_command(agent: Arc<RwLock<Agent>>) -> anyhow::Result<LoopStatus> {
    match Select::new("Select message:", get_messages()).prompt()? {
        MessagesCommand::ConnectionRequest(request) => process_connection_request_message_command(agent, request.clone()).await,
        MessagesCommand::ConnectionResponse(response) => {
            process_connection_response_message_command(agent, response).await
        }
        MessagesCommand::GoBack => Ok(LoopStatus::GoBack),
        _ => {
            info!("Not implemented yet");
            Ok(LoopStatus::Continue)
        }
    }
}

pub async fn messages_command_loop(agent: Arc<RwLock<Agent>>) -> anyhow::Result<LoopStatus> {
    loop {
        match process_messages_command(agent.clone()).await {
            Ok(LoopStatus::Continue) => break,
            Ok(LoopStatus::GoBack) => continue,
            Ok(LoopStatus::Exit)  => continue,
            Err(err) => {
                error!("Error processing messages command: {}", err);
                break;
            }
        }
    }
    Ok(LoopStatus::Continue)
}
