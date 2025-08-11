use crate::extension::postgres::Type;
use crate::sea_orm::{DbBackend, EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        {
            manager
                .create_table(
                    Table::create()
                        .table(Users::Table)
                        .col(pk_uuid(Users::UserId))
                        .col(string_null(Users::GoogId).unique_key())
                        .col(string_null(Users::Picture))
                        .to_owned(),
                )
                .await?;
        }

        {
            manager
                .create_table(
                    Table::create()
                        .table(Services::Table)
                        .col(pk_uuid(Services::ServiceId))
                        .col(uuid(Services::OwnerId))
                        .col(string(Services::Name))
                        .col(string(Services::VapidPublic))
                        .col(string(Services::VapidPrivate))
                        .to_owned(),
                )
                .await?;

            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .from(Services::Table, Services::OwnerId)
                        .to(Users::Table, Users::UserId)
                        .to_owned(),
                )
                .await?;
        }

        {
            manager
                .create_table(
                    Table::create()
                        .table(Subscribers::Table)
                        .col(pk_uuid(Subscribers::SubscriberId))
                        .col(string_null(Subscribers::Name))
                        .col(string_null(Subscribers::Email))
                        .col(string(Subscribers::Endpoint))
                        .col(string(Subscribers::ClientKey))
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .name("unique_subscriber_endpoint")
                        .table(Subscribers::Table)
                        .col(Subscribers::Endpoint)
                        .unique()
                        .to_owned(),
                )
                .await?;
        }

        {
            manager
                .create_table(
                    Table::create()
                        .table(Groups::Table)
                        .to_owned()
                        .col(uuid(Groups::ServiceId))
                        .col(uuid(Groups::GroupId))
                        .col(string(Groups::Name))
                        .col(timestamp_null(Groups::LastNotified))
                        .primary_key(Index::create().col(Groups::ServiceId).col(Groups::GroupId))
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .name("unique_group_name")
                        .table(Groups::Table)
                        .col(Groups::ServiceId)
                        .col(Groups::Name)
                        .unique()
                        .to_owned(),
                )
                .await?;

            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .from(Groups::Table, Groups::ServiceId)
                        .to(Services::Table, Services::ServiceId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        {
            manager
                .create_table(
                    Table::create()
                        .table(GroupSubscribers::Table)
                        .col(uuid(GroupSubscribers::ServiceId))
                        .col(uuid(GroupSubscribers::GroupId))
                        .col(uuid(GroupSubscribers::SubscriberId))
                        .primary_key(
                            Index::create()
                                .col(GroupSubscribers::ServiceId)
                                .col(GroupSubscribers::GroupId)
                                .col(GroupSubscribers::SubscriberId),
                        )
                        .to_owned(),
                )
                .await?;

            manager.create_index(
                Index::create()
                    .table(GroupSubscribers::Table)
                    .col(GroupSubscribers::ServiceId)
                    .col(GroupSubscribers::GroupId)
                    .to_owned()
            ).await?;

            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .from(GroupSubscribers::Table, GroupSubscribers::ServiceId)
                        .from(GroupSubscribers::Table, GroupSubscribers::GroupId)
                        .to(Groups::Table, Groups::ServiceId)
                        .to(Groups::Table, Groups::GroupId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .from(GroupSubscribers::Table, GroupSubscribers::SubscriberId)
                        .to(Subscribers::Table, Subscribers::SubscriberId)
                        .to_owned(),
                )
                .await?;
        }

        {
            manager
                .create_table(
                    Table::create()
                        .table(ApiKeys::Table)
                        .col(uuid(ApiKeys::ServiceId))
                        .col(pk_uuid(ApiKeys::KeyId))
                        .col(string(ApiKeys::Name))
                        .col(binary(ApiKeys::Key))
                        .col(timestamp_null(ApiKeys::LastUsed))
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .name("unique_api_key_name")
                        .table(ApiKeys::Table)
                        .col(ApiKeys::ServiceId)
                        .col(ApiKeys::Name)
                        .unique()
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .name("unique_api_key_key")
                        .table(ApiKeys::Table)
                        .col(ApiKeys::Key)
                        .unique()
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .name("api_keys_service_id")
                        .table(ApiKeys::Table)
                        .col(ApiKeys::ServiceId)
                        .to_owned(),
                )
                .await?;

            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .from(ApiKeys::Table, ApiKeys::ServiceId)
                        .to(Services::Table, Services::ServiceId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        {
            assert_eq!(manager.get_database_backend(), DbBackend::Postgres);

            manager
                .create_type(
                    Type::create()
                        .as_enum("key_scope")
                        .values(KeyScope::iter())
                        .to_owned(),
                )
                .await?;

            manager
                .create_table(
                    Table::create()
                        .table(ApiKeyScopes::Table)
                        .col(pk_uuid(ApiKeyScopes::ScopeId))
                        .col(uuid(ApiKeyScopes::KeyId))
                        .col(uuid(ApiKeyScopes::ServiceId))
                        .col(uuid_null(ApiKeyScopes::GroupId))
                        .col(enumeration(
                            ApiKeyScopes::Scope,
                            "key_scope",
                            KeyScope::iter(),
                        ))
                        .to_owned(),
                )
                .await?;

            manager
                .create_index(
                    Index::create()
                        .table(ApiKeyScopes::Table)
                        .col(ApiKeyScopes::KeyId)
                        .col(ApiKeyScopes::ServiceId)
                        .col(ApiKeyScopes::GroupId)
                        .to_owned(),
                )
                .await?;

            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .from(ApiKeyScopes::Table, ApiKeyScopes::KeyId)
                        .to(ApiKeys::Table, ApiKeys::KeyId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .from(ApiKeyScopes::Table, ApiKeyScopes::ServiceId)
                        .to(Services::Table, Services::ServiceId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .from(ApiKeyScopes::Table, ApiKeyScopes::ServiceId)
                        .from(ApiKeyScopes::Table, ApiKeyScopes::GroupId)
                        .to(Groups::Table, Groups::ServiceId)
                        .to(Groups::Table, Groups::GroupId)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ApiKeyScopes::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name("key_scope").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ApiKeys::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(GroupSubscribers::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Groups::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Subscribers::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Services::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
    GoogId,
    Picture,
}

#[derive(DeriveIden)]
enum Services {
    Table,
    ServiceId,
    OwnerId,
    Name,
    VapidPrivate,
    VapidPublic,
}

#[derive(DeriveIden)]
enum Subscribers {
    Table,
    SubscriberId,
    Name,
    Email,
    Endpoint,
    ClientKey,
}

#[derive(DeriveIden)]
enum Groups {
    Table,
    ServiceId,
    GroupId,
    Name,
    LastNotified,
}

#[derive(DeriveIden)]
enum GroupSubscribers {
    Table,
    ServiceId,
    GroupId,
    SubscriberId,
}

#[derive(DeriveIden)]
enum ApiKeys {
    Table,
    ServiceId,
    KeyId,
    Name,
    Key,
    LastUsed,
}

#[derive(Iden, EnumIter)]
pub enum KeyScope {
    #[iden = "sub"]
    Subscribe,
    #[iden = "notify"]
    Notify,
}

#[derive(DeriveIden)]
enum ApiKeyScopes {
    Table,
    ScopeId,
    ServiceId,
    KeyId,
    Scope,
    GroupId,
}
