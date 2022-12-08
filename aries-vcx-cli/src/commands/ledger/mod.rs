mod command;

use std::sync::{Arc, RwLock};

use aries_vcx_agent::{Agent, aries_vcx::common::primitives::credential_definition::CredentialDefConfigBuilder};
use inquire::{Text, Select};

use self::command::{get_options, LedgerCommand};

use super::LoopStatus;

async fn process_ledger_command(agent: Arc<RwLock<Agent>>) -> anyhow::Result<LoopStatus> {
    match Select::new("Select command:", get_options()).prompt()?.clone() {
        LedgerCommand::CreateSchema => {
            let schema_name = Text::new("Schema name:").prompt()?;
            let schema_version = Text::new("Schema version:").prompt()?;
            let schema_attrs = Text::new("Schema attributes (comma separated):").prompt()?;
            let schema_attrs = schema_attrs.split(",").map(str::to_string).collect();
            let schema_id = agent.read().unwrap()
                .schemas()
                .create_schema(&schema_name, &schema_version, &schema_attrs)
                .await
                .map_err(|err| anyhow!("Failed to create schema: {}", err))?;
            println!("Schema created with id: {}", schema_id);
            Ok(LoopStatus::Continue)
        }
        LedgerCommand::CreateCredDef => {
            let schema_id = Text::new("Schema id:").prompt()?;
            let config = CredentialDefConfigBuilder::default()
                .issuer_did(agent.read().unwrap().agent_config().config_issuer.institution_did.clone()) // TODO: Remove
                .schema_id(&schema_id)
                .build()
                .map_err(|err| anyhow!("Failed to build credential def config: {}", err))?;
            let cred_def_id = agent.read().unwrap()
                .cred_defs()
                .create_cred_def(config)
                .await
                .map_err(|err| anyhow!("Failed to create credential definition: {}", err))?;
            println!("Credential definition created with id: {}", cred_def_id);
            Ok(LoopStatus::Continue)
        }
        LedgerCommand::GoBack => {
            Ok(LoopStatus::GoBack)
        }
    }
}

pub async fn ledger_command_loop(agent: Arc<RwLock<Agent>>) -> anyhow::Result<LoopStatus> {
    loop {
        match process_ledger_command(agent.clone()).await {
            Ok(LoopStatus::Continue) => break,
            Ok(LoopStatus::GoBack) => continue,
            Ok(LoopStatus::Exit)  => continue,
            Err(err) => {
                error!("Error processing ledger command: {}", err);
                break;
            }
        }
    }
    Ok(LoopStatus::Continue)
}
