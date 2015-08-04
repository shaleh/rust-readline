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

pub fn add_history(line: &str) {
    unsafe {
        ext_readline::add_history(CString::new(line).unwrap().as_ptr());
    }
}

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

            Some(String::from_utf8_lossy(bytes).into_owned())
        }
    }
}
