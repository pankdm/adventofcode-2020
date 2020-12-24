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

type Range = (i64, i64, i64, i64);

pub fn inside_range(x: i64, range: &Range) -> bool {
    let (a, b, c, d) = range.clone();
    (a <= x && x <= b) || (c <= x && x <= d)
}

pub fn is_outside_every(x: i64, ranges: &Vec<Range>) -> bool {
    for r in ranges.iter() {
        if inside_range(x, r) {
            return false;
        }
    }
    true
}

pub fn part1(data: &String) -> i64 {
    let mut res = 0;
    // TODO: code here
    let info = split_string(data, "\n\n");
    let line0 = split_string(&info[0], "\n");

    let mut ranges = Vec::new();
    for line in line0.iter() {
        let parts = split_string(line, ": ");
        let s = &parts[1];
        let (a, b, c, d): Range = serde_scan::scan!("{}-{} or {}-{}" <- s).unwrap();
        // dbg!((a, b, c, d));
        ranges.push((a, b, c, d));
    }

    let line2 = split_string(&info[2], "\n");
    for line in line2.iter().skip(1) {
        let nums = parse_ints(&line, ",");
        for x in nums {
            if is_outside_every(x, &ranges) {
                res += x;
            }
        }
    }

    res
}

pub fn part2(data: &String) -> i64 {
    // let mut res = 0;
    // TODO: code here

    let info = split_string(data, "\n\n");
    let line0 = split_string(&info[0], "\n");

    let mut ranges = Vec::new();
    for line in line0.iter() {
        let parts = split_string(line, ": ");
        let s = &parts[1];
        let (a, b, c, d): Range = serde_scan::scan!("{}-{} or {}-{}" <- s).unwrap();
        // dbg!((a, b, c, d));
        ranges.push((a, b, c, d));
    }

    let mut tickets = Vec::new();
    let mut my_ticket = Vec::new();

    let line1 = split_string(&info[1], "\n");
    for line in line1.iter().skip(1) {
        let nums = parse_ints(&line, ",");
        my_ticket = nums.clone();
        tickets.push(nums.clone());
    }

    let line2 = split_string(&info[2], "\n");
    for line in line2.iter().skip(1) {
        let nums = parse_ints(&line, ",");
        let invalid = nums.iter().any(|x| is_outside_every(*x, &ranges));
        if invalid {
            continue;
        }

        tickets.push(nums);
    }

    dbg!(tickets.len());

    let fields = my_ticket.len();
    let mut matched_range = vec![-1 as i64; fields];
    let mut matched_field = vec![-1 as i64; fields];
    let mut num_matched = 0;
    loop {
        if num_matched == fields {
            break;
        }
        for i in 0..fields {
            if matched_field[i] != -1 {
                continue;
            }

            let mut num_ok = 0;
            let mut oks = Vec::new();
            for (index, r) in ranges.iter().enumerate() {
                if matched_range[index] != -1 {
                    continue;
                }
                let ok = tickets.iter().all(|t| inside_range(t[i], r));
                if ok {
                    oks.push(index);
                    num_ok += 1;
                }
            }
            assert!(oks.len() >= 1);
            if oks.len() == 1 {
                let index = oks[0];
                matched_field[i] = index as i64;
                matched_range[index] = i as i64;
                num_matched += 1;
                break;
            }
            // println!("i = {}, num_ok = {}", i, num_ok);
        }
    }

    let mut res = 1;
    for i in 0..6 {
        let field_index = matched_range[i];
        res *= my_ticket[field_index as usize];
    }
    res
}
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 27911);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 737176602479);
    }
}

pub fn read_main_input() -> String {
    let input = fs::read_to_string("input/day16/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    // to_lines(&input)
    input.trim().to_string()
}

