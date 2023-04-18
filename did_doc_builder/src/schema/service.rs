use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::DIDDocumentBuilderError;

use super::{types::uri::Uri, utils::OneOrList};

type ServiceTypeAlias = OneOrList<String>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    id: Uri,
    r#type: ServiceTypeAlias,
    service_endpoint: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    extra: HashMap<String, Value>,
}

impl Service {
    pub fn id(&self) -> &Uri {
        &self.id
    }

    pub fn r#type(&self) -> &ServiceTypeAlias {
        &self.r#type
    }

    pub fn service_endpoint(&self) -> &str {
        self.service_endpoint.as_ref()
    }

    pub fn extra(&self, key: &str) -> Option<&Value> {
        self.extra.get(key)
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ServiceBuilder {
    id: Uri,
    r#type: Vec<String>,
    service_endpoint: String,
    extra: HashMap<String, Value>,
}

#[allow(dead_code)]
impl ServiceBuilder {
    pub fn new(id: Uri, service_endpoint: String) -> Self {
        Self {
            id,
            service_endpoint,
            ..Default::default()
        }
    }

    pub fn add_type(&mut self, r#type: String) -> &mut Self {
        self.r#type.push(r#type);
        self
    }

    pub fn add_extra(&mut self, key: String, value: Value) -> &mut Self {
        self.extra.insert(key, value);
        self
    }

    pub fn build(self) -> Result<Service, DIDDocumentBuilderError> {
        if self.r#type.is_empty() {
            Err(DIDDocumentBuilderError::MissingField("type"))
        } else {
            Ok(Service {
                id: self.id,
                r#type: OneOrList::List(self.r#type),
                service_endpoint: self.service_endpoint,
                extra: self.extra,
            })
        }
    }
}
