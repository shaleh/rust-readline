//! A Simple wrapper for libreadline or libedit
//!
//! Exports two functions:
//!
//!   - `add_history`
//!   - `readline`
//!

extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;
use std::string::String;

mod ext_readline {
    use libc::c_char;

    extern "C" {
        pub fn add_history(line: *const c_char);
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}

/// Update the internal history of input lines
///
/// Call this after a successful `readline()` call to add that line to the
/// history.
pub fn add_history(line: &str) {
    unsafe {
        ext_readline::add_history(CString::new(line).unwrap().as_ptr());
    }
}

/// Invoke the external `readline()`. 
///
/// Returns an `Option<String>` representing whether a `String` was returned
/// or NULL. `None` indicates the user has signal end of input.
pub fn readline(prompt: &str) -> Option<String> {
    let cprmt = CString::new(prompt).unwrap().as_ptr();
    unsafe {
        let ret = ext_readline::readline(cprmt);
        if ret.is_null() {  // user pressed Ctrl-D
            None
        }
        else {
            let slice = CStr::from_ptr(ret);
            let bytes = slice.to_bytes();

            // the return from readline needs to be explicitly freed
            // so clone the input first
            let line = String::from_utf8_lossy(bytes).into_owned().clone();

            libc::free(ret as *mut libc::c_void);

            Some(line)
        }
    }
}
