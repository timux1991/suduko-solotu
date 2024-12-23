use std::sync::Arc;

use super::sudoku_core::SudokuLogic;
use super::sudoku_spec::SudokuField;

#[derive(Clone)]
pub struct SudokuController {
    logic: Arc<dyn SudokuLogic + Send + Sync>,
}

impl SudokuController {
    pub fn new(logic: Arc<impl SudokuLogic + 'static + Send + Sync>) -> SudokuController {
        return SudokuController { logic };
    }

    pub fn get_field(&self) -> SudokuField {
        self.logic.get_field()
    }

    pub fn set_cell(&self, row: u8, col: u8, value: u8) {
        self.logic.set_cell(row, col, value);
    }

    pub fn reset_cell(&self, row: u8, col: u8) {
        self.logic.reset_cell(row, col);
    }

    pub fn check(&self) -> SudokuField {
        self.logic.check()
    }

    pub fn generate_field(&self, number_count: u8) -> SudokuField {
        self.logic.generate_field(number_count)
    }
}
