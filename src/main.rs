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

    //Stack and StackSwitcher
    let stack = Stack::new();
    stack.add_titled(&Label::new("Programs"), "Programs", "Programs");
    stack.add_titled(&Label::new("Firewall"), "Firewall", "Firewall");
    stack.add_titled(&Label::new("Forbiden webs"), "Forbiden webs", "Forbiden webs");

    let stack_switcher = StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));
        
    //Main box
    let v_box = Box::new(Orientation::Vertical, 10);
    let h_box = Box::new(Orientation::Horizontal, 50);


    h_box.pack_start(&stack_switcher, true, false, 50);
    v_box.pack_start(&h_box, false, false, 20);
    v_box.pack_start(&stack, true, true, 20);

    //Add the box to the window
    window.add(&v_box);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    /*button.connect_clicked(|_| {
        println!("Clicked!");
    });*/

    gtk::main();
}