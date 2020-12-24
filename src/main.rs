// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

extern crate aoc;
use aoc::*;

mod utils;
use utils::*;

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day24/in.txt").unwrap();
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

    dbg!(day24::part1(&lines));
    dbg!(day24::part2(&lines));
}
