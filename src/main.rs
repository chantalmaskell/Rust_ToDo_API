use serde::Serialize;
use warp::{reply::json, Filter, Rejection, Reply};

type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)] // rust attribute, implements from Serialise trait from serde crate (allows struct to be converted to json etc)
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub async fn health_check_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "james stinks";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

#[tokio::main]
async fn main() {
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "api=info");
    // }
    pretty_env_logger::init(); // logs the http info in terminal

    // immutable let or mut for mutable
    let health_checker = warp::path!("api" / "healthchecker")
    .and(warp::get())
    .and_then(health_check_handler); // set path to api/healthchecker

    let routes = health_checker
    .with(warp::log("api"));

    println!("server started");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await; //bind address 
}

// random notes:
// cargo-watch used for hot reload (starts warp http server)
// run: cargo watch -q (quiet so output is hidden) -c (clears terminal before running) -w src/ (watches src dir for any file changes for hot reload) -x run (executes cargo run when change detected)