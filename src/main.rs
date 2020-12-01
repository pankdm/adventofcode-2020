// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashSet, HashMap, VecDeque};

extern crate adventofcode;

use adventofcode::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    // TODO: code here
    let mut res = 0;
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    res
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
    let lines = read_input("input/day00/in.txt");

    println!("part1 = {}", part1(&lines));
    // println!("part2 = {}", part2(&lines));
}
