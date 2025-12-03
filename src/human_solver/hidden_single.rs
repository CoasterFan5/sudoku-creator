use crate::{
    grid::Grid,
    grid_utils::{
        col_index_from_true_index, house_index_from_row_col, row_index_from_true_index,
        true_index_from_col_index, true_index_from_house_index, true_index_from_row_index,
    },
    human_solver::solve_details::SolveDetails,
};

pub fn solve_hidden_single(grid: &mut Grid) -> Option<SolveDetails> {
    for i in 0..81 {
        let valid_placements = grid.get_valid_placements(i);
        let value = grid.get_value(i);
        if value != 0 {
            continue;
        }
        // to check for a hidden single, we need to check if there is a candidate in this cell that is not a candidate anywhere else in the row, column, or house
        // row check
        let row_index = row_index_from_true_index(i);
        let col_index = col_index_from_true_index(i);
        let house_index = house_index_from_row_col(row_index, col_index);

        // for each valid placement
        for valid in valid_placements {
            // check if this valid placement exists anywhere
            // Row check
            let mut other_row_has = false;
            for sub_index in 0..9 {
                let true_row_index = true_index_from_row_index(row_index, sub_index);
                if true_row_index != i {
                    let v = grid.get_valid_placements(true_row_index);
                    if v.contains(&valid) {
                        other_row_has = true;
                    }
                }
            }
            if !other_row_has {
                grid.place_value(i, valid);
                return Some(SolveDetails {
                    true_index: i,
                    log_statement: format!(
                        "r{:?}c{:?}={:?} (row)",
                        row_index + 1,
                        col_index + 1,
                        valid.trailing_zeros() + 1
                    ),
                });
            }
            // Col check
            let mut other_col_has = false;
            for sub_index in 0..9 {
                let true_col_index = true_index_from_col_index(col_index, sub_index);
                if true_col_index != i {
                    let v = grid.get_valid_placements(true_col_index);
                    if v.contains(&valid) {
                        other_col_has = true;
                    }
                }
            }
            if !other_col_has {
                grid.place_value(i, valid);
                return Some(SolveDetails {
                    true_index: i,
                    log_statement: format!(
                        "r{:?}c{:?}={:?} (col)",
                        row_index + 1,
                        col_index + 1,
                        valid.trailing_zeros() + 1
                    ),
                });
            }
            //House check
            let mut other_house_has = false;
            for sub_index in 0..9 {
                let true_house_index = true_index_from_house_index(house_index, sub_index);
                if true_house_index != i {
                    let v = grid.get_valid_placements(true_house_index);
                    if v.contains(&valid) {
                        other_house_has = true;
                    }
                }
            }
            if !other_house_has {
                grid.place_value(i, valid);
                return Some(SolveDetails {
                    true_index: i,
                    log_statement: format!(
                        "r{:?}c{:?}={:?} (house)",
                        row_index + 1,
                        col_index + 1,
                        valid.trailing_zeros() + 1
                    ),
                });
            }
        }
    }

    return None;
}

#[test]
fn test_hidden_single_row() {
    let grid = &mut Grid::new();
    grid.load_str(
        ".....3.7919427....7....9....5.89271..4.157....71634.8....7....6.1.9453274.73.....",
    );
    match solve_hidden_single(grid) {
        None => {
            assert!(false)
        }
        Some(_) => {
            assert!(true)
        }
    }
}

#[test]
fn test_hidden_single_col() {
    let grid = &mut Grid::new();
    grid.load_str(
        "......63.7.31628...56483......6.75..827....69.658..........671.17495832663.......",
    );
    match solve_hidden_single(grid) {
        None => {
            assert!(false)
        }
        Some(_) => {
            assert!(true)
        }
    }
}

#[test]
fn test_hidden_single_house() {
    let grid = &mut Grid::new();
    grid.load_str(
        "9...4.8.1..713.......5..2..574.2....6.841.9.7....7...41623.4...7..2614..843.....2",
    );
    match solve_hidden_single(grid) {
        None => {
            assert!(false)
        }
        Some(_) => {
            assert!(true)
        }
    }
}

#[test]
fn test_hidden_single_fail() {
    let grid = &mut Grid::new();
    grid.load_str(
        "9...4.8.1..713.......59.2..574.2....6.841.9.7....7...4162384...7..2614..843.5...2",
    );
    match solve_hidden_single(grid) {
        None => {
            assert!(true)
        }
        Some(_) => {
            assert!(false)
        }
    }
}
