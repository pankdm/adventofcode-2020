// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use serde_scan;

extern crate aoc;
use aoc::*;

fn extract_bag_color(s: &str) -> String {
    s.trim_end_matches(" bag")
        .trim_end_matches(" bags")
        .to_string()
}

struct Rule {
    input_bag: String,
    output_bags: Vec<(i64, String)>,
}

fn parse_input(lines: &Vec<String>) -> Vec<Rule> {
    let mut res = Vec::new();
    for line in lines {
        let re = Regex::new("(.*) contain (.*)[.]").unwrap();
        let cap = re.captures(line).unwrap();
        // println!("matched {:?} and {:?}", cap.get(1).unwrap(), cap.get(2).unwrap());
        let src = cap.get(1).unwrap().as_str();
        let input_bag = extract_bag_color(src);

        let dsts = cap.get(2).unwrap().as_str();
        if dsts == "no other bags" {
            continue;
        }
        let parts = split_string(dsts, ", ");
        let mut output = Vec::new();
        for part in parts.iter() {
            // println!("parts = {:?}, part {}", parts, part);
            let (num, bag): (i64, String) = serde_scan::scan!("{} {}" <- part).unwrap();
            // println!("src = {}: num {} -> {}", src, num, bag);
            let output_bag = extract_bag_color(&bag);
            output.push((num, output_bag));
        }
        res.push(Rule {
            input_bag: input_bag,
            output_bags: output,
        });
    }
    res
}

fn dfs(now: &String, g: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>) {
    if !g.contains_key(now) {
        return;
    }
    for next in g[now].iter() {
        if !visited.contains(next) {
            visited.insert(next.clone());
            dfs(&next, g, visited);
        }
    }
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    let mut g = HashMap::new();
    let rules = parse_input(lines);
    for r in rules.iter() {
        for (num, output) in r.output_bags.iter() {
            g.entry(output.clone())
                .or_insert(Vec::new())
                .push(r.input_bag.clone());
        }
    }

    let start = "shiny gold".to_string();
    let mut visited = HashSet::new();
    visited.insert(start.clone());

    dfs(&start, &g, &mut visited);
    (visited.len() - 1) as i64
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

    let rules = parse_input(lines);

    let mut g = HashMap::new();
    for r in rules {
        g.insert(r.input_bag, r.output_bags);
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
