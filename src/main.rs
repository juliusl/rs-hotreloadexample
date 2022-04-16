use cstr::cstr;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{os::raw::c_char, sync::mpsc::channel, time::Duration, path::PathBuf};

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
                if let DebouncedEvent::Write(path) = event {
                    if path.ends_with("libgreet.so") {
                        if let Err(e) = load_and_print(&path) {
                            eprintln!("error: {}", e);
                        }
                    }
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
        greet(cstr!("reloading").as_ptr());

        Ok(())
    }
}
