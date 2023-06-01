use crate::filters::util;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Extract {
    pub id: i64,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Reply {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub weight_val: f64,
    pub count_val: i32,
}

pub async fn handler(extract: Extract, db: util::Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;

    let reply = sqlx::query_as!(
        Reply,
        "SELECT id, name, description, weight_val, count_val FROM train WHERE id = $1",
        extract.id,
    )
    .fetch_one(&*db)
    .await
    .map_err(|_| util::ErrorMessage::new("failed to get a training"))?;

    Ok(warp::reply::json(&reply))
}

pub fn filter(
    db: util::Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_training")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
}
