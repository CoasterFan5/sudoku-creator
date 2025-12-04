use crate::grid_utils::{
    col_index_from_true_index, house_index_from_row_col, row_index_from_true_index,
};

pub struct Grid {
    pub values: [u16; 81],
    cell_masks: [u16; 81],
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
            cell_masks: [0b111111111; 81],
        };
    }

    #[allow(dead_code)]
    pub fn load(&mut self, values: [usize; 81]) {
        for (index, item) in values.iter().enumerate() {
            if *item == 0 {
                self.place_value(index, 0);
            } else {
                self.place_value(index, 1 << (*item - 1));
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_values(&self) -> [u32; 81] {
        let mut new_vals: [u32; 81] = [0; 81];
        for i in 0..81 {
            let v = self.get_value(i);
            if v == 0 {
                new_vals[i] = 0;
            } else {
                new_vals[i] = v.trailing_zeros() + 1;
            }
        }
        return new_vals;
    }

    #[allow(dead_code)]
    pub fn load_str(&mut self, s: &'static str) -> bool {
        let vals: Vec<char> = s.chars().collect();
        let mut items: [usize; 81] = [0; 81];
        for i in 0..81 {
            let val = vals[i].to_digit(10);
            match val {
                Some(v) => {
                    items[i] = v as usize;
                }
                None => items[i] = 0,
            }
        }
        self.load(items);
        return true;
    }

    /**
     * This can not be reversed
     */
    pub fn deny_cell_candidate(&mut self, true_index: usize, v: CellValue) {
        self.cell_masks[true_index] = self.cell_masks[true_index] & !v;
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
        let true_index = (row_index * 9) + col_index;

        let mut out: Vec<u16> = vec![];
        let mask = self.row_valid_items[row_index]
            & self.col_valid_items[col_index]
            & self.house_valid_items[house_index]
            & self.cell_masks[true_index];

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

    pub fn is_solved(&self) -> bool {
        for i in 0..81 {
            if self.get_value(i) == 0 {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn test_grid_load_str() {
    let g = &mut Grid::new();
    g.load_str("2...38.6.3....2.....5..6238..4.2..53952.....673..1.4.282.7..3........829.9.28...7");
    g.display();
    assert!(
        g.get_values()
            == [
                2, 0, 0, 0, 3, 8, 0, 6, 0, 3, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 5, 0, 0, 6, 2, 3, 8, 0,
                0, 4, 0, 2, 0, 0, 5, 3, 9, 5, 2, 0, 0, 0, 0, 0, 6, 7, 3, 0, 0, 1, 0, 4, 0, 2, 8, 2,
                0, 7, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 8, 2, 9, 0, 9, 0, 2, 8, 0, 0, 0, 7
            ]
    )
}

#[test]
fn test_grid_load_str_equals_grid_load() {
    let g = &mut Grid::new();
    let g2 = &mut Grid::new();

    g.load([
        2, 0, 0, 0, 3, 8, 0, 6, 0, 3, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 5, 0, 0, 6, 2, 3, 8, 0, 0, 4,
        0, 2, 0, 0, 5, 3, 9, 5, 2, 0, 0, 0, 0, 0, 6, 7, 3, 0, 0, 1, 0, 4, 0, 2, 8, 2, 0, 7, 0, 0,
        3, 0, 0, 0, 0, 0, 0, 0, 0, 8, 2, 9, 0, 9, 0, 2, 8, 0, 0, 0, 7,
    ]);

    g2.load_str(
        "2...38.6.3....2.....5..6238..4.2..53952.....673..1.4.282.7..3........829.9.28...7",
    );

    assert!(g.get_values() == g2.get_values());
}

#[test]
fn test_is_grid_solved() {
    let g = &mut Grid::new();
    let g2 = &mut Grid::new();
    g.load_str("694527183517348269238961457163895742452176398879432516321659874945783621786214935");
    g2.load_str(
        "694527183517348269238961457163895742452176398879432516321659874945783621786214930",
    );

    assert!(g.is_solved());
    assert!(!g2.is_solved())
}

#[test]
fn test_deny_cell_candidate() {
    let g = &mut Grid::new();
    g.deny_cell_candidate(0, 0b100000000);
    assert!(!g.get_valid_placements(0).contains(&0b100000000))
}
