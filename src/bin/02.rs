static EX1: &str = include_str!("../../data/02/ex1");
static IN1: &str = include_str!("../../data/02/in1");

fn main() {
    println!("ex1: {}", run(EX1, true));
    println!("pt1: {}", run(IN1, true));
    println!("ex2: {}", run(EX1, false));
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
        println!("> v: {v} run len {run_len} pow_slice: {pow_slice}");
        for idx in (0..(digits / run_len)) {
            let div_amt =
                10_usize.pow(((digits / run_len - idx - 1) * run_len) as u32);
            let mod_amt = pow_slice.pow((idx) as u32);
            let div = v / div_amt;
            println!(
                "> idx: {idx} run_len: {run_len} div_amt: {div_amt} div: {div} foo:{mod_amt}"
            );
        }
    }
    false
}

fn slices_of(v: usize, run_len: usize) -> Vec<usize> {
    vec![]
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

fn run_lens(num_digits: usize) -> Vec<usize> {
    (1..=(num_digits / 2)).filter(|x| num_digits % x == 0).collect()
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
    fn test_slices_of() {
        assert_eq!(slices_of(10, 1), vec![1, 0], "(10, 1)");
        assert_eq!(slices_of(11, 1), vec![1, 1], "(11, 1");
        assert_eq!(slices_of(1024, 2), vec![10, 24], "(1024, 2)");
        assert_eq!(slices_of(1004, 2), vec![10, 4], "(1004, 2)");
    }

    #[test]
    fn test_run_lens() {
        assert_eq!(run_lens(0), vec![]);
        assert_eq!(run_lens(1), vec![]);
        assert_eq!(run_lens(2), vec![1]);
        assert_eq!(run_lens(3), vec![1]);
        assert_eq!(run_lens(4), vec![1, 2]);
        assert_eq!(run_lens(5), vec![1]);
        assert_eq!(run_lens(6), vec![1, 2, 3]);
        assert_eq!(run_lens(7), vec![1]);
        assert_eq!(run_lens(8), vec![1, 2, 4]);
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
        //assert_eq!(run(IN1, false), 5961);
    }
}
