mod greedy_remover;
mod grid;
mod grid_utils;
mod human_solver;
mod remover;
mod solver;

use crate::{
    greedy_remover::greedy_remover,
    grid::Grid,
    human_solver::{HumanSolverResponse, human_solver},
    solver::solve_game,
};
use rand::prelude::*;

use std::time::Instant;

struct GenerationResult {
    human_solver_response: HumanSolverResponse,
    unsolved_puzzle: Grid,
    solved_puzzle: Grid,
}

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

fn generate() -> GenerationResult {
    let mut rng = rand::rng();
    let mut grid = Grid::new();

    fill_grid(&mut grid, 0, &mut rng);

    let start = Instant::now();

    greedy_remover(&mut grid);

    println!(
        "Greedy removal done in {:?} micros",
        start.elapsed().as_micros()
    );

    if !solve_game(&mut grid) {
        println!("Game invalid")
    }

    let unsolved = grid.get_clone();

    // we need to get the score
    let score = human_solver(&mut grid);
    // no we have made a perfectly valid puzzle
    return GenerationResult {
        human_solver_response: score,
        unsolved_puzzle: unsolved,
        solved_puzzle: grid,
    };
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
    let mut high_score = -1;
    let mut best: Option<GenerationResult> = None;
    for i in 0..1000 {
        let res = generate();
        if res.human_solver_response.score >= high_score {
            high_score = res.human_solver_response.score;
            best = Some(res);
        };
        println!("Done with {i}")
    }

    println!("Best score: {high_score}");
    match best {
        None => {
            println!("Best was never initalzied")
        }
        Some(v) => {
            for item in v.human_solver_response.techniques_used {
                println!("{item}")
            }
            v.unsolved_puzzle.display();
            v.solved_puzzle.display();
        }
    }

    //let a = save_puzzle(puzzle);

    //}
}
