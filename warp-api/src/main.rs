use warp::Filter;
mod handlers;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let hello2 = warp::get()
        .and(warp::path("hello2"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(handlers::hello2);

    let health = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(handlers::health);

    let routes = warp::get()
        .and(hello)
        .or(hello2)
        .or(health)
        .with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
