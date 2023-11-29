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
                    .col(ColumnDef::new(Template::Background).string())
                    // Font
                    .col(ColumnDef::new(Template::FontSize).integer().not_null())
                    .col(ColumnDef::new(Template::FontStyle).string().not_null())
                    .col(ColumnDef::new(Template::FontColor).string())
                    .col(ColumnDef::new(Template::FontWeight).integer().not_null())
                    // Text Shadow
                    .col(
                        ColumnDef::new(Template::TextShadow)
                            .boolean()
                            .not_null()
                            .default(Value::Bool(Some(false))),
                    )
                    .col(ColumnDef::new(Template::TextShadowBlur).integer())
                    .col(ColumnDef::new(Template::TextShadowColor).string())
                    .col(ColumnDef::new(Template::TextShadowVertical).integer())
                    .col(ColumnDef::new(Template::TextShadowHorizontal).integer())
                    // Alignment
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
                    // Side Text Font
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
                    // Side Text Text Shadow
                    .col(
                        ColumnDef::new(Template::SideTextShadow)
                            .boolean()
                            .not_null()
                            .default(Value::Bool(Some(false))),
                    )
                    .col(ColumnDef::new(Template::SideTextShadowBlur).integer())
                    .col(ColumnDef::new(Template::SideTextShadowColor).string())
                    .col(ColumnDef::new(Template::SideTextShadowVertical).integer())
                    .col(ColumnDef::new(Template::SideTextShadowHorizontal).integer())
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
pub enum Template {
    Table,
    Id,
    Name,
    FontSize,
    FontColor,
    FontStyle,
    Background,
    FontWeight,
    TextShadow,
    TextShadowBlur,
    TextShadowColor,
    TextShadowVertical,
    TextShadowHorizontal,
    TextAlignment,
    VerticalAlignment,
    HorizontalAlignment,
    SideTextFontSize,
    SideTextFontStyle,
    SideTextFontColor,
    SideTextFontWeight,
    SideTextShadow,
    SideTextShadowBlur,
    SideTextShadowColor,
    SideTextShadowVertical,
    SideTextShadowHorizontal,
    SideTextTextAlignment,
    SideTextVerticalAlignment,
    SideTextHorizontalAlignment,
}
