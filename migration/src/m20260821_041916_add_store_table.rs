use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Store::Table)
          .if_not_exists()
          .col(pk_auto(Store::Id))
          .col(string(Store::Name))
          .col(string(Store::Address))
          .col(
            timestamp(Store::CreatedAt)
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .col(
            timestamp(Store::UpdatedAt)
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Store::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Store {
  Table,
  Id,
  Name,
  Address,
  CreatedAt,
  UpdatedAt,
}
