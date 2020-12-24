// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

// use serde_scan;

use crate::utils::*;

fn walk(mut line: &str) -> (i64, i64) {
    let dirs = vec![
        ("e", (1, 0)),
        ("w", (-1, 0)),
        ("ne", (0, 1)),
        ("sw", (0, -1)),
        ("nw", (-1, 1)),
        ("se", (1, -1)),
    ];

    let mut x = 0;
    let mut y = 0;
    while !line.is_empty() {
        for (k, (dx, dy)) in dirs.iter() {
            if line.starts_with(k) {
                x += dx;
                y += dy;
                line = &line[k.len()..];
            }
        }
    }
    (x, y)
}

pub fn parse_grid(lines: &Vec<String>) -> HashSet<(i64, i64)> {
    let mut tiles = HashMap::new();

    for line in lines {
        let key = walk(line);
        *tiles.entry(key).or_insert(0) += 1;
    }
    tiles
        .iter()
        .filter(|(k, v)| *v % 2 == 1)
        .map(|x| x.0)
        .cloned()
        .collect()
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let tiles = parse_grid(lines);
    tiles.len() as i64
}

pub fn nearby(pt: &(i64, i64)) -> Vec<(i64, i64)> {
    let dirs = vec![
        ("e", (1, 0)),
        ("w", (-1, 0)),
        ("ne", (0, 1)),
        ("sw", (0, -1)),
        ("nw", (-1, 1)),
        ("se", (1, -1)),
    ];

    let (x, y) = *pt;
    let mut res = Vec::new();
    for (_, (dx, dy)) in dirs.iter() {
        res.push((x + dx, y + dy));
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut tiles = parse_grid(lines);
    for day in 1..=100 {
        let mut new_tiles = HashSet::new();
        let to_process: HashSet<_> = tiles.iter().flat_map(|x| nearby(x)).collect();
        for pt in to_process.iter() {
            let num_black = nearby(pt).iter().filter(|x| tiles.contains(x)).count();
            if tiles.contains(pt) {
                if num_black == 1 || num_black == 2 {
                    new_tiles.insert(pt.clone());
                }
            } else {
                if num_black == 2 {
                    new_tiles.insert(pt.clone());
                }
            }
        }
        tiles = new_tiles;
        if day <= 10 || day % 10 == 0 {
            println!("day = {}, len = {}", day, tiles.len());
        }
    }
    tiles.len() as i64
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 354);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 3608);
    }

    #[test]
    fn test_walk() {
        assert_eq!(walk("nwwswee"), (0, 0))
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day24/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}
