use anyhow::Result;
use axum::{routing::get, Json, Router};
use http::Method;
use serde::Serialize;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

struct Config {
    addr: SocketAddr,
}

impl Config {
    pub fn new() -> Result<Self> {
        let addr = env::var("SERVER_ADDRESS")?.parse()?;
        Ok(Self { addr })
    }
}

#[derive(Serialize)]
struct Post {
    id: String,
    text: String,
}

#[derive(Serialize)]
struct HomeTimeline {
    data: Vec<Post>,
}

async fn timeline() -> Json<HomeTimeline> {
    let timeline = HomeTimeline {
        data: vec![
            Post {
                id: "345".to_string(),
                text: "hoge".to_string(),
            },
            Post {
                id: "456".to_string(),
                text: "piyo".to_string(),
            },
        ],
    };
    Json(timeline)
}

async fn get_user() {
    todo!()
}

async fn get_post() {
    todo!()
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::new()?;

    let app = Router::new()
        .route("/timeline", get(timeline))
        .route("/user/", get(get_user))
        .route("/post/", get(get_post))
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST])
                    .allow_origin(Any),
            ),
        );

    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
