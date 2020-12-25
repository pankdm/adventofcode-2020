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

const MULT: i64 = 7;
const MD: i64 = 20201227;

pub fn find_size(value: i64) -> i64 {
    let mut now = 1;
    let mut iter = 0;
    loop {
        if now == value {
            return iter;
        }
        now *= MULT;
        now %= MD;
        iter += 1;
    }
}

pub fn find_handshake(card: i64, door: i64) -> i64 {
    let card_loop = find_size(card);
    dbg!(card_loop);

    let door_loop = find_size(door);
    dbg!(door_loop);

    let mut now = 1;
    for i in 0..door_loop {
        now *= card;
        now %= MD;
    }

    now
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let card = parse_i64(&lines[0]);
    let door = parse_i64(&lines[1]);

    find_handshake(card, door)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    // TODO: code here
    -1
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
    let input = fs::read_to_string("input/day25/in.txt").unwrap();
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

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_input_from_args(&args);

    dbg!(find_handshake(5764801, 17807724));

    println!("part1 = {}", part1(&lines));
    // println!("part2 = {}", part2(&lines));
}
