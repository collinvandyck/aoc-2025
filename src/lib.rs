use anyhow::{Context, Result};

pub fn load_data(day: &str) -> String {
    std::fs::read(&format!("data/{day}"))
        .context("could not read data")
        .and_then(|bs| String::from_utf8(bs).context("convert bytes to utf8"))
        .map(|s| s.trim().to_string())
        .unwrap()
}
