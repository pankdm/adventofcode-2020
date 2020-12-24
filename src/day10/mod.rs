// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use serde_scan;

use crate::utils::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut v = Vec::new();
    v.push(0);
    for line in lines {
        v.push(parse_i64(line));
    }
    v.sort();

    let mut n1 = 0;
    let mut n3 = 0;
    for i in 1..v.len() {
        let d = v[i] - v[i - 1];
        if d == 1 {
            n1 += 1;
        }
        if d == 3 {
            n3 += 1;
        }
    }

    n1 * (n3 + 1)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut res = 0;
    let mut v = Vec::new();
    v.push(0);
    for line in lines {
        v.push(parse_i64(line));
    }
    v.sort();
    let last = *v.last().unwrap();

    let mut set = HashSet::new();
    for x in v {
        set.insert(x);
    }

    let mut dp = vec![0; (last + 1) as usize];
    dp[last as usize] = 1;

    let mut current = last - 1;
    loop {
        if current < 0 {
            break;
        }
        for delta in 1..=3 {
            let next = current + delta;
            if set.contains(&next) {
                dp[current as usize] += dp[next as usize];
            }
        }

        current -= 1;
    }
    dp[0]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 2368);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 1727094849536);
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day10/in.txt").unwrap();
    // let input = fs::read_to_string("input/day10/demo1.txt").unwrap();

    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}
