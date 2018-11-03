#[derive(Copy, Clone)]
pub struct Cell {
    values: [i32; 3],
}

impl Cell {
    pub fn new(values: [i32; 3]) -> Cell {
        Cell { values }
    }

    pub fn get_position(&self, pos: u32) -> i32 {
        self.values[pos as usize]
    }
}

pub struct Matrix {
    data: [[Cell; 8]; 8]
}

impl Matrix {
    pub fn new(data: [[Cell; 8]; 8]) -> Matrix {
        Matrix { data }
    }

    pub fn get_cell_position(&self, row: u32, column: u32) -> &Cell {
        &self.data[row as usize][column as usize]
    }

    pub fn get_cell_value(&self, row: u32, column: u32, position: u32) -> i32 {
        let cell = self.get_cell_position(row, column);
        cell.get_position(position)
    }
}

pub struct File {
    name: String,
    contents: String,
}

impl File {
    pub fn new(name: String, contents: String) -> File {
        // TODO check if filename is empty... shouldn't happen... but never know
        File { name, contents }
    }

    pub fn get_contents(&self) -> &str {
        &self.contents
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
