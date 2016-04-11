extern crate backtrace;

pub mod error;

#[cfg(test)]
mod tests;

use std::str;
use method_name::error::Error;

pub fn get(levels_up: usize) -> Result<String, Error> {
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

fn resolve_method(symbol: &backtrace::Symbol, level: usize, levels_up: usize) -> Result<String, Error> {
    // 0 - backtrace::trace
    // 1 - get_method_name
    // 2 - calling_method => levels_up = 0
    // 3 - method_name_1 => levels_up = 1
    if level < 2 + levels_up {
        return Err(Error::InvalidLevel);
    }

    let name = try!(symbol.name().ok_or(Error::NoSymbolName));
    let demangled = try!(get_demangled(name));

    let index = try!(demangled.rfind(':').ok_or(Error::NoMethodSeparator));
    println!("{}", demangled);
    let method_name = try!(str::from_utf8(&(demangled.as_bytes())[..(index - 1)]));

    Ok(method_name.to_string())
}

fn get_demangled(name: &[u8]) -> Result<String, Error> {
    let mut demangled = String::new();
    let mangled = try!(str::from_utf8(name));

    try!(backtrace::demangle(&mut demangled, mangled));

    Ok(demangled)
}
