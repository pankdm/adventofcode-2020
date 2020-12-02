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
    for line in lines {
        let (a, b, ch, pass): (usize, usize, char, String) =
            serde_scan::scan!("{}-{} {}: {}" <- line).unwrap();
        let mut count = 0;
        for c in pass.chars() {
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
        let (a, b, ch, pass): (usize, usize, char, String) =
            serde_scan::scan!("{}-{} {}: {}" <- line).unwrap();

        let a1 = pass.chars().nth(a - 1).unwrap();
        let b1 = pass.chars().nth(b - 1).unwrap();
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
