use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use tracing::debug;

use crate::{error::AppError, response::RpcResponse};

pub struct RpcRequest {
    payload: String,
}

impl RpcRequest {
    pub fn new(payload: String) -> Self {
        Self { payload }
    }

    pub fn send(&self, stream: &mut UnixStream) -> Result<RpcResponse, AppError> {
        let mut resp = String::new();
        stream.write_all(self.payload.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;
        stream.read_to_string(&mut resp)?;
        debug!("Received response: {}", resp);

        serde_json::from_str(&resp)
            .map_err(|x| AppError::BackendInvalidResponse(anyhow::anyhow!(x)))
    }
}
