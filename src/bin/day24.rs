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

extern crate aoc;
use aoc::*;

// as e, se, sw, w, nw, and ne

fn walk(line: &str) -> (i64, i64) {
    let v = to_v_char(line);
    let mut i = 0;
    let mut x = 0;
    let mut y = 0;
    loop {
        if i >= v.len() {
            break;
        }
        match v[i] {
            'e' => {
                x += 1;
                i += 1;
            }
            'w' => {
                x -= 1;
                i += 1;
            }
            'n' => match v[i + 1] {
                'e' => {
                    y += 1;
                    i += 2;
                }
                'w' => {
                    y += 1;
                    x -= 1;
                    i += 2;
                }
                _ => unreachable!(),
            },
            's' => match v[i + 1] {
                'e' => {
                    y -= 1;
                    x += 1;
                    i += 2;
                }
                'w' => {
                    y -= 1;
                    i += 2;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    (x, y)
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut tiles = HashMap::new();

    for line in lines {
        let key = walk(line);
        if tiles.contains_key(&key) {
            tiles.insert(key, tiles[&key] + 1);
        } else {
            tiles.insert(key, 1);
        }
    }
    // TODO: code here
    let mut res = 0;
    for (k, v) in tiles.iter() {
        res += v % 2;
    }
    res
}

pub fn nearby(x: i64, y: i64) -> Vec<(i64, i64)> {
    let d = vec!["e", "se", "sw", "w", "nw", "ne"];
    let mut res = Vec::new();
    for dir in d.iter() {
        let (dx, dy) = walk(dir);
        res.push((x + dx, y + dy));
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut tiles_tmp = HashMap::new();

    for line in lines {
        let key = walk(line);
        if tiles_tmp.contains_key(&key) {
            tiles_tmp.insert(key, tiles_tmp[&key] + 1);
        } else {
            tiles_tmp.insert(key, 1);
        }
    }

    let mut tiles: HashSet<(i64, i64)> = HashSet::new();
    let mut res = 0;
    for (k, v) in tiles_tmp.iter() {
        if v % 2 == 1 {
            tiles.insert(k.clone());
        }
    }

    for day in 1..=100 {
        let mut new_tiles = HashSet::new();
        let mut to_process = HashSet::new();
        for (x, y) in tiles.iter() {
            for next_pt in nearby(*x, *y) {
                to_process.insert(next_pt);
            }
        }
        for pt in to_process.iter() {
            let (x, y) = pt;
            let mut num_black = 0;
            for next_pt in nearby(*x, *y) {
                if tiles.contains(&next_pt) {
                    num_black += 1;
                }
            }
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
        assert_eq!(part1(&lines), -1);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), -1);
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day24/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}

pub fn read_input_from_args(args: &Vec<String>) -> Vec<String> {
    // unreachable!();
    println!("args: {:?}", args);
    if args.len() <= 1 {
        return read_main_input();
    }
    let input = fs::read_to_string(&args[1]).unwrap();
    to_lines(&input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_input_from_args(&args);

    dbg!(walk("esew"));
    dbg!(walk("nwwswee"));

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
