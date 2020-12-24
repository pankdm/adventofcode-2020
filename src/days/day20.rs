// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use itertools::iproduct;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

use serde_scan;

use crate::utils::*;

type Side = Vec<char>;

struct Rotation {
    // goes from left in cw order
    // the side itself goes from left to right
    sides: Vec<Side>,
    grid: Grid,
}

struct Tile {
    id: i64,
    rotations: Vec<Rotation>,
}

type Grid = Vec<Vec<char>>;

fn flip_grid(t: &Grid) -> Grid {
    let mut r = t.clone();
    for i in 0..t.len() {
        r[i] = t[t.len() - 1 - i].clone();
    }
    r
}

fn rotate_grid(t: &Grid) -> Grid {
    assert_eq!(t.len(), t[0].len());
    let size = t.len();
    let mut r = t.clone();
    for i in 0..size {
        for j in 0..size {
            r[j][size - 1 - i] = t[i][j];
        }
    }
    r
}

fn read_rotation(t: &Grid) -> Rotation {
    let height = t.len();
    let width = t[0].len();
    let left: Side = (0..height).map(|i| t[i][0]).collect();
    let up: Side = (0..width).map(|i| t[0][i]).collect();
    let right: Side = (0..height).map(|i| t[i][width - 1]).collect();
    let down: Side = (0..width).map(|i| t[height - 1][i]).collect();
    Rotation {
        sides: vec![left, up, right, down],
        grid: t.clone(),
    }
}

fn generate_all_grids(t: &Grid) -> Vec<Grid> {
    let mut flipped = Vec::new();
    flipped.push(t.clone());
    flipped.push(flip_grid(&t));

    let mut all = Vec::new();
    for flip in flipped.iter() {
        let mut now = flip.clone();
        for i in 0..4 {
            all.push(now.clone());
            now = rotate_grid(&now);
        }
    }
    all
}

fn generate_all_rotations(t: &Grid) -> Vec<Rotation> {
    let all = generate_all_grids(t);
    all.iter().map(|t| read_rotation(&t)).collect()
}

// Index of tile + the index of rotation
type TilePos = (usize, usize);
type Solution = Vec<TilePos>;

fn to_rc(index: usize, dims: usize) -> (usize, usize) {
    (index / dims, index % dims)
}

fn to_index(r: usize, c: usize, dims: usize) -> usize {
    r * dims + c
}

fn check_placement(tiles: &Vec<Tile>, dims: usize, s: &mut Solution, index: usize) -> bool {
    let (cur_t, cur_r) = s[index];
    let cur_rot = &tiles[cur_t].rotations[cur_r];
    let (r, c) = to_rc(index, dims);
    if r > 0 {
        let prev = to_index(r - 1, c, dims);
        let (prev_t, prev_r) = s[prev];
        let prev_rot = &tiles[prev_t].rotations[prev_r];
        if cur_rot.sides[1] != prev_rot.sides[3] {
            return false;
        }
    }
    if c > 0 {
        let prev = to_index(r, c - 1, dims);
        let (prev_t, prev_r) = s[prev];
        let prev_rot = &tiles[prev_t].rotations[prev_r];
        if cur_rot.sides[0] != prev_rot.sides[2] {
            return false;
        }
    }
    true
}

fn solve(
    tiles: &Vec<Tile>,
    dims: usize,
    s: &mut Solution,
    next: usize,
    taken: &mut Vec<bool>,
) -> bool {
    if next >= s.len() {
        return true;
    }
    for (ti, tile) in tiles.iter().enumerate() {
        if taken[ti] {
            continue;
        }
        for (ri, rotation) in tile.rotations.iter().enumerate() {
            s[next] = (ti, ri);
            if !check_placement(tiles, dims, s, next) {
                continue;
            }
            taken[ti] = true;
            let ok = solve(tiles, dims, s, next + 1, taken);
            if ok {
                return true;
            }
            taken[ti] = false;
        }
    }

    false
}

fn parse_data(data: &String) -> Vec<Tile> {
    let tiles_str = split_string(data, "\n\n");

    let mut tiles = Vec::new();

    for tile in tiles_str.iter() {
        let lines = split_string(&tile, "\n");
        let line = &lines[0].clone();
        if line.is_empty() {
            continue;
        }
        let id: i64 = serde_scan::scan!("Tile {}:" <- line).unwrap();

        let mut other_lines = Vec::new();
        for i in 1..lines.len() {
            other_lines.push(lines[i].clone());
        }
        let pixels = to_vv_char(&other_lines);

        tiles.push(Tile {
            id: id,
            rotations: generate_all_rotations(&pixels),
        });
    }
    tiles
}

