// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};

use serde_scan;

use crate::utils::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    check_slope(lines, 3, 1)
}

fn check_slope(lines: &Vec<String>, c: usize, r: usize) -> i64 {
    let mut res = 0;
    let mut row = 0;
    let mut col = 0;

    let map = to_vv_char(lines);
    loop {
        row += r;
        col += c;
        if row >= map.len() {
            return res;
        }
        if map[row][col % map[row].len()] == '#' {
            res += 1;
        }
    }
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 1;
    // Right 1, down 1.
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.
    let args = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for (c, r) in args.iter() {
        res *= check_slope(lines, *c, *r);
    }
    res
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day03/in.txt")
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 207);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 2655892800);
    }
}
