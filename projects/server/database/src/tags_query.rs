mod article_tag;
mod tag_with_count;

use std::collections::HashMap;

use domain::{
    articles::ArticleUuid,
    common::out::StorageError,
    tags::{Tag, TagUuid, TagWithCount, out::repository::TagsRepository},
};
use itertools::Itertools;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityOrSelect, EntityTrait, IntoActiveModel, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};

use crate::{
    DbArticleUuid, DbTagUuid,
    entities::{article_tags, tags},
    tags_query::{article_tag::ArticleTag, tag_with_count::DbTagWithCount},
};

/// implementation of the [TagsRepository] to query tags
#[derive(Clone, Debug)]
pub struct TagsRepositoryImpl(DatabaseConnection);

impl TagsRepositoryImpl {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self(database_connection)
    }

    pub(crate) async fn insert_trx(
        trx: &impl ConnectionTrait,
        tag: Tag,
    ) -> Result<(), StorageError> {
        tags::Entity::insert(tags::Model::from(tag).into_active_model())
            .exec(trx)
            .await
            .map_err(StorageError::db_int_err)?;

        Ok(())
    }

    pub(crate) async fn delete_trx(
        trx: &impl ConnectionTrait,
        tag: TagUuid,
    ) -> Result<(), StorageError> {
        tags::Entity::delete_by_id(DbTagUuid::from(tag))
            .exec(trx)
            .await
            .map_err(StorageError::db_int_err)?;

        Ok(())
    }
}

impl TagsRepository for TagsRepositoryImpl {
    async fn all(&self) -> Result<Vec<Tag>, StorageError> {
        let query = tags::Entity::find()
            .select_only()
            .column(tags::Column::Uuid)
            .column(tags::Column::Name)
            .column(tags::Column::Description)
            .into_model::<tags::Model>()
            .all(&self.0)
            .await
            .map_err(StorageError::db_int_err)?
            .into_iter()
            .map(Tag::from)
            .collect_vec();

        Ok(query)
    }

    async fn all_with_count(&self) -> Result<Vec<domain::tags::TagWithCount>, StorageError> {
        let query = tags::Entity::find()
            .select_only()
            .column(tags::Column::Uuid)
            .column(tags::Column::Name)
            .column(tags::Column::Description)
            .column_as(article_tags::Column::TagUuid.count(), "count")
            .join(JoinType::LeftJoin, tags::Relation::ArticleTags.def())
            .group_by(tags::Column::Uuid)
            .group_by(tags::Column::Name)
            .group_by(tags::Column::Description)
            .group_by(article_tags::Column::TagUuid);

        Ok(query
            .into_model::<DbTagWithCount>()
            .all(&self.0)
            .await
            .map_err(StorageError::db_int_err)?
            .into_iter()
            .map(TagWithCount::from)
            .collect_vec())
    }

    async fn all_of_articles(
        &self,
        articles: Vec<&ArticleUuid>,
    ) -> Result<HashMap<ArticleUuid, Vec<Tag>>, StorageError> {
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
            .all(&self.0)
            .await
            .map_err(StorageError::db_int_err)?
            .into_iter()
            .map(|art_tag| (art_tag.article_uuid.into(), Tag::from(art_tag)))
            .into_group_map())
    }

    async fn all_of_article(&self, article: &ArticleUuid) -> Result<Vec<Tag>, StorageError> {
        let article = DbArticleUuid::from(**article);
        let tags = tags::Entity::find()
            .select_only()
            .column(tags::Column::Uuid)
            .column(tags::Column::Name)
            .column(tags::Column::Description)
            .join(JoinType::InnerJoin, tags::Relation::ArticleTags.def())
            .filter(article_tags::Column::ArticleUuid.eq(article))
            .all(&self.0)
            .await
            .map_err(StorageError::db_int_err)?
            .into_iter()
            .map(Tag::from)
            .collect_vec();

        Ok(tags)
    }

    async fn insert(&self, tag: Tag) -> Result<(), StorageError> {
        Self::insert_trx(&self.0, tag).await
    }

    async fn delete(&self, tag: TagUuid) -> Result<(), StorageError> {
        Self::delete_trx(&self.0, tag).await
    }

    async fn update(&self, tag: Tag) -> Result<(), StorageError> {
        let tags::Model {
            uuid,
            name,
            description,
        } = tags::Model::from(tag);

        tags::ActiveModel {
            uuid: ActiveValue::Unchanged(uuid),
            name: ActiveValue::Set(name),
            description: ActiveValue::Set(description),
        }
        .update(&self.0)
        .await
        .map_err(StorageError::db_int_err)?;

        Ok(())
    }

    async fn by_name(&self, name: &str) -> Result<Option<Tag>, StorageError> {
        let tag = tags::Entity::find()
            .select()
            .filter(tags::Column::Name.eq(name))
            .one(&self.0)
            .await
            .map_err(StorageError::db_int_err)?
            .map(Tag::from);

        Ok(tag)
    }

    async fn name_exists(&self, name: &str) -> Result<bool, StorageError> {
        let empty = tags::Entity::find()
            .select()
            .filter(tags::Column::Name.eq(name))
            .all(&self.0)
            .await
            .map_err(StorageError::db_int_err)?
            .is_empty();

        Ok(!empty)
    }
}
