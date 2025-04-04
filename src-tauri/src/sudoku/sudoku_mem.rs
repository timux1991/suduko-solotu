use std::sync::Mutex;

use super::sudoku_spec::{SudokuField, SudokuRepo};

pub struct MemSudokuRepo {
    field: Mutex<SudokuField>,
}

impl MemSudokuRepo {
    pub fn new() -> MemSudokuRepo {
        return MemSudokuRepo {
            field: Mutex::new(SudokuField::new()),
        };
    }
}

impl SudokuRepo for MemSudokuRepo {
    fn get_field(&self) -> SudokuField {
        self.field.lock().unwrap().clone()
    }

    fn set_field(&self, new_field: SudokuField) {
        let mut field = self.field.lock().unwrap();
        *field = new_field;
    }

    fn set_cell(&self, row: u8, col: u8, value: u8) {
        let mut new_field = self.get_field();
        if !new_field.rows[row as usize].cells[col as usize].fixed {
            new_field.rows[row as usize].cells[col as usize].value = Some(value);
            self.set_field(new_field);
        }
    }

    fn set_cell_fixed(&self, row: u8, col: u8, value: u8) {
        let mut new_field = self.get_field();
        new_field.rows[row as usize].cells[col as usize].value = Some(value);
        new_field.rows[row as usize].cells[col as usize].fixed = true;
        self.set_field(new_field);
    }

    // fn set_cell_validity(&self, row: u8, col: u8, validity: bool) {
    //     let mut new_field = self.get_field();
    //     new_field.rows[row as usize].cells[col as usize].invalid = !validity;
    //     self.set_field(new_field);
    // }

    fn reset_cell(&self, row: u8, col: u8) {
        let mut new_field = self.get_field();
        if !new_field.rows[row as usize].cells[col as usize].fixed {
            new_field.rows[row as usize].cells[col as usize].value = None;
            self.set_field(new_field);
        }
    }

    fn reset_fixed_cell(&self, row: u8, col: u8) {
        let mut new_field = self.get_field();
        new_field.rows[row as usize].cells[col as usize].value = None;
        self.set_field(new_field);
    }
}
