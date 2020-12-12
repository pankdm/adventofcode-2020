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

// Action N means to move north by the given value.
// Action S means to move south by the given value.
// Action E means to move east by the given value.
// Action W means to move west by the given value.
// Action L means to turn left the given number of degrees.
// Action R means to turn right the given number of degrees.
// Action F means to move forward by the given value in the direction the ship is currently facing.

pub fn rotate_left(v: Vector2d) -> Vector2d {
    Vector2d::new(-v.y, v.x)
}

pub fn rotate_right(v: Vector2d) -> Vector2d {
    Vector2d::new(v.y, -v.x)
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    let mut actions = Vec::new();
    for line in lines {
        let d = line.chars().nth(0).unwrap();
        let value = parse_i64(&line[1..]);
        actions.push((d, value));
    }

    let mut dir = Vector2d::new(1, 0);
    let mut pos = Vector2d::new(0, 0);

    for (d, v) in actions {
        match d {
            'N' => {
                pos.y += v;
            }
            'S' => {
                pos.y -= v;
            }
            'E' => {
                pos.x += v;
            }
            'W' => {
                pos.x -= v;
            }
            'L' => {
                let mut deg = v;
                loop {
                    if deg == 0 {
                        break;
                    }
                    dir = rotate_left(dir);
                    deg -= 90;
                }
            }
            'R' => {
                let mut deg = v;
                loop {
                    if deg == 0 {
                        break;
                    }
                    dir = rotate_right(dir);
                    deg -= 90;
                }
            }
            'F' => {
                pos = pos + dir * v;
            }
            _ => {}
        }
    }
    pos.x.abs() + pos.y.abs()
}

// Action N means to move the waypoint north by the given value.
// Action S means to move the waypoint south by the given value.
// Action E means to move the waypoint east by the given value.
// Action W means to move the waypoint west by the given value.
// Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
// Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
// Action F means to move forward to the waypoint a number of times equal to the given value.
// The waypoint starts 10 units east and 1 unit north relative to the ship.
// The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    let mut actions = Vec::new();
    for line in lines {
        let d = line.chars().nth(0).unwrap();
        let value = parse_i64(&line[1..]);
        actions.push((d, value));
    }

    let mut dir = Vector2d::new(1, 0);
    let mut ship = Vector2d::new(0, 0);
    let mut pos = Vector2d::new(10, 1);

    for (d, v) in actions {
        match d {
            'N' => {
                pos.y += v;
            }
            'S' => {
                pos.y -= v;
            }
            'E' => {
                pos.x += v;
            }
            'W' => {
                pos.x -= v;
            }
            'L' => {
                let mut deg = v;
                loop {
                    if deg == 0 {
                        break;
                    }
                    pos = rotate_left(pos);
                    deg -= 90;
                }
            }
            'R' => {
                let mut deg = v;
                loop {
                    if deg == 0 {
                        break;
                    }
                    pos = rotate_right(pos);
                    deg -= 90;
                }
            }
            'F' => {
                ship = ship + pos * v;
            }
            _ => {}
        }
    }
    ship.x.abs() + ship.y.abs()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 582);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 52069);
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day12/in.txt").unwrap();
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
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
