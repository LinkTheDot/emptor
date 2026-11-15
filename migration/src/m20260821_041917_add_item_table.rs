use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Item::Table)
          .if_not_exists()
          .col(pk_auto(Item::Id))
          .col(string(Item::Name))
          .col(integer(Item::StoreId).null())
          .col(
            timestamp(Item::CreatedAt)
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .col(
            timestamp(Item::UpdatedAt)
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-item-store_id")
              .from(Item::Table, Item::StoreId)
              .to(Store::Table, Store::Id)
              .on_delete(ForeignKeyAction::SetNull),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Item::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Item {
  Table,
  Id,
  Name,
  StoreId,
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveIden)]
enum Store {
  Table,
  Id,
  _Name,
  _Address,
  _CreatedAt,
  _UpdatedAt,
}
