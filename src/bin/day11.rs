// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use serde_scan;

extern crate aoc;
use aoc::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut grid = to_vv_char(lines);

    let d: Vec<i64> = vec![-1, 0, 1];

    loop {
        let mut new_grid = grid.clone();
        let mut changed = false;
        let mut res = 0;
        for y in 0..new_grid.len() {
            for x in 0..new_grid[y].len() {
                let mut occupied = 0;
                if new_grid[y][x] == '#' {
                    res += 1;
                }
                for dx in d.iter() {
                    for dy in d.iter() {
                        if *dx == 0 && *dy == 0 {
                            continue;
                        }
                        let y1 = y as i64 + dy;
                        let x1 = x as i64 + dx;
                        if 0 <= x1 && x1 < grid[y].len() as i64 && 0 <= y1 && y1 < grid.len() as i64
                        {
                            if grid[y1 as usize][x1 as usize] == '#' {
                                occupied += 1;
                            }
                        }
                    }
                }
                let c = new_grid[y][x];
                if c == 'L' && occupied == 0 {
                    changed = true;
                    new_grid[y][x] = '#';
                }
                if c == '#' && occupied >= 4 {
                    changed = true;
                    new_grid[y][x] = 'L';
                }
            }
        }
        if !changed {
            return res;
        }
        grid = new_grid;
    }
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut grid = to_vv_char(lines);

    let d: Vec<i64> = vec![-1, 0, 1];

    let mut iter = 0;
    loop {
        // println!("iter = {}", iter);
        iter += 1;
        let mut new_grid = grid.clone();
        let mut changed = false;
        let mut res = 0;
        for y in 0..new_grid.len() {
            for x in 0..new_grid[y].len() {
                let mut occupied = 0;
                if new_grid[y][x] == '#' {
                    res += 1;
                }
                for dx in d.iter() {
                    for dy in d.iter() {
                        if *dx == 0 && *dy == 0 {
                            continue;
                        }
                        let mut iter = 1;
                        loop {
                            let y1 = y as i64 + iter * dy;
                            let x1 = x as i64 + iter * dx;
                            if 0 <= x1
                                && x1 < grid[y].len() as i64
                                && 0 <= y1
                                && y1 < grid.len() as i64
                            {
                                if grid[y1 as usize][x1 as usize] == '#' {
                                    occupied += 1;
                                    break;
                                }
                                if grid[y1 as usize][x1 as usize] == 'L' {
                                    break;
                                }
                                if grid[y1 as usize][x1 as usize] == '.' {
                                    iter += 1;
                                    continue;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                let c = new_grid[y][x];
                if c == 'L' && occupied == 0 {
                    changed = true;
                    new_grid[y][x] = '#';
                }
                if c == '#' && occupied >= 5 {
                    changed = true;
                    new_grid[y][x] = 'L';
                }
            }
        }
        if !changed {
            return res;
        }
        grid = new_grid;
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 2251);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 2019);
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day11/in.txt").unwrap();
    // let input = fs::read_to_string("input/day11/demo.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}

fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
