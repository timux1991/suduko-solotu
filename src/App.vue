<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface Cell {
  value: number | null;
  fixed: boolean;
  invalid: boolean;
}

interface Row {
  cells: Cell[];
}

interface Field {
  rows: Row[];
  valid: boolean;
  solved: boolean;
}

let field: Ref<Field> = ref({ rows: [], valid: false, solved: false });
let selectedRow: Ref<number | null> = ref(null);
let selectedCol: Ref<number | null> = ref(null);
let editMode: Ref<boolean> = ref(false);

async function refresh() {
  field.value = <Field>await invoke("get_field", {});
  console.log(field);
}

async function setCell(row: number, col: number, value: number) {
  if (editMode.value === true) {
    await invoke("set_cell_fixed", { row, col, value });
  } else {
    await invoke("set_cell", { row, col, value });
  }
  refresh();
  selectedCol.value = (col + 1) % 9;
  if (selectedCol.value === 0) {
    selectedRow.value = (row + 1) % 9;
  }
}

async function resetCell(row: number, col: number) {
  if (editMode.value === true) {
    await invoke("reset_fixed_cell", { row, col });
  } else {
    await invoke("reset_cell", { row, col });
  }
  refresh();
}

async function check() {
  field.value = <Field>await invoke("check", {});
  console.log(field.value);
}

async function generateField() {
  field.value = <Field>await invoke("generate_field", {});
  console.log(field.value);
}

async function clearField() {
  field.value = <Field>await invoke("clear_field");
}

async function enableEditMode() {
  editMode.value = true;
}

async function disableEditMode() {
  editMode.value = false;
}

function getCellClasses(row: number, col: number, cell: Cell) {
  const classes = [];
  if (row % 3 === 0) {
    classes.push("bold-top");
  }
  if (row === 8) {
    classes.push("bold-bottom");
  }
  if (col % 3 === 0) {
    classes.push("bold-left");
  }
  if (col === 8) {
    classes.push("bold-right");
  }
  if (row === selectedRow.value && col === selectedCol.value) {
    classes.push("selected-cell");
  }
  if (cell.fixed) {
    classes.push("fixed-cell");
  }
  if (cell.invalid) {
    classes.push("invalid-cell");
  }
  return classes.join(" ");
}

function selectCell(row: number, col: number) {
  console.log(row, col);
  selectedRow.value = row;
  selectedCol.value = col;
}

refresh();
setCell(0, 0, 1);
setCell(1, 1, 2);
setCell(2, 2, 3);
setCell(3, 3, 4);
setCell(4, 4, 5);
setCell(5, 5, 6);
setCell(6, 6, 7);
setCell(7, 7, 8);
setCell(8, 8, 9);

document.addEventListener(
  "keyup",
  (event) => {
    const keyName = event.key;

    // As the user releases the Ctrl key, the key is no longer active,
    // so event.ctrlKey is false.
    if (keyName === "ArrowDown") {
      onPressArrowDown();
    } else if (keyName === "ArrowUp") {
      onPressArrowUp();
    } else if (keyName === "ArrowRight") {
      onPressArrowRight();
    } else if (keyName === "ArrowLeft") {
      onPressArrowLeft();
    } else if (
      keyName === "1" ||
      keyName === "2" ||
      keyName === "3" ||
      keyName === "4" ||
      keyName === "5" ||
      keyName === "6" ||
      keyName === "7" ||
      keyName === "8" ||
      keyName === "9"
    ) {
      onPressNumber(parseInt(keyName));
    } else if (
      keyName === "0" ||
      keyName === "Backspace" ||
      keyName === "Delete" ||
      keyName === "d" ||
      keyName === "r"
    ) {
      onPressReset();
    } else if (keyName === "c") {
      check();
    }
  },
  false
);

function onPressArrowDown() {
  if (selectedRow.value === null || selectedCol.value === null) {
    selectedRow.value = 0;
    selectedCol.value = 0;
    return;
  }
  if (selectedRow.value === 8) {
    return;
  }
  selectedRow.value += 1;
}

function onPressArrowUp() {
  if (selectedRow.value === null || selectedCol.value === null) {
    selectedRow.value = 0;
    selectedCol.value = 0;
    return;
  }
  if (selectedRow.value === 0) {
    return;
  }
  selectedRow.value -= 1;
}

function onPressArrowRight() {
  if (selectedRow.value === null || selectedCol.value === null) {
    selectedRow.value = 0;
    selectedCol.value = 0;
    return;
  }
  if (selectedCol.value === 8) {
    return;
  }
  selectedCol.value += 1;
}

function onPressArrowLeft() {
  if (selectedRow.value === null || selectedCol.value === null) {
    selectedRow.value = 0;
    selectedCol.value = 0;
    return;
  }
  if (selectedCol.value === 0) {
    return;
  }
  selectedCol.value -= 1;
}

function onPressNumber(nmb: number) {
  if (selectedRow.value !== null && selectedCol.value !== null) {
    setCell(selectedRow.value, selectedCol.value, nmb);
  }
}

function onPressReset() {
  if (selectedRow.value !== null && selectedCol.value !== null) {
    resetCell(selectedRow.value, selectedCol.value);
  }
}
</script>

<template>
  <main class="main-container">
    <h1>Sudoku</h1>

    <div class="container">
      <div class="row">
        <div>
          <div>
          <table>
            <tbody>
              <tr v-for="(row, rowIndex) in field.rows">
                <td
                  v-for="(cell, colIndex) in row.cells"
                  :class="
                    getCellClasses(
                      rowIndex,
                      colIndex,
                      field.rows[rowIndex].cells[colIndex]
                    )
                  "
                  @click="selectCell(rowIndex, colIndex)"
                >
                  {{ cell.value === null ? " " : cell.value }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        <div>
          <p v-show="editMode">Editing...</p>
          <p v-show="!editMode && !field.solved && field.valid">Keep on going...</p>
          <p v-show="!editMode && field.solved">Done. Well done!</p>
          <p v-show="!editMode && !field.valid">Please double check your numbers.</p>
        </div>
        </div>
        <div>
          <form @submit.prevent="refresh">
            <button type="submit">Refresh</button>
          </form>
          <form @submit.prevent="check">
            <button type="submit">Check</button>
          </form>
          <form @submit.prevent="generateField">
            <button type="submit">Generate</button>
          </form>
          <form @submit.prevent="clearField">
            <button type="submit">Clear</button>
          </form>
          <form @submit.prevent="enableEditMode" v-show="editMode === false">
            <button type="submit">Edit mode</button>
          </form>
          <form @submit.prevent="disableEditMode" v-show="editMode === true">
            <button type="submit">Finish edit mode</button>
          </form>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.main-container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

table,
th,
td {
  border-collapse: collapse;
  border: 1px solid #0f0f0f;
}

th,
td {
  height: 40px;
  width: 40px;
  text-align: center;
}

form {
  margin: 10px;
}

.bold-left {
  border-left-width: 3px;
}

.bold-right {
  border-right-width: 3px;
}

.bold-top {
  border-top-width: 3px;
}

.bold-bottom {
  border-bottom-width: 3px;
}

.selected-cell {
  background-color: #686767;
}

.fixed-cell {
  font-weight: 800;
}

.invalid-cell {
  color: #ff0000;
}
</style>
