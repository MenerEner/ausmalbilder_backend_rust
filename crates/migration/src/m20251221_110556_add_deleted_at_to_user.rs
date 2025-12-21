use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add deleted_at column
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(Alias::new("deleted_at")).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // Drop the old unique index on Email
        // Note: SeaORM's unique_key() usually creates a constraint/index named "idx-user-email" or similar depending on the DB.
        // In PostgreSQL, it's often "user_email_key".
        // Let's try to drop it and add a conditional unique index.

        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE \"user\" DROP CONSTRAINT IF EXISTS \"user_email_key\"")
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-user-email-active")
                    .table(User::Table)
                    .col(User::Email)
                    .unique()
                    .and_where(Expr::col(Alias::new("deleted_at")).is_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-user-email-active").to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(Alias::new("deleted_at"))
                    .to_owned(),
            )
            .await?;

        // Re-add unique constraint (this might fail if there are duplicates now, but that's what down is for)
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum User {
    Table,
    Email,
}
