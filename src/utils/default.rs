use chrono::{NaiveDateTime, Utc};
use md5::{Digest, Md5};

pub fn default_created_at() -> Option<NaiveDateTime> {
    Some(Utc::now().naive_utc())
}

pub fn encrypt_password(password: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let hex_string = format!("{:x}", result);
    hex_string.to_string()
}
