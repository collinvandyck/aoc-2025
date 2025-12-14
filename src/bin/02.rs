use anyhow::Result;
use std::{collections::VecDeque, iter, num::ParseIntError, ops, usize};

static EX1: &str = include_str!("../../data/02/ex1");
static IN1: &str = include_str!("../../data/02/in1");

fn main() {
    println!("ex1: {}", run(EX1, true));
    println!("pt1: {}", run(IN1, true));
    //println!("ex2: {}", run(EX1, false));
    //println!("pt2: {}", run(IN1, false));
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Range {
    from: usize,
    to: usize,
}

fn run(data: &str, pt1: bool) -> usize {
    let mut digits = vec![];
    parse(data)
        .into_iter()
        .flat_map(|r| (r.from..=r.to))
        .filter(|v| {
            set_digits(*v, &mut digits);
            repeats_twice(v, &digits)
        })
        .sum()
}

fn repeats_twice(v: &usize, digits: &[u8]) -> bool {
    if digits.len() % 2 != 0 {
        return false;
    }
    let mid = digits.len() / 2;
    &digits[..mid] == &digits[mid..]
}

fn set_digits(mut v: usize, buf: &mut Vec<u8>) {
    buf.clear();
    while v != 0 {
        buf.push((v % 10) as u8);
        v /= 10;
    }
}

fn parse(data: &str) -> Vec<Range> {
    data.trim()
        .split(',')
        .map(|p| {
            let (from, to) = p.trim().split_once('-').unwrap();
            let from = from.parse().unwrap();
            let to = to.parse().unwrap();
            assert!(from <= to);
            Range { from, to }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_ex() {
        assert_eq!(run(EX1, true), 1227775554);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(run(IN1, true), 12586854255);
    }

    #[test]
    fn test_pt2_ex() {
        //assert_eq!(run(EX1, false), 6);
    }

    #[test]
    fn test_pt2() {
        //assert_eq!(run(IN1, false), 5961);
    }
}
