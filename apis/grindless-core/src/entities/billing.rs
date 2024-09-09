use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, DeriveRelation};
use serde::{Deserialize, Serialize};

use crate::{ID, Timestamp};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, DeriveEntityModel)]
#[serde(rename_all = "snake_case")]
#[sea_orm(table_name = "billings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ID,

    #[sea_orm(column_type = "BigInteger")]
    pub account_id: ID,

    #[sea_orm(column_type = "BigInteger")]
    pub credits: i64,

    #[sea_orm(column_type = "Double")]
    pub total_spent_usd: f64,

    #[sea_orm(column_type = "BigInteger")]
    pub created_at: Timestamp,
    #[sea_orm(column_type = "BigInteger")]
    pub updated_at: Timestamp,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entities::account::Entity",
        from = "Column::AccountId",
        to = "crate::entities::account::Column::Id"
    )]
    Account,
}

impl Related<crate::entities::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
