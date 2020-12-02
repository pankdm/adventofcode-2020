// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};

extern crate adventofcode;

use adventofcode::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    for line in lines {
        let parts = split_string(line, " ");
        let nums = split_string(&parts[0], "-");
        let a = parse_i64(&nums[0]);
        let b = parse_i64(&nums[1]);
        let ch = parts[1].chars().nth(0).unwrap();
        // println!("a={} b={}, ch={}", a, b, ch);
        let mut count = 0;
        for c in parts[2].chars() {
            if c == ch {
                count += 1;
            }
        }
        if a <= count && count <= b {
            res += 1;
        }
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    for line in lines {
        let parts = split_string(line, " ");
        let nums = split_string(&parts[0], "-");
        let a = parse_i64(&nums[0]) - 1;
        let b = parse_i64(&nums[1]) - 1;
        let ch = parts[1].chars().nth(0).unwrap();
        let a1 = parts[2].chars().nth(a as usize).unwrap();
        let b1 = parts[2].chars().nth(b as usize).unwrap();
        let mut count = 0;
        if a1 == ch {
            count += 1;
        }
        if b1 == ch {
            count += 1;
        }
        if count == 1 {
            res += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("input/day02/in.txt");
        assert_eq!(part1(&lines), 465);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("input/day02/in.txt");
        assert_eq!(part2(&lines), 294);
    }
}

fn main() {
    let lines = read_input("input/day02/in.txt");

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
