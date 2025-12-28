use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, rc::Rc, usize};

static EX1: &str = include_str!("../../data/05/ex1");
static IN1: &str = include_str!("../../data/05/in1");

fn main() {}

fn run(s: &str, pt1: bool) -> usize {
    let state = parse(s);
    state.count_fresh()
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
}

#[derive(Debug, Clone, Copy)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn contains(&self, id: usize) -> bool {
        self.from <= id && self.to >= id
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
}
