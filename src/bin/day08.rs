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

#[derive(Debug, Clone)]
struct Op {
    cmd: String,
    value: i64,
}

fn run_prog(prog: &Vec<Op>) -> (bool, i64) {
    let mut acc = 0;
    let mut index = 0 as i64;
    let mut visited = HashMap::new();

    loop {
        if index as usize >= prog.len() {
            return (true, acc);
        }
        // println!("at index = {} -> {:?}", index, prog[index as usize]);
        let key = (index, 0);
        if visited.contains_key(&key) {
            return (false, acc);
        }
        visited.insert(key, true);
        let p = &prog[index as usize];
        if p.cmd == "acc" {
            acc += p.value;
            index += 1;
        }
        if p.cmd == "jmp" {
            index += p.value;
            if index < 0 {
                index = 0;
            }
        }
        if p.cmd == "nop" {
            index += 1;
        }
    }
    unreachable!();
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    let mut prog = Vec::new();
    for line in lines {
        let parts = split_string(line, " ");
        let cmd = parts[0].clone();
        let value = parse_i64(&parts[1]);
        prog.push(Op {
            cmd: cmd,
            value: value,
        });
    }

    let mut acc = 0;
    let mut index = 0 as i64;
    let mut visited = HashMap::new();

    loop {
        // println!("at index = {} -> {:?}", index, prog[index as usize]);
        let key = (index, 0);
        if visited.contains_key(&key) {
            return acc;
        }
        visited.insert(key, true);
        let p = &prog[index as usize];
        if p.cmd == "acc" {
            acc += p.value;
            index += 1;
        }
        if p.cmd == "jmp" {
            index += p.value;
            if index < 0 {
                index = 0;
            }
        }
        if p.cmd == "nop" {
            index += 1;
        }
    }

    // TODO: code here
    -1
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    let mut prog = Vec::new();
    for line in lines {
        let parts = split_string(line, " ");
        let cmd = parts[0].clone();
        let value = parse_i64(&parts[1]);
        prog.push(Op {
            cmd: cmd,
            value: value,
        });
    }

    for index in 0..prog.len() {
        let p = &prog[index];
        let mut new_prog = prog.clone();
        if p.cmd == "nop" {
            new_prog[index].cmd = "jmp".to_string();
        } else if p.cmd == "jmp" {
            new_prog[index].cmd = "nop".to_string();
        }
        let (ok, acc) = run_prog(&new_prog);
        if ok {
            return acc;
        }
    }

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
        assert_eq!(part1(&lines), 1684);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 2188);
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day08/in.txt").unwrap();
    to_lines(&input)
}

fn main() {
    let lines = read_main_input();

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
