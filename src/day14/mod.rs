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

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut mem = HashMap::new();
    let mut mask = "X".repeat(36);

    for line in lines {
        if line.starts_with("mem") {
            let (pos, value): (i64, i64) = serde_scan::scan!("mem[{}] = {}" <- line).unwrap();
            let mut res = value;
            // println!("patsed mem at {}, value = {}, res = {}", pos, value, res);
            for bit in 0..36 {
                let m = mask.as_bytes()[mask.len() - 1 - bit] as char;
                if m == '1' {
                    res |= 1 << bit;
                    println!(" bit {} is set to 1: res = {}", bit, res);
                }
                if m == '0' {
                    res ^= (res & 1 << bit);
                    println!(" bit {} is set to 0: res = {}", bit, res);
                }
            }
            println!("mem at {} -> {:b}, inserted {:b}", pos, value, res);
            mem.insert(pos, res);
        } else {
            let _mask: String = serde_scan::scan!("mask = {}" <- line).unwrap();
            mask = _mask;
            println!("mask = {}", mask);
        }
    }

    let mut s = 0;
    for (k, v) in mem.iter() {
        println!("  >> final mem at {} -> {}", k, v);
        s += v;
    }
    s
}

fn update_mem(mem: &mut HashMap<i64, i64>, bits: &Vec<usize>, pos: i64, value: i64, index: usize) {
    if index >= bits.len() {
        println!("  updated {} with {}", pos, value);
        mem.insert(pos, value);
        return;
    }
    update_mem(mem, bits, pos, value, index + 1);
    update_mem(mem, bits, pos + (1 << bits[index]), value, index + 1);
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut mem = HashMap::new();
    let mut mask = "X".repeat(36);

    let mut total = 0;
    for line in lines {
        if line.starts_with("mem") {
            let (pos, value): (i64, i64) = serde_scan::scan!("mem[{}] = {}" <- line).unwrap();
            let mut res = pos;
            println!("parsed mem at {}, value = {}, res = {}", pos, value, res);
            let mut bits = Vec::new();
            for bit in 0..36 {
                let m = mask.as_bytes()[mask.len() - 1 - bit] as char;
                if m == '1' {
                    res |= 1 << bit;
                    // println!(" bit {} is set to 1: res = {}", bit, res);
                }
                if m == 'X' {
                    bits.push(bit);
                    res ^= (res & 1 << bit);
                    // println!(" bit {} is set to 0: res = {}", bit, res);
                }
            }
            // let cnt = mask.chars().filter(|x| *x =='X').count();
            // total += (2 as i64).pow(cnt as u32);
            // println!("num X = {}", total);

            // println!("mem at {} -> {:b}, inserted {:b}", pos, value, res);
            // mem.insert(pos, res);
            update_mem(&mut mem, &bits, res, value, 0);
        } else {
            let _mask: String = serde_scan::scan!("mask = {}" <- line).unwrap();
            mask = _mask;
            // println!("mask = {}", mask);
        }
    }

    let mut s = 0;
    for (k, v) in mem.iter() {
        println!("  >> final mem at {} -> {}", k, v);
        s += v;
    }
    s
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
    let input = fs::read_to_string("input/day14/in.txt").unwrap();
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

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
