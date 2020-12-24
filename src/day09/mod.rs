// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use serde_scan;

extern crate aoc;
use aoc::*;

// const N: usize = 25;
const N: usize = 25;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut v = Vec::new();
    for line in lines {
        v.push(parse_i64(line));
    }

    let is_valid = |x, start, end| {
        for i in start..end {
            for j in start..i {
                if v[i] + v[j] == x {
                    return true;
                }
            }
        }
        return false;
    };

    for i in N..v.len() {
        if !is_valid(v[i], i - N, i) {
            return v[i];
        }
    }

    -1
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let v: Vec<_> = lines.iter().map(|s| parse_i64(s)).collect();

    let ans = 731031916;
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            let s: i64 = v[i..=j].iter().sum();
            if s == ans {
                let min = v[i..=j].iter().min().unwrap();
                let max = v[i..=j].iter().max().unwrap();
                return min + max;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 731031916);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 93396727);
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day09/in.txt").unwrap();
    to_lines(&input)
}

fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
