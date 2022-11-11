use std::{env, net::SocketAddr, path::PathBuf};

use crate::error::AppError;

#[derive(Clone)]
pub struct Config {
    pub addr: SocketAddr,
    pub sock_path: PathBuf,
    pub database_url: String,
    pub session_secret: String,
}

impl Config {
    pub fn new() -> Result<Self, AppError> {
        let addr = env::var("SERVER_ADDRESS")?.parse()?;
        let sock_path = env::var("SOCKET_PATH")?.into();
        let database_url = env::var("DATABASE_URL")?.into();
        let session_secret = env::var("SESSION_SECRET")?.into();
        Ok(Self {
            addr,
            sock_path,
            database_url,
            session_secret,
        })
    }
}
