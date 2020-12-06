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

pub fn part1(data: &str) -> i64 {
    data.trim()
        .split("\n\n")
        .map(|x| x.replace("\n", ""))
        .map(|x| {
            let set: HashSet<_> = x.chars().collect();
            set.len() as i64
        })
        .sum()
}

pub fn part2(data: &str) -> i64 {
    let mut res = 0;
    for lines in data
        .trim()
        .split("\n\n")
        .map(|x| split_string(&x.to_string(), "\n"))
    {
        let mut set: HashSet<_> = lines[0].chars().collect();
        for line in &lines[1..] {
            let other_set: HashSet<_> = line.chars().collect();
            set = set.intersection(&other_set).cloned().collect()
        }
        res += set.len() as i64;
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_main_input();
        assert_eq!(part1(&data), 6763);
    }

    #[test]
    fn test_part2() {
        let data = read_main_input();
        assert_eq!(part2(&data), 3512);
    }
}

pub fn read_main_input() -> String {
    include_str!("../../input/day06/in.txt").to_string()
    // read_input("input/day06/in.txt")
    // unreachable!()
}

fn main() {
    let data = read_main_input();

    println!("part1 = {}", part1(&data));
    println!("part2 = {}", part2(&data));
}
