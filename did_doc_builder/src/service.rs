use serde::{Deserialize, Serialize};

use crate::{error::DIDDocumentBuilderError, uri::Uri, utils::OneOrList};

pub type ServiceTypeAlias = OneOrList<String>;

// TODO: It seems that this may contain pretty much anything?
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    id: Uri,
    r#type: ServiceTypeAlias,
    service_endpoint: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ServiceBuilder {
    id: Uri,
    r#type: Vec<String>,
    service_endpoint: String,
}

#[allow(dead_code)]
impl ServiceBuilder {
    pub fn new(id: Uri, service_endpoint: String) -> Self {
        Self {
            id,
            service_endpoint,
            r#type: Vec::new(),
        }
    }

    pub fn add_type(&mut self, r#type: String) -> &mut Self {
        self.r#type.push(r#type);
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
            })
        }
    }
}
