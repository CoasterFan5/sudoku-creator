use crate::{
    grid::Grid,
    grid_utils::{
        col_index_from_true_index, house_index_from_true_index, row_index_from_true_index,
        true_index_from_col_index, true_index_from_house_index, true_index_from_row_index,
    },
    human_solver::solve_details::SolveDetails,
};

/*
 * If all candidates of a digit in a house are confined to a single row or column,
 * it eliminates that digit as a candidate from all other cells in the row or column
 */
pub fn solve_locked_candidate_pointing(grid: &mut Grid) -> Option<SolveDetails> {
    // we need to check if all the candidates of a given digit within a house are confined to a single row or column
    // loop through each house
    for house_index in 0..9 {
        // go through each row and extract the candidates
        let mut row_candidate_vecs: [Vec<u16>; 3] = [vec![], vec![], vec![]];
        let mut col_candidate_vecs: [Vec<u16>; 3] = [vec![], vec![], vec![]];
        for item_house_position in 0..9 {
            let true_index = true_index_from_house_index(house_index, item_house_position);
            let row_index = row_index_from_true_index(true_index) % 3; // we dont actually care about the real value, just the value within this house
            let col_index = col_index_from_true_index(true_index) % 3;
            row_candidate_vecs[row_index].append(&mut grid.get_valid_placements(true_index));
            col_candidate_vecs[col_index].append(&mut grid.get_valid_placements(true_index));
        }
        // now we need to loop through each row andc check if any other row contains that value
        for r in 0..3 {
            let true_row_index = (house_index / 3) * 3 + r;
            for candidate in &row_candidate_vecs[r] {
                let mut good = true;
                for r_other in 0..3 {
                    if r_other == r {
                        continue;
                    }
                    if row_candidate_vecs[r_other].contains(&candidate) {
                        good = false;
                    }
                }
                let mut log_statement = String::new();
                let mut found = false;
                if good {
                    // now we need to check the rest of the row
                    for row_item_index in 0..9 {
                        let row_item_true_index =
                            true_index_from_row_index(true_row_index, row_item_index);
                        let row_item_house_index = house_index_from_true_index(row_item_true_index);
                        if row_item_house_index == house_index {
                            continue;
                        }
                        if grid
                            .get_valid_placements(row_item_true_index)
                            .contains(candidate)
                        {
                            found = true;
                            grid.deny_cell_candidate(row_item_true_index, *candidate);
                            let cell_col = col_index_from_true_index(row_item_true_index);
                            log_statement = format!(
                                "{log_statement} r{:?}c{:?}<{:?}>",
                                true_row_index + 1,
                                cell_col + 1,
                                candidate.trailing_zeros() + 1
                            );
                        }
                    }
                }
                if found {
                    return Some(SolveDetails {
                        true_index: 0,
                        log_statement: format!("r{:?}{log_statement}", true_row_index + 1),
                    });
                }
            }
        }

        for c in 0..3 {
            let true_col_index = (house_index % 3) * 3 + c;
            for candidate in &row_candidate_vecs[c] {
                let mut good = true;
                for c_other in 0..3 {
                    if c_other == c {
                        continue;
                    }
                    if col_candidate_vecs[c_other].contains(&candidate) {
                        good = false;
                    }
                }
                let mut log_statement = String::new();
                let mut found = false;
                if good {
                    // now we need to check the rest of the row
                    for col_item_index in 0..9 {
                        let col_item_true_index =
                            true_index_from_col_index(true_col_index, col_item_index);
                        let col_item_house_index = house_index_from_true_index(col_item_true_index);
                        if col_item_house_index == house_index {
                            continue;
                        }
                        if grid
                            .get_valid_placements(col_item_true_index)
                            .contains(candidate)
                        {
                            found = true;
                            grid.deny_cell_candidate(col_item_true_index, *candidate);
                            let cell_row = row_index_from_true_index(col_item_true_index);
                            log_statement = format!(
                                "{log_statement} r{:?}c{:?}<{:?}>",
                                cell_row + 1,
                                true_col_index + 1,
                                candidate.trailing_zeros() + 1
                            );
                        }
                    }
                }
                if found {
                    return Some(SolveDetails {
                        true_index: 0,
                        log_statement: format!("c{:?}{log_statement}", true_col_index + 1),
                    });
                }
            }
        }
    }

    return None;
}

#[test]
fn test_locked_candidate_pointing_row() {
    let g = &mut Grid::new();
    g.load_str("...7.....9.45.187...6.39..5.1.9.532.543.72.9.26931..5.3..29.5...92153........7...");
    g.display();

    match solve_locked_candidate_pointing(g) {
        Some(_) => {
            assert!(true)
        }
        None => {
            assert!(false)
        }
    }
}

#[test]
fn test_locked_candidate_pointing_col() {
    let g = &mut Grid::new();
    g.load_str("...6.4.....18294..42...5.9.36.792.41.4..56.7.2.74.8.69.7.9.1.2...42837....25.79..");
    g.display();

    match solve_locked_candidate_pointing(g) {
        Some(_) => {
            assert!(true)
        }
        None => {
            assert!(false)
        }
    }
}

#[test]
fn locked_candidate_pointing_removes_candidates() {
    let g = &mut Grid::new();
    g.load_str("...19765.1678..2.99...261.7...762.9..78.1.32..9..837....6.7......96.147.73.2489..");
    solve_locked_candidate_pointing(g);

    assert!(!g.get_valid_placements(21).contains(&(1 << (5 - 1))));
    assert!(g.get_valid_placements(21).contains(&(1 << (4 - 1))));
    assert!(g.get_valid_placements(21).contains(&(1 << (3 - 1))));
}

#[test]
fn test_false_locked_candidate_pointing() {
    let g = &mut Grid::new();
    g.load_str("185724963934561872726839415.1.945326.4.672198269318754378296541492153687651487239");

    match solve_locked_candidate_pointing(g) {
        Some(_) => {
            assert!(false)
        }
        None => {
            assert!(true)
        }
    }
}
