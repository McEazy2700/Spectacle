use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Template::Table)
                    .col(
                        ColumnDef::new(Template::Id)
                            .integer()
                            .primary_key()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Template::Name).string().not_null())
                    .col(ColumnDef::new(Template::FontSize).integer().not_null())
                    .col(ColumnDef::new(Template::FontStyle).string().not_null())
                    .col(ColumnDef::new(Template::FontColor).string())
                    .col(ColumnDef::new(Template::Background).string())
                    .col(ColumnDef::new(Template::FontWeight).integer().not_null())
                    .col(ColumnDef::new(Template::TextAlignment).string().not_null())
                    .col(
                        ColumnDef::new(Template::VerticalAlignment)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Template::HorizontalAlignment)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Template::SideTextFontSize)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Template::SideTextFontStyle)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Template::SideTextFontColor).string())
                    .col(
                        ColumnDef::new(Template::SideTextFontWeight)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Template::SideTextTextAlignment)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Template::SideTextVerticalAlignment)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Template::SideTextHorizontalAlignment)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Template::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Template {
    Table,
    Id,
    Name,
    FontSize,
    FontStyle,
    FontColor,
    Background,
    FontWeight,
    TextAlignment,
    VerticalAlignment,
    HorizontalAlignment,
    SideTextFontSize,
    SideTextFontStyle,
    SideTextFontColor,
    SideTextFontWeight,
    SideTextTextAlignment,
    SideTextVerticalAlignment,
    SideTextHorizontalAlignment,
}
