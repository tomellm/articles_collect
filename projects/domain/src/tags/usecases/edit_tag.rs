use std::str::FromStr;

use uuid::Uuid;

use crate::{
    common::out::StorageError,
    tags::{Tag, out::repository::TagsRepository},
};

pub async fn edit_tag(
    tag_uuid: String,
    tag_name: String,
    tag_description: String,
    repo: &impl TagsRepository,
) -> Result<(), EditTagError> {
    if tag_name.is_empty() {
        return Err(EditTagError::EmptyName);
    }

    let tag_uuid = Uuid::from_str(&tag_uuid).map_err(|_| EditTagError::NotUuid(tag_uuid))?;
    let tag_name = tag_name.to_lowercase();

    let existing_tag = repo.by_name(&tag_name).await.map_err(EditTagError::from)?;

    if existing_tag
        .map(|tag| !tag.uuid.0.eq(&tag_uuid))
        .unwrap_or_default()
    {
        return Err(EditTagError::NameExists(tag_name));
    }

    let tag = Tag::builder()
        .uuid(tag_uuid)
        .name(tag_name)
        .description(tag_description)
        .build();

    repo.update(tag).await.map_err(EditTagError::from)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum EditTagError {
    #[error("the name of the tag was left empty")]
    EmptyName,
    #[error("another tag already has the name '{0}', change it to something else")]
    NameExists(String),
    #[error("the provided uuid string cannot be parsed: '{0}'")]
    NotUuid(String),
    #[error("{0}")]
    Db(StorageError),
}

impl From<StorageError> for EditTagError {
    fn from(value: StorageError) -> Self {
        Self::Db(value)
    }
}
