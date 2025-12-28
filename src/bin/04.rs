use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, rc::Rc, usize};

static EX1: &str = include_str!("../../data/04/ex1");
static IN1: &str = include_str!("../../data/04/in1");

fn main() {
    println!("ex1: {}", run(EX1, true));
    println!("in1: {}", run(IN1, true));
}

fn run(s: &str, pt1: bool) -> usize {
    let grid = parse(s);
    grid.max_neighbors(3)
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
    fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter().flat_map(|row| row.iter())
    }
    fn max_neighbors(&self, max: usize) -> usize {
        self.iter()
            .filter(|t| t.val == '@' as u8)
            .filter(|t| self.paper_neighbors(t) <= max)
            .count()
    }
    fn paper_neighbors(&self, tile: &Tile) -> usize {
        let mut res = 0;
        let rows = self.tiles.len();
        let cols = self.tiles.len();
        let &Tile { row, col, val } = tile;
        // left
        if col > 0 && self.is_paper(row, col - 1) {
            res += 1;
        }
        // up
        if row > 0 && self.is_paper(row - 1, col) {
            res += 1;
        }
        // right
        if col < cols - 1 && self.is_paper(row, col + 1) {
            res += 1;
        }
        // down
        if row < rows - 1 && self.is_paper(row + 1, col) {
            res += 1;
        }
        // left up
        if col > 0 && row > 0 && self.is_paper(row - 1, col - 1) {
            res += 1;
        }
        // left down
        if col > 0 && row < rows - 1 && self.is_paper(row + 1, col - 1) {
            res += 1;
        }
        // right up
        if col < cols - 1 && row > 0 && self.is_paper(row - 1, col + 1) {
            res += 1;
        }
        // right down
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
}
