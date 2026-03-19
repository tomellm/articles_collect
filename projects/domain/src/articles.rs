use serde::{Deserialize, Serialize};
use type_state_builder::TypeStateBuilder;

use crate::{tags::SimpleTag, type_uuid};

type_uuid!(ArticleUuid);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, TypeStateBuilder)]
pub struct Article {
    #[builder(default = ArticleUuid::new(), impl_into = true)]
    pub uuid: ArticleUuid,
    #[builder(required)]
    pub title: String,
    #[builder(required)]
    pub url: String,
    pub tags: Vec<SimpleTag>,
}

impl Article {
    pub fn new(uuid: ArticleUuid, title: String, url: String, tags: Vec<SimpleTag>) -> Self {
        Self {
            uuid,
            title,
            url,
            tags,
        }
    }

    pub fn from_parts(title: String, url: String, tags: Vec<SimpleTag>) -> Self {
        Self {
            uuid: ArticleUuid::new(),
            title,
            url,
            tags,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::articles::Article;

    #[test]
    fn article_from_parts_sets_valid_uuid() {
        let title = String::from("title");
        let url = String::from("url");
        let art = Article::from_parts(title.clone(), url.clone(), vec![]);
        assert!(!art.uuid.is_nil());
        assert!(!art.uuid.is_max());
        assert_eq!(title, art.title);
        assert_eq!(url, art.url);
    }
}
