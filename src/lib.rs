//! A Simple wrapper for libreadline or libedit
//!
//! Exports seven functions:
//!
//!   - `add_history`
//!   - `history`
//!   - `history_expand`
//!   - `history_is_stifled`
//!   - `stifle_history`
//!   - `unstifle_history`
//!   - `readline`
//!

extern crate libc;

use libc::c_char;

use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;
use std::string::String;

mod ext_readline {
    use libc::{c_char, c_void};

    #[repr(C)]
    pub struct HIST_ENTRY {
	pub line: *const c_char,
        pub data: *mut c_void,
    }

    extern "C" {
        /* History Support */
        pub fn add_history(line: *const c_char);
        pub fn next_history() -> *const HIST_ENTRY;
        pub fn previous_history() -> *const HIST_ENTRY;
        pub fn history_expand(input: *const c_char, expansion: *mut *mut c_char) -> i32;
        pub fn stifle_history(n: i32);
        pub fn unstifle_history() -> i32;
        pub fn history_is_stifled() -> i32;

        /* readline */
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

pub fn stifle_history(n: i32) {
    unsafe {
        ext_readline::stifle_history(n);
    }
}

pub fn unstifle_history() -> i32 {
    unsafe {
        ext_readline::unstifle_history()
    }
}

pub fn history_is_stifled() -> bool {
    unsafe {
        ext_readline::history_is_stifled() != 0
    }
}

pub fn history_expand(input: &str) -> Result<Option<String>, String> {
    unsafe {
        let mut expansion: *mut c_char = ptr::null_mut();
        let result = ext_readline::history_expand(CString::new(input).unwrap().as_ptr(),
                                                  (&mut expansion));
        if result == 0 {
            return Ok(None);
        }

        let slice = CStr::from_ptr(expansion);
        let bytes = slice.to_bytes();
        let output = String::from_utf8_lossy(bytes).into_owned().clone();

        libc::free(expansion as *mut libc::c_void);

        if result < 0 || result == 2 {
            Err(output)
        }
        else {
            Ok(Some(output))
        }
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

pub fn history() -> Vec<String> {
  unsafe {
    loop {
        let value = ext_readline::previous_history();
        if value.is_null() {
            break;
        }
    }

    let mut result: Vec<String> = Vec::new();

    loop {
        let value = ext_readline::next_history();
        if value.is_null() {
            break;
        }

        let slice = CStr::from_ptr((*value).line);
        let bytes = slice.to_bytes();
        let output = String::from_utf8_lossy(bytes).into_owned().clone();

        result.push(output);
    }

    result
  }
}
