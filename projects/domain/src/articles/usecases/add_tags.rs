use std::collections::HashSet;

use crate::{
    articles::{ArticleUuid, out::repository::ArticlesRepository},
    common::out::StorageError,
    tags::TagUuid,
};

/// Assigns tags to an article after checking that none are already assigned.
pub async fn add_tags(
    article_uuid: ArticleUuid,
    tag_uuids: HashSet<TagUuid>,
    repo: &impl ArticlesRepository,
) -> Result<(), AddTagsError> {
    if tag_uuids.is_empty() {
        return Ok(());
    }

    match repo
        .assigned_tags(article_uuid, tag_uuids.clone())
        .await
        .map_err(AddTagsError::from)?
        .as_slice()
    {
        [] => Ok(()),
        [single] => Err(AddTagsError::TagAlreadyAssigned(*single)),
        other => Err(AddTagsError::TagsAlreadyAssigned(other.to_vec())),
    }?;

    repo.add_tags(article_uuid, tag_uuids)
        .await
        .map_err(AddTagsError::from)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum AddTagsError {
    #[error("the Tag with uuid {0} is already assigned to this article")]
    TagAlreadyAssigned(TagUuid),
    #[error("the following tags are already assigned to this article: {0:?}")]
    TagsAlreadyAssigned(Vec<TagUuid>),
    #[error("{0}")]
    Db(#[from] StorageError),
}
