pub async fn hello2(param: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Hello 2, {}", param))
}

pub async fn health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("Ok")
}
