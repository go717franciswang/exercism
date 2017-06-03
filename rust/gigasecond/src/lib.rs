extern crate chrono;
use chrono::*;

pub fn after(d: DateTime<UTC>) -> DateTime<UTC> {
    d + Duration::seconds(1_000_000_000)
}
