use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum User {
    Table,
    IsEmailVerified,
    Role,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(
                        ColumnDef::new(User::IsEmailVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .add_column(
                        ColumnDef::new(User::Role)
                            .string()
                            .not_null()
                            .default("user"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::IsEmailVerified)
                    .drop_column(User::Role)
                    .to_owned(),
            )
            .await
    }
}
