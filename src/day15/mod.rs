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

extern crate aoc;
use aoc::*;

// If that was the first time the number has been spoken, the current player says 0.
// Otherwise, the number had been spoken before; the current player
// announces how many turns apart the number is from when it was previously spoken.

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let parts = split_string(&lines[0], ",");

    let mut nums = Vec::new();
    let mut turn = 1;
    for part in parts.iter() {
        nums.push((parse_i64(part), turn));
        turn += 1;
    }

    loop {
        if nums.len() > 2020 {
            break;
        }
        let last = *nums.last().unwrap();
        println!(" at turn {} last = {:?}", turn, last);
        let mut diff = -1;
        for i in (0..nums.len() - 1).rev() {
            if nums[i].0 == last.0 {
                diff = last.1 - nums[i].1;
                break;
            }
        }
        if diff > 0 {
            nums.push((diff, turn));
        } else {
            nums.push((0, turn));
        }
        turn += 1;
    }

    dbg!(nums[2019]);
    dbg!(nums[2020]);
    nums[2020 - 1].0
}

pub fn part2(lines: &Vec<String>) -> i64 {
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

    let n = 30000000;
    // let n = 10;
    // let n = 2020;

    loop {
        if nums.len() > n {
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
        if !before.contains_key(&diff) {
            before.insert(diff, vec![turn]);
        } else {
            let mut prev = &before[&diff];
            before.insert(diff, vec![*prev.last().unwrap(), turn]);
        }
        turn += 1;
    }

    dbg!(nums[n - 1]);
    dbg!(nums[n]);
    nums[n - 1].0
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
    let input = fs::read_to_string("input/day15/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}

pub fn read_input_from_args(args: &Vec<String>) -> Vec<String> {
    println!("args: {:?}", args);
    if args.len() <= 1 {
        return read_main_input();
    }
    let input = fs::read_to_string(&args[1]).unwrap();
    to_lines(&input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_input_from_args(&args);

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
