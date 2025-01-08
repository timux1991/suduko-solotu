use rand::Rng;
use std::sync::Mutex;
use std::{rc::Rc, sync::Arc};

use super::sudoku_spec::{SudokuCell, SudokuField, SudokuRepo};

pub trait SudokuLogic {
    fn get_field(&self) -> SudokuField;

    fn set_cell(&self, row: u8, col: u8, value: u8);

    fn set_cell_fixed(&self, row: u8, col: u8, value: u8);

    fn reset_cell(&self, row: u8, col: u8);
    fn reset_fixed_cell(&self, row: u8, col: u8);

    fn check(&self) -> SudokuField;

    fn check_field(&self, field: Arc<Mutex<SudokuField>>) -> Arc<Mutex<SudokuField>>;

    fn generate_field(&self, numbers: u8) -> SudokuField;

    fn clear_field(&self) -> SudokuField;
}

pub struct SudokuLogicImpl {
    sudoku_repo: Arc<dyn SudokuRepo + Send + Sync>,
}

impl SudokuLogicImpl {
    pub fn new(sudoku_repo: Arc<impl SudokuRepo + 'static + Send + Sync>) -> SudokuLogicImpl {
        return SudokuLogicImpl { sudoku_repo };
    }
}

impl SudokuLogic for SudokuLogicImpl {
    fn get_field(&self) -> SudokuField {
        self.sudoku_repo.get_field()
    }

    fn set_cell(&self, row: u8, col: u8, value: u8) {
        self.sudoku_repo.set_cell(row, col, value);
    }

    fn set_cell_fixed(&self, row: u8, col: u8, value: u8) {
        self.sudoku_repo.set_cell_fixed(row, col, value);
    }

    fn reset_cell(&self, row: u8, col: u8) {
        self.sudoku_repo.reset_cell(row, col);
    }

    fn reset_fixed_cell(&self, row: u8, col: u8) {
        self.sudoku_repo.reset_fixed_cell(row, col);
    }

    fn check(&self) -> SudokuField {
        let field = Arc::new(Mutex::new(self.sudoku_repo.get_field()));
        let checked_field = self.check_field(field);
        self.sudoku_repo
            .set_field(self.check_field(checked_field).lock().unwrap().clone());

        return self.sudoku_repo.get_field();
    }

    fn check_field(&self, field: Arc<Mutex<SudokuField>>) -> Arc<Mutex<SudokuField>> {
        let mut row_duplicates: [[i8; 9]; 9] = [[0; 9]; 9];
        let mut col_duplicates: [[i8; 9]; 9] = [[0; 9]; 9];
        let mut box_duplicates: [[i8; 9]; 9] = [[0; 9]; 9];

        let mut locked_field = field.lock().unwrap();

        // Check if values are unique in rows, columns and boxes
        for row_index in 0..9 {
            let mut detected_numbers: [i8; 9] = [0; 9];
            for col_index in 0..9 {
                let box_index = (row_index / 3) * 3 + col_index / 3;
                let cell_value: Option<u8> = locked_field.rows[row_index].cells[col_index].value;
                if cell_value != None {
                    detected_numbers[(cell_value.unwrap() as usize) - 1] += 1;
                    row_duplicates[row_index][cell_value.unwrap() as usize - 1] += 1;
                    col_duplicates[col_index][cell_value.unwrap() as usize - 1] += 1;
                    box_duplicates[box_index][cell_value.unwrap() as usize - 1] += 1;
                }
            }
        }

        // apply validity flags to cells
        for row_index in 0..9 {
            for col_index in 0..9 {
                let box_index = (row_index / 3) * 3 + col_index / 3;
                let cell: &SudokuCell = &locked_field.rows[row_index].cells[col_index];
                let cell_value: Option<u8> = locked_field.rows[row_index].cells[col_index].value;
                if !cell.fixed
                    && cell.value != None
                    && (row_duplicates[row_index][cell_value.unwrap() as usize - 1] > 1
                        || col_duplicates[col_index][cell_value.unwrap() as usize - 1] > 1
                        || box_duplicates[box_index][cell_value.unwrap() as usize - 1] > 1)
                {
                    locked_field.rows[row_index].cells[col_index].invalid = true;
                } else {
                    locked_field.rows[row_index].cells[col_index].invalid = false;
                }
            }
        }

        locked_field.valid = locked_field.is_valid();
        locked_field.solved = locked_field.is_solved();

        return Arc::new(Mutex::new(locked_field.clone()));
    }

    fn generate_field(&self, numbers: u8) -> SudokuField {
        // TODO: fix this as it's unlikely to generate a valid field
        let mut rng = rand::thread_rng();
        let mut field = SudokuField::new();
        let mut added_numbers = 0;

        while added_numbers < numbers {
            let row_index = rng.gen_range(0..9);
            let col_index = rng.gen_range(0..9);
            let cell = &field.rows[row_index as usize].cells[col_index as usize];
            if cell.value == None {
                let value = rng.gen_range(1..10);

                let field_candidate = Arc::new(Mutex::new(field.clone()));
                field_candidate.lock().unwrap().rows[row_index as usize].cells
                    [col_index as usize]
                    .value = Some(value as u8);

                if self.check_field(field_candidate).lock().unwrap().valid {
                    field.rows[row_index as usize].cells[col_index as usize].value =
                        Some(value as u8);
                    field.rows[row_index as usize].cells[col_index as usize].fixed = true;
                    added_numbers += 1;
                }
            }
        }

        self.sudoku_repo.set_field(field.clone());

        return field;
    }

    fn clear_field(&self) -> SudokuField {
        self.sudoku_repo.set_field(SudokuField::new());
        return self.sudoku_repo.get_field();
    }

    // fn generate_all_possible_rows(&self) {
    //     for row_value in 123456789..987654321 {
    //         let row = row_value.to_string();
    //         let mut row_values: [u8; 9] = [0; 9];
    //         for i in 0..9 {
    //             row_values[i] = row.chars().nth(i).unwrap().to_digit(10).unwrap() as u8;
    //         }
    //     }
    // }
}
