use anyhow::Result;
use std::{isize, iter, num::ParseIntError, ops};

static EX1: &str = include_str!("../../data/02/ex1");
static IN1: &str = include_str!("../../data/02/in1");
const BND: isize = 100;

fn main() {
    println!("ex1: {}", run(EX1, true));
    println!("pt1: {}", run(IN1, true));
    println!("ex2: {}", run(EX1, false));
    println!("pt2: {}", run(IN1, false));
}

fn run(data: &str, pt1: bool) -> isize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_ex() {
        assert_eq!(run(EX1, true), 3);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(run(IN1, true), 980);
    }

    #[test]
    fn test_pt2_ex() {
        assert_eq!(run(EX1, false), 6);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(run(IN1, false), 5961);
    }
}
