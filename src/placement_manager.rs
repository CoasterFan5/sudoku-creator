/*
 * This function does two things
 * It creates valid placements that are simple to update for each 0 index
 * It sorts the empty indexes by candidates
 */

use std::vec;

use crate::grid::Grid;

struct PlacementManager {
    valid_placements: [u16; 81],
    zero_indexes: Vec<usize>,
}

impl PlacementManager {
    fn new(grid: Grid) -> PlacementManager {
        // create the valid placements

        let manager = PlacementManager {
            valid_placements: [0b0000000000000000; 81],
            zero_indexes: vec![],
        };

        for i in 0..81 {
            let row_index = i / 9;
            let col_index = i % 9;
        }
    }
}
