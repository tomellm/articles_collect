use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(pk_uuid(Tags::Uuid))
                    .col(string(Tags::Name))
                    .col(string(Tags::Description))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ArticleTags::Table)
                    .if_not_exists()
                    .col(uuid(ArticleTags::TagUuid))
                    .col(uuid(ArticleTags::ArticleUuid))
                    .primary_key(
                        Index::create()
                            .col(ArticleTags::TagUuid)
                            .col(ArticleTags::ArticleUuid),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_tags_articles")
                            .from(ArticleTags::Table, ArticleTags::TagUuid)
                            .to(Tags::Table, Tags::Uuid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_article_tags")
                            .from(ArticleTags::Table, ArticleTags::ArticleUuid)
                            .to(Articles::Table, Articles::Uuid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ArticleTags::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tags {
    Table,
    Uuid,
    Name,
    Description,
}

#[derive(DeriveIden)]
enum ArticleTags {
    Table,
    TagUuid,
    ArticleUuid,
}

#[derive(DeriveIden)]
enum Articles {
    Table,
    Uuid,
}
