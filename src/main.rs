mod get_valid_placements;
mod grid;
mod solver;

use crate::solver::solve_game;
use grid::Grid;
use rand::prelude::*;
use serde::Serialize;
use serde_json;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Instant;

fn random_one_nine() -> usize {
    let mut rng = rand::rng();
    let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    digits.shuffle(&mut rng);
    return digits[0];
}

/*
 * This function needs to take in a grid and return true or false if a number is placed, then call itself on the next cell
 */
fn fill_grid(grid: &mut Grid, grid_state: Grid, cell_index: usize) -> bool {
    let mut grid_clone = grid_state.clone();
    let total_size = grid.len() * grid[0].len();

    if cell_index >= total_size {
        *grid = grid_state;
        return true;
    }

    let mut rng = rand::rng();
    let mut possible_values = get_valid_placements::get_possible_values(grid_clone, cell_index);

    if possible_values.len() < 1 {
        return false;
    }

    possible_values.shuffle(&mut rng);
    for _ in 0..possible_values.len() {
        grid_clone[cell_index / 9][cell_index % 9] = possible_values.remove(0);
        // remove that possible value we dont need it again

        if fill_grid(grid, grid_clone, cell_index + 1) {
            return true;
        }
    }

    return false;
}

#[derive(Serialize, Hash)]
struct GenerationResult {
    squares: usize,
    solution: Grid,
    puzzle: Grid,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn generate() -> GenerationResult {
    let mut rng = rand::rng();
    let grid: &mut Grid = &mut [[0; 9]; 9];

    let start = Instant::now();
    fill_grid(grid, grid.clone(), 0);
    let elapsed = start.elapsed().as_micros();

    // no we have made a perfectly valid puzzle
    println!("Generated raw in {elapsed} micros");

    let mut cells = grid[0].len() * grid.len();
    let solution = grid.clone();
    // create a vec of the indexes

    let mut grid_index_vec: Vec<usize> = vec![];
    for i in 0..cells {
        grid_index_vec.push(i);
    }
    grid_index_vec.shuffle(&mut rng);
    println!("{:?}", grid_index_vec);

    loop {
        let mut has_removed = false;
        for cell_index in &grid_index_vec {
            let row_index = cell_index / 9;
            let col_index = cell_index % 9;

            let cell_value = grid[row_index][col_index];

            if cell_value == 0 {
                continue;
            }

            grid[row_index][col_index] = 0;

            if !solve_game(*grid) {
                grid[row_index][col_index] = cell_value
            } else {
                cells -= 1;
                has_removed = true;
            }
        }

        if !has_removed {
            break;
        }
    }

    let full_elapsed = start.elapsed().as_micros();

    println!("Kept {cells} items, done in {full_elapsed} micros");
    return GenerationResult {
        squares: cells,
        solution,
        puzzle: *grid,
    };
}

fn save_puzzle(g: &mut GenerationResult) -> serde_json::Result<()> {
    let p_json = serde_json::to_string(g)?;

    let h = calculate_hash(g);
    let directory_path = format!("./puzzles/{:?}", g.squares);
    let file_path = format!("{directory_path}/{:?}.puzzle", h);

    println!("{file_path}");

    match fs::create_dir_all(&directory_path) {
        Err(e) => println!("{:?}", e),
        Ok(_) => {}
    };

    match fs::write(&file_path, p_json) {
        Err(e) => println!("{:?}", e),
        Ok(_) => {}
    };

    return Ok(());
}

fn main() {
    loop {
        let puzzle = &mut generate();

        let a = save_puzzle(puzzle);
        if a.is_err() {
            println!("A error")
        }
    }
}
