// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use serde_scan;

use crate::utils::*;

#[derive(Debug, Clone)]
struct Op {
    name: String,
    value: i64,
}

fn parse_prog(lines: &Vec<String>) -> Vec<Op> {
    let mut prog = Vec::new();
    for line in lines {
        let parts = split_string(line, " ");
        prog.push(Op {
            name: parts[0].clone(),
            value: parse_i64(&parts[1]),
        });
    }
    prog
}

fn run_prog(prog: &Vec<Op>) -> (bool, i64) {
    let mut acc = 0;
    let mut ip = 0;
    let mut visited = HashMap::new();

    loop {
        if ip as usize >= prog.len() {
            return (true, acc);
        }
        let key = ip;
        if visited.contains_key(&key) {
            return (false, acc);
        }
        visited.insert(key, true);
        let p = &prog[ip as usize];
        match p.name.as_str() {
            "acc" => {
                acc += p.value;
                ip += 1;
            }
            "jmp" => {
                ip = (ip + p.value).max(0);
            }
            _ => {
                ip += 1;
            }
        }
    }
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let prog = parse_prog(lines);
    let (ok, acc) = run_prog(&prog);
    acc
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let prog = parse_prog(lines);
    for index in 0..prog.len() {
        let p = &prog[index];
        let mut new_prog = prog.clone();
        match p.name.as_str() {
            "nop" => {
                new_prog[index].name = "jmp".to_string();
            }
            "jmp" => {
                new_prog[index].name = "nop".to_string();
            }
            _ => {
                continue;
            }
        }
        let (ok, acc) = run_prog(&new_prog);
        if ok {
            return acc;
        }
    }
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
