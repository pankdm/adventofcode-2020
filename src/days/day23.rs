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

// Each move, the crab does the following actions:

// The crab picks up the three cups that are immediately clockwise of the current cup.
// They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.

// The crab selects a destination cup: the cup with a label equal to the current cup's label minus one.

// If this would select one of the cups that was just picked up,
// the crab will keep subtracting one until it finds a cup that wasn't just picked up.
// If at any point in this process the value goes below the lowest value on any cup's label,
// it wraps around to the highest value on any cup's label instead.

// The crab places the cups it just picked up so that they are immediately clockwise of the destination cup.
// They keep the same order as when they were picked up.

// The crab selects a new current cup: the cup which is immediately clockwise of the current cup.

fn find_by_value(cups: &Vec<i64>, value: i64) -> Option<usize> {
    cups.iter().position(|&x| x == value)
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    let line = &lines[0];
    let mut cups = Vec::new();

    let mut current = 0;
    for c in line.chars() {
        cups.push((c as u8 - '0' as u8) as i64);
    }
    let cnt = cups.len();

    for step in 0..100 {
        let mut removed = Vec::new();
        let mut new_cups = Vec::new();
        let current_value = cups[current];
        for i in 0..cnt {
            let c = cups[(current + i) % cnt];
            if 0 < i && i <= 3 {
                removed.push(c);
            } else {
                new_cups.push(c);
            }
        }
        println!(
            "step = {}, cups = {:?}, current = {}",
            step + 1,
            cups,
            current
        );
        println!("  current_value = {}", current_value);
        println!("  new_cups = {:?}", new_cups);
        println!("  removed = {:?}", removed);

        let mut label = cups[current] - 1;
        let mut dst;
        loop {
            if label <= 0 {
                label += cnt as i64;
            }
            let mut found = false;
            if let Some(index) = find_by_value(&new_cups, label) {
                dst = index;
                break;
            }
            label -= 1;
        }
        println!("  inserting after cup={}, label = {}", cups[dst], label);
        for i in 0..removed.len() {
            new_cups.insert(dst + 1 + i, removed[i]);
        }
        current = find_by_value(&new_cups, current_value).unwrap();
        current += 1;
        current %= cnt;
        cups = new_cups;
    }

    let start = find_by_value(&cups, 1).unwrap();
    let mut ans = Vec::new();
    for i in 1..cnt {
        let ch = (cups[(start + i) % cnt] as u8 + '0' as u8) as char;
        ans.push(ch);
    }
    let res: String = ans.iter().collect();
    println!("{}", res);

    -1
}

#[derive(Clone)]
struct Range {
    a: i64,
    b: i64,
}

struct Circle {
    // link to next node index
    // always starts with 1
    // start: usize,
    // forward: Vec<usize>,
    // nodes: Vec<Range>,
    // ranges: Vec<Range>,

    // value to next value
    next: Vec<usize>,
}

impl Circle {
    fn print(&self) {
        print!("[");
        // starts with 1
        let mut x = 1;
        for i in 0..10 {
            print!("{}, ", x);
            x = self.next[x];
        }
        // for r in self.ranges.iter() {
        //     if r.a == r.b {
        //         print!("{}, ", r.a);
        //     } else {
        //         print!(" ({}..{}), ", r.a, r.b);
        //     }
        // }
        println!("]");
    }

    fn next_value(&mut self, value: usize) -> usize {
        self.next[value]
    }

    fn remove_after(&mut self, value: usize) -> usize {
        let ret = self.next[value];
        self.next[value] = self.next[ret];
        return ret;
    }

    fn insert_after(&mut self, value: usize, x: usize) {
        let prev = self.next[value];
        self.next[x] = prev;
        self.next[value] = x;
    }

    // fn next_value(&mut self, value: i64) -> i64 {
    //     let mut index = self.ranges.iter().position(|x| x.a <= value && value <= x.b).unwrap();
    //     let r = &self.ranges[index];
    //     if value < r.b {
    //         return value + 1;
    //     }
    //     index += 1;
    //     index %= self.ranges.len();
    //     self.ranges[index].a
    // }

    // fn remove_after(&mut self, value: i64) -> i64 {
    //     let mut index = self.ranges.iter().position(|x| x.a <= value && value <= x.b).unwrap();
    //     let mut r = self.ranges[index].clone();

    //     if value == r.b {
    //         index += 1;
    //         index %= self.ranges.len();
    //         let ret = self.ranges[index].a;
    //         if self.ranges[index].a < self.ranges[index].b {
    //             self.ranges[index].a += 1;
    //             return ret;
    //         } else {
    //             self.ranges.remove(index);
    //             return ret;
    //         }
    //     } else {
    //         let ret = value + 1;
    //         self.ranges[index].b = value;
    //         if value + 1 < r.b {
    //             self.ranges.insert(index, Range{a: value + 2, b: r.b});
    //         }
    //         return ret;
    //     }
    // }

    // fn insert_after(&mut self, value: i64, x: i64) {
    //     let mut index = self.ranges.iter().position(|x| x.a <= value && value <= x.b).unwrap();
    //     self.ranges.insert(index + 1, Range{a: x, b: x});
    // }
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let line = &lines[0];
    let mut cups = Vec::new();

    for c in line.chars() {
        cups.push((c as u8 - '0' as u8) as usize);
    }

    let mut current_cup = cups[0];

    let max = 1000000;
    let mut next = vec![0; max + 1];
    for i in 1..cups.len() {
        let prev = cups[i - 1];
        let x = cups[i];
        next[prev] = x;
    }
    next[cups[cups.len() - 1]] = 10;
    for i in 10..max {
        next[i] = i + 1;
    }
    next[max] = cups[0];

    let mut ranges = Vec::new();
    for x in cups {
        ranges.push(Range {
            a: x as i64,
            b: x as i64,
        });
    }
    ranges.push(Range {
        a: 10,
        b: max as i64,
    });

    let n = 10000000;
    // let mut circle = Circle{ ranges: ranges };
    let mut circle = Circle { next: next };
    for step in 0..n {
        // if step % 100000 == 0 {
        //     println!("step = {}", step);
        //     // println!("step = {}, ranges = {}", step, circle.ranges.len());
        //     // circle.print();
        // }
        // println!("");
        // println!("step = {}, current = ({})", step, current_cup);
        // circle.print();

        let mut removed = Vec::with_capacity(3);
        for i in 0..3 {
            removed.push(circle.remove_after(current_cup))
        }
        // print!("after removal: ");
        // circle.print();
        let mut dst = current_cup - 1;
        loop {
            if dst <= 0 {
                dst = max;
            }
            if !removed.contains(&dst) {
                break;
            }
            dst -= 1;
        }
        // println!("dst = {}", dst);
        for i in 0..3 {
            // print!(" after insert {}, dst={}, ", i, dst);
            // circle.print();
            circle.insert_after(dst, removed[i]);
            dst = removed[i];
        }
        current_cup = circle.next_value(current_cup);
    }
    let a1 = circle.next_value(1);
    let a2 = circle.next_value(a1);

    a1 as i64 * a2 as i64
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
    let input = fs::read_to_string("input/day23/in.txt").unwrap();
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
