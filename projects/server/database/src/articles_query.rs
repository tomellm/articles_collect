use std::{collections::HashSet, sync::Arc};

use domain::{
    articles::{
        Article, ArticleUuid, builder::ArticleMissingTags, out::repository::ArticlesRepository,
    },
    common::out::StorageError,
    tags::{TagUuid, out::repository::TagsRepository},
};
use itertools::Itertools;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter,
};

use crate::{
    DbArticleUuid, DbTagUuid,
    entities::{self, article_tags, articles},
    tags_query::TagsRepositoryImpl,
};

/// implementation of the [ArticlesRepository] trait, is dependent on a
/// [TagsRepository] implementation
#[derive(Clone, Debug)]
pub struct ArticlesRepositoryImpl {
    /// the connection with which the requests are made
    db: DatabaseConnection,
    /// the [TagsRepository] implemenetation used to query tags
    tags: Arc<TagsRepositoryImpl>,
}

impl ArticlesRepositoryImpl {
    pub fn new(db: DatabaseConnection, tags: Arc<TagsRepositoryImpl>) -> Self {
        Self { db, tags }
    }
}

impl ArticlesRepository for ArticlesRepositoryImpl {
    async fn all(&self) -> Result<Vec<Article>, StorageError> {
        let articles: Vec<ArticleMissingTags> = articles::Entity::find()
            .all(&self.db)
            .await
            .map(|art| art.into_iter().map(Into::into).collect())
            .map_err(StorageError::db_int_err)?;

        let mut tags = self
            .tags
            .all_of_articles(articles.iter().map(|a| a.get_uuid()).collect_vec())
            .await
            .map_err(StorageError::db_int_err)?;

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

    async fn one(&self, uuid: ArticleUuid) -> Result<Option<Article>, StorageError> {
        let Some(art) = articles::Entity::find_by_id(DbArticleUuid::from(uuid))
            .one(&self.db)
            .await
            .map_err(StorageError::db_int_err)?
        else {
            return Ok(None);
        };
        let art = ArticleMissingTags::from(art);
        let tags = self
            .tags
            .all_of_article(art.get_uuid())
            .await
            .map_err(StorageError::db_int_err)?
            .into_boxed_slice();

        Ok(Some(art.tags(tags).build()))
    }

    async fn insert_many(&self, articles: Vec<Article>) -> Result<(), StorageError> {
        let entities = articles
            .into_iter()
            .map(|a| articles::Model::from(a).into_active_model());

        articles::Entity::insert_many(entities)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(StorageError::db_int_err)
    }

    async fn delete(&self, uuid: ArticleUuid) -> Result<(), StorageError> {
        articles::Entity::delete_by_id(DbArticleUuid::from(uuid))
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(StorageError::db_int_err)
    }

    async fn has_tag(
        &self,
        uuid: ArticleUuid,
        tag: domain::tags::TagUuid,
    ) -> Result<bool, StorageError> {
        article_tags::Entity::find()
            .filter(
                article_tags::Column::ArticleUuid
                    .eq(DbArticleUuid::from(*uuid))
                    .and(article_tags::Column::TagUuid.eq(DbTagUuid::from(*tag))),
            )
            .one(&self.db)
            .await
            .map(|e| e.is_some())
            .map_err(StorageError::db_int_err)
    }

    /// Finds which of the requested tags are already assigned to the article.
    async fn assigned_tags(
        &self,
        uuid: ArticleUuid,
        tags: impl IntoIterator<Item = TagUuid>,
    ) -> Result<Vec<TagUuid>, StorageError> {
        let tags = tags.into_iter().map(DbTagUuid::from).collect_vec();

        if tags.is_empty() {
            return Ok(vec![]);
        }

        article_tags::Entity::find()
            .filter(
                article_tags::Column::ArticleUuid
                    .eq(DbArticleUuid::from(*uuid))
                    .and(article_tags::Column::TagUuid.is_in(tags)),
            )
            .all(&self.db)
            .await
            .map(|tags| {
                tags.into_iter()
                    .map(|tag| tag.tag_uuid.into())
                    .collect_vec()
            })
            .map_err(StorageError::db_int_err)
    }

    /// Inserts all article-tag relationships with one bulk insert.
    async fn add_tags(
        &self,
        uuid: ArticleUuid,
        tags: HashSet<TagUuid>,
    ) -> Result<(), StorageError> {
        if tags.is_empty() {
            return Ok(());
        }

        let article_tags = tags
            .into_iter()
            .map(|tag| {
                entities::article_tags::Model {
                    tag_uuid: tag.into(),
                    article_uuid: uuid.into(),
                }
                .into_active_model()
            })
            .collect::<Vec<_>>();

        article_tags::Entity::insert_many(article_tags)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(StorageError::db_int_err)
    }

    async fn remove_tag(
        &self,
        uuid: ArticleUuid,
        tag: domain::tags::TagUuid,
    ) -> Result<(), StorageError> {
        entities::article_tags::Model {
            tag_uuid: tag.into(),
            article_uuid: uuid.into(),
        }
        .delete(&self.db)
        .await
        .map(|_| ())
        .map_err(StorageError::db_int_err)
    }
}
