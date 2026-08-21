use domain::tags::Tag;
use sea_orm::FromQueryResult;

use crate::{DbArticleUuid, DbTagUuid};

#[derive(FromQueryResult)]
pub struct ArticleTag {
    pub article_uuid: DbArticleUuid,
    pub uuid: DbTagUuid,
    pub name: String,
    pub description: String,
}

impl From<ArticleTag> for Tag {
    fn from(value: ArticleTag) -> Self {
        Tag::builder()
            .uuid(value.uuid)
            .name(value.name)
            .description(value.description)
            .build()
    }
}
