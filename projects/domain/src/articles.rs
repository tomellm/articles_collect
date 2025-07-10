use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Article {
    pub uuid: Uuid,
    pub title: String,
    pub url: String,
}

impl Article {
    pub fn new(uuid: Uuid, title: String, url: String) -> Self {
        Self { uuid, title, url }
    }

    pub fn from_parts(title: String, url: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            title,
            url,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::articles::Article;

    #[test]
    fn article_from_parts_sets_valid_uuid() {
        let art = Article::from_parts(String::from("title"), String::from("url"));
        assert!(!art.uuid.is_nil());
        assert!(!art.uuid.is_max());
    }
}
