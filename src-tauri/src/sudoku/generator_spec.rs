use super::sudoku_spec::SudokuField;

pub trait SudokuGenerator {
    fn generate(&self) -> Option<SudokuField>;
}
