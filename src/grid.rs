use crate::grid_utils::{
    col_index_from_true_index, house_index_from_row_col, row_index_from_true_index,
};

pub struct Grid {
    pub values: [u16; 81],
    row_valid_items: [u16; 9],
    col_valid_items: [u16; 9],
    house_valid_items: [u16; 9],
}

pub type CellValue = u16;

impl Grid {
    pub fn new() -> Grid {
        return Grid {
            values: [0b0; 81],
            row_valid_items: [0b111111111; 9],
            col_valid_items: [0b111111111; 9],
            house_valid_items: [0b111111111; 9],
        };
    }

    pub fn place_value(&mut self, cell_index: usize, value: CellValue) {
        let row_index = row_index_from_true_index(cell_index);
        let col_index = col_index_from_true_index(cell_index);
        let house_index = house_index_from_row_col(row_index, col_index);

        let old_value = self.values[cell_index];

        self.row_valid_items[row_index] = self.row_valid_items[row_index] | old_value;
        self.row_valid_items[row_index] = self.row_valid_items[row_index] & !value;
        self.col_valid_items[col_index] = self.col_valid_items[col_index] | old_value;
        self.col_valid_items[col_index] = self.col_valid_items[col_index] & !value;
        self.house_valid_items[house_index] = self.house_valid_items[house_index] | old_value;
        self.house_valid_items[house_index] = self.house_valid_items[house_index] & !value;

        self.values[cell_index] = value;
    }

    pub fn get_valid_placements(&self, cell_index: usize) -> Vec<CellValue> {
        if self.get_value(cell_index) == 0 {
            return self.get_valid_placements_with_row_col(cell_index / 9, cell_index % 9);
        } else {
            return vec![];
        }
    }

    pub fn get_valid_placements_with_row_col(
        &self,
        row_index: usize,
        col_index: usize,
    ) -> Vec<u16> {
        let house_index = (row_index / 3) * 3 + (col_index / 3);

        let mut out: Vec<u16> = vec![];
        let mask = self.row_valid_items[row_index]
            & self.col_valid_items[col_index]
            & self.house_valid_items[house_index];

        for i in 0..9 {
            if (1 << i) & mask != 0 {
                out.push(1 << i);
            }
        }

        return out;
    }

    pub fn get_value(&self, cell_index: usize) -> u16 {
        return self.values[cell_index];
    }

    pub fn display(&self) {
        for r in 0..9 {
            for c in 0..9 {
                let real_index = r * 9 + c;

                let value = self.get_value(real_index);
                let digit = if value == 0 {
                    0
                } else {
                    (value.trailing_zeros() as u8) + 1
                };
                print!("{digit} ")
            }
            println!()
        }
        println!("-- -- -- -- -- -- -- ")
    }
}
