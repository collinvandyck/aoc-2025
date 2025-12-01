use std::iter;

fn main() {
    let ex1 = aoc::load_data("01/ex1");
    let in1 = aoc::load_data("01/in1");
    println!("ex1: {}", run(&ex1, true));
    println!("pt1: {}", run(&in1, true));
    println!("ex2: {}", run(&ex1, false));
    println!("pt2: {}", run(&in1, false));
}

fn run(data: &str, pt1: bool) -> usize {
    let mut cur = 50;
    let mut res = 0;
    let deltas = data.lines().map(|s| {
        let dir = &s[0..1];
        let mut amt = (&s[1..]).parse::<i64>().unwrap();
        if dir == "L" {
            amt *= -1
        }
        amt
    });
    for amt in deltas {
        cur = (((cur + amt) % 100) + 100) % 100;
        res += (cur == 0).then_some(1).unwrap_or_default();
    }
    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_spans() {
        let cur = 50;
        let amt = -60;
        // i want to verify that we can calculate the number of clicks
        // that include 0 are correct.
        let dlt = cur + amt;
        assert_eq!(dlt, -10);
        // so now we have --10 -> 50
        let r = std::ops::Range {
            start: dlt,
            end: cur,
        };

        // verify that our modulo arithmetic is ok
        let cur = (((cur + amt) % 100) + 100) % 100;
        assert_eq!(cur, 90)
    }
}
