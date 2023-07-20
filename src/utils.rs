use crate::defs::FIELD_LEN_LIMIT;

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
