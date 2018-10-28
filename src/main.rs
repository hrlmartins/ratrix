#[macro_use]
extern crate clap;

use clap::App;
use clap::ArgMatches;

use std::fs;
use std::path::Path;
use std::process;
use std::process::Command;
use std::str::FromStr;

fn main() {
    let yaml = load_yaml!("../cmd.yml");
    let matches = App::from_yaml(yaml).get_matches();

    process_request(matches);
}

fn process_request(matches: ArgMatches) {
    // The arguments are mandatory. If it reaches here they must be defined!
    let file_location = matches.value_of("file").unwrap();
    let matrix_positions = matches.value_of("positions").unwrap();

    // Having the file location we have to:
    // - Decrypt using the pgp command line tool
    // - Read the file
    let file = decrypt_and_read_file(file_location).unwrap_or_else(|err| {
        eprintln!("Problem trying to read encrypted file: {}", err);
        process::exit(1);
    });

    // - process into friendly format
    let matrix = process_file_contents(&file);

    // - Retrieve and print the values required based on the positions provided
    print_values(&matrix, matrix_positions);

    // - delete the decrypted file before exiting the application.
    delete_temp_file(file.name.as_str()).unwrap_or_else(|err| {
        eprintln!("Problem deleting the temporary file:\n{}\n", err);
        process::exit(1);
    });
}

fn print_values(matrix: &Matrix, positions: &str) {
    let positions_separated = positions.split(",");
    for position in positions_separated {
        print_position_value(matrix, position);
    }
}

fn print_position_value(matrix: &Matrix, position: &str) {
    let converted = position.chars().map(|ch| convert_position(ch)).collect();
    let value = matrix.get_cell_value(
        converted.get(0),
        converted.get(1),
        converted.get(2)
    );

    println!("{}: {}", position, value);
}

fn convert_position(ch: char) -> i32 {
    match ch {
        'A' | '1' => 0,
        'B' | '2' => 1,
        'C' | '3' => 2,
        'D' | '4' => 3,
        'E' | '5' => 4,
        'F' | '6' => 5,
        'G' | '7' => 6,
        'H' | '8' => 7,
    }
}

fn process_file_contents(file: &File) -> Matrix {
    let contents = file.get_contents();
    let lines = contents.lines();
    let mut matrix: [[Cell; 8]; 8];

    for (i, line) in lines.enumerate() {
        let cells = line.split(";");
        for (j, cell) in cells.enumerate() {
            let mut values: [i32; 3];
            let codes = cell.split(",");
            for (k, code) in codes.enumerate() {
                values[k] = FromStr::from_str(code).unwrap();
            }

            matrix[i][j] = Cell::new(values);
        }
    }

    Matrix::new(matrix)
}

struct Cell {
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

struct Matrix {
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

struct File {
    name: String,
    contents: String,
}

impl File {
    pub fn new(name: String, contents: String) -> File {
        // TODO check if filename is empty... shouldn't happen... but never know
        File { name, contents }
    }

    pub fn get_contents<'a>(self) -> &'a str {
        &self.contents
    }
}

fn decrypt_and_read_file(file_location: &str) -> Result<File, &str> {
    let decrypted_file_name = "file_dec";
    let file_path = Path::new(file_location).to_path_buf();
    let file_path_str = file_path.to_str();

    if let None = file_path_str {
        return Err("Provided path is invalid");
    }

    decrypt_file(decrypted_file_name, file_path_str.unwrap());

    Ok(File::new(decrypted_file_name.to_string(), read_file(decrypted_file_name)?))
}

fn decrypt_file(decrypted_file_name: &str, file_path_str: &str) {
    let mut process =
        match Command::new("gpg")
            .arg("--output")
            .arg(decrypted_file_name)
            .arg("--decrypt")
            .arg(file_path_str)
            .spawn() {
            Err(reason) => panic!(reason),
            Ok(process) => process,
        };

    // Wait for the password input.
    process.wait().expect("Nothing is running...");
}

fn read_file(decrypted_file_name: &str) -> Result<String, &str> {
    let fetch_result = fs::read_to_string(decrypted_file_name);
    if let Err(_) = fetch_result { // TODO pass propagate error message
        return Err("Unable to read decrypted file");
    }

    Ok(fetch_result.unwrap())
}

fn delete_temp_file(file: &str) -> Result<(), &str> {
    if let Err(_) = fs::remove_file(file) {
        return Err("Unable to delete temp file!!");
    }

    Ok(())
}
