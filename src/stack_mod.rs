extern crate gdk;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Label, Box, Orientation};

pub fn programs_stack() -> Box {
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 10);

    h_box.pack_start(&Label::new("Programs"), true, false, 0);
    v_box.pack_start(&h_box, false, false, 0);

    return v_box;
}

pub fn firewall_stack() -> Box {
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 10);

    h_box.pack_start(&Label::new("Firewall"), true, false, 0);
    v_box.pack_start(&h_box, false, false, 0);

    return v_box;
}

pub fn hosts_stack() -> Box {
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 10);

    h_box.pack_start(&Label::new("Forbiden Webs"), true, false, 0);
    v_box.pack_start(&h_box, false, false, 0);

    return v_box;
}
