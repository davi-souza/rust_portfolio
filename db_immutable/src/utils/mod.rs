use chrono::prelude::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;

pub fn datetime_from_str(raw: &str) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S%z").expect("datetime parse"),
        Utc,
    )
}

pub fn stringify_datetime(date: DateTime<Utc>) -> String {
    format!("{}", date.format("%Y-%m-%dT%H:%M:%S%z"))
}

pub fn uuid_from_str(raw: &str) -> Uuid {
    Uuid::parse_str(raw).expect("uuid parse")
}

pub fn stringify_uuid(uuid: Uuid) -> String {
    format!("{}", uuid)
}
