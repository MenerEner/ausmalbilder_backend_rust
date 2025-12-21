use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum EmailVerificationToken {
    Table,
    Token,
    UserId,
    CreatedAt,
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EmailVerificationToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailVerificationToken::Token)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(EmailVerificationToken::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailVerificationToken::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-email_verification_token-user_id")
                            .from(EmailVerificationToken::Table, EmailVerificationToken::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(EmailVerificationToken::Table)
                    .to_owned(),
            )
            .await
    }
}
