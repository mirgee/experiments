use std::{fmt::Display, sync::{RwLock, Arc}};

use aries_vcx_agent::{Agent, aries_vcx::messages::connection::response::SignedResponse as ConnectionResponse};
use inquire::Select;

use crate::commands::LoopStatus;

enum ConnectionResponseMessageCommand {
    SendAck,
    GoBack
}

impl Display for ConnectionResponseMessageCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendAck => f.write_str("Send Ack"),
            Self::GoBack => f.write_str("Back"),
        }
    }
}

impl ConnectionResponseMessageCommand {
    pub fn iter() -> impl Iterator<Item = &'static ConnectionResponseMessageCommand> {
        [
            Self::SendAck,
            Self::GoBack,
        ]
        .iter()
    }
}

pub async fn process_connection_response_message_command(agent: Arc<RwLock<Agent>>, response: &ConnectionResponse) -> anyhow::Result<LoopStatus> {
    match Select::new("Select command:", ConnectionResponseMessageCommand::iter().collect()).prompt()? {
        ConnectionResponseMessageCommand::SendAck => {
            let tid = response.get_thread_id();
            agent
                .read().unwrap()
                .connections()
                .send_ack(&tid)
                .await
                .map_err(|err| anyhow!("Error acking connection response: {}", err))?;
            Ok(LoopStatus::Continue)
        }
        ConnectionResponseMessageCommand::GoBack => {
            Ok(LoopStatus::GoBack)
        }
    }
}
