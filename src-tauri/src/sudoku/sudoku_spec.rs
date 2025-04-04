use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SudokuField {
    pub rows: [SudokuRow; 9],
    pub valid: bool,
    pub solved: bool,
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
            valid: true,
            solved: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.rows.iter().all(|row| row.is_valid())
    }

    pub fn is_solved(&self) -> bool {
        self.rows.iter().all(|row| row.is_solved())
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

    fn is_valid(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_valid())
    }

    fn is_solved(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_solved())
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

    fn is_solved(&self) -> bool {
        self.value != None
    }

    fn is_valid(&self) -> bool {
        !self.invalid
    }
}

pub trait SudokuRepo {
    fn get_field(&self) -> SudokuField;

    fn set_field(&self, field: SudokuField);

    fn set_cell(&self, row: u8, col: u8, value: u8);

    fn set_cell_fixed(&self, row: u8, col: u8, value: u8);

    // fn set_cell_validity(&self, row: u8, col: u8, validity: bool);

    fn reset_cell(&self, row: u8, col: u8);

    fn reset_fixed_cell(&self, row: u8, col: u8);
}
