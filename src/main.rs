use axum::extract::Query;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Json, Router};
use http::{header, Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{env, net::SocketAddr, os::unix::net::UnixStream, path::PathBuf};
use thiserror::Error;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, error, info};

const JSONRPC_VERSION: &str = "2.0";

#[derive(Clone)]
struct Config {
    addr: SocketAddr,
    sock_path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, AppError> {
        let addr = env::var("SERVER_ADDRESS")?.parse()?;
        let sock_path = env::var("SOCKET_PATH")?.into();
        Ok(Self { addr, sock_path })
    }
}

#[derive(Debug, Error)]
enum AppError {
    #[error("failed to load an environmental variable: {0}")]
    EnvVar(#[from] env::VarError),
    #[error("invalid address: {0}")]
    InvalidAddr(#[from] std::net::AddrParseError),
    #[error("could not bind to the address")]
    Bind,
    #[error("could not connect to the backend. is it running?: {0}")]
    SocketConnect(std::io::Error),
    #[error("socket error: {0}")]
    Socket(#[from] std::io::Error),
    #[error("invalid parameters")]
    Params,
    #[error("invalid response from backend: {0}")]
    BackendInvalidResponse(serde_json::Error),
    #[error("other error")]
    OtherInternal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        /*
        let pair = match self {
            AppError::OtherInternal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Socket(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::BackendInvalidResponse(${0:_}) => todo!(),
        };
        pair.into_response()
        */
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Post(serde_json::Value);

#[derive(Serialize, Deserialize)]
pub enum RpcObject {
    #[serde(rename = "result")]
    Result(RpcResponseResult),
    #[serde(rename = "error")]
    Error(RpcResponseError),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RpcResponseResult {
    Plain {
        meta: RpcResponsePlainMeta,
        body: serde_json::Value,
    },
    HomeTimeline {
        meta: RpcResponsePlainMeta,
        body: Vec<Post>,
    },
    Status {
        version: String,
    },
    AccountList {
        user_ids: Vec<String>,
    },
    AccountAdd {
        user_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponsePlainMeta {
    pub api_calls_remaining: usize,
    pub api_calls_reset: usize, // in epoch sec
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponseError {
    pub code: isize,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct RpcResponse {
    id: String,
    jsonrpc: String,
    #[serde(flatten)]
    object: RpcObject,
}
// TODO: handle error objects

async fn timeline(
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    info!("Received request for timeline: {params:?}");
    let (mut stream, id) = prepare_rpc()?;
    let user_id = params.get("user_id").ok_or(AppError::Params)?;

    let payload = json!({
        "jsonrpc": JSONRPC_VERSION,
        "id": id,
        "params": {"user_id": user_id, "api_params": {}},
        "method": "v0.home_timeline",
    })
    .to_string();
    debug!("Sending request: {}", payload);

    let mut resp = String::new();
    stream.write_all(payload.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    stream.read_to_string(&mut resp)?;
    debug!("Received response: {}", resp);

    let resp: RpcResponse =
        serde_json::from_str(&resp).map_err(AppError::BackendInvalidResponse)?;
    let res: RpcResponseResult = match resp.object {
        RpcObject::Result(result) => result,
        RpcObject::Error(err) => {
            error!("{:?}", err);
            Err(AppError::OtherInternal)?
        }
    };
    Ok((
        StatusCode::OK,
        [
            (header::CACHE_CONTROL, "no-cache"),
            (header::CONTENT_TYPE, "application/json"),
        ],
        Json(res),
    ))
}

async fn accounts() -> Result<impl IntoResponse, AppError> {
    info!("Received request for account list");
    let (mut stream, id) = prepare_rpc()?;

    let payload = json!({
        "jsonrpc": JSONRPC_VERSION,
        "id": id,
        "method": "v0.account.list",
    })
    .to_string();
    debug!("Sending request: {}", payload);

    let mut resp = String::new();
    stream.write_all(payload.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    stream.read_to_string(&mut resp)?;
    debug!("Received response: {}", resp);

    let resp: RpcResponse =
        serde_json::from_str(&resp).map_err(AppError::BackendInvalidResponse)?;
    let res: RpcResponseResult = match resp.object {
        RpcObject::Result(result) => result,
        RpcObject::Error(err) => Err(AppError::OtherInternal)?,
    };
    Ok((StatusCode::OK, Json(res)))
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

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::new()?;

    let app = Router::new()
        .route("/timeline", get(timeline))
        .route("/accounts", get(accounts))
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
        .await
        .map_err(|_| AppError::Bind)?;

    Ok(())
}
