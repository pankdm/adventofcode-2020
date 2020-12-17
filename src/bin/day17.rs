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

fn neighbours(x: i64, y: i64, z: i64) -> Vec<(i64, i64, i64)> {
    let d = vec![-1, 0, 1];

    let mut res = Vec::new();
    for dx in d.iter() {
        for dy in d.iter() {
            for dz in d.iter() {
                if *dx == 0 && *dy == 0 && *dz == 0 {
                    continue;
                }
                res.push((x + dx, y + dy, z + dz));
            }
        }
    }
    res
}

fn neighbours4(x: i64, y: i64, z: i64, w: i64) -> Vec<(i64, i64, i64, i64)> {
    let d = vec![-1, 0, 1];

    let mut res = Vec::new();
    for dx in d.iter() {
        for dy in d.iter() {
            for dz in d.iter() {
                for dw in d.iter() {
                    if *dx == 0 && *dy == 0 && *dz == 0 && *dw == 0 {
                        continue;
                    }
                    res.push((x + dx, y + dy, z + dz, w + dw));
                }
            }
        }
    }
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let grid = to_vv_char(lines);

    let mut space = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                let key = (x as i64, y as i64, 0 as i64, 0 as i64);
                space.insert(key);
            }
        }
    }
    dbg!(space.len());

    for i in 0..6 {
        // println!("i = {}, len = {}", i, space.len());
        // dbg!(&space);
        let mut new_space = HashSet::new();
        let xmin = space.iter().map(|p| p.0).min().unwrap();
        let xmax = space.iter().map(|p| p.0).max().unwrap();

        let ymin = space.iter().map(|p| p.1).min().unwrap();
        let ymax = space.iter().map(|p| p.1).max().unwrap();

        let zmin = space.iter().map(|p| p.2).min().unwrap();
        let zmax = space.iter().map(|p| p.2).max().unwrap();

        let wmin = space.iter().map(|p| p.3).min().unwrap();
        let wmax = space.iter().map(|p| p.3).max().unwrap();
        // println!("  {:?} {:?} {:?}", (xmin, xmax), (ymin, ymax), (zmin, zmax));

        for w in (wmin - 1)..=(wmax + 1) {
            for z in (zmin - 1)..=(zmax + 1) {
                for x in (xmin - 1)..=(xmax + 1) {
                    for y in (ymin - 1)..=(ymax + 1) {
                        let mut active = 0;
                        let key = (x, y, z, w);
                        for pt in neighbours4(x, y, z, w) {
                            // if z == 0 {
                            //     println!(" key = {:?}, neighbour = {:?}", key, pt);
                            // }
                            if space.contains(&pt) {
                                active += 1;
                            }
                        }
                        // if z == 0 {
                        //     println!("checking at {:?}, active = {}", key, active);
                        // }
                        if space.contains(&key) {
                            if active == 2 || active == 3 {
                                new_space.insert(key);
                            }
                        } else {
                            if active == 3 {
                                new_space.insert(key);
                            }
                        }
                    }
                }
            }
        }
        space = new_space;
    }

    space.len() as i64
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let grid = to_vv_char(lines);

    let mut space = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                let key = (x as i64, y as i64, 0 as i64);
                space.insert(key);
            }
        }
    }
    dbg!(space.len());

    for i in 0..6 {
        // println!("i = {}, len = {}", i, space.len());
        // dbg!(&space);
        let mut new_space = HashSet::new();
        let xmin = space.iter().map(|p| p.0).min().unwrap();
        let xmax = space.iter().map(|p| p.0).max().unwrap();

        let ymin = space.iter().map(|p| p.1).min().unwrap();
        let ymax = space.iter().map(|p| p.1).max().unwrap();

        let zmin = space.iter().map(|p| p.2).min().unwrap();
        let zmax = space.iter().map(|p| p.2).max().unwrap();
        // println!("  {:?} {:?} {:?}", (xmin, xmax), (ymin, ymax), (zmin, zmax));

        for z in (zmin - 1)..=(zmax + 1) {
            for x in (xmin - 1)..=(xmax + 1) {
                for y in (ymin - 1)..=(ymax + 1) {
                    let mut active = 0;
                    let key = (x, y, z);
                    for pt in neighbours(x, y, z) {
                        // if z == 0 {
                        //     println!(" key = {:?}, neighbour = {:?}", key, pt);
                        // }
                        if space.contains(&pt) {
                            active += 1;
                        }
                    }
                    // if z == 0 {
                    //     println!("checking at {:?}, active = {}", key, active);
                    // }
                    if space.contains(&key) {
                        if active == 2 || active == 3 {
                            new_space.insert(key);
                        }
                    } else {
                        if active == 3 {
                            new_space.insert(key);
                        }
                    }
                }
            }
        }
        space = new_space;
    }

    space.len() as i64
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 223);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 1884);
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day17/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}

pub fn read_input_from_args(args: &Vec<String>) -> Vec<String> {
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

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
