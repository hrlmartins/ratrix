extern crate clap;

use clap::ArgMatches;

use std::fs;
use std::path::Path;
use std::process;
use std::process::Command;

use structure::Cell;
use structure::File;
use structure::Matrix;

mod structure;

pub fn process_request(matches: ArgMatches) {
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
    delete_temp_file(file.get_name()).unwrap_or_else(|err| {
        eprintln!("Problem deleting the temporary file:\n{}\n", err);
        process::exit(1);
    });
}

fn print_values(matrix: &Matrix, positions: &str) {
    positions.split(",")
        .for_each(|pos| print_position_value(matrix, pos))
}

fn print_position_value(matrix: &Matrix, position: &str) {
    let converted: Vec<u32> = position.chars().map(|ch| convert_position(ch)).collect();
    let value = matrix.get_cell_value(
        converted[0],
        converted[1],
        converted[2]
    );

    println!("{}: {}", position, value);
}

fn convert_position(ch: char) -> u32 {
    match ch {
        'A' | '1' => 0,
        'B' | '2' => 1,
        'C' | '3' => 2,
        'D' | '4' => 3,
        'E' | '5' => 4,
        'F' | '6' => 5,
        'G' | '7' => 6,
        'H' | '8' => 7,
        _ => panic!("Invalid position provided {}", ch)
    }
}

fn process_file_contents(file: &File) -> Matrix {
    let contents = file.get_contents();
    let mut matrix: [[Cell; 8]; 8] = [[Cell::new([0; 3]); 8]; 8];

    for (i, line) in contents.lines().enumerate() {
        for (j, cell) in line.split(' ').enumerate() {
            matrix[i][j] = Cell::new(extract_cell_values(cell));
        }
    }

    Matrix::new(matrix)
}

fn extract_cell_values(cell: &str) -> [i32; 3] {
    let mut values: [i32; 3] = [0; 3];
    for (k, code) in cell.chars().enumerate() {
        values[k] = code.to_digit(10).unwrap() as i32;
    }

    values
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
