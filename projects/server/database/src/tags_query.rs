use std::collections::HashMap;

use domain::{articles::ArticleUuid, tags::Tag};
use sea_orm::EntityTrait;

use crate::entities::tags;

pub async fn all_of_articles<C>(
    db: &C,
    articles: &[&ArticleUuid],
) -> HashMap<ArticleUuid, Vec<Tag>> {
    tags::Entity::find().
}
