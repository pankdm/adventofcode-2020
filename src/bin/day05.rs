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
    let mut all = Vec::new();
    for line in lines {
        let s = to_v_char(line);
        let mut lo = 0;
        let mut hi = 128;
        for i in 0..7 {
            let mid = (lo + hi) / 2;
            let c = s[i];
            if c == 'F' {
                hi = mid;
            } else {
                lo = mid;
            }
        }

        let mut clo = 0;
        let mut chi = 8;
        for i in 0..3 {
            let mid = (clo + chi) / 2;
            let c = s[7 + i];
            if c == 'L' {
                chi = mid;
            } else {
                clo = mid;
            }
        }
        let id = lo * 8 + clo;
        all.push(id as i64);
    }
    *all.iter().max().unwrap()
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut all = Vec::new();
    for line in lines {
        let s = to_v_char(line);
        let mut lo = 0;
        let mut hi = 128;
        for i in 0..7 {
            let mid = (lo + hi) / 2;
            let c = s[i];
            if c == 'F' {
                hi = mid;
            } else {
                lo = mid;
            }
        }

        let mut clo = 0;
        let mut chi = 8;
        for i in 0..3 {
            let mid = (clo + chi) / 2;
            let c = s[7 + i];
            if c == 'L' {
                chi = mid;
            } else {
                clo = mid;
            }
        }
        let id = lo * 8 + clo;
        all.push(id as i64);
    }
    let max = *all.iter().max().unwrap();
    let min = *all.iter().min().unwrap();

    for s in min..=max {
        if !all.contains(&s) {
            return s;
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
        assert_eq!(part1(&lines), 848);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 682);
    }
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day05/in.txt")
    // unreachable!()
}

fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
