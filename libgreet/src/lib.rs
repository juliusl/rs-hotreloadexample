use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

// new!
#[no_mangle]
pub unsafe extern "C" fn greet(name: *const c_char) {
    let cstr = CStr::from_ptr(name);
    println!("Greetings, {}!", cstr.to_str().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn greet_format(name: *const c_char) -> *mut c_char {
    let cstr = CStr::from_ptr(name);

    if let Ok(formatted) = cstr.to_str() {
        let formatted = format!("formatting {}", formatted);

        let val = match CString::new(formatted.as_str()) {
            Ok(v) => v,
            Err(_) => CString::default(),
        };

        val.into_raw()
    } else {
        CString::default().into_raw()
    }
}
