// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::env;

use serde_scan;

extern crate aoc;
use aoc::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    // TODO: code here
    -1
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    // TODO: code here
    -1
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

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day12/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}

pub fn read_input_from_args(args: &Vec<String>) -> Vec<String> {
    unreachable!();
    if args.len() <= 1 {
        return read_main_input();
    }
    let input = fs::read_to_string(&args[1]).unwrap();
    to_lines(&input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    // println!("part2 = {}", part2(&lines));
}
