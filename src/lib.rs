use anyhow::{Context, Result};
use std::thread::panicking;

pub fn load_data(day: &str) -> String {
    std::fs::read(&format!("data/{day}"))
        .context("could not read data")
        .and_then(|bs| String::from_utf8(bs).context("convert bytes to utf8"))
        .map(|s| s.trim().to_string())
        .unwrap()
}

pub fn num_digits<T: TryInto<isize>>(v: T) -> usize {
    let mut v: isize = v.try_into().ok().expect("invalid value");
    if v == 0 {
        return 1;
    }
    let mut res = 0;
    while v > 0 {
        res += 1;
        v /= 10;
    }
    res
}
