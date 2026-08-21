use std::collections::HashMap;

use crate::{
    articles::ArticleUuid,
    common::out::StorageError,
    tags::{Tag, TagUuid, TagWithCount},
};

/// repository for fetching, inserting, deleting tags and any relationships
/// with these tags
pub trait TagsRepository: Clone + Send + Sync + 'static {
    /// Finds all tags
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn all(&self) -> Result<Vec<Tag>, StorageError>;

    /// Finds all tags and also counts how often these tags are beeing used
    /// in other articles
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn all_with_count(&self) -> Result<Vec<TagWithCount>, StorageError>;

    /// Takes a list of article ids and then finds all of the tags for each of
    /// those articles
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn all_of_articles(
        &self,
        articles: Vec<&ArticleUuid>,
    ) -> Result<HashMap<ArticleUuid, Vec<Tag>>, StorageError>;

    /// Takes a single article id and then finds all of the tags for that
    /// specific article
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn all_of_article(&self, article: &ArticleUuid) -> Result<Vec<Tag>, StorageError>;

    /// Inserts the passed tag in the database
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn insert(&self, tag: Tag) -> Result<(), StorageError>;

    /// Deletes the tag that has the specified tag id
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn delete(&self, tag: TagUuid) -> Result<(), StorageError>;

    /// Updates the tag in the database to conform to the tag parameter
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn update(&self, tag: Tag) -> Result<(), StorageError>;

    /// Tries to find tags that have the same name as passed name. Will return
    /// the found tag, if one was found
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn by_name(&self, name: &str) -> Result<Option<Tag>, StorageError>;

    /// Tries to find tags that have the same name as passed name, will only
    /// confirm existence with a boolean
    ///
    /// # Errors
    ///
    /// Returns any database errors wrapped as StorageErrors
    async fn name_exists(&self, name: &str) -> Result<bool, StorageError>;
}
