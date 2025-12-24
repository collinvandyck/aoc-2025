use itertools::Itertools;

static EX1: &str = include_str!("../../data/02/ex1");
static IN1: &str = include_str!("../../data/02/in1");

fn main() {
    println!("ex1: {}", run(EX1, true));
    println!("pt1: {}", run(IN1, true));
    println!("ex2: {}", run(EX1, false));
    println!("pt2: {}", run(IN1, false));
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Range {
    from: usize,
    to: usize,
}

fn run(data: &str, pt1: bool) -> usize {
    parse(data)
        .into_iter()
        .flat_map(|r| r.from..=r.to)
        .filter(|v| (pt1 && repeats_twice(v)) || (!pt1 && repeats_any(*v)))
        .sum()
}

fn repeats_any(v: usize) -> bool {
    if v < 10 {
        return false;
    }
    let num_digits = num_digits(v);
    (1..=(num_digits / 2)).filter(|x| num_digits % x == 0).any(|run_len| {
        (0..(num_digits / run_len))
            .map(|part| (v / tenpow(part * run_len)) % tenpow(run_len))
            .all_equal()
    })
}

fn tenpow(amt: usize) -> usize {
    10_usize.pow(amt as u32)
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
    if v == 0 {
        return 1;
    }
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
    fn test_repeats_any() {
        assert_eq!(repeats_any(1), false);
        assert_eq!(repeats_any(10), false);
        assert_eq!(repeats_any(101), false);
        assert_eq!(repeats_any(1010), true);

        assert_eq!(repeats_any(11), true);
        assert_eq!(repeats_any(111), true);
        assert_eq!(repeats_any(1111), true);
    }

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(1234), 4);
    }

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
        assert_eq!(run(IN1, false), 17298174201);
    }
}
