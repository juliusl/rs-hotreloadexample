use std::{ffi::CStr, os::raw::c_char};

// new!
#[no_mangle]
pub unsafe extern "C" fn greet(name: *const c_char) {
    let cstr = CStr::from_ptr(name);
    println!("Vodka vodka whiskey, {}!", cstr.to_str().unwrap());
}
