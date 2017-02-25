extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{Window, WindowType, Stack, StackSwitcher, Box, Orientation, HeaderBar};

mod programs;
mod firewall;
mod hosts;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(950, 700);
    
    //Header
    let header = HeaderBar::new();
    header.set_show_close_button(true);
    window.set_titlebar(Some(&header));

    //Basic structure
    let stack_switcher = StackSwitcher::new();
    let stack = stack_manager();
    stack_switcher.set_stack(Some(&stack));
    
    //Box to center the header
    let header_box = Box::new(Orientation::Horizontal, 0);
    header_box.pack_start(&stack_switcher, true, false, 250);

    //Main box
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 50);

    //h_box.pack_start(&stack_switcher, true, false, 50);
    header.pack_start(&header_box);
    v_box.pack_start(&h_box, false, false, 15);
    v_box.pack_start(&stack, true, true, 20);
   
    //Add the box to the window
    window.add(&v_box);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

 fn stack_manager() -> Stack {
    let stack = Stack::new();
    stack.add_titled(&programs::get_box(), "Programs", "Programs");
    stack.add_titled(&firewall::get_box(), "Firewall", "Firewall");
    stack.add_titled(&hosts::get_box(), "Forbiden webs", "Forbiden webs");

    return stack;
}
 
