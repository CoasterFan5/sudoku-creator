use crate::grid::Grid;

pub fn get_possible_values(grid: Grid, cell_index: usize) -> Vec<i32> {
    let mut base = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // remove all existing numbers in the row
    let row_index = cell_index / 9;
    for t_col_index in 0..9 {
        base.retain(|&x| x != grid[row_index][t_col_index])
    }

    // remove all existing numbers in the column
    let col_index = cell_index % 9;
    for t_row_index in 0..9 {
        base.retain(|&x| x != grid[t_row_index][col_index]);
    }

    // remove all existing numbers in the major grid (the 3x3)
    let major_row_offset = (row_index / 3) * 3;
    let major_col_offset = (col_index / 3) * 3;
    for t_row_index_raw in 0..3 {
        let true_row_index = major_row_offset + t_row_index_raw;
        for t_col_index_raw in 0..3 {
            let true_col_index = major_col_offset + t_col_index_raw;
            base.retain(|&x| x != grid[true_row_index][true_col_index]);
        }
    }
    // iterate

    return base;
}
