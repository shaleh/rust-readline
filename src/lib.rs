#![crate_type = "lib"]
extern crate libc;

use std::c_str;

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
        ext_readline::add_history(line.to_c_str().as_ptr());
    }
}

pub fn readline(prompt: &str) -> Option<String> {
    let cprmt = prompt.to_c_str();
    unsafe {
        let ret = ext_readline::readline(cprmt.as_ptr());
        if ret.is_null() {  // user pressed Ctrl-D
            None
        }
        else {
            c_str::CString::new(ret, true).as_str().map(|ret| ret.to_string())
        }
    }
}
