use crate::util;
use warp::Filter;

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Extract {
    pub token: String,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Reply {
    pub token: String,
}

pub async fn handler(extract: Extract, db: util::AppDb) -> Result<Reply, warp::Rejection> {
    let db = db.lock().await;

    // トークンが指すユーザを取得
    let reply = sqlx::query_as!(
        Reply,
        "SELECT token FROM users WHERE token = $1",
        extract.token
    )
    .fetch_one(&*db)
    .await
    .map_err(util::error)?;

    Ok(reply)
}

pub fn filter(
    db: util::AppDb,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("get_user")
        .and(warp::post())
        .and(util::json_body())
        .and(util::with_db(db))
        .and_then(handler)
        .map(|reply| warp::reply::json(&reply))
}
