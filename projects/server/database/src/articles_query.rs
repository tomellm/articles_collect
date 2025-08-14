use domain::articles::Article;
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, IntoActiveModel};
use uuid::Uuid;

use crate::entities::articles;

pub async fn all<C>(db: &C) -> Result<Vec<Article>, DbErr>
where
    C: ConnectionTrait,
{
    articles::Entity::find()
        .all(db)
        .await
        .map(|art| art.into_iter().map(Into::into).collect())
}

pub async fn one<C>(db: &C, uuid: Uuid) -> Result<Option<Article>, DbErr>
where
    C: ConnectionTrait,
{
    articles::Entity::find_by_id(uuid)
        .one(db)
        .await
        .map(|opt_art| opt_art.map(|art| art.into()))
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

pub async fn delete<C>(article_uuid: Uuid, db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    articles::Entity::delete_by_id(article_uuid)
        .exec(db)
        .await
        .map(|_| ())
}
