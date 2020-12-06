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


fn process_group(g: &Vec<String>) -> i64 {
    // println!("g = {:?}", g);
    let mut r = HashMap::new();
    for line in g {
        for c in line.chars() {
            r.insert(c, 1);
        }
    }
    r.len() as i64
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut index = 0;

    let mut group = Vec::new();
    loop {
        let line = if index < lines.len() {
            lines[index].clone()
        } else {
            "".to_string()
        };
        if line.is_empty() {
            res += process_group(&group);
            group = Vec::new();
        } else {
            group.push(line);
        }
        if index >= lines.len() {
            break
        }
        index += 1;
    }
    res as i64
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut index = 0;

    let mut group = Vec::new();
    loop {
        let line = if index < lines.len() {
            lines[index].clone()
        } else {
            "".to_string()
        };
        if line.is_empty() {
            res += process_group2(&group);
            group = Vec::new();
        } else {
            group.push(line);
        }
        if index >= lines.len() {
            break
        }
        index += 1;
    }
    res as i64
}

fn process_group2(g: &Vec<String>) -> i64 {
    // println!("g = {:?}", g);
    let mut r = HashMap::new();
    for line in g {
        for c in line.chars() {
            if !r.contains_key(&c) {
                r.insert(c, 1);
            } else {
                r.insert(c, r[&c] + 1);
            }
        }
    }
    let mut res = 0;
    for (k, v) in r.iter() {
        if *v as i64 == g.len() as i64 {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 6763);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 3512);
    }
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day06/in.txt")
    // unreachable!()
}

fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
