
use sqlx::{Error};
use sqlx::{pool::PoolConnection, Postgres};
use sqlx::types::Uuid;

use crate::errors::app_error::AppError;
use crate::models::promotion_model::Promotion;

pub async fn get_promotion_by_id(mut conn: PoolConnection<Postgres>, promotion_id: Uuid) -> Option<Promotion> {
    dbg!(&promotion_id);
    match sqlx::query_as::<_, Promotion>("SELECT * FROM promotion WHERE id = $1")
        .bind(promotion_id)
        //fetch_one would return error if not found
        .fetch_one(&mut conn)
        .await {
            Ok(promotion) => {
                dbg!(&promotion);
                return Some(promotion)
            },
            Err(_) => None,
        }
}

pub async fn get_all_promotions(mut conn: PoolConnection<Postgres>) -> Result<Vec<Promotion>, AppError> {
    match sqlx::query_as::<_, Promotion>("SELECT * FROM promotion")
    .fetch_all( &mut conn)
    .await {
        Ok(promotions) => Ok(promotions),
        _ => Err(AppError::DefaultError),
    }
    // let promotions = sqlx::query_as::<_, Promotion>("SELECT * FROM promotion")
    //     .fetch_all( &mut conn)
    //     .await?;
    // Ok(promotions)
}

pub async fn update_promotion(mut conn: PoolConnection<Postgres>, promotion_id: Uuid, name: String) -> Result<Promotion, AppError> {
    match sqlx::query_as::<_, Promotion>("UPDATE promotion SET name = $1 WHERE id = $2 RETURNING name")
    .bind(name)
    .bind(promotion_id)
    .fetch_one(&mut conn)
    .await {
        Ok(result) => Ok(result),
        Err(err) => Err(AppError::DefaultError),
    }
}

pub async fn add_promotion(mut conn: PoolConnection<Postgres>, name: String) -> Result <Promotion, Error> {
    let result = sqlx::query_as::<_,Promotion>("INSERT INTO promotion (id, name) VALUES($1, $2) RETURNING id, name")
    .bind(Uuid::new_v4())
    .bind(name)
    .fetch_one(&mut conn)
    .await.unwrap();

    Ok(result)
}
