use aoc::num_digits;
use itertools::{Itertools, max};

static EX1: &str = include_str!("../../data/06/ex1");
static IN1: &str = include_str!("../../data/06/in1");

type V = isize;

fn main() {}

fn run(s: &str, pt1: bool) -> V {
    parse(s, pt1)
        .operations
        .into_iter()
        .map(|op| {
            op.values
                .into_iter()
                .reduce(|acc, v| match op.operator {
                    '+' => acc + v,
                    '*' => acc * v,
                    _ => unreachable!(),
                })
                .expect("no data")
        })
        .sum()
}

#[derive(Debug)]
struct Operation {
    values: Vec<V>,
    operator: char,
}

#[derive(Debug)]
struct Sheet {
    operations: Vec<Operation>,
}

fn parse(s: &str, pt1: bool) -> Sheet {
    println!("{s}");
    let mut lines =
        s.trim().lines().map(|l| l.trim().split_whitespace().collect_vec()).collect_vec();
    let nums = lines.iter().take(lines.len() - 1).collect_vec();
    let operators = lines.iter().last().unwrap();
    let mut sheet = Sheet { operations: vec![] };
    if pt1 {
        for col in 0..nums[0].len() {
            let values = (0..nums.len()).map(|xlrg| nums[xlrg][col].parse().unwrap()).collect();
            let operator = operators[sheet.operations.len()].chars().next().unwrap();
            sheet.operations.push(Operation { values, operator });
        }
    } else {
        for col in 0..nums[0].len() {
            let values =
                (0..nums.len()).map(|xlrg| nums[xlrg][col].parse::<V>().unwrap()).collect_vec();
            let max_digits = values.iter().map(|v| num_digits(*v)).max().unwrap_or_default();
            println!("values: {values:?} max_digits: {max_digits}");
            for digit in (1..=max_digits) {
                let pos_digit = max_digits - digit + 1;
                let digits = values
                    .iter() //
                    .filter(|v| num_digits(**v) >= digit)
                    .map(|v| *v)
                    .collect_vec();
                println!(">> digits: {digits:?} ({digit}, {pos_digit})");
                //
            }
            let operator = operators[sheet.operations.len()].chars().next().unwrap();
            sheet.operations.push(Operation { values, operator });
        }
    }
    println!("{sheet:#?}");
    sheet
}

mod tests {
    use super::*;

    #[test]
    fn test_pt1_ex1() {
        assert_eq!(run(EX1, true), 4277556);
    }

    #[test]
    fn test_pt1_in1() {
        assert_eq!(run(IN1, true), 5335495999141);
    }

    #[test]
    fn test_pt2_ex1() {
        assert_eq!(run(EX1, false), 3263827);
    }
}
