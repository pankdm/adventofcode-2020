// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use num_bigint::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

use serde_scan;

extern crate aoc;
use aoc::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;

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

    -1
}

fn gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if a == (0 as i64).to_bigint().unwrap() {
        return (b.clone(), 0.to_bigint().unwrap(), 1.to_bigint().unwrap());
    }
    let (d, x1, y1) = gcd(b.clone() % a.clone(), a.clone());
    let x = y1 - (b / a) * x1.clone();
    let y = x1;
    return (d, x, y);
}

fn abs_bigint(a: &BigInt) -> BigInt {
    if *a < (0 as i64).to_bigint().unwrap() {
        return (-1 as i64).to_bigint().unwrap() * a;
    }
    a.clone()
}

fn find_any_solution(a: &BigInt, b: &BigInt, c: &BigInt) -> Option<(BigInt, BigInt, BigInt)> {
    let (g, mut x0, mut y0) = gcd(abs_bigint(&a), abs_bigint(&b));
    if c % g.clone() != (0 as i64).to_bigint().unwrap() {
        return None;
    }
    x0 *= c / g.clone();
    y0 *= c / g.clone();
    if *a < (0 as i64).to_bigint().unwrap() {
        x0 *= -1;
    }
    if *b < (0 as i64).to_bigint().unwrap() {
        y0 *= -1;
    }
    return Some((x0, y0, g));
}

pub fn part2_impl(line: &str) -> String {
    let parts = split_string(line, ",");
    let mut tmp = Vec::new();
    for (offset, part) in parts.iter().enumerate() {
        if part != "x" {
            tmp.push((
                parse_i64(part).to_bigint().unwrap(),
                offset.to_bigint().unwrap(),
            ));
        }
    }

    // buses = [(29, 0), (41, 19), (521, 29), (23, 37), (13, 42), (17, 46), (601, 60), (37, 66), (19, 79)]
    // buses = [(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)]

    let mut buses = Vec::new();
    let mut ans = 1 as i64;
    for (x, offset) in tmp.iter() {
        let r = ((x - offset) % x + x) % x;
        buses.push((x.clone(), r.clone()));
        // println!("A = {} mod {}", r, x);
    }
    // }
    // 7x
    // 7x + 1 % 13
    // 7x + 1 = 13y
    // 13y - 1
    // 7 * (13a - 1) * (59b - 4) * (31c - 6) * (19d - 7) * d

    let mut a = (1 as i64).to_bigint().unwrap();
    let mut A = (0 as i64).to_bigint().unwrap();

    // println!("{:?}", find_any_solution(7, -13, 12));
    for (b, B) in buses.iter() {
        let (x0, y0, g) =
            find_any_solution(&a.clone(), &(-b.clone()), &(B.clone() - A.clone())).unwrap();
        // assert!(g == 1, "g={}", g);
        // x = x0 + k * b
        // a * x + A
        // a * (x0 + k * b) + A
        // k * a * b + a * x0 + A
        A = x0 * a.clone() + A.clone();
        a = a.clone() * b;
        // println!("    After ({}, {}) -> {} * x + {}", b, B, a, A);
    }
    // let (g, _, _) = gcd(a, A);
    let k = -A.clone() / a.clone();
    // println!("  k = {}", k);
    let res = k * a + A;
    // println!("{}", res);

    // (a * x + A) = (b * y + B)
    format!("{}", res)
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut res = 0;

    let start = parse_i64(&lines[0]);
    return part2_impl(&lines[1]);
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
        assert_eq!(part2(&lines), "725850285300475");
    }

    #[test]
    fn test_others() {
        assert_eq!(part2_impl("17,x,13,19"), "3417");
        assert_eq!(part2_impl("67,7,59,61"), "754018");
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day13/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}

pub fn read_input_from_args(args: &Vec<String>) -> Vec<String> {
    if args.len() <= 1 {
        return read_main_input();
    }
    let input = fs::read_to_string(&args[1]).unwrap();
    to_lines(&input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let lines = read_input_from_args(&args);

    println!("part1 = {}", part1(&lines));
    println!("part2 = {:?}", part2(&lines));
}
