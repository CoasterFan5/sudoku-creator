use crate::{
    grid::Grid,
    grid_utils::{col_index_from_true_index, row_index_from_true_index},
    human_solver::solve_details::SolveDetails,
};

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
            let row_index = row_index_from_true_index(i);
            let col_index = col_index_from_true_index(i);
            return Some(SolveDetails {
                true_index: i,
                log_statement: format!(
                    "r{:?}c{:?}={:?}",
                    row_index + 1,
                    col_index + 1,
                    valid_placements[0].trailing_zeros() + 1
                ),
            });
        };
    }
    return None;
}

#[test]
fn test_naked_single() {
    let grid = &mut Grid::new();
    grid.load([
        8, 2, 9, 1, 6, 3, 5, 7, 4, 7, 1, 3, 5, 4, 9, 8, 2, 6, 5, 6, 4, 8, 2, 7, 1, 3, 9, 9, 4, 1,
        3, 5, 8, 2, 6, 7, 3, 7, 5, 6, 9, 2, 4, 8, 1, 6, 8, 2, 7, 1, 4, 9, 5, 3, 2, 3, 8, 9, 7, 1,
        6, 4, 5, 4, 9, 6, 0, 3, 5, 7, 1, 8, 1, 5, 7, 4, 8, 6, 3, 9, 2,
    ]);
    match solve_naked_single(grid) {
        None => {
            assert!(false)
        }
        Some(_) => {
            assert!(true)
        }
    }
}

#[test]
fn test_naked_single_fail() {
    let grid = &mut Grid::new();
    grid.load_str(
        "12876..95.7...86.1.4.9...8...1.7695.....5.....5218.....1...7.6.3.56...7.267.....9",
    );
    match solve_naked_single(grid) {
        None => {
            assert!(true)
        }
        Some(_) => {
            assert!(false)
        }
    }
}
