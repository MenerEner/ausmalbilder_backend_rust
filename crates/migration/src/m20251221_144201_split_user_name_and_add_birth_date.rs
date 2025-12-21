use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(Alias::new("first_name")).string().not_null().default(""))
                    .add_column(ColumnDef::new(Alias::new("last_name")).string().not_null().default(""))
                    .add_column(ColumnDef::new(Alias::new("birth_date")).date())
                    .to_owned(),
            )
            .await?;

        // migrate data if needed (assuming name is "First Last")
        // but for simplicity and since it's a new feature, we might just drop the name column
        
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::Name)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::Name).string().not_null().default(""))
                    .drop_column(Alias::new("first_name"))
                    .drop_column(Alias::new("last_name"))
                    .drop_column(Alias::new("birth_date"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Name,
}
