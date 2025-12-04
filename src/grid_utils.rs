pub fn row_index_from_true_index(true_index: usize) -> usize {
    return true_index / 9;
}

pub fn col_index_from_true_index(true_index: usize) -> usize {
    return true_index % 9;
}

pub fn house_index_from_row_col(row_index: usize, col_index: usize) -> usize {
    return (row_index / 3) * 3 + (col_index / 3);
}

/**
 * Prefer `house_index_from_row_col` as this removes 2 calculations.
 */
pub fn house_index_from_true_index(true_index: usize) -> usize {
    let row_index = row_index_from_true_index(true_index);
    let col_index = col_index_from_true_index(true_index);
    return house_index_from_row_col(row_index, col_index);
}

/**
 * Get the index of a cell within a row
 * `row_index` represents the row in the grid
 * `row_position` represents the position within the row
 */
pub fn true_index_from_row_index(row_index: usize, row_position: usize) -> usize {
    let true_index = row_index * 9 + row_position;
    return true_index;
}

/**
 * Get the index of a cell within a column
 * `col_index` represents the column in the grid
 * `col_position` represents the position within the column
 */
pub fn true_index_from_col_index(col_index: usize, col_position: usize) -> usize {
    let true_index = col_index + col_position * 9;
    return true_index;
}

/**
 * Get the index of a cell within a house
 * `house_index` represents the house in the grid where 0 is the top left house and 8 is the bottom right house
 * `house_position` represents the position within the house where 0 is the top left and 8 is the bottom right
 */
pub fn true_index_from_house_index(house_index: usize, house_position: usize) -> usize {
    let row_index = (house_index / 3) * 3 + (house_position / 3);
    let col_index = (house_index % 3) * 3 + house_position % 3;
    let true_index = row_index * 9 + col_index;
    return true_index;
}