pub fn part1(data: &String) -> i64 {
    let tiles = parse_data(data);
    let dims = (tiles.len() as f64).sqrt().round() as usize;
    assert_eq!(dims * dims, tiles.len());

    let mut solution = vec![(0, 0); tiles.len()];
    let mut taken = vec![false; tiles.len()];

    let ok = solve(&tiles, dims, &mut solution, 0, &mut taken);
    assert!(ok);

    let mut res = 1;
    for (r, c) in iproduct!(vec![0, dims - 1], vec![0, dims - 1]) {
        let index = to_index(r, c, dims);
        res *= tiles[solution[index].0].id;
    }
    res
}

fn get_image(dims: usize, tiles: &Vec<Tile>, s: &Solution) -> Grid {
    let grid_size = tiles[0].rotations[0].grid.len();
    let full_size = (grid_size - 2) * dims;

    let mut full_grid = Vec::new();
    for i in 0..full_size {
        full_grid.push(vec!['?'; full_size]);
    }

    for (index, (ti, ri)) in s.iter().enumerate() {
        let rot = &tiles[*ti].rotations[*ri];
        let (r, c) = to_rc(index, dims);
        println!("");
        println!("Solution at {}, (r, c) = ({}, {})", index, r, c,);
        print_grid(&rot.grid);
        for local_r in 1..(grid_size - 1) {
            for local_c in 1..(grid_size - 1) {
                let global_r = r * (grid_size - 2) + (local_r - 1);
                let global_c = c * (grid_size - 2) + (local_c - 1);
                full_grid[global_r][global_c] = rot.grid[local_r][local_c];
            }
        }
    }
    full_grid
}

fn print_grid(img: &Grid) {
    for row in img.iter() {
        let to_print: String = row.iter().collect();
        println!("{}", to_print);
    }
}

fn update(r: usize, c: usize, p: &Grid, taken: &mut Vec<Vec<bool>>) {
    for dr in 0..p.len() {
        for dc in 0..p[dr].len() {
            let ir = r + dr;
            let ic = c + dc;
            if ir >= taken.len() || ic >= taken[ir].len() {
                unreachable!();
            }
            if p[dr][dc] == '#' {
                taken[ir][ic] = true;
            }
        }
    }
}

fn matches(img: &Grid, r: usize, c: usize, p: &Grid) -> bool {
    for dr in 0..p.len() {
        for dc in 0..p[dr].len() {
            let ir = r + dr;
            let ic = c + dc;
            if ir >= img.len() || ic >= img[ir].len() {
                return false;
            }
            if p[dr][dc] == '#' {
                if img[ir][ic] != '#' {
                    return false;
                }
            }
        }
    }
    return true;
}

fn find_sea_monsters(img: &Grid) -> (i64, i64) {
    let mut p = Vec::new();
    p.push("                  # ");
    p.push("#    ##    ##    ###");
    p.push(" #  #  #  #  #  #   ");

    let mut pattern = Vec::new();
    for s in p.iter() {
        pattern.push(to_v_char(s));
    }

    let mut taken = Vec::new();
    for row in img.iter() {
        taken.push(vec![false; row.len()]);
    }

    let mut num_sea = 0;
    for r in 0..img.len() {
        for c in 0..img[0].len() {
            if matches(img, r, c, &pattern) {
                num_sea += 1;
                update(r, c, &pattern, &mut taken);
            }
        }
    }

    let mut not_part = 0;
    for r in 0..img.len() {
        for c in 0..img[0].len() {
            if img[r][c] == '#' && taken[r][c] == false {
                not_part += 1;
            }
        }
    }

    (num_sea, not_part)
}

pub fn part2(data: &String) -> i64 {
    let tiles = parse_data(data);
    let dims = (tiles.len() as f64).sqrt().round() as usize;
    assert_eq!(dims * dims, tiles.len());

    let mut solution = vec![(0, 0); tiles.len()];
    let mut taken = vec![false; tiles.len()];

    let ok = solve(&tiles, dims, &mut solution, 0, &mut taken);
    assert!(ok);

    let img = get_image(dims, &tiles, &solution);
    // print_grid(&img);

    let all_imgs = generate_all_grids(&img);
    for cur_img in all_imgs.iter() {
        dbg!(find_sea_monsters(&cur_img));
    }

    -1
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // #[test]
    // fn test_part1() {
    //     let lines = read_main_input();
    //     assert_eq!(part1(&lines), -1);
    // }

    // #[test]
    // fn test_part2() {
    //     let lines = read_main_input();
    //     assert_eq!(part2(&lines), -1);
    // }
}

pub fn read_main_input() -> String {
    let input = fs::read_to_string("input/day20/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    // to_lines(&input)
    input
}
