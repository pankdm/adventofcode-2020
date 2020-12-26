// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

use serde_scan;

use crate::utils::*;

// If that was the first time the number has been spoken, the current player says 0.
// Otherwise, the number had been spoken before; the current player
// announces how many turns apart the number is from when it was previously spoken.

pub fn part1(lines: &Vec<String>) -> i64 {
    solution_impl(lines, 2020)
}

pub fn solution_impl(lines: &Vec<String>, times: usize) -> i64 {
    let mut res = 0;
    let parts = split_string(&lines[0], ",");

    let mut nums = Vec::new();
    let mut turn = 1 as i64;
    let mut before = HashMap::new();
    for part in parts.iter() {
        let v = parse_i64(part);
        nums.push((v, turn));
        before.insert(v, vec![turn]);
        turn += 1;
    }

    loop {
        if nums.len() > times {
            break;
        }
        let last = *nums.last().unwrap();
        if turn % 1000000 == 0 {
            println!(" at turn {} last = {:?}", turn, last);
        }
        let mut diff = 0;

        let v: &Vec<i64> = &before[&last.0];
        if v.len() >= 2 {
            assert_eq!(v[v.len() - 1], last.1);
            diff = last.1 - v[v.len() - 2];
        }

        nums.push((diff, turn));
        before.entry(diff).or_insert(vec![]).push(turn);
        turn += 1;
    }

    dbg!(nums[times - 1]);
    dbg!(nums[times]);
    nums[times - 1].0
}

pub fn part2(lines: &Vec<String>) -> i64 {
    solution_impl(lines, 30000000)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 1373);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 112458);
    }
}

pub fn read_input_from_args(args: &Vec<String>) -> Vec<String> {
    println!("args: {:?}", args);
    if args.len() <= 1 {
        return read_main_input();
    }
    let input = fs::read_to_string(&args[1]).unwrap();
    to_lines(&input)
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_input_from_args(&args);

    dbg!(part1(&lines));
    dbg!(part2(&lines));
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day15/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}
