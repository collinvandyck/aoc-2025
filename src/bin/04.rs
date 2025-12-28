use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, rc::Rc, usize};

static EX1: &str = include_str!("../../data/04/ex1");
static IN1: &str = include_str!("../../data/04/in1");

fn main() {
    println!("pt2: {}", run(IN1, false));
}

fn run(s: &str, pt1: bool) -> usize {
    let mut grid = parse(s);
    if pt1 { grid.max_neighbors() } else { grid.max_neighbors_remove() }
}

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy)]
struct Tile {
    row: usize,
    col: usize,
    val: u8,
}

impl Grid {
    fn max_neighbors_remove(&mut self) -> usize {
        let mut res = 0;
        loop {
            let found: Option<Tile> = self
                .tiles
                .iter()
                .flat_map(|r| r.iter())
                .filter(|t| t.val == '@' as u8)
                .find(|t| self.paper_neighbors(t) <= 3)
                .copied();
            let Some(found) = found else { break };
            res += 1;
            self.tiles[found.row][found.col].val = '.' as u8;
        }
        res
    }
    fn max_neighbors(&self) -> usize {
        self.tiles
            .iter()
            .flat_map(|r| r.iter())
            .filter(|t| t.val == '@' as u8)
            .filter(|t| self.paper_neighbors(t) <= 3)
            .count()
    }
    fn paper_neighbors(&self, tile: &Tile) -> usize {
        let mut res = 0;
        let rows = self.tiles.len();
        let cols = self.tiles.len();
        let &Tile { row, col, val } = tile;
        if col > 0 && self.is_paper(row, col - 1) {
            res += 1;
        }
        if row > 0 && self.is_paper(row - 1, col) {
            res += 1;
        }
        if col < cols - 1 && self.is_paper(row, col + 1) {
            res += 1;
        }
        if row < rows - 1 && self.is_paper(row + 1, col) {
            res += 1;
        }
        if col > 0 && row > 0 && self.is_paper(row - 1, col - 1) {
            res += 1;
        }
        if col > 0 && row < rows - 1 && self.is_paper(row + 1, col - 1) {
            res += 1;
        }
        if col < cols - 1 && row > 0 && self.is_paper(row - 1, col + 1) {
            res += 1;
        }
        if col < cols - 1 && row < rows - 1 && self.is_paper(row + 1, col + 1) {
            res += 1;
        }
        res
    }
    fn is_paper(&self, row: usize, col: usize) -> bool {
        self.tiles[row][col].val == '@' as u8
    }
}

fn parse(s: &str) -> Grid {
    Grid {
        tiles: s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.trim().bytes().enumerate().map(|(col, val)| Tile { row, col, val }).collect()
            })
            .collect(),
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_pt1_ex1() {
        assert_eq!(run(EX1, true), 13);
    }

    #[test]
    fn test_pt1_in1() {
        assert_eq!(run(IN1, true), 1464);
    }

    #[test]
    fn test_pt2_ex1() {
        assert_eq!(run(EX1, false), 43);
    }

    #[test]
    fn test_pt2_in1() {
        assert_eq!(run(IN1, false), 8409);
    }
}
