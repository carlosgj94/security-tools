extern crate gdk;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Label, Box, Orientation};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn get_box() -> Box {
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 10);

    h_box.pack_start(&Label::new("Forbiden Webs"), true, false, 0);
    v_box.pack_start(&h_box, false, false, 0);
    let file_string = get_hosts_file();
    println!("{}", &file_string);

    return v_box;
}

fn get_hosts_file() -> String {
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open("/etc/hosts") {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open '/etc/hosts': {}",why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read '/etc/hosts': {}", why.description()),
        Ok(_) => print!("'/etc/hosts' contains:\n{}", s),
    }
    return s;
}
