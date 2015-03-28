#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;

use syntax::codemap::Span;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult};
use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("approve", expand_approve);
}

fn expand_approve(_cx: &mut ExtCtxt, sp: Span, _args: &[TokenTree]) -> Box<MacResult + 'static> {
    // FIXME: get method name and call approve_file
    DummyResult::any(sp)
}

#[macro_export]
macro_rules! approve_file {
    ($actual: ident, $file: ident) => {
        {
            use std::fs::{File, TempDir};
            use std::io::{Read, Write};
            use std::path::Path;
            use std::process::Command;

            let expected_filename = format!("expected/{}.txt", stringify!($file));
            let expected_path = Path::new(&expected_filename);
            let mut expected_file = match File::open(expected_path) {
                Ok(f) => f,
                Err(open_err) => {
                    match File::create(expected_path) {
                        Ok(f) => f,
                        Err(create_err) => panic!("Failed to open file '{:?}'.\nOpen error: {:?}.\nCreate error: {:?}", expected_path, open_err, create_err)
                    }
                }
            };

            let mut expected = String::with_capacity($actual.len());
            match expected_file.read_to_string(&mut expected) {
                Err(err) => panic!("Failed to read expected data: {:?}", err),
                _ => {}
            }

            if $actual != expected {
                let temp_dir = match TempDir::new("approvals") {
                    Ok(d) => d,
                    Err(err) => panic!("Failed to create temp dir: {:?}", err)
                };

                let actual_path = temp_dir.path().join(Path::new("actual.txt"));
                let mut actual_file = match File::create(actual_path.clone()) {
                    Ok(f) => f,
                    Err(err) => panic!("Failed to create temp file: {:?}", err)
                };

                match actual_file.write_all($actual.as_bytes()) {
                    Err(err) => panic!("Failed to write actual data for comparing: {:?}", err),
                    _ => {}
                }

                let diff_tool = "opendiff"; // FIXME: Configure

                let output = Command::new(diff_tool)
                    .arg(actual_path.as_os_str())
                    .arg(expected_path.as_os_str())
                    .output();

                match output {
                    Err(err) => panic!("Failed to launch diff tool '{}': {:?}", diff_tool, err),
                    _ => {}
                }

                panic!("strings are not equal")
            }
        }
    }
}
