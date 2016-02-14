extern crate backtrace;

mod tests;

fn get_method_name() -> Option<String> {
    let mut result = None;
    let mut level = 0;

    backtrace::trace(&mut |frame| {
        let ip = frame.ip();

        backtrace::resolve(ip, &mut |symbol| {
            // 0 - backtrace::trace
            // 1 - get_method_name
            // 2 - approve
            // 3 - your_method_name
            if level < 3 {
                return
            }

            if let Some(name) = symbol.name() {
                let mut demangled = String::new();

                if let Ok(mangled) = std::str::from_utf8(name) {
                    if let Ok(()) = backtrace::demangle(&mut demangled, mangled) {
                        if let Some(index) = demangled.rfind(':') {
                            if let Ok(method_name) = std::str::from_utf8(&(demangled.as_bytes())[0..(index - 1)]) {
                                result = Some(method_name.to_string());
                            }
                        }
                    }
                }
            }
        });

        level = level + 1;
        level < 4
    });

    result
}


// #[macro_export]
// macro_rules! approve {
//     ($actual: ident) => {
//         {
//             use std::fs;
//             use std::fs::{File, PathExt};
//             use std::io::{Read, Write};
//             use std::path::Path;
//             use std::process::Command;

//             let mut backtrace = vec![];
//             ::std::rt::backtrace::write(&mut backtrace);
//             let s = String::from_utf8(backtrace).unwrap();

//             let method_name = s
//                 .as_str()
//                 .lines()
//                 .skip(2)
//                 .next()
//                 .unwrap()
//                 .split("-")
//                 .last()
//                 .unwrap()
//                 .split(":")
//                 .next()
//                 .unwrap()
//                 .trim_left_matches(" "); // FIXME: Handle all results/options

//             let approvals_dirname = "approvals";
//             let approvals_dir = Path::new(approvals_dirname);
//             if !approvals_dir.exists() {
//                 match fs::create_dir(approvals_dir) {
//                     Err(err) => panic!("Failed to create directory for approvals data: {:?}", err),
//                     _ => {}
//                 }
//             } else if !approvals_dir.is_dir() {
//                 panic!("Path for approvals data is not a directory")
//             }

//             let expected_filename = format!("{}/{}_expected.txt", approvals_dirname, method_name);
//             let expected_path = Path::new(&expected_filename);
//             let expected = match File::open(expected_path) {
//                 Ok(mut f) => {
//                     let mut data = String::with_capacity($actual.len());
//                     match f.read_to_string(&mut data) {
//                         Err(err) => panic!("Failed to read expected data: {:?}", err),
//                         _ => {}
//                     }

//                     data
//                 },
//                 Err(open_err) => {
//                     match File::create(expected_path) {
//                         Ok(_) => String::new(),
//                         Err(create_err) => panic!("Failed to open file '{:?}'.\nOpen error: {:?}.\nCreate error: {:?}", expected_path, open_err, create_err)
//                     }
//                 }
//             };

//             if $actual.trim_right_matches("\n") != expected.trim_right_matches("\n") {
//                 let actual_filename = format!("{}/{}_actual.txt", approvals_dirname, method_name);
//                 let mut actual_file = match File::create(actual_filename.clone()) {
//                     Ok(f) => f,
//                     Err(err) => panic!("Failed to create temp file: {:?}", err)
//                 };

//                 match actual_file.write_all($actual.as_bytes()) {
//                     Err(err) => panic!("Failed to write actual data for comparing: {:?}", err),
//                     _ => {}
//                 }

//                 let diff_tool = "vimdiff"; // FIXME: Configure

//                 let output = match
//                     Command::new(diff_tool).arg(::std::ffi::OsStr::new(actual_filename.as_str()))
//                     .arg(expected_path.as_os_str())
//                     .spawn() {
//                     Ok(p) => p.wait_with_output(),
//                     Err(err) => panic!("Failed to launch diff tool '{}': {:?}", diff_tool, err)
//                 };

//                 match output {
//                     Err(err) => panic!("Failed to launch diff tool '{}': {:?}", diff_tool, err),
//                     _ => {}
//                 }

//                 fs::remove_file(actual_filename);

//                 panic!("strings are not equal")
//             }
//         }
//     }
// }
