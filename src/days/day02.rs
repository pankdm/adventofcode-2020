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
        let a1 = pass.as_bytes()[a - 1] as char;
        let b1 = pass.as_bytes()[b - 1] as char;
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

pub fn read_main_input() -> Vec<String> {
    read_input("input/day02/in.txt")
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 465);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 294);
    }
}

