use crate::{
    common::error::AppResult,
    database::{
        entities::{points, prelude::Points},
        Storage,
    },
};
use sea_orm::*;

#[derive(FromQueryResult, Debug)]
struct AggregationResult {
    total_points: Option<i64>, // Match the alias name
}

impl Storage {
    pub async fn award_points(
        &self,
        user_uid: String,
        point_type: &str,
        points: i32,
        description: &str,
    ) -> AppResult<points::Model> {
        let point_entry = points::ActiveModel {
            user_uid: Set(user_uid),
            point_type: Set(point_type.to_owned()),
            points: Set(points),
            description: Set(Some(description.to_owned())),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        let point = point_entry.insert(self.conn.as_ref()).await?;

        Ok(point)
    }

    pub async fn get_user_points(&self, user_uid: &str) -> AppResult<i64> {
        match Points::find()
            .filter(points::Column::UserUid.eq(user_uid))
            .select_only()
            .column_as(points::Column::Points.sum(), "total_points")
            .into_model::<AggregationResult>()
            .one(self.conn.as_ref())
            .await?
        {
            Some(aggr_result) => Ok(aggr_result.total_points.unwrap_or(0)),
            None => Ok(0),
        }
    }
    pub async fn cleanup_expired_point(&self) -> AppResult<()> {
        use sea_orm::EntityTrait;

        points::Entity::delete_many()
            .filter(points::Column::ExpiresAt.lt(chrono::Utc::now()))
            .exec(self.conn.as_ref())
            .await?;

        Ok(())
    }
}
