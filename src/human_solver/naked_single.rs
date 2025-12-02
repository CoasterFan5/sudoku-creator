use crate::{grid::Grid, human_solver::solve_details::SolveDetails};

/*
 * For any given
 */
pub fn solve_naked_single(grid: &mut Grid) -> Option<SolveDetails> {
    for i in 0..81 {
        let valid_placements = grid.get_valid_placements(i);
        let value = grid.get_value(i);
        if value != 0 {
            continue;
        }
        if valid_placements.len() == 1 {
            grid.place_value(i, valid_placements[0]);
            return Some(SolveDetails {
                true_index: i,
                value: valid_placements[0],
            });
        };
    }
    return None;
}
