use std::collections::HashSet;

use crate::{
    articles::{Article, ArticleUuid},
    common::out::StorageError,
    tags::TagUuid,
};

/// Repository for fetching, inserting, deleting articles and any relationships
/// with these articles
pub trait ArticlesRepository: Clone + Send + Sync + 'static {
    /// Finds all articles
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn all(&self) -> Result<Vec<Article>, StorageError>;

    /// Finde the article with the specified uuid
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn one(&self, uuid: ArticleUuid) -> Result<Option<Article>, StorageError>;

    /// Instert all of the defined articles in the database
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn insert_many(&self, articles: Vec<Article>) -> Result<(), StorageError>;

    /// Delete the specified article and all of the attached tags
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn delete(&self, uuid: ArticleUuid) -> Result<(), StorageError>;

    /// Checks if the specific article has this tag attached to it
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn has_tag(&self, uuid: ArticleUuid, tag: TagUuid) -> Result<bool, StorageError>;

    /// Tries to find all of the tags and check if they have been assigned to
    /// the article. Will return all of the tags that have already been
    /// assigned to the article
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn assigned_tags(
        &self,
        uuid: ArticleUuid,
        tags: impl IntoIterator<Item = TagUuid>,
    ) -> Result<Vec<TagUuid>, StorageError>;

    /// Adds the list of tags to the specified article
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn add_tags(&self, uuid: ArticleUuid, tags: HashSet<TagUuid>)
    -> Result<(), StorageError>;

    /// Removes the specific tag from the specified article
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn remove_tag(&self, uuid: ArticleUuid, tag: TagUuid) -> Result<(), StorageError>;
}
