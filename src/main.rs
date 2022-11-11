use anyhow::{anyhow, Context};
use async_sqlx_session::PostgresSessionStore;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Extension;
use axum::{
    routing::{get, get_service},
    Json, Router,
};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use axum_sessions::{SameSite, SessionLayer};
use config::Config;
use http::{header, Method, StatusCode};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::os::unix::net::UnixStream;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::{debug, error, info};

use crate::error::AppError;
use crate::request::RpcRequest;
use crate::response::{RpcObject, RpcResponseResult};
use crate::session::AccountList;

mod config;
mod error;
mod post;
mod request;
mod response;
mod session;

const JSONRPC_VERSION: &str = "2.0";

async fn add_account(mut session: WritableSession) -> Result<impl IntoResponse, AppError> {
    info!("adding account to: {}", session.id());
    let (mut stream, id) = prepare_rpc()?;

    let mut accs = session.get::<AccountList>("accounts").unwrap_or_default();

    let payload = json!({
        "jsonrpc": JSONRPC_VERSION,
        "id": id,
        "params": {
            "session_key": accs.owner_key(),
        },
        "method": "v0.account.add"
    })
    .to_string();

    let request = RpcRequest::new(payload);
    let resp = request.send(&mut stream)?;
    let res: RpcResponseResult = match resp.object {
        RpcObject::Result(result) => result,
        RpcObject::Error(err) => {
            error!("{:?}", err);
            Err(AppError::OtherInternal)?
        }
    };

    let (user_id, key) = match res {
        RpcResponseResult::AccountAdd {
            user_id,
            session_key,
        } => (user_id, session_key),
        _ => {
            return Err(AppError::BackendInvalidResponse(anyhow!(
                "invalid response type"
            )))
        }
    };

    // update accounts
    // (or call v0.account.list?)
    accs.rpc_session_keys.insert(user_id.clone(), key);
    if accs.owner_id.is_none() {
        accs.owner_id = Some(user_id.clone());
    }
    session
        .insert("accounts", accs)
        .map_err(|err| AppError::BackendInvalidResponse(anyhow!(err)))?;

    Ok(ok_nocache_str(json!({ "user_id": user_id }).to_string()))
}

// TODO: signouts (invalidate sessions on minichotan and delete tokens on binchotan-backend)

async fn timeline(
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    info!("Received request for timeline: {params:?}");
    let (mut stream, id) = prepare_rpc()?;
    let user_id = params.get("user_id").ok_or(AppError::Params)?;

    let payload = json!({
        "jsonrpc": JSONRPC_VERSION,
        "id": id,
        "params": {
            "user_id": user_id,
            "api_params": {
                "expansions": "author_id,referenced_tweets.id,referenced_tweets.id.author_id",
            },
        },
        "method": "v0.home_timeline",
    })
    .to_string();
    debug!("Sending request: {}", payload);

    let request = RpcRequest::new(payload);
    let resp = request.send(&mut stream)?;
    let res: RpcResponseResult = match resp.object {
        RpcObject::Result(result) => result,
        RpcObject::Error(err) => Err(AppError::RpcResponse(err))?,
    };
    Ok(ok_nocache_json(res))
}

async fn accounts(session: ReadableSession) -> Result<impl IntoResponse, AppError> {
    info!("listing account for: session id = {}", session.id());
    let (mut stream, id) = prepare_rpc()?;

    let accs = session.get::<AccountList>("accounts").unwrap_or_default();

    if !accs.signed_in() {
        // return empty list
        return Ok(ok_nocache_str(
            json!({
                "owner": None::<String>,
                "session_keys": {}
            })
            .to_string(),
        ));
    }

    let payload = json!({
        "jsonrpc": JSONRPC_VERSION,
        "id": id,
        "params": {
            "session_key": accs.owner_key(),
        },
        "method": "v0.account.list",
    })
    .to_string();
    debug!("Sending request: {}", payload);

    let request = RpcRequest::new(payload);
    let resp = request.send(&mut stream)?;

    let res: RpcResponseResult = match resp.object {
        RpcObject::Result(result) => result,
        RpcObject::Error(err) => Err(AppError::RpcResponse(err))?,
    };
    let json = match res {
        RpcResponseResult::AccountList {
            owner,
            session_keys,
        } => {
            let user_ids: Vec<String> = session_keys.keys().cloned().collect();
            json!({
                "owner_id": owner,
                "user_ids": user_ids,
            })
            .to_string()
        }
        _ => {
            return Err(AppError::BackendInvalidResponse(anyhow!(
                "invalid response type"
            )))
        }
    };
    Ok(ok_nocache_str(json))
}

async fn userinfo(
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    info!("Received request for timeline: {params:?}");
    let (mut stream, id) = prepare_rpc()?;
    let user_id = params.get("user_id").ok_or(AppError::Params)?;

    let payload = json!({
        "jsonrpc": JSONRPC_VERSION,
        "id": id,
        "params": {"user_id": user_id, "api_params": {}, "endpoint": "users/:id", "http_method": "GET"},
        "method": "v0.plain",
    })
    .to_string();
    debug!("Sending request: {}", payload);

    let request = RpcRequest::new(payload);
    let resp = request.send(&mut stream)?;

    let res: RpcResponseResult = match resp.object {
        RpcObject::Result(result) => result,
        RpcObject::Error(err) => {
            error!("{:?}", err);
            Err(AppError::OtherInternal)?
        }
    };
    Ok(ok_nocache_json(res))
}

async fn get_user() {
    todo!()
}

async fn get_post() {
    todo!()
}

fn prepare_rpc() -> Result<(UnixStream, String), AppError> {
    let config = Config::new()?;
    let stream = UnixStream::connect(config.sock_path).map_err(AppError::SocketConnect)?;
    let id = uuid::Uuid::new_v4().to_string();

    Ok((stream, id))
}

fn ok_nocache_json(resp: RpcResponseResult) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CACHE_CONTROL, "no-cache"),
            (header::CONTENT_TYPE, "application/json"),
        ],
        Json(resp),
    )
}

fn ok_nocache_str(resp: String) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CACHE_CONTROL, "no-cache"),
            (header::CONTENT_TYPE, "application/json"),
        ],
        resp,
    )
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::new()?;
    let frontend_service = get_service(
        ServeDir::new("static").append_index_html_on_directories(true),
    )
    .handle_error(|err: std::io::Error| async move {
        (
            StatusCode::NOT_FOUND,
            format!("not found in frontend dir: {}", err),
        )
    });

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .context("could not connect to database")
        .map_err(AppError::Database)?;
    // FIXME: from_client(pool) emits type-mismatch error
    let store = PostgresSessionStore::new(&config.database_url)
        .await
        .context("could not connect to database")
        .map_err(AppError::Database)?;
    store
        .migrate()
        .await
        .context("could not initialize the session table")
        .map_err(AppError::Database)?;

    let secret = config.session_secret.as_bytes();
    let session_layer = SessionLayer::new(store, secret).with_same_site_policy(SameSite::Lax);

    let app = Router::new()
        .fallback(frontend_service)
        .route("/api/accounts/add", get(add_account))
        .route("/api/timeline", get(timeline))
        .route("/api/accounts", get(accounts))
        .route("/api/userinfo", get(userinfo))
        .route("/api/user/", get(get_user))
        .route("/api/post/", get(get_post))
        .layer(Extension(pool))
        .layer(session_layer)
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST])
                    .allow_origin(Any),
            ),
        );

    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await
        .map_err(|_| AppError::Bind)?;

    Ok(())
}
