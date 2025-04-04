use std::sync::Arc;

use super::{generator_spec::SudokuGenerator, sudoku_spec::{SudokuCell, SudokuField, SudokuRepo}};

pub trait SudokuLogic {
    fn get_field(&self) -> SudokuField;

    fn set_cell(&self, row: u8, col: u8, value: u8);

    fn set_cell_fixed(&self, row: u8, col: u8, value: u8);

    fn reset_cell(&self, row: u8, col: u8);
    fn reset_fixed_cell(&self, row: u8, col: u8);

    fn check(&self) -> SudokuField;

    fn check_field(&self, field: SudokuField) -> SudokuField;

    fn generate_field(&self) -> SudokuField;

    fn clear_field(&self) -> SudokuField;
}

pub struct SudokuLogicImpl {
    sudoku_repo: Arc<dyn SudokuRepo + Send + Sync>,
    generator: Arc<dyn SudokuGenerator + Send + Sync>,
}

impl SudokuLogicImpl {
    pub fn new(sudoku_repo: Arc<impl SudokuRepo + 'static + Send + Sync>, generator: Arc<impl SudokuGenerator + 'static + Send + Sync>) -> SudokuLogicImpl {
        return SudokuLogicImpl { sudoku_repo, generator };
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
        let field = self.sudoku_repo.get_field();
        let checked_field = self.check_field(field);
        self.sudoku_repo
            .set_field(self.check_field(checked_field).clone());

        return self.sudoku_repo.get_field();
    }

    fn check_field(&self, mut field: SudokuField) -> SudokuField {
        let mut row_duplicates: [[i8; 9]; 9] = [[0; 9]; 9];
        let mut col_duplicates: [[i8; 9]; 9] = [[0; 9]; 9];
        let mut box_duplicates: [[i8; 9]; 9] = [[0; 9]; 9];

        // Check if values are unique in rows, columns and boxes
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
        }

        // apply validity flags to cells
        for row_index in 0..9 {
            for col_index in 0..9 {
                let box_index = (row_index / 3) * 3 + col_index / 3;
                let cell: &SudokuCell = &field.rows[row_index].cells[col_index];
                let cell_value: Option<u8> = field.rows[row_index].cells[col_index].value;
                if !cell.fixed
                    && cell.value != None
                    && (row_duplicates[row_index][cell_value.unwrap() as usize - 1] > 1
                        || col_duplicates[col_index][cell_value.unwrap() as usize - 1] > 1
                        || box_duplicates[box_index][cell_value.unwrap() as usize - 1] > 1)
                {
                    field.rows[row_index].cells[col_index].invalid = true;
                } else {
                    field.rows[row_index].cells[col_index].invalid = false;
                }
            }
        }

        field.valid = field.is_valid();
        field.solved = field.is_solved();

        return field;
    }

    fn generate_field(&self) -> SudokuField {
        // TODO: fix this as it's unlikely to generate a valid field
        let field = self.generator.generate();

        if field.is_some() {
            self.sudoku_repo.set_field(field.unwrap());
        } else {
            self.sudoku_repo.set_field(SudokuField::new());
        }

        return self.sudoku_repo.get_field();
    }

    fn clear_field(&self) -> SudokuField {
        self.sudoku_repo.set_field(SudokuField::new());
        return self.sudoku_repo.get_field();
    }

}
