pub use sea_orm_migration::prelude::*;

mod m20231101_184734_create_video_thumbnail_table;
mod m20231102_123234_create_template_table;
mod m20231126_112928_create_default_template_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231101_184734_create_video_thumbnail_table::Migration),
            Box::new(m20231102_123234_create_template_table::Migration),
            Box::new(m20231126_112928_create_default_template_table::Migration),
        ]
    }
}
