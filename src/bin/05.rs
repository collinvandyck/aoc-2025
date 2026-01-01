use itertools::Itertools;
use std::{cell::RefCell, cmp, collections::HashMap, fmt::Debug, rc::Rc, usize};

static EX1: &str = include_str!("../../data/05/ex1");
static IN1: &str = include_str!("../../data/05/in1");

fn main() {}

fn run(s: &str, pt1: bool) -> usize {
    let state = parse(s);
    if pt1 { state.count_fresh() } else { state.all_fresh() }
}

#[derive(Debug, Clone)]
struct State {
    fresh: Vec<Range>,
    ingredients: Vec<usize>,
}

impl State {
    fn count_fresh(&self) -> usize {
        self.ingredients.iter().filter(|&&id| self.fresh.iter().any(|rng| rng.contains(id))).count()
    }

    fn all_fresh(&self) -> usize {
        let sorted = self.fresh.iter().sorted().copied().collect_vec();
        println!("{sorted:?}");
        let mut ranges = vec![];
        let mut last = Range::default();
        for rng in self.fresh.iter().sorted().copied() {
            println!("{ranges:?}");
            let ahead = rng.from > last.to;
            last = rng;
            if ahead {
                ranges.push(rng);
                continue;
            }
        }
        println!("{ranges:?}");
        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
struct Range {
    from: usize,
    to: usize,
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.from, self.to)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.from.cmp(&other.from).then(self.to.cmp(&other.to))
    }
}

impl Range {
    fn contains(&self, id: usize) -> bool {
        self.from <= id && self.to >= id
    }
    fn len(&self) -> usize {
        self.to - self.from
    }
}

fn parse(s: &str) -> State {
    let mut iter = s.trim().lines().map(|l| l.trim());
    let fresh = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (lhs, rhs) = l.split_once('-').expect("bad input");
            Range { from: lhs.parse().expect("bad int"), to: rhs.parse().expect("bad int") }
        })
        .collect();
    let ingredients = iter.map(|l| l.parse().expect("bad int")).collect();
    State { fresh, ingredients }
}

mod tests {
    use super::*;

    #[test]
    fn test_pt1_ex1() {
        assert_eq!(run(EX1, true), 3);
    }

    #[test]
    fn test_pt1_in1() {
        assert_eq!(run(IN1, true), 679);
    }

    #[test]
    fn test_pt2_ex1() {
        assert_eq!(run(EX1, false), 14);
    }
}
