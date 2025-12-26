use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, rc::Rc, usize};

static EX1: &str = include_str!("../../data/03/ex1");
static IN1: &str = include_str!("../../data/03/in1");

fn main() {
    println!("ex1: {}", run(EX1, true));
    println!("in1: {}", run(IN1, true));
}

fn run(s: &str, pt1: bool) -> usize {
    parse(s).iter().map(|b| largest_joltage(b, 2)).sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bank {
    batteries: Vec<usize>,
}

fn largest_joltage(bank: &Bank, count: usize) -> usize {
    let b = &bank.batteries;
    let len = b.len();
    let mut res = 0;
    let mut start = 0;
    for remaining in (1..=count).rev() {
        // end needs to be calculated wrt start
        let end = len - remaining + 1;
        let next = &b[start..end];
        let (max_pos, max) = next.iter().enumerate().max_by(|v1, v2| v1.1.cmp(&v2.1)).unwrap();
        start = max_pos + 1;
        res = res * 10 + max;
    }
    res
}

fn parse(s: &str) -> Vec<Bank> {
    s.trim()
        .lines()
        .map(|line| {
            let batteries =
                line.trim().chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect();
            Bank { batteries }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_ex1() {
        assert_eq!(run(EX1, true), 357);
    }

    #[test]
    fn test_pt1_in1() {
        assert_eq!(run(IN1, true), 16993);
    }
}
