use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Serialize, Clone, Debug, PartialEq, Default)]
pub struct DidUrl(String);

impl DidUrl {
    pub fn new(did_url: String) -> Self {
        todo!()
    }
}

impl<'de> Deserialize<'de> for DidUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if is_valid_did_url(&s) {
            Ok(Self(s))
        } else {
            Err(de::Error::invalid_value(
                de::Unexpected::Str(&s),
                &"a valid DID URL",
            ))
        }
    }
}

fn is_valid_did_url(did: &str) -> bool {
    did.starts_with("did:")
}
