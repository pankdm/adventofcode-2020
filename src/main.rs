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
    // TODO: code here
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    // TODO: code here
    res
}

pub fn read_main_input() -> Vec<String> {
    // read_input("input/dayXX/in.txt")
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), -1);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), -1);
    }
}

fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    // println!("part2 = {}", part2(&lines));
}
