use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Recipt::Table)
          .if_not_exists()
          .col(
            integer(Recipt::Id)
              .integer()
              .not_null()
              .primary_key()
              .auto_increment(),
          )
          .col(
            timestamp(Recipt::Timestamp)
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .col(string(Recipt::Description).null())
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Recipt::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Recipt {
  Table,
  Id,
  Timestamp,
  Description,
}
