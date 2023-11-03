use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
};

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
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Template::Name).string().not_null())
                    .col(ColumnDef::new(Template::FontSize).integer())
                    .col(ColumnDef::new(Template::Font).string())
                    .col(ColumnDef::new(Template::BackgroundURL).string())
                    .col(
                        ColumnDef::new(Template::FontWeight)
                            .enumeration(FontWeight::Table, FontWeight::iter().skip(1)),
                    )
                    .col(ColumnDef::new(Template::BackgroundType).enumeration(
                        BackgroundType::Table,
                        [BackgroundType::Image, BackgroundType::Video],
                    ))
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

#[derive(Iden, EnumIter)]
enum FontWeight {
    Table,
    Light,
    Normal,
    SemiBold,
    Bold,
    ExtraBold,
}

#[derive(Iden, EnumIter)]
enum BackgroundType {
    Table,
    Image,
    Video,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Template {
    Table,
    Id,
    Name,
    Font,
    FontSize,
    FontWeight,
    BackgroundURL,
    BackgroundType,
}
