use std::cmp::max;

use rand::seq::SliceRandom;

use crate::{grid::Grid, solver::solve_game};

/**
 * Recursive removal system
 * `grid` - A non-mutable grid
 * Returns a grid with the maximum possible cells removed
 */
pub fn remover(grid: Grid) -> i32 {
    let mut rng = rand::rng();
    let cell_count = grid.len() * grid[0].len();

    // generate a random vec of indexes
    let mut grid_index_vec: Vec<usize> = vec![];
    let max_size = &mut 0;
    for i in 0..cell_count {
        grid_index_vec.push(i);
    }
    grid_index_vec.shuffle(&mut rng);

    // the first step is going to be making a copy of the grid
    let max = do_removal(grid, &grid_index_vec, 0, 0, max_size);
    println!("Max: {max}");
    return max;
}

fn do_removal(
    mut g: Grid,
    grid_index_vec: &Vec<usize>,
    grid_index_vec_pos: usize,
    tree_max: i32,
    max_size: &mut i32,
) -> i32 {
    // see if we can remove the first index
    //println!("{grid_index_vec_pos}");

    if grid_index_vec_pos >= grid_index_vec.len() {
        //println!("Hit an end");
        return 0;
    }

    // are we even able to beat our current max size?
    if tree_max + grid_index_vec.len() as i32 - grid_index_vec_pos as i32 <= *max_size {
        return 0;
    }

    let row_index = grid_index_vec[grid_index_vec_pos] / 9;
    let col_index = grid_index_vec[grid_index_vec_pos] % 9;
    let old_value = g[row_index][col_index];

    let mut with_removal = 0;
    g[row_index][col_index] = 0;
    if solve_game(g) {
        with_removal = 1 + do_removal(
            g,
            &grid_index_vec,
            grid_index_vec_pos + 1,
            tree_max + 1,
            max_size,
        );

        if tree_max > *max_size {
            println!("{tree_max}/64");
            *max_size = tree_max
        }
    }

    g[row_index][col_index] = old_value;
    let with_no_removal = do_removal(
        g,
        &grid_index_vec,
        grid_index_vec_pos + 1,
        tree_max,
        max_size,
    );

    return max(with_removal, with_no_removal);
}
