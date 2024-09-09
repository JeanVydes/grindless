use sea_orm::entity::prelude::*;
use sea_orm::{DeriveActiveEnum, DeriveEntityModel, DeriveRelation};
use serde::{Deserialize, Serialize};

use crate::{ID, Timestamp};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, DeriveEntityModel)]
#[serde(rename_all = "snake_case")]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ID,
    
    #[sea_orm(indexed)]
    pub google_id: String,

    #[sea_orm(column_type = "Text")]
    pub email: String,

    #[sea_orm(column_type = "Text")]
    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub avatar: Option<String>,

    pub flags: Vec<AccountFlags>,

    #[sea_orm(column_type = "BigInteger")]
    pub billing_id: ID,

    // Timestamps
    #[sea_orm(column_type = "BigInteger")]
    pub created_at: Timestamp,
    #[sea_orm(column_type = "BigInteger")]
    pub updated_at: Timestamp,

    // Deletion
    #[sea_orm(column_type = "Boolean")]
    pub deleted: bool,
    #[sea_orm(column_type = "BigInteger", nullable)]
    pub deletion_requested_at: Option<Timestamp>,
    #[sea_orm(column_type = "Text", nullable)]
    pub deletion_reason: Option<String>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[serde(rename_all = "snake_case")]
pub enum AccountFlags {
    #[sea_orm(num_value = 64)]
    Beta,

    #[sea_orm(num_value = 32)]
    Verified,
    #[sea_orm(num_value = 16)]
    Partner,

    #[sea_orm(num_value = 8)]
    Support,
    #[sea_orm(num_value = 4)]
    Moderator,
    #[sea_orm(num_value = 2)]
    Administrator,
    #[sea_orm(num_value = 1)]
    Developer,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "crate::entities::billing::Entity")]
    Billing,
}

impl Related<crate::entities::billing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Billing.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
