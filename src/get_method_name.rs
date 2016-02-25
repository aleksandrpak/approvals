extern crate backtrace;

use std::fmt;
use std::str;

#[derive(Debug)]
pub enum Error {
    InvalidLevel,
    NoSymbolName,
    NoMethodSeparator,
    Utf8(str::Utf8Error),
    Demangle(fmt::Error),
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Error {
        Error::Demangle(err)
    }
}

pub fn get_method_name(levels_up: usize) -> Result<String, Error> {
    let mut result = Err(Error::InvalidLevel);
    let mut level = 0;

    backtrace::trace(&mut |frame| {
        let ip = frame.ip();

        backtrace::resolve(ip, &mut |s| result = resolve_method(s, level, levels_up));

        level = level + 1;

        match result {
            Err(Error::InvalidLevel) => true,
            _ => false,
        }
    });

    result
}

fn resolve_method(symbol: &backtrace::Symbol,
                  level: usize,
                  levels_up: usize)
                  -> Result<String, Error> {
    // 0 - backtrace::trace
    // 1 - get_method_name
    // 2 - calling_method => levels_up = 0
    // 3 - method_name_1 => levels_up = 1
    if level < 2 + levels_up {
        return Err(Error::InvalidLevel);
    }

    let name = try!(symbol.name().ok_or(Error::NoSymbolName));

    let mut demangled = String::new();
    let mangled = try!(str::from_utf8(name));

    try!(backtrace::demangle(&mut demangled, mangled));

    let index = try!(demangled.rfind(':').ok_or(Error::NoMethodSeparator));
    let method_name = try!(str::from_utf8(&(demangled.as_bytes())[..(index - 1)]));

    Ok(method_name.to_string())
}
