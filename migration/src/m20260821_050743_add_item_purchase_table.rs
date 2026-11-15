use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(ItemPurchase::Table)
          .if_not_exists()
          .col(pk_auto(ItemPurchase::Id))
          .col(integer(ItemPurchase::ItemId).null())
          .col(float(ItemPurchase::Amount))
          .col(timestamp(ItemPurchase::PurchaseDate))
          .col(
            timestamp(ItemPurchase::CreatedAt)
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .col(
            timestamp(ItemPurchase::UpdatedAt)
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-item_purchase-item_id")
              .from(ItemPurchase::Table, ItemPurchase::ItemId)
              .to(Item::Table, Item::Id)
              .on_delete(ForeignKeyAction::SetNull),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(ItemPurchase::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum ItemPurchase {
  Table,
  Id,
  ItemId,
  Amount,
  PurchaseDate,
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveIden)]
enum Item {
  Table,
  Id,
  _Name,
  _StoreId,
  _CreatedAt,
  _UpdatedAt,
}
