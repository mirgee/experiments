use serde::{Deserialize, Serialize};

use crate::{uri::Uri, utils::OneOrList};

pub type ServiceTypeAlias = OneOrList<String>;

// TODO: It seems that this may contain pretty much anything?
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    id: Uri,
    r#type: ServiceTypeAlias,
    service_endpoint: String,
}
