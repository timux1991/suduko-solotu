use std::sync::Arc;

use sudoku::sudoku_controller::SudokuController;
use sudoku::sudoku_core::SudokuLogicImpl;
use sudoku::sudoku_mem::MemSudokuRepo;
use sudoku::sudoku_spec::SudokuField;
use tauri::{Builder, Manager, State};

mod sudoku;

struct AppState {
    controller: Arc<SudokuController>,
}

#[tauri::command]
fn get_field(state: State<'_, AppState>) -> SudokuField {
    state.controller.get_field()
}

#[tauri::command]
fn set_cell(state: State<'_, AppState>, row: u8, col: u8, value: u8) {
    state.controller.set_cell(row, col, value);
}

#[tauri::command]
fn reset_cell(state: State<'_, AppState>, row: u8, col: u8) {
    state.controller.reset_cell(row, col);
}

#[tauri::command]
fn check(state: State<'_, AppState>) -> SudokuField {
    state.controller.check()
}

#[tauri::command]
fn generate_field(state: State<'_, AppState>, number_count: u8) -> SudokuField {
    state.controller.generate_field(number_count)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let repo = Arc::new(MemSudokuRepo::new());
    let logic = Arc::new(SudokuLogicImpl::new(repo));
    let controller = Arc::new(SudokuController::new(logic));

    Builder::default()
        .setup(|app| {
            app.manage(AppState {
                controller: controller,
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_field,
            set_cell,
            reset_cell,
            check,
            generate_field
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
