// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

extern crate aoc;
use aoc::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut x = Vec::new();
    for line in lines {
        let n = parse_i64(line);
        x.push(n);
    }
    let v = x
        .iter()
        .cloned()
        .combinations(2)
        .filter(|it| it.iter().sum::<i64>() == 2020)
        .next()
        .unwrap();
    v[0] * v[1]
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut x = Vec::new();
    for line in lines {
        let n = parse_i64(line);
        x.push(n);
    }
    let v = x
        .iter()
        .cloned()
        .combinations(3)
        .filter(|it| it.iter().sum::<i64>() == 2020)
        .next()
        .unwrap();
    v[0] * v[1] * v[2]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("input/day01/in.txt");
        assert_eq!(part1(&lines), 960075);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("input/day01/in.txt");
        assert_eq!(part2(&lines), 212900130);
    }
}

fn main() {
    let lines = read_input("input/day01/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
