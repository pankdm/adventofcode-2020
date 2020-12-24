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

    -1
}

mod my_bigint {
    use num_bigint::*;
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

    pub fn find_any_solution(
        a: &BigInt,
        b: &BigInt,
        c: &BigInt,
    ) -> Option<(BigInt, BigInt, BigInt)> {
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

    let mut buses = Vec::new();
    let mut ans = 1 as i64;
    for (x, offset) in tmp.iter() {
        let r = ((x - offset) % x + x) % x;
        buses.push((x.clone(), r.clone()));
    }

    let mut a = (1 as i64).to_bigint().unwrap();
    let mut A = (0 as i64).to_bigint().unwrap();

    for (b, B) in buses.iter() {
        // find_any_solution(a, b, c) -> (g, x, y) solves and equation so that
        // a * x + b * y = c
        // Here we find a solution to a * x + A = b * y + B
        // which is translated to a * x - b * y = B - A
        let (x0, y0, g) =
            my_bigint::find_any_solution(&a.clone(), &(-b.clone()), &(B.clone() - A.clone()))
                .unwrap();
        A = x0 * a.clone() + A.clone();
        a = a.clone() * b;
    }
    let k = -A.clone() / a.clone();
    let res = k * a + A;
    format!("{}", res)
}

pub fn part2(lines: &Vec<String>) -> String {
    let start = parse_i64(&lines[0]);
    return part2_impl(&lines[1]);
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

