use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Subscribers::Table)
                    .add_column(string_null(Subscribers::P256dh))
                    .add_column(string_null(Subscribers::Auth))
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::update()
                    .table(Subscribers::Table)
                    .values([
                        (
                            Subscribers::P256dh,
                            Expr::cust(r#"(client_key::json ->> 'p256dh')"#),
                        ),
                        (
                            Subscribers::Auth,
                            Expr::cust(r#"(client_key::json ->> 'auth')"#),
                        ),
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Subscribers::Table)
                    .modify_column(ColumnDef::new(Subscribers::P256dh).not_null())
                    .modify_column(ColumnDef::new(Subscribers::Auth).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Subscribers::Table)
                    .drop_column(Subscribers::ClientKey)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Subscribers::Table)
                    .add_column(string_null(Subscribers::ClientKey))
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::update()
                    .table(Subscribers::Table)
                    .values([(
                        Subscribers::ClientKey,
                        Expr::cust(r#"json_build_object('auth', "auth", 'p256dh', "p256dh")"#),
                    )])
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Subscribers::Table)
                    .modify_column(ColumnDef::new(Subscribers::ClientKey).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Subscribers::Table)
                    .drop_column(Subscribers::P256dh)
                    .drop_column(Subscribers::Auth)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Subscribers {
    Table,
    ClientKey,
    P256dh,
    Auth,
}
