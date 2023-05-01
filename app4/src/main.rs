use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Greeting {
    message: String,
}

async fn handle_get_greeting(
    greetings: Arc<Mutex<Vec<Greeting>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let greetings = greetings.lock().unwrap();
    Ok(warp::reply::json(&*greetings))
}

async fn handle_post_greeting(
    new_greeting: Greeting,
    greetings: Arc<Mutex<Vec<Greeting>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut greetings = greetings.lock().unwrap();
    greetings.push(new_greeting);
    Ok(warp::reply::json(&*greetings))
}

fn with_greetings(
    greetings: Arc<Mutex<Vec<Greeting>>>,
) -> impl Filter<Extract = (Arc<Mutex<Vec<Greeting>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || greetings.clone())
}

#[tokio::main]
async fn main() {
    let greetings = Arc::new(Mutex::new(Vec::new()));

    let get_greeting = warp::path!("greeting")
        .and(warp::get())
        .and(with_greetings(greetings.clone()))
        .and_then(handle_get_greeting);

    let post_greeting = warp::path!("greeting")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_greetings(greetings.clone()))
        .and_then(handle_post_greeting);

    let routes = get_greeting.or(post_greeting);

    println!("Starting server at 127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

}
