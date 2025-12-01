use std::iter;

fn main() {
    let ex1 = aoc::load_data("01/ex1").unwrap();
    let in1 = aoc::load_data("01/in1").unwrap();
    println!("ex1: {}", run(&ex1, true));
    println!("pt1: {}", run(&in1, true));
    println!("ex2: {}", run(&ex1, false));
    println!("pt2: {}", run(&in1, false));
}

fn run(data: &str, pt1: bool) -> usize {
    data.lines()
        .map(|s| {
            let dir = &s[0..1];
            let mut amt = (&s[1..]).parse::<i64>().unwrap();
            if dir == "L" {
                amt *= -1
            }
            amt
        })
        .fold((0, 50), |(mut res, mut cur), amt| {
            if pt1 {
                cur = (((cur + amt) % 100) + 100) % 100;
                res += (cur == 0).then_some(1).unwrap_or_default();
            } else {
                let delta = cur + amt;
                cur = (((cur + amt) % 100) + 100) % 100;
            }
            (res, cur)
        })
        .0
}
