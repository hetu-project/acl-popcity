use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{prelude::Users, users},
        Storage,
    },
};
use sea_orm::prelude::Expr;
use sea_orm::*;
impl Storage {
    pub async fn create_user(&self, active_user: users::ActiveModel) -> AppResult<users::Model> {
        tracing::info!("user model: {:?}", active_user);

        let user_uid: String = active_user
            .get(users::Column::Uid)
            .try_as_ref()
            .ok_or(AppError::CustomError(
                "cannot get uid from active user".into(),
            ))?
            .to_string();

        let user_invite_code: String = active_user
            .get(users::Column::InviteCode)
            .try_as_ref()
            .ok_or(AppError::CustomError(
                "cannot get invite_code from active user".into(),
            ))?
            .to_string();

        let user_email: String = active_user
            .get(users::Column::Email)
            .try_as_ref()
            .ok_or(AppError::CustomError(
                "cannot get email from active user".into(),
            ))?
            .to_string();

        //let user: users::Model = active_user.clone().try_into_model()?;
        match self
            .is_user_exists(&user_uid, &user_invite_code, &user_email)
            .await?
        {
            true => {
                return Err(AppError::UserExisted(format!(
                    "User: {} already exists",
                    user_email
                )))
            }
            false => (),
        }

        //let mut active_user = user.into_active_model();
        //active_user.id = NotSet;
        //active_user.created_at = Set(Some(chrono::Utc::now().into()));
        //active_user.updated_at = Set(Some(chrono::Utc::now().into()));

        let created_user = active_user.insert(self.conn.as_ref()).await?;

        Ok(created_user)
    }

    pub async fn is_user_exists_by_email(&self, user_email: &str) -> AppResult<bool> {
        let user = Users::find()
            .filter(users::Column::Email.eq(user_email))
            .one(self.conn.as_ref())
            .await?;

        Ok(user.is_some())
    }

    pub async fn get_user_by_email(&self, user_email: &str) -> AppResult<users::Model> {
        match Users::find()
            .filter(users::Column::Email.eq(user_email))
            .one(self.conn.as_ref())
            .await?
        {
            Some(user) => Ok(user),
            None => Err(AppError::UserUnExisted(format!(
                "User {} has not existed",
                user_email
            ))),
        }
    }

    pub async fn is_user_exists_by_code(&self, code: &str) -> AppResult<bool> {
        let existing = Users::find()
            .filter(users::Column::InviteCode.eq(code))
            .one(self.conn.as_ref())
            .await?;

        Ok(existing.is_some())
    }

    pub async fn is_user_exists(&self, uid: &str, code: &str, email: &str) -> AppResult<bool> {
        let existing = Users::find()
            .filter(
                Expr::col(users::Column::InviteCode)
                    .eq(code)
                    .or(Expr::col(users::Column::Email).eq(email))
                    .or(Expr::col(users::Column::Uid).eq(uid)),
            )
            .one(self.conn.as_ref())
            .await?;

        Ok(existing.is_some())
    }

    pub async fn count_invited_users_by_email(&self, email: &str) -> AppResult<u64> {
        let user = match Users::find()
            .filter(users::Column::Email.eq(email))
            .one(self.conn.as_ref())
            .await?
        {
            Some(u) => u,
            None => {
                return Err(AppError::UserUnExisted(format!(
                    "User: {} not exists",
                    email
                )))
            }
        };

        Ok(Users::find()
            .filter(users::Column::InvitedBy.eq(Some(user.invite_code)))
            .count(self.conn.as_ref())
            .await
            .unwrap_or(0))
    }

    pub async fn get_inviter_by_code(&self, code: &str) -> AppResult<users::Model> {
        match Users::find()
            .filter(users::Column::InviteCode.eq(code.to_string()))
            .one(self.conn.as_ref())
            .await?
        {
            Some(user) => Ok(user),
            None => Err(AppError::UserUnExisted(format!(
                "Inviter {} has not existed",
                code
            ))),
        }
    }

    pub async fn count_invited_users_by_code(&self, code: &str) -> AppResult<u64> {
        Ok(Users::find()
            .filter(users::Column::InvitedBy.eq(Some(code.to_string())))
            .count(self.conn.as_ref())
            .await?)
    }

    pub async fn get_invited_users_by_code(&self, code: &str) -> AppResult<Vec<users::Model>> {
        Ok(Users::find()
            .filter(users::Column::InvitedBy.eq(Some(code.to_string())))
            .all(self.conn.as_ref())
            .await?)
    }

    pub async fn count_total_users(&self) -> AppResult<u64> {
        Ok(Users::find().count(self.conn.as_ref()).await?)
    }
}
