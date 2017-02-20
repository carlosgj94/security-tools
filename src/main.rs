extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{Window, WindowType, Stack, StackSwitcher, Label, Box, Orientation};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(950, 700);

    //Add the box to the window
    window.add(&box_structure());
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

fn box_structure() -> Box {
    //Stack and StackSwitcher
    
    let stack_switcher = StackSwitcher::new();
    let stack = stack_manager();
    stack_switcher.set_stack(Some(&stack));
        
    //Main box
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 50);

    h_box.pack_start(&stack_switcher, true, false, 50);
    v_box.pack_start(&h_box, false, false, 15);
    v_box.pack_start(&stack, true, true, 20);
   
    return v_box;
}

fn stack_manager() -> Stack {
    let stack = Stack::new();
    stack.add_titled(&programs_stack(), "Programs", "Programs");
    stack.add_titled(&firewall_stack(), "Firewall", "Firewall");
    stack.add_titled(&hosts_stack(), "Forbiden webs", "Forbiden webs");

    return stack;
}

fn programs_stack() -> Box {
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 10);

    h_box.pack_start(&Label::new("Programs"), true, false, 0);
    v_box.pack_start(&h_box, false, false, 0);

    return v_box;
}

fn firewall_stack() -> Box {
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 10);

    h_box.pack_start(&Label::new("Firewall"), true, false, 0);
    v_box.pack_start(&h_box, false, false, 0);

    return v_box;
}

fn hosts_stack() -> Box {
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 10);

    h_box.pack_start(&Label::new("Forbiden Webs"), true, false, 0);
    v_box.pack_start(&h_box, false, false, 0);

    return v_box;
}
