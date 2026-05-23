use domain::tags::TagWithCount;
use sea_orm::FromQueryResult;

use crate::DbTagUuid;

#[derive(FromQueryResult)]
pub struct DbTagWithCount {
    pub uuid: DbTagUuid,
    pub name: String,
    pub description: String,
    pub count: i64,
}

impl From<DbTagWithCount> for TagWithCount {
    fn from(
        DbTagWithCount {
            uuid,
            name,
            description,
            count,
        }: DbTagWithCount,
    ) -> Self {
        Self {
            uuid: uuid.into(),
            name,
            description,
            count: count as usize,
        }
    }
}
