use std::sync::Arc;

use super::sudoku_spec::{SudokuCell, SudokuField, SudokuRepo};

pub trait SudokuLogic {
    fn get_field(&self) -> SudokuField;

    fn set_cell(&self, row: u8, col: u8, value: u8);

    fn reset_cell(&self, row: u8, col: u8);

    fn check(&self) -> SudokuField;
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

    fn reset_cell(&self, row: u8, col: u8) {
        self.sudoku_repo.reset_cell(row, col);
    }

    fn check(&self) -> SudokuField {
        let mut field = self.sudoku_repo.get_field();

        let mut row_duplicates: [[i8; 9]; 9] = [[0; 9]; 9];
        let mut col_duplicates: [[i8; 9]; 9] = [[0; 9]; 9];
        let mut box_duplicates: [[i8; 9]; 9] = [[0; 9]; 9];

        // Check if values are unique in rows
        for row_index in 0..9 {
            let mut detected_numbers: [i8; 9] = [0; 9];
            for col_index in 0..9 {
                let box_index = (row_index / 3) * 3 + col_index / 3;
                let cell_value: Option<u8> = field.rows[row_index].cells[col_index].value;
                if cell_value != None {
                    detected_numbers[(cell_value.unwrap() as usize) - 1] += 1;
                    row_duplicates[row_index][cell_value.unwrap() as usize - 1] += 1;
                    col_duplicates[col_index][cell_value.unwrap() as usize - 1] += 1;
                    box_duplicates[box_index][cell_value.unwrap() as usize - 1] += 1;
                }
            }

            // for col_index in 0..9 {
            //     let cell: &SudokuCell = &field.rows[row_index].cells[col_index];
            //     // cell has value

            //     if !cell.fixed
            //         && cell.value != None
            //         // && cell.value == Some(number as u8)
            //         && detected_numbers[cell.value.unwrap() as usize - 1] > 1
            //     {
            //         println!(
            //             "found invalid field at row: {}, col: {}",
            //             row_index, col_index
            //         );
            //         self.sudoku_repo
            //             .set_cell_validity(row_index as u8, col_index as u8, false);
            //         field.rows[row_index].cells[col_index].invalid = true;
            //     } else {
            //         self.sudoku_repo
            //             .set_cell_validity(row_index as u8, col_index as u8, true);
            //         field.rows[row_index].cells[col_index].invalid = false;
            //     }
            // }
        }

        for row_index in 0..9 {
            for col_index in 0..9 {
                let box_index = (row_index / 3) * 3 + col_index / 3;
                let cell: &SudokuCell = &field.rows[row_index].cells[col_index];
                let cell_value: Option<u8> = cell.value;
                if !cell.fixed
                    && cell.value != None
                    && (row_duplicates[row_index][cell_value.unwrap() as usize - 1] > 1
                        || col_duplicates[col_index][cell_value.unwrap() as usize - 1] > 1
                        || box_duplicates[box_index][cell_value.unwrap() as usize - 1] > 1)
                {
                    self.sudoku_repo
                        .set_cell_validity(row_index as u8, col_index as u8, false);
                    field.rows[row_index].cells[col_index].invalid = true;
                } else {
                    self.sudoku_repo
                        .set_cell_validity(row_index as u8, col_index as u8, true);
                    field.rows[row_index].cells[col_index].invalid = false;
                }
            }
        }

        return field;
    }
}
