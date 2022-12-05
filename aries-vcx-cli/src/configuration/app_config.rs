use clap::{arg, command, Parser, ValueEnum};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, ValueEnum)]
pub enum KeyDerivationMethod {
    RAW,
    ARGON2I_MOD,
    ARGON2I_INT,
}

impl std::fmt::Display for KeyDerivationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RAW => f.write_str("RAW"),
            Self::ARGON2I_MOD => f.write_str("ARGON2I_MOD"),
            Self::ARGON2I_INT => f.write_str("ARGON2I_INT"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppConfig {
    #[arg(short, long, default_value = "http://localhost:9000")]
    ledger_url: Option<String>,
    #[arg(short, long, default_value = None)]
    genesis_file: Option<String>,
    #[arg(short, long, default_value = None)]
    trustee_seed: Option<String>,
    #[arg(short, long, default_value = "5050")]
    port: u32,
    #[arg(short, long, default_value = "localhost")]
    host: u32,
    #[arg(short, long, default_value = "info")]
    log_level: String,
    #[arg(short, long, default_value = "true")]
    accept_taa: bool,
    #[arg(short, long, default_value = "8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY")]
    wallet_key: String,
    #[arg(short, long, value_enum, default_value = "RAW")]
    wallet_kdf: KeyDerivationMethod,
    #[arg(short, long)]
    agent_name: Option<String>,
}

impl AppConfig {
    pub fn ledger_url(&self) -> Option<&str> {
        self.ledger_url.as_deref()
    }

    pub fn port(&self) -> u32 {
        self.port
    }

    pub fn host(&self) -> u32 {
        self.host
    }

    pub fn log_level(&self) -> &str {
        self.log_level.as_ref()
    }

    pub fn accept_taa(&self) -> bool {
        self.accept_taa
    }

    pub fn agent_name(&self) -> Option<&str> {
        self.agent_name.as_deref()
    }

    pub fn wallet_key(&self) -> &str {
        self.wallet_key.as_ref()
    }

    pub fn wallet_kdf(&self) -> &KeyDerivationMethod {
        &self.wallet_kdf
    }

    pub fn genesis_file(&self) -> Option<&String> {
        self.genesis_file.as_ref()
    }

    pub fn trustee_seed(&self) -> Option<&String> {
        self.trustee_seed.as_ref()
    }

    // TODO: Suboptimal. Separate InitAppConfig and AppConfig
    pub(in crate::configuration) fn set_trustee_seed(&mut self, trustee_seed: String) {
        self.trustee_seed = Some(trustee_seed);
    }

    // TODO: Suboptimal. Separate InitAppConfig and AppConfig
    pub(in crate::configuration) fn set_genesis_file(&mut self, genesis_file: String) {
        self.genesis_file = Some(genesis_file);
    }

    // TODO: Suboptimal. Separate InitAppConfig and AppConfig
    pub(in crate::configuration) fn assure_agent_name(&mut self) {
        self.agent_name = Some(
            self.agent_name
                .clone()
                .unwrap_or(Alphanumeric.sample_string(&mut thread_rng(), 32))
                .to_string(),
        );
    }
}

pub fn load_config() -> AppConfig {
    // TODO: Config from file / defaults should have preference over CLI
    AppConfig::parse()
}
