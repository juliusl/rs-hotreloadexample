use cstr::cstr;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{os::raw::c_char, sync::mpsc::channel, time::Duration, path::PathBuf, ffi::{CStr, CString}};

use libloading::{Library, Symbol};

fn main() {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher
        .watch(
            "target/debug/deps",
            RecursiveMode::Recursive,
        )
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Write(path) => if path.ends_with("libgreet.so") {
                        if let Err(e) = load_and_print(&path) {
                            eprintln!("error: {}", e);
                        }
                    },
                    // TODO: Not sure how to handle MacOS/Apple Silicon yet 
                    DebouncedEvent::Create(path) => if path.ends_with("libgreet.dylib") {
                        if let Err(e) = load_and_print(&path) {
                            eprintln!("error: {}", e);
                        }
                    },
                    _ => continue, 
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn load_and_print(libpath: &PathBuf) -> Result<(), libloading::Error> {
    unsafe {
        let lib = Library::new(libpath)?;
        let greet: Symbol<unsafe extern "C" fn(name: *const c_char)> = lib.get(b"greet")?;
        let format: Symbol<unsafe extern "C" fn(name: *const c_char) -> *mut c_char> = lib.get(b"greet_format")?; 
        greet(cstr!("reloading").as_ptr());

        let output = format(cstr!(formatting).as_ptr());

        // Since we're getting a *mut c_char from this function, we need to 
        // use CString::from_raw to handle the ptr safely
        let output = CString::from_raw(output);

        println!("Result: {}", output.to_str().unwrap());

        Ok(())
    }
}
