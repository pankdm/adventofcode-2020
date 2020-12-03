// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};

use serde_scan;

extern crate aoc;
use aoc::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut row = 1;
    let mut col = 3;

    loop {
        if row >= lines.len() {
            return res;
        }
        col = col % lines[row].len();
        let ch = lines[row].chars().nth(col).unwrap();
        if ch == '#' {
            res += 1;
        }
        row += 1;
        col += 3;
    }
    res
}

fn check_slope(lines: &Vec<String>, c: usize, r: usize) -> i64 {
    let mut res = 0;
    let mut row = r;
    let mut col = c;

    loop {
        if row >= lines.len() {
            return res;
        }
        col = col % lines[row].len();
        let ch = lines[row].chars().nth(col).unwrap();
        if ch == '#' {
            res += 1;
        }
        row += r;
        col += c;
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 1;
    res *= check_slope(lines, 1, 1);
    res *= check_slope(lines, 3, 1);
    res *= check_slope(lines, 5, 1);
    res *= check_slope(lines, 7, 1);
    res *= check_slope(lines, 1, 2);
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[test]
    fn test_part1() {
        let lines = read_input("input/day03/in.txt");
        assert_eq!(part1(&lines), 207);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("input/day03/in.txt");
        assert_eq!(part2(&lines), 2655892800);
    }}

fn main() {
    let lines = read_input("input/day03/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}