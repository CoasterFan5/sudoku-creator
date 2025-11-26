use std::cmp::max;

use rand::seq::SliceRandom;

use crate::{grid::Grid, solver::solve_game};

/**
 * Recursive removal system
 * `grid` - A non-mutable grid
 * Returns a grid with the maximum possible cells removed
 */
pub fn remover(grid: &mut Grid) -> i32 {
    let mut rng = rand::rng();
    let opps_count = &mut 0;

    // generate a random vec of indexes
    let mut grid_index_vec: Vec<usize> = vec![];
    let max_size = &mut 0;
    for i in 0..81 {
        grid_index_vec.push(i);
    }
    grid_index_vec.shuffle(&mut rng);

    // the first step is going to be making a copy of the grid
    let max = do_removal(grid, &grid_index_vec, 0, 0, max_size, opps_count);
    println!("Max: {max}");
    return max;
}

fn do_removal(
    g: &mut Grid,
    grid_index_vec: &Vec<usize>,
    grid_index_vec_pos: usize,
    tree_max: i32,
    max_size: &mut i32,
    opps_count: &mut i32,
) -> i32 {
    *opps_count += 1;
    if (*opps_count % 1000 == 0) {
        println!("{opps_count} opps");
    }
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

    let index = grid_index_vec[grid_index_vec_pos];

    let value = g.get_value(index);
    let mut with_removal = 0;
    g.place_value(index, 0);

    if solve_game(g) {
        with_removal = 1 + do_removal(
            g,
            &grid_index_vec,
            grid_index_vec_pos + 1,
            tree_max + 1,
            max_size,
            opps_count,
        );

        if tree_max > *max_size {
            println!("{tree_max}/64 in {opps_count} opps");
            *max_size = tree_max
        }
    }

    // revert
    g.place_value(index, value);
    let with_no_removal = do_removal(
        g,
        &grid_index_vec,
        grid_index_vec_pos + 1,
        tree_max,
        max_size,
        opps_count,
    );

    return max(with_removal, with_no_removal);
}
