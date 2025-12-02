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
                    value: valid,
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
                    value: valid,
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
                    value: valid,
                });
            }
        }
    }

    return None;
}
