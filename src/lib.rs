// syn docs: https://docs.rs/syn/2.0.60/syn/index.html
extern crate syn;

pub mod common;
pub mod errors;
pub mod instruction;
pub mod optimize;
pub mod rust_to_dtr_c;
pub mod translate;

use std::ffi::{CStr, CString};

use std::os::raw::c_char;
use std::ptr;

use instruction::Instruction;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn parse_to_dtr(rust_code: &str) -> Result<String, errors::NotTranslatableError> {
    rust_to_dtr_c::parse_to_dtr(rust_code)
}

#[no_mangle]
pub extern "C" fn rust_string_to_dtr_string(rust_code: *const c_char) -> *const c_char {
    if rust_code.is_null() {
        return ptr::null();
    }

    // Convert the C string to a Rust &str
    let c_str = unsafe { CStr::from_ptr(rust_code) };
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null(),
    };

    // Call the function to convert the Rust string to DTR string
    let dtr_string = rust_to_dtr_c::parse_to_dtr(rust_str).unwrap_or_default();

    // Convert the DTR string to CString and return the raw pointer
    let c_string = match CString::new(dtr_string) {
        Ok(s) => s,
        Err(_) => return ptr::null(),
    };

    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_cstring(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

mod tests;
