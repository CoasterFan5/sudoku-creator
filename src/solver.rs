/*
 * Take in a non-mut grid return true if it's solvable and false if not
 */

use crate::grid::Grid;

pub fn solve_game(grid: &mut Grid) -> bool {
    // create an index vec
    let mut zero_index_vec: Vec<usize> = vec![];
    for i in 0..81 {
        if grid.get_value(i) == 0 {
            zero_index_vec.push(i);
        }
    }

    // create placement manager (this will help with calculations heavily)

    let solution_count = solver(grid, 0, &zero_index_vec, 0);
    return solution_count == 1;
}

/*
 * iterate through the grid recursively, if we come across a 0 (an empty tile) we gather all the possible values
 * and fill it with one and attempt to fill the rest of the grid, returing the result
 * of a recursive call to this function, if there are none we return None
 */
fn solver(grid: &mut Grid, zero_index: usize, zero_index_vec: &Vec<usize>, sol_count: i32) -> i32 {
    if sol_count >= 2 {
        return 0;
    }

    if zero_index >= zero_index_vec.len() {
        return 1;
    }

    let index = zero_index_vec[zero_index];

    let value = grid.get_value(index);

    if value != 0 {
        return solver(grid, zero_index + 1, zero_index_vec, 0);
    } else {
        // get the possible values
        let possible_values = grid.get_valid_placements(index);

        if possible_values.len() < 1 {
            return 0;
        }

        let mut sol = sol_count;
        for v in possible_values {
            grid.place_value(index, v);
            sol += solver(grid, zero_index + 1, zero_index_vec, sol);
            if sol >= 2 {
                grid.place_value(index, 0b0);
                break;
            }
        }
        grid.place_value(index, 0b0);

        return sol;
    }
}
