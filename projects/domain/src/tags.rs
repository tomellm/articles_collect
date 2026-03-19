use serde::{Deserialize, Serialize};

use crate::type_uuid;

type_uuid!(TagUuid);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Tag {
    pub uuid: TagUuid,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SimpleTag {
    pub uuid: TagUuid,
    pub name: String,
}
