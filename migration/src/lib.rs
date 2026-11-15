pub use sea_orm_migration::prelude::*;

mod m20260818_003437_add_recipt_table;
mod m20260821_041916_add_store_table;
mod m20260821_041917_add_item_table;
mod m20260821_050743_add_item_purchase_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
            Box::new(m20260818_003437_add_recipt_table::Migration),
            Box::new(m20260821_041916_add_store_table::Migration),
            Box::new(m20260821_041917_add_item_table::Migration),
            Box::new(m20260821_050743_add_item_purchase_table::Migration),
        ]
  }
}
