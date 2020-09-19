extern crate gio;
extern crate gtk;
extern crate glib;

use gtk::prelude::*;
use gio::prelude::*;
use glib::clone;

use gtk::{ApplicationWindow, Builder, TextView, Button};

use std::env::args;
use std::rc::Rc;
use std::cell::RefCell;

fn build_ui(application: &gtk::Application) {
    let calc = Rc::new(RefCell::new(String::new()));

    let glade_src = include_str!("./ui/gtk3.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window").expect("Cound't get window");
    let text: TextView = builder.get_object("text").expect("Cound't get text view");
    let text_buffer = text.get_buffer().expect("Can't ct");

    let button_equal: Button = builder.get_object("=").expect("Coun't get Button");
    button_equal.connect_clicked(clone!(@weak text_buffer => move |_| {
        text_buffer.set_text("=");
    }));

    let button_undo: Button = builder.get_object("undo").expect("Coun't get Button");
    let calc_clone = calc.clone();
    button_undo.connect_clicked(clone!(@weak text_buffer => move |_| {
        calc_clone.borrow_mut().pop();
        text_buffer.set_text(&calc_clone.borrow_mut());
    }));

    let button_undo: Button = builder.get_object("clear").expect("Coun't get Button");
    let calc_clone = calc.clone();
    button_undo.connect_clicked(clone!(@weak text_buffer => move |_| {
        calc_clone.borrow_mut().clear();
        text_buffer.set_text(&calc_clone.borrow_mut());
    }));

    window.set_application(Some(application));
    let digits: &'static [char; 16] = &['1','2','3','4','5','6','7','8','9','0',',','-','+','/','x','%'];
    for digit in digits.iter() {
        let button:Button = builder.get_object(&digit.to_string()).expect("Cout't get Button");
        let calc_clone = calc.clone();
        button.connect_clicked(clone!(@weak text => move |_| {
            match digit {
                '+'|'/'|'x'|','|'%' => {
                    let last_char = calc_clone.borrow_mut().chars().last().unwrap_or_default();
                    println!("{}:{}", digit, last_char)
                },
                _ => {
                    calc_clone.borrow_mut().push(*digit);
                    text.get_buffer().expect("Can't get buffer").set_text(&calc_clone.borrow_mut());
                }
            }
        }));
    }
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.altart.calculadora"),
        Default::default(),
    )
    .expect("Initialization failed");
    application.connect_activate(|app| {
        build_ui(app)
    });
    application.run(&args().collect::<Vec<_>>());
}