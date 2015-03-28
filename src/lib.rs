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

//fn get_method_name(_cx: &mut ExtCtxt) -> String {
    //let filelines = cx.codemap().span_to_lines(cx.call_site());
//    "test_123".to_string()
//}

fn expand_approve(_cx: &mut ExtCtxt, sp: Span, _args: &[TokenTree]) -> Box<MacResult + 'static> {
    // let actual = match args {
    //     [TtToken(_, token::Ident(s, _))] => token::get_ident(s).to_string(),
    //     _ => {
    //         cx.span_err(sp, "argument should be a single identifier");
    //         return DummyResult::any(sp);
    //     }
    // };

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

            let mut expected = String::new();
            expected_file.read_to_string(&mut expected);

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

                actual_file.write_all($actual.as_bytes());

                let actual_os_path = actual_path.as_os_str();
                let expected_os_path = expected_path.as_os_str();
                let diff_tool = "opendiff";

                let mut command1 = Command::new(diff_tool);
                let command2 = command1.arg(actual_os_path);
                let command3 = command2.arg(expected_os_path);
                println!("Command: {:?}", command3);
                command3.output();

                panic!("strings are not equal")
            }
        }
    }
}
