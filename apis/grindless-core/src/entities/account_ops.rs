use sea_orm::*;
use crate::ID;

use super::{account::{self, ActiveModel, Model}, billing};

pub struct AccountMutationCore;
pub struct AccountQueryCore;

impl AccountMutationCore {
    async fn set_account_active_model(form_data: &Model) -> ActiveModel {
        ActiveModel {
            id: Set(form_data.id.to_owned()),
            google_id: Set(form_data.google_id.to_owned()),
            email: Set(form_data.email.to_owned()),
            name: Set(form_data.name.to_owned()),
            avatar: Set(form_data.avatar.to_owned()),
            flags: Set(form_data.flags.to_owned()),
            billing_id: Set(form_data.billing_id.to_owned()),
            created_at: Set(form_data.created_at.to_owned()),
            updated_at: Set(form_data.updated_at.to_owned()),
            deleted: Set(form_data.deleted.to_owned()),
            deletion_requested_at: Set(form_data.deletion_requested_at.to_owned()),
            deletion_reason: Set(form_data.deletion_reason.to_owned()),
        }
    }

    pub async fn create_account(db: &DbConn, mut form_data: Model, billing: billing::Model) -> Result<Model, DbErr> {
        // create billing
        billing::ActiveModel {
            id: Set(billing.id.to_owned()),
            account_id: Set(billing.account_id.to_owned()),
            credits: Set(billing.credits.to_owned()),
            total_spent_usd: Set(billing.total_spent_usd.to_owned()),
            created_at: Set(billing.created_at.to_owned()),
            updated_at: Set(billing.updated_at.to_owned()),
        }
        .insert(db)
        .await?;
    
        form_data.billing_id = billing.id;

        Self::set_account_active_model(&form_data)
        .await
        .insert(db)
        .await
    }

    pub async fn update_account_by_id(
        db: &DbConn,
        id: ID,
        form_data: Model,
    ) -> Result<Model, DbErr> {
        let _: ActiveModel = account::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find account.".to_owned()))
            .map(Into::into)?;

        Self::set_account_active_model(&form_data)
            .await
            .update(db)
            .await
    }

    pub async fn update_account_billing_by_account_id(
        db: &DbConn,
        account_id: ID,
        form_data: billing::Model,
    ) -> Result<billing::Model, DbErr> {
        let _: billing::ActiveModel = billing::Entity::find()
            .filter(billing::Column::AccountId.eq(account_id))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find billing.".to_owned()))
            .map(Into::into)?;

        billing::ActiveModel {
            id: Set(form_data.id.to_owned()),
            account_id: Set(form_data.account_id.to_owned()),
            credits: Set(form_data.credits.to_owned()),
            total_spent_usd: Set(form_data.total_spent_usd.to_owned()),
            created_at: Set(form_data.created_at.to_owned()),
            updated_at: Set(form_data.updated_at.to_owned()),
        }
        .update(db)
        .await
    }
}

impl AccountQueryCore {
    /// # Get Account By ID
    ///
    /// Get the account by its ID.
    pub async fn get_account_by_id(db: &DbConn, id: ID) -> Result<Option<Model>, DbErr> {
        let model = account::Entity::find_by_id(id).one(db).await?;

        let acc = match model {
            Some(m) => m,
            None => return Ok(None),
        };

        Ok(Some(acc))
    }

    pub async fn get_account_by_id_with_billing(
        db: &DbConn,
        id: ID,
    ) -> Result<Option<(Model, Option<billing::Model>)>, DbErr> {
        let model = account::Entity::find()
            .find_also_related(billing::Entity)
            .filter(account::Column::Id.eq(id))
            .one(db)
            .await?;

        let (m,b) = match model {
            Some(m) => m,
            None => return Ok(None),
        };

        Ok(Some((m, b)))
    }

    pub async fn get_account_by_google_id(
        db: &DbConn,
        google_id: &str,
    ) -> Result<Option<Model>, DbErr> {
        let model = account::Entity::find()
            .filter(account::Column::GoogleId.eq(google_id))
            .one(db)
            .await?;

        match model {
            Some(m) => Ok(Some(m)),
            None => Ok(None),
        }
    }

    /// # Get Account By Email
    ///
    /// Get the account by its email address.
    pub async fn get_account_by_email(
        db: &DbConn,
        email: &str,
    ) -> Result<Option<Model>, DbErr> {
        let model = account::Entity::find()
            .filter(account::Column::Email.eq(email))
            .one(db)
            .await?;

        match model {
            Some(m) => Ok(Some(m)),
            None => Ok(None),
        }
    }
}
