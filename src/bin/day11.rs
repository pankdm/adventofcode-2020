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

    let mut i = 0;
    loop {
        // println!("iter = {}", i);
        i += 1;
        let mut new_grid = grid.clone();
        let mut changed = false;
        for y in 0..new_grid.len() {
            for x in 0..new_grid[y].len() {
                let mut occupied = 0;
                let v = Vector2d::new(x as i64, y as i64);
                for dv in neighbours8() {
                    let v1 = v + dv;
                    if !grid.inside(v1) {
                        continue;
                    }
                    if grid.get(v1) == '#' {
                        occupied += 1;
                    }
                }
                let c = grid.get(v);
                if c == 'L' && occupied == 0 {
                    changed = true;
                    new_grid.set(v, '#');
                }
                if c == '#' && occupied >= 4 {
                    changed = true;
                    new_grid.set(v, 'L');
                }
            }
        }
        if !changed {
            break;
        }
        grid = new_grid;
    }
    grid.iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x == '#')
        .count() as i64
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let mut grid = to_vv_char(lines);

    let mut i = 0;
    loop {
        // println!("iter = {}", i);
        i += 1;
        let mut new_grid = grid.clone();
        let mut changed = false;
        for y in 0..new_grid.len() {
            for x in 0..new_grid[y].len() {
                let mut occupied = 0;
                let v = Vector2d::new(x as i64, y as i64);
                for dv in neighbours8() {
                    let mut iter = 1;
                    loop {
                        let v1 = v + iter * dv;
                        if !grid.inside(v1) {
                            break;
                        }
                        match grid.get(v1) {
                            '#' => {
                                occupied += 1;
                                break;
                            }
                            'L' => {
                                break;
                            }
                            _ => {}
                        }
                        iter += 1;
                    }
                }
                let c = grid.get(v);
                if c == 'L' && occupied == 0 {
                    changed = true;
                    new_grid.set(v, '#');
                }
                if c == '#' && occupied >= 5 {
                    changed = true;
                    new_grid.set(v, 'L');
                }
            }
        }
        if !changed {
            break;
        }
        grid = new_grid;
    }

    grid.iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x == '#')
        .count() as i64
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
