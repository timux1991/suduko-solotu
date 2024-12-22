use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SudokuField {
    pub rows: [SudokuRow; 9],
}

impl SudokuField {
    pub fn new() -> SudokuField {
        SudokuField {
            rows: [
                SudokuRow::new(),
                SudokuRow::new(),
                SudokuRow::new(),
                SudokuRow::new(),
                SudokuRow::new(),
                SudokuRow::new(),
                SudokuRow::new(),
                SudokuRow::new(),
                SudokuRow::new(),
            ],
        }
    }
}

#[derive(Clone, Serialize)]
pub struct SudokuRow {
    pub cells: [SudokuCell; 9],
}

impl SudokuRow {
    fn new() -> SudokuRow {
        SudokuRow {
            cells: [
                SudokuCell::new(),
                SudokuCell::new(),
                SudokuCell::new(),
                SudokuCell::new(),
                SudokuCell::new(),
                SudokuCell::new(),
                SudokuCell::new(),
                SudokuCell::new(),
                SudokuCell::new(),
            ],
        }
    }
}

#[derive(Clone, Serialize)]
pub struct SudokuCell {
    pub value: Option<u8>,
    pub fixed: bool,
    pub invalid: bool,
    options: [bool; 9],
}

impl SudokuCell {
    fn new() -> SudokuCell {
        SudokuCell {
            value: None,
            fixed: false,
            invalid: false,
            options: [
                false, false, false, false, false, false, false, false, false,
            ],
        }
    }
}

pub trait SudokuRepo {
    fn get_field(&self) -> SudokuField;

    fn set_field(&self, field: SudokuField);

    fn set_cell(&self, row: u8, col: u8, value: u8);

    fn set_cell_validity(&self, row: u8, col: u8, validity: bool);

    fn reset_cell(&self, row: u8, col: u8);
}
