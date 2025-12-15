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
    parse(data)
        .into_iter()
        .flat_map(|r| (r.from..=r.to))
        .filter(|v| (pt1 && repeats_twice(v)) || (!pt1 && repeats_any(v)))
        .sum()
}

fn repeats_any(&v: &usize) -> bool {
    if v != 1012 {
        return false;
    }

    let digits = num_digits(v); // eg: 8
    println!("repeats any: {v} digits: {digits}");
    for run_len in 1..=(digits / 2) {
        if digits % run_len != 0 {
            continue;
        }
        let pow = 10_usize.pow(digits as u32);
        let pow_slice = 10_usize.pow(run_len as u32);
        println!("> run len {run_len} pow_slice: {pow_slice}");
        for batch in 0..(digits / run_len) {
            // we need to compute the value of this batch.
            // start from lhs (most sig bits)
            let div_amt = 10_usize.pow((batch * run_len) as u32);
            let div = v / div_amt;
            let other = 10_usize.pow((batch) as u32);
            println!(
                "> batch {batch} div_amt: {div_amt} div: {div} other: {other}"
            );
        }
    }
    false
}

fn repeats_twice(&v: &usize) -> bool {
    let digits = num_digits(v);
    if digits % 2 != 0 {
        return false;
    }
    let mid_pow10 = 10_usize.pow(digits as u32 / 2);
    let (lhs, rhs) = (v / mid_pow10, v % mid_pow10);
    lhs == rhs
}

fn num_digits(mut v: usize) -> usize {
    let mut digits = 0;
    while v != 0 {
        digits += 1;
        v /= 10;
    }
    digits
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
        assert_eq!(run(EX1, false), 4174379265);
    }

    #[test]
    fn test_pt2() {
        //assert_eq!(run(IN1, false), 5961);
    }
}
