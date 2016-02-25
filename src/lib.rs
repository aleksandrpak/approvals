mod get_method_name;

#[cfg(test)]
mod tests;

use std::io::{Read, Write};
use std::fs;
use std::path::Path;
use std::process::Command;
use get_method_name::get_method_name;
use get_method_name::Error as GetMethodNameError;

pub fn approve(actual: &str) {
    let method_name = extract_method_name(get_method_name(1));

    let approvals_dir = "approvals"; // TODO: configure
    get_dir(approvals_dir); // TODO: make create dir
    let contents = get_file_contents(&method_name, approvals_dir);

    if actual.trim_right_matches("\n") == contents.trim_right_matches("\n") {
        return;
    }

    write_actual(actual, &method_name, approvals_dir);

    let command = "vimdiff"; // TODO: configure
    launch_diff(command, &method_name, approvals_dir);

    panic!("Strings are not identical");
}

fn extract_method_name(result: Result<String, GetMethodNameError>) -> String {
    match result {
        Ok(method_name) => method_name,
        Err(err) => {
            match err {
                GetMethodNameError::NoSymbolName => panic!("Could not retrieve method name"),
                GetMethodNameError::NoMethodSeparator => panic!("Invalid format of method name"),
                GetMethodNameError::InvalidLevel => {
                    panic!("Invalid level of searching for method name")
                }
                GetMethodNameError::Utf8(utf8_err) => {
                    panic!("Could not decode UTF8 method name: {}", utf8_err)
                }
                GetMethodNameError::Demangle(fmt_err) => {
                    panic!("Could not demangle method name: {}", fmt_err)
                }
            }
        }
    }
}

fn get_dir(dir_name: &str) -> &Path {
    let path = Path::new(dir_name);

    if !path.exists() {
        // TODO: check multi level
        match fs::create_dir(path) {
            Err(err) => panic!("Failed to create directory for approvals data: {}", err),
            _ => {}
        }
    } else if !path.is_dir() {
        panic!("Approvals path is not a directory");
    }

    path
}

fn get_file_contents(method_name: &str, dir_name: &str) -> String {
    let filename = format!("{}/{}_approved.txt", dir_name, method_name);
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

fn write_actual(actual: &str, method_name: &str, dir_name: &str) {
    let filename = format!("{}/{}_received.txt", dir_name, method_name);
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

fn launch_diff(command: &str, method_name: &str, dir_name: &str) {
    let received = format!("{}/{}_received.txt", dir_name, method_name);
    let approved = format!("{}/{}_approved.txt", dir_name, method_name);

    // TODO: replace match like this with unwrap_or_else
    match Command::new(command).arg(received).arg(approved).status() {
        Err(err) => panic!("Failed to launch diff tool: {}", err),
        _ => {}
    }
}
