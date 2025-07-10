use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Article {
    pub uuid: Uuid,
    pub title: String,
    pub url: String,
}

impl Article {
    pub fn new(uuid: Uuid, title: String, url: String) -> Self {
        Self { uuid, title, url }
    }

    pub fn from_into_string(title: String, url: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            title,
            url,
        }
    }
}
