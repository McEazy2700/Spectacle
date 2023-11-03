use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VideoThumbnail::Table)
                    .col(
                        ColumnDef::new(VideoThumbnail::VideoURL)
                            .string()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(VideoThumbnail::ThumbnailURL)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VideoThumbnail::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum VideoThumbnail {
    Table,
    VideoURL,
    ThumbnailURL,
}
