#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20260929_170427_workspaces;
mod m20260929_172136_workpace_members;
mod m20260929_172648_projects;
mod m20260929_174934_tasks;
mod m20260929_181028_add_user_to_tasks;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20260929_170427_workspaces::Migration),
            Box::new(m20260929_172136_workpace_members::Migration),
            Box::new(m20260929_172648_projects::Migration),
            Box::new(m20260929_174934_tasks::Migration),
            Box::new(m20260929_181028_add_user_to_tasks::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
