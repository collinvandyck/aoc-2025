use anyhow::Result;
use std::{isize, iter, num::ParseIntError, ops};

static EX1: &str = include_str!("../../data/01/ex1");
static IN1: &str = include_str!("../../data/01/in1");
const BND: isize = 100;

fn main() {
    println!("ex1: {}", run(EX1, true));
    println!("pt1: {}", run(IN1, true));
    println!("ex2: {}", run(EX1, false));
    println!("pt2: {}", run(IN1, false));
}

fn run(data: &str, pt1: bool) -> isize {
    let mut res = 0;
    let mut cur: isize = 50;
    for line in data.trim().lines() {
        let dlt = parse_dlt(line);
        let nxt = (((cur + dlt) % BND) + BND) % BND;
        if pt1 {
            if nxt == 0 {
                res += 1;
            }
        } else {
            let min = cur.min(cur + dlt);
            let max = cur.max(cur + dlt);
            if min % 100 == 0 || max % 100 == 0 {
                res += (max - min) / 100;
                if nxt % 100 == 0 {
                    res += 1;
                }
            } else {
                res += max.div_euclid(100) - min.div_euclid(100);
            }
        }
        cur = nxt;
    }
    res
}

fn parse_dlt(line: &str) -> isize {
    let res: isize = (&line[1..]).parse().unwrap();
    if &line[0..1] == "L" {
        return -res;
    }
    res
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
