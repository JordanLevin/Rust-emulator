extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType, Dialog};
use std::thread;
use std::io::{self, Read};

fn main(){
    run();
}

fn run() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    //let button = Button::new_with_label("Click me!");
    //window.add(&button);
    let dialog = Dialog::new();
    window.add(&dialog);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
        let mut buffer: [u8;1] = [1];
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        handle.read_exact(&mut buffer);

            println!("{:?}", buffer);
    });

    gtk::main();
}
