use domain::articles::{Article, ArticleUuid, builder::ArticleMissingTags};
use itertools::Itertools;
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, IntoActiveModel};

use crate::{DbArticleUuid, entities::articles, tags_query};

pub async fn all<C>(db: &C) -> Result<Vec<Article>, DbErr>
where
    C: ConnectionTrait,
{
    let articles: Vec<ArticleMissingTags> = articles::Entity::find()
        .all(db)
        .await
        .map(|art| art.into_iter().map(Into::into).collect())?;

    let mut tags =
        tags_query::all_of_articles(db, articles.iter().map(|a| a.get_uuid()).collect_vec())
            .await?;

    let articles = articles
        .into_iter()
        .map(|article| {
            let tags = tags
                .remove(article.get_uuid())
                .unwrap_or(vec![])
                .into_boxed_slice();
            article.tags(tags).build()
        })
        .collect_vec();

    Ok(articles)
}

pub async fn one<C>(db: &C, uuid: ArticleUuid) -> Result<Option<Article>, DbErr>
where
    C: ConnectionTrait,
{
    let Some(art) = articles::Entity::find_by_id(DbArticleUuid::from(uuid))
        .one(db)
        .await?
    else {
        return Ok(None);
    };
    let art = ArticleMissingTags::from(art);
    let tags = tags_query::all_of_article(db, art.get_uuid())
        .await?
        .into_boxed_slice();

    Ok(Some(art.tags(tags).build()))
}

pub async fn insert_many<C>(articles: Vec<Article>, db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    let entities = articles
        .into_iter()
        .map(|a| articles::Model::from(a).into_active_model());

    articles::Entity::insert_many(entities)
        .exec(db)
        .await
        .map(|_| ())
}

pub async fn delete<C>(article_uuid: ArticleUuid, db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    articles::Entity::delete_by_id(DbArticleUuid::from(article_uuid))
        .exec(db)
        .await
        .map(|_| ())
}
