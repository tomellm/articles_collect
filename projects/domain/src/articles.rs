use serde::{Deserialize, Serialize};
use type_state_builder::TypeStateBuilder;

use crate::{tags::Tag, type_uuid};

type_uuid!(ArticleUuid);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, TypeStateBuilder)]
pub struct Article {
    #[builder(default = ArticleUuid::new(), impl_into = true)]
    pub uuid: ArticleUuid,
    #[builder(required)]
    pub title: String,
    #[builder(required)]
    pub url: String,
    #[builder(required)]
    pub tags: Box<[Tag]>,
}

impl Article {
    pub fn from_parts(title: String, url: String) -> Self {
        Self {
            uuid: ArticleUuid::new(),
            title,
            url,
            tags: vec![].into_boxed_slice(),
        }
    }
}

pub mod builder {
    use crate::articles::ArticleBuilder_HasTitle_HasUrl_MissingTags;

    pub type ArticleMissingTags = ArticleBuilder_HasTitle_HasUrl_MissingTags;
}

#[cfg(test)]
mod tests {
    use crate::articles::Article;

    #[test]
    fn article_from_parts_sets_valid_uuid() {
        let title = String::from("title");
        let url = String::from("url");
        let art = Article::from_parts(title.clone(), url.clone());
        assert!(!art.uuid.is_nil());
        assert!(!art.uuid.is_max());
        assert_eq!(title, art.title);
        assert_eq!(url, art.url);
    }
}
