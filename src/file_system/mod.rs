#[cfg(test)]
mod tests;

use std::fs;
use std::path::Path;
use std::io::{Read, Write};

pub fn get_dir_path(dir_name: &str) -> &Path {
    let path = Path::new(dir_name);

    if !path.exists() {
        match fs::create_dir(path) {
            Err(err) => panic!("Failed to create directory for approvals data: {}", err),
            _ => {}
        }
    } else if !path.is_dir() {
        panic!("Approvals path is not a directory");
    }

    path
}

pub fn get_file_contents(method_name: &str, dir_path: &Path) -> String {
    let filename = format!("{}/{}_approved.txt", dir_path.display(), method_name);
    let path = Path::new(&filename);

    match fs::File::open(path) {
        Ok(mut f) => {
            let mut contents = String::new();
            match f.read_to_string(&mut contents) {
                Err(err) => panic!("Failed to read file contents: {}", err),
                _ => contents,
            }
        }
        Err(open_err) => {
            match fs::File::create(path) {
                Err(create_err) => {
                    panic!("Failed to open file: {}. Failed to create file: {}",
                           open_err,
                           create_err)
                }
                _ => String::new(),
            }
        }
    }
}

pub fn write_actual(actual: &str, method_name: &str, dir_path: &Path) {
    let filename = format!("{}/{}_received.txt", dir_path.display(), method_name);
    let path = Path::new(&filename);

    let mut file = match fs::File::create(path) {
        Ok(f) => f,
        Err(err) => panic!("Failed to create file for actual value: {}", err),
    };

    match file.write_all(&actual.as_bytes()) {
        Err(err) => panic!("Failed to write actual value to file: {}", err),
        _ => {}
    }
}
