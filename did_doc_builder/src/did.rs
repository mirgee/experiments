use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Serialize, Clone, Debug, PartialEq, Default)]
pub struct Did(String);

impl Did {
    pub fn new(did: String) -> Result<Self, std::io::Error> {
        if is_valid_did(&did) {
            Ok(Self(did))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid DID: {did}"),
            ))
        }
    }
}

impl<'de> Deserialize<'de> for Did {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if is_valid_did(&s) {
            Ok(Self(s))
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Str(&s),
                &"a valid DID",
            ))
        }
    }
}

fn is_valid_did(did: &str) -> bool {
    // TODO
    did.starts_with("did:")
}