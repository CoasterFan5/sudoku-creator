use std::time::{Duration, Instant};

use crate::{get_valid_placements::get_possible_values, grid::Grid};

/*
 * Take in a non-mut grid return true if it's solvable and false if not
 */

pub fn solve_game(grid: Grid) -> bool {
    let grid_size = grid.len() * grid[0].len();

    // create an index vec
    let mut zero_index_vec: Vec<usize> = vec![];
    for i in 0..grid_size {
        if grid[i / 9][i % 9] == 0 {
            zero_index_vec.push(i);
        }
    }

    // create placement manager (this will help with calculations heavily)

    let s = Instant::now();
    let solution_count = solver(grid, 0, &zero_index_vec, 0);
    let elapsed = s.elapsed().as_micros();
    eprintln!("Solve in {elapsed}micros");
    return solution_count == 1;
}

/*
 * iterate through the grid recursively, if we come across a 0 (an empty tile) we gather all the possible values
 * and fill it with one and attempt to fill the rest of the grid, returing the result
 * of a recursive call to this function, if there are none we return None
 */
fn solver(mut grid: Grid, zero_index: usize, zero_index_vec: &Vec<usize>, sol_count: i32) -> i32 {
    if sol_count >= 2 {
        return 0;
    }

    if zero_index >= zero_index_vec.len() {
        return 1;
    }

    let index = zero_index_vec[zero_index];
    let row_index = index / 9;
    let col_index = index % 9;

    let value = grid[row_index][col_index];

    if value != 0 {
        return solver(grid, zero_index + 1, zero_index_vec, 0);
    } else {
        // get the possible values
        let possible_values = get_possible_values(&grid, index);

        if possible_values.len() < 1 {
            return 0;
        }

        let mut sol = sol_count;
        for v in possible_values {
            grid[row_index][col_index] = v;
            sol += solver(grid, zero_index + 1, zero_index_vec, sol);
            if sol >= 2 {
                break;
            }
        }

        return sol;
    }
}
