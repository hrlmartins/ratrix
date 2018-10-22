#[macro_use]
extern crate clap;

use clap::App;
use clap::ArgMatches;
use std::fs;
use std::path::Path;
use std::process;
use std::process::Command;

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

    println!("file contents:\n {}\n", file.contents);
    // - process into friendly format
    // - Retrieve and print the values required based on the positions provided
    // - delete the decrypted file before exiting the application.
    delete_temp_file(file.name.as_str()).unwrap_or_else(|err| {
        eprintln!("Problem deleting the temporary file:\n{}\n", err);
        process::exit(1);
    });
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
