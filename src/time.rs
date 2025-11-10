use chrono::{DateTime, FixedOffset, TimeZone, Utc};

/// Get current Unix timestamp in JST (milliseconds)
pub fn get_jst_timestamp() -> i64 {
    let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap(); // JST is UTC+9
    let now_utc = Utc::now();
    let now_jst: DateTime<FixedOffset> = now_utc.with_timezone(&jst_offset);
    now_jst.timestamp_millis()
}

/// Convert Unix timestamp (milliseconds) to JST RFC 3339 format
pub fn timestamp_to_jst_rfc3339(timestamp_millis: i64) -> String {
    let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap(); // JST is UTC+9
    let seconds = timestamp_millis / 1000;
    let nanos = ((timestamp_millis % 1000) * 1_000_000) as u32;
    let dt = jst_offset.timestamp_opt(seconds, nanos).unwrap();
    dt.to_rfc3339()
}
