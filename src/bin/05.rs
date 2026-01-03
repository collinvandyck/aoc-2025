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
        println!("sorted: {sorted:?}\n");

        let mut ranges: Vec<Range> = vec![];
        for rng in sorted.iter().sorted().copied() {
            ranges.sort();
            if ranges.is_empty() {
                ranges.push(rng.clone());
                continue;
            }
            let last = &ranges[ranges.len() - 1];
            if rng.to == last.to {
                continue;
            }
            if rng.from > last.to {
                // push after
                ranges.push(rng.clone());
                continue;
            }
            if rng.from == last.to {
                if rng.to - rng.from > 1 {
                    // left bound
                    ranges.push(Range { from: rng.from + 1, to: rng.to });
                    continue;
                } else {
                    // skip empty range
                    continue;
                }
            }
            if rng.to > last.to {
                let nf = last.to + 1;
                let nt = rng.to;
                if nt.checked_sub(nf).map(|v| v > 0).unwrap_or_default() {
                    // push overlap
                    ranges.push(Range { from: nf, to: nt });
                    continue;
                } else {
                    // overlap sub not valid
                    continue;
                }
            }
        }
        ranges.iter().map(|r| r.to - r.from + 1).sum()
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

    #[test]
    fn test_pt2_in1() {
        assert_eq!(run(IN1, false), 358155203664116);
    }
}
