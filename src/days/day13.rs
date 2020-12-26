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

use num_bigint::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let start = parse_i64(&lines[0]);
    let parts = split_string(&lines[1], ",");
    let mut buses = Vec::new();
    for part in parts.iter() {
        if part != "x" {
            buses.push(parse_i64(part));
        }
    }
    let mut now = start;
    loop {
        for bus in buses.iter() {
            if now % bus == 0 {
                return (now - start) * bus;
            }
        }
        now += 1;
    }
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let start = parse_i64(&lines[0]);
    return part2_crt(&lines[1]);
}

pub fn part2_crt(line: &str) -> i64 {
    let parts = split_string(line, ",");
    let mut buses = Vec::new();
    for (offset, part) in parts.iter().enumerate() {
        if part == "x" {
            continue;
        }
        let x = parse_i64(part);
        let a = (-(offset as i64) % x + x) % x;
        buses.push((x, a));
    }

    println!("buses = {:?}", buses);

    let md: i64 = buses.iter().map(|x| x.0).product();
    // println!("mod = {}", md);
    let mut res = 0;
    for (x, offset) in buses {
        let prod = md / x;
        let inv = mod_inverse(prod, x);
        // println!("  inverse to {} is {} (mod {})", prod, inv, x);
        res += offset * inv * prod;
        res %= md;
    }
    (res % md + md) % md
}

pub fn part2_sieving(line: &str) -> i64 {
    let parts = split_string(line, ",");

    let mut buses = Vec::new();
    for (offset, part) in parts.iter().enumerate() {
        if part == "x" {
            continue;
        }
        let x = parse_i64(part);
        buses.push((x, offset as i64));
    }

    println!(
        "modules = {:?}",
        buses.iter().map(|x| x.0).collect::<Vec<_>>()
    );

    let mut now = 0;
    let mut step = 1;
    for (x, offset) in buses {
        loop {
            if (now + offset) % x == 0 {
                break;
            }
            now += step;
        }
        step *= x;
    }
    now
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 4207);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 725850285300475);
    }

    #[test]
    fn test_others() {
        assert_eq!(part2_crt("17,x,13,19"), 3417);
        assert_eq!(part2_crt("67,7,59,61"), 754018);
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day13/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}
