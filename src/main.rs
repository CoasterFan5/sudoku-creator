mod greedy_remover;
mod grid;
mod remover;
mod solver;

use crate::{greedy_remover::greedy_remover, grid::Grid, remover::remover};
use rand::prelude::*;

use std::time::Instant;

/*
 * This function needs to take in a grid and return true or false if a number is placed, then call itself on the next cell
 */
fn fill_grid(grid: &mut Grid, cell_index: usize, rng: &mut ThreadRng) -> bool {
    if cell_index >= 81 {
        return true;
    }

    //let mut possible_values = get_valid_placements::get_possible_values(&grid_clone, cell_index);
    let mut values = grid.get_valid_placements(cell_index);

    if values.len() < 1 {
        return false;
    }

    values.shuffle(rng);

    for val in values {
        grid.place_value(cell_index, val);
        if fill_grid(grid, cell_index + 1, rng) {
            return true;
        }
    }
    grid.place_value(cell_index, 0b0);

    return false;
}

fn generate() -> Grid {
    let mut rng = rand::rng();
    let mut grid = Grid::new();

    let mut start = Instant::now();
    fill_grid(&mut grid, 0, &mut rng);
    let elapsed = start.elapsed().as_micros();
    println!("Generated raw in {elapsed} micros");
    println!("Starting remover...");
    start = Instant::now();
    greedy_remover(&mut grid);
    grid.display();
    println!("Removal done in {:?}", start.elapsed().as_micros());

    // no we have made a perfectly valid puzzle
    println!("{:?}", grid.values);
    return grid;
}

/* fn save_puzzle(g: &mut GenerationResult) -> serde_json::Result<()> {
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
} */

fn main() {
    //loop {
    generate();

    //let a = save_puzzle(puzzle);

    //}
}
