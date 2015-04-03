#[macro_export]
macro_rules! approve {
    ($actual: ident) => {
        {
            use std::fs;
            use std::fs::{File, PathExt};
            use std::io::{Read, Write};
            use std::path::Path;
            use std::process::Command;

            let mut backtrace = vec![];
            ::std::rt::backtrace::write(&mut backtrace);
            let s = String::from_utf8(backtrace).unwrap();

            let method_name = s
                .as_str()
                .lines()
                .skip(2)
                .next()
                .unwrap()
                .split("-")
                .last()
                .unwrap()
                .split(":")
                .next()
                .unwrap()
                .trim_left_matches(" ");

            let approvals_dirname = "approvals";
            let approvals_dir = Path::new(approvals_dirname);
            if !approvals_dir.exists() {
                match fs::create_dir(approvals_dir) {
                    Err(err) => panic!("Failed to create directory for approvals data: {:?}", err),
                    _ => {}
                }
            } else if !approvals_dir.is_dir() {
                panic!("Path for approvals data is not a directory")
            }

            let expected_filename = format!("{}/{}_expected.txt", approvals_dirname, method_name);
            let expected_path = Path::new(&expected_filename);
            let expected = match File::open(expected_path) {
                Ok(mut f) => {
                    let mut data = String::with_capacity($actual.len());
                    match f.read_to_string(&mut data) {
                        Err(err) => panic!("Failed to read expected data: {:?}", err),
                        _ => {}
                    }
                    data
                },
                Err(open_err) => {
                    match File::create(expected_path) {
                        Ok(_) => String::new(),
                        Err(create_err) => panic!("Failed to open file '{:?}'.\nOpen error: {:?}.\nCreate error: {:?}", expected_path, open_err, create_err)
                    }
                }
            };

            if $actual != expected {
                let actual_filename = format!("{}/{}_actual.txt", approvals_dirname, method_name);
                let mut actual_file = match File::create(actual_filename.clone()) {
                    Ok(f) => f,
                    Err(err) => panic!("Failed to create temp file: {:?}", err)
                };

                match actual_file.write_all($actual.as_bytes()) {
                    Err(err) => panic!("Failed to write actual data for comparing: {:?}", err),
                    _ => {}
                }

                let diff_tool = "opendiff"; // FIXME: Configure

                let output = Command::new(diff_tool)
                    .arg(::std::ffi::OsStr::new(actual_filename.as_str()))
                    .arg(expected_path.as_os_str())
                    .output();

                match output {
                    Err(err) => panic!("Failed to launch diff tool '{}': {:?}", diff_tool, err),
                    _ => {}
                }

                fs::remove_file(actual_filename);

                panic!("strings are not equal")
            }
        }
    }
}
