use rand::prelude::*;

fn display_grid(grid: [[i32; 9]; 9]) {
    for row_index in 0..grid.len() {
        for col_index in 0..grid[row_index].len() {
            print!("{:?}", grid[row_index][col_index])
        }
        println!()
    }
}

fn pick_number(options: &mut Vec<i32>) -> i32 {
    let mut rng = rand::rng();
    options.shuffle(&mut rng);
    let item = options[0];
    options.remove(0);
    // generate a random number 1-9,  taking into account the not allowed values passed
    return item;
}

fn generate() -> &'static str {
    let mut grid: [[i32; 9]; 9] = [[0; 9]; 9];

    // generate the first row of the grid
    let v = &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for (_, item) in grid[0].iter_mut().enumerate() {
        // Get all of the numbers in the current row
        *item = pick_number(v);
    }

    for row_index in 1..9 {
        let row_allowed_numbers = &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for col_index in 0..9 {
            let allowed_numbers = &mut row_allowed_numbers.clone();

            // allowed numbers for columns
            for r in 0..9 {
                let col_item = grid[r][col_index];
                allowed_numbers.retain(|&x| x != col_item);
            }

            // allowed numbers for the grid major (3x3)
            let major_grid_row_offset = (row_index / 3) * 3;
            let major_grid_col_offset = (col_index / 3) * 3;
            // get all numbers in the major grid
            for major_minor_row in 0..3 {
                for major_minor_col in 0..3 {
                    let major_minor_item = grid[major_minor_row + major_grid_row_offset]
                        [major_minor_col + major_grid_col_offset];
                    println!("Major minor item: {:?}", major_minor_item);
                    allowed_numbers.retain(|&x| x != major_minor_item);
                }
            }

            // Find all the allowed numbers for this cell, and pick one!
            println!("{:?}", allowed_numbers);
            display_grid(grid);
            let picked_number = pick_number(allowed_numbers);
            row_allowed_numbers.retain(|&x| x != picked_number);
            grid[row_index][col_index] = picked_number;
        }
    }

    return "Hello!";
}

fn main() {
    let puzzle = generate();
    println!("{puzzle}");
}
