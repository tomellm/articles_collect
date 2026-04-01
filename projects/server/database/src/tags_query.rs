mod article_tag;

use std::collections::HashMap;

use domain::{
    articles::ArticleUuid,
    tags::{Tag, TagUuid},
};
use itertools::Itertools;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityOrSelect, EntityTrait, IntoActiveModel, JoinType,
    QueryFilter, QuerySelect, RelationTrait,
};

use crate::{
    DbArticleUuid, DbTagUuid,
    entities::{article_tags, tags},
    tags_query::article_tag::ArticleTag,
};

pub async fn all_of_articles<C>(
    db: &C,
    articles: Vec<&ArticleUuid>,
) -> Result<HashMap<ArticleUuid, Vec<Tag>>, DbErr>
where
    C: ConnectionTrait,
{
    let articles = articles
        .into_iter()
        .map(|a| DbArticleUuid::from(**a))
        .collect_vec();

    let query = tags::Entity::find()
        .select_only()
        .column(tags::Column::Uuid)
        .column(tags::Column::Name)
        .column(tags::Column::Description)
        .column(article_tags::Column::ArticleUuid)
        .join(JoinType::InnerJoin, tags::Relation::ArticleTags.def())
        .filter(article_tags::Column::ArticleUuid.is_in(articles));

    Ok(query
        .into_model::<ArticleTag>()
        .all(db)
        .await?
        .into_iter()
        .map(|art_tag| (art_tag.article_uuid.into(), Tag::from(art_tag)))
        .into_group_map())
}

pub async fn all_of_article<C>(db: &C, article: &ArticleUuid) -> Result<Vec<Tag>, DbErr>
where
    C: ConnectionTrait,
{
    let article = DbArticleUuid::from(**article);
    let tags = tags::Entity::find()
        .select_only()
        .column(tags::Column::Uuid)
        .column(tags::Column::Name)
        .column(tags::Column::Description)
        .join(JoinType::InnerJoin, tags::Relation::ArticleTags.def())
        .filter(article_tags::Column::ArticleUuid.eq(article))
        .all(db)
        .await?
        .into_iter()
        .map(Tag::from)
        .collect_vec();

    Ok(tags)
}

pub async fn insert<C>(db: &C, tag: Tag) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    tags::Entity::insert(tags::Model::from(tag).into_active_model())
        .exec(db)
        .await?;

    Ok(())
}

pub async fn delete<C>(db: &C, tag: TagUuid) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    tags::Entity::delete_by_id(DbTagUuid::from(tag))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn name_exists<C>(db: &C, name: &String) -> Result<bool, DbErr>
where
    C: ConnectionTrait,
{
    let empty = tags::Entity::find()
        .select()
        .filter(tags::Column::Name.eq(name))
        .all(db)
        .await?
        .is_empty();

    Ok(empty)
}
