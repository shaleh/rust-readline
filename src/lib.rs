#![crate_id = "readline"]
#![crate_type = "lib"]

use std::c_str;

mod ext_readline {
    use std::libc::c_char;
    #[link(name = "readline")]
    extern {
        pub fn add_history (line: *c_char);
        pub fn readline (p: *c_char) -> * c_char;
    }
}

pub fn add_history(line: &str) {
    let c_line = line.to_c_str();
    c_line.with_ref(|c_buf| {
        unsafe {
            ext_readline::add_history(c_buf);
        }
    });
}

pub fn readline (prompt: &str) -> Option<~str> {
    let cprmt = prompt.to_c_str();
    cprmt.with_ref(|c_buf| {
        unsafe {
            let ret = c_str::CString::new (ext_readline::readline (c_buf), true);
            if ret.is_null() {  // user pressed Ctrl-D
                None
            }
            else {
                ret.as_str().map(|ret| ret.to_owned())
            }
        }
    })
}
