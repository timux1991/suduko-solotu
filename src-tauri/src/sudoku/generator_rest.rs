use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use super::{generator_spec::SudokuGenerator, sudoku_spec::SudokuField};

#[derive(Serialize, Deserialize, Debug)]
struct GenerateSudokuResponse {
    newboard: NewBoard,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewBoard {
    grids: [Grid; 1],
}

#[derive(Serialize, Deserialize, Debug)]
struct Grid {
    value: [[u8; 9]; 9],
}

pub struct RestSudokuGenerator {
}

impl RestSudokuGenerator {
    pub fn new() -> RestSudokuGenerator {
        return RestSudokuGenerator {};
    }
}

impl SudokuGenerator for RestSudokuGenerator {
    fn generate(&self) -> Option<SudokuField> {
        let mut field = SudokuField::new();

        field.rows[0].cells[0].value = Some(1);
        field.rows[0].cells[1].value = Some(2);
        field.rows[0].cells[2].value = Some(3);

        let client = Client::new();

        // Make the request and handle potential errors
        let response = match client
            .get("https://sudoku-api.vercel.app/api/dosuku?query={newboard(limit:1){grids{value,solution}}}")
            .send() {
                Ok(response) => response,
                Err(_) => return None,
            };

        // Parse the JSON response
        let sudoku_data = match response.json::<GenerateSudokuResponse>() {
            Ok(data) => data,
            Err(_) => return None,
        };

        // Extract the sudoku grid and convert to your SudokuField format
        let mut field = SudokuField::new();
        
        // Assuming your API returns a structure like this:
        // { "newboard": { "grids": [{ "value": [[...9x9 grid...]], "solution": [[...9x9 grid...]] }] } }
        if let Some(grid) = sudoku_data.newboard.grids.get(0) {
            // Fill the field with values from the API response
            for (row_idx, row_values) in grid.value.iter().enumerate() {
                for (col_idx, &value) in row_values.iter().enumerate() {
                    // Assuming 0 represents empty cells
                    field.rows[row_idx].cells[col_idx].value = if value == 0 { None } else { Some(value) };
                    field.rows[row_idx].cells[col_idx].fixed = if value == 0 { false } else { true };
                }
            }
            return Some(field);
        }

        None
    }
}
