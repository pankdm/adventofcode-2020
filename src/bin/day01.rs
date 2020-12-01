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
    // TODO: code here
    let mut res = 0;
    let mut x = Vec::new();
    for line in lines {
        let n = parse_i64(line);
        x.push(n);
    }
    for i in 0..x.len() {
        for j in i + 1..x.len() {
            if x[i] + x[j] == 2020 {
                return x[i] * x[j];
            }
        }
    }

    -1
}

pub fn part2(lines: &Vec<String>) -> i64 {
    // TODO: code here
    let mut res = 0;
    let mut x = Vec::new();
    for line in lines {
        let n = parse_i64(line);
        x.push(n);
    }
    for i in 0..x.len() {
        for j in i + 1..x.len() {
            for k in j + 1..x.len() {
                if x[i] + x[j] + x[k] == 2020 {
                    return x[i] * x[j] * x[k];
                }
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(1 + 1, 2);
    }
}

fn main() {
    let lines = read_input("input/day01/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
