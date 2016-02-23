mod get_method_name;

#[cfg(test)]
mod tests;

use get_method_name::get_method_name;
use get_method_name::Error as GetMethodNameError;

pub fn approve(actual: &str) {
    let method_name = extract_method_name(get_method_name());
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
