#![crate_type = "lib"]
extern crate libc;

use std::str;
use std::ffi::{CString, c_str_to_bytes};

mod ext_readline {
    use libc::c_char;
    #[link(name = "readline")]
    extern {
        pub fn add_history(line: *const c_char);
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}

pub fn add_history(line: &str) {
    unsafe {
        ext_readline::add_history(CString::from_slice(line.as_bytes()).as_ptr());
    }
}

pub fn readline(prompt: &str) -> Option<String> {
    let cprmt = CString::from_slice(prompt.as_bytes());
    unsafe {
        let ret = ext_readline::readline(cprmt.as_ptr());
        if ret.is_null() {  // user pressed Ctrl-D
            None
        }
        else {
            match str::from_utf8(c_str_to_bytes(&ret)) {
                Ok(s) => Some(s.to_string()),

                _ => None
            }
        }
    }
}

