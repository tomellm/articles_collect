use crate::{
    common::out::StorageError,
    tags::{Tag, out::repository::TagsRepository},
};

pub async fn add_new_tag(
    tag_name: String,
    tag_description: String,
    repo: &impl TagsRepository,
) -> Result<Tag, AddTagError> {
    if tag_name.is_empty() {
        return Err(AddTagError::EmptyName);
    }

    let tag_name = tag_name.to_lowercase();

    if repo
        .name_exists(&tag_name)
        .await
        .map_err(AddTagError::from)?
    {
        return Err(AddTagError::NameExists(tag_name));
    }

    let tag = Tag::builder()
        .name(tag_name.clone())
        .description(tag_description.clone())
        .build();

    repo.insert(tag.clone()).await.map_err(AddTagError::from)?;

    tracing::info!(
        "created tag '{}' with name '{}' and description of length '{}'",
        tag.uuid,
        tag_name,
        tag_description.len()
    );

    Ok(tag)
}

#[derive(Debug, thiserror::Error)]
pub enum AddTagError {
    #[error("the name of the tag was left empty")]
    EmptyName,
    #[error("the name '{0}' alredy exists as a tag name")]
    NameExists(String),
    #[error("{0}")]
    Db(StorageError),
}

impl From<StorageError> for AddTagError {
    fn from(value: StorageError) -> Self {
        Self::Db(value)
    }
}
