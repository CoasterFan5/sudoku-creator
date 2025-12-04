use rand::seq::SliceRandom;

use crate::{grid::Grid, solver::solve_game};

pub fn greedy_remover(g: &mut Grid) -> &mut Grid {
    // get all the indexes
    let mut rng = rand::rng();
    let mut index_vec: Vec<usize> = vec![];

    for i in 0..81 {
        index_vec.push(i);
    }

    index_vec.shuffle(&mut rng);

    for index in &index_vec {
        let old_value = g.get_value(*index);
        if old_value == 0 {
            continue;
        }
        g.place_value(*index, 0);
        if !solve_game(g) {
            g.place_value(*index, old_value);
        }
    }

    return g;
}
