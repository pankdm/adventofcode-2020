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

fn extract_bag_color(s: &String) -> String {
    s.trim_end_matches(" bag")
        .trim_end_matches(" bags")
        .to_string()
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    let mut g = HashMap::new();

    for _line in lines {
        let line = _line[.._line.len() - 1].to_string();
        let split = split_string(&line, " contain ");
        let src = split[0].clone();
        let input_bag = extract_bag_color(&src);

        let dsts = split[1].clone();
        if dsts == "no other bags" {
            continue;
        }
        let parts = split_string(&dsts, ", ");
        for part in parts.iter() {
            // println!("parts = {:?}, part {}", parts, part);
            let (num, bag): (i32, String) = serde_scan::scan!("{} {}" <- part).unwrap();
            // println!("src = {}: num {} -> {}", src, num, bag);
            let output_bag = extract_bag_color(&bag);

            g.entry(output_bag)
                .or_insert(Vec::new())
                .push(input_bag.clone());
        }
    }

    let mut q = VecDeque::new();
    let mut visited = HashMap::new();
    q.push_back("shiny gold".to_string());

    while !q.is_empty() {
        let now = q.pop_back().unwrap();
        if !g.contains_key(&now) {
            continue;
        }
        for next in g[&now].iter() {
            if !visited.contains_key(&next) {
                visited.insert(next, 1);
                q.push_back(next.clone());
            }
        }
    }
    visited.len() as i64
}

type Graph = HashMap<String, Vec<(i64, String)>>;

pub fn count_contains(now: &String, g: &Graph, cache: &mut HashMap<String, i64>) -> i64 {
    if cache.contains_key(now) {
        return cache[now];
    }
    if !g.contains_key(now) {
        return 1;
    }
    let mut res = 1;
    for (num, next) in g[now].iter() {
        res += num * count_contains(next, g, cache);
    }
    cache.insert(now.clone(), res);
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    let mut g = HashMap::new();

    for _line in lines {
        let line = _line[.._line.len() - 1].to_string();
        let split = split_string(&line, " contain ");
        let src = split[0].clone();
        let input_bag = extract_bag_color(&src);

        let dsts = split[1].clone();
        if dsts == "no other bags" {
            continue;
        }
        let parts = split_string(&dsts, ", ");
        for part in parts.iter() {
            // println!("parts = {:?}, part {}", parts, part);
            let (num, bag): (i64, String) = serde_scan::scan!("{} {}" <- part).unwrap();
            // println!("src = {}: num {} -> {}", src, num, bag);
            let output_bag = extract_bag_color(&bag);

            g.entry(input_bag.clone())
                .or_insert(Vec::new())
                .push((num, output_bag.clone()));
        }
    }

    let mut cache = HashMap::new();
    let start = "shiny gold".to_string();
    count_contains(&start, &g, &mut cache) - 1
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 148);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 24867);
    }

    #[test]
    fn test_demo() {
        let input = fs::read_to_string("input/day07/demo.txt").unwrap();
        let lines = to_lines(&input);
        assert_eq!(part1(&lines), 4);
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day07/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}

fn main() {
    let lines = read_main_input();
    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
