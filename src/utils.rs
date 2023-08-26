use crate::defs::FIELD_LEN_LIMIT;

use axum::http::header::HeaderMap;
use std::net::SocketAddr;

#[inline]
pub(crate) fn is_valid_field(s: &str) -> bool {
    !s.is_empty() && s.len() < FIELD_LEN_LIMIT
}

#[inline]
pub(crate) fn is_valid_key_field(s: &str) -> bool {
    !s.is_empty() && s.len() < FIELD_LEN_LIMIT && !s.contains(':')
}

#[inline]
pub(crate) fn now_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[inline]
pub(crate) fn get_remote_addr(
    headers: &HeaderMap,
    addr: &SocketAddr
) -> String {
    let addr: String = if headers.contains_key("x-real-ip") {
        headers
            .get("x-real-ip")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_string()
    } else {
        addr.to_string()
    };
    return addr;
}
