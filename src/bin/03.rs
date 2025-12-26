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

#[derive(Debug, Clone)]
struct PV {
    pos: usize,
    val: usize,
    rest: Vec<SPV>,
}

#[derive(Clone)]
struct SPV(Rc<RefCell<PV>>);

impl std::fmt::Debug for SPV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = self.0.borrow();
        let rest = this.rest.iter().map(|x| x.val()).collect_vec();
        write!(f, "(v:{} r:{:?})", this.val, rest)
    }
}

impl SPV {
    fn new(pos: usize, val: usize) -> Self {
        Self(Rc::new(RefCell::new(PV::new(pos, val))))
    }
    fn pos(&self) -> usize {
        self.0.borrow().pos
    }
    fn val(&self) -> usize {
        self.0.borrow().val
    }
    fn rest_len(&self) -> usize {
        self.0.borrow().rest.len()
    }
    fn add(&self, other: &SPV) {
        if other.pos() > self.pos() {
            self.0.borrow_mut().rest.push(other.clone());
        }
    }
}

impl PV {
    fn new(pos: usize, val: usize) -> Self {
        Self { pos, val, rest: vec![] }
    }
}

fn largest_joltage(bank: &Bank, count: usize) -> usize {
    // println!("batts: {:?}", bank.batteries);
    // reverse sorted PV { pos, val }
    let s: Vec<SPV> = bank
        .batteries
        .clone()
        .into_iter()
        .enumerate()
        .map(|(pos, val)| SPV::new(pos, val))
        .sorted_by(|v1, v2| v1.val().cmp(&v2.val()).reverse())
        .collect_vec();

    // populate the rest fields in the PVs
    for i in 0..s.len() {
        for j in 0..s.len() {
            s[i].add(&s[j]);
        }
    }

    // filter the pvs by those that have the right count
    s.iter()
        .filter(|v| v.rest_len() >= count - 1)
        .cloned()
        .map(|found| {
            let mut pvs = vec![found.clone()];
            pvs.extend(found.0.borrow().rest.clone());
            pvs.iter().take(count).fold(0_usize, |acc, pv| acc * 10 + pv.val())
        })
        .max()
        .unwrap_or_default()
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
