extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate meval;

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

    let digits: &'static [char; 19] = &['1','2','3','4','5','6','7','8','9','0','.','-','+','/','*','%','c','u','='];
    for digit in digits.iter() {
        let button:Button = builder.get_object(&digit.to_string()).expect("Cout't get Button");
        let calc_clone = calc.clone();
        button.connect_clicked(clone!(@weak text_buffer => move |_| {
            let last_char = calc_clone.borrow_mut().chars().last().unwrap_or_default();
            let is_number = last_char.to_string().parse::<u8>().is_ok();
            match digit {
                '+'|'/'|'*'|'.'|'%' => {
                    if is_number {
                        calc_clone.borrow_mut().push(*digit);
                    }
                },
                '-' => {
                    if last_char == '\u{0}' || is_number {
                        calc_clone.borrow_mut().push(*digit);
                    }
                },
                'c' => {
                    calc_clone.borrow_mut().clear();
                },
                'u' => {
                    calc_clone.borrow_mut().pop();
                },
                '=' => {
                    let result = meval::eval_str(&*calc_clone.borrow_mut());
                    match result {
                        Ok(number) => {
                            text_buffer.set_text(&number.to_string());
                            calc_clone.replace(number.to_string());
                        },
                        Err(_) => {
                            text_buffer.set_text("Error");
                            calc_clone.borrow_mut().clear();
                        },
                    }
                },
                _ => {
                    calc_clone.borrow_mut().push(*digit);
                }
            }
            text_buffer.set_text(&calc_clone.borrow());
        }));
    }

    window.connect_key_press_event(
        move |_, key| {
            let keyval = key.get_keyval().to_unicode().unwrap_or_default();
            let key_state = key.get_state();
            let key_code = key.get_hardware_keycode();
            let last_char = calc.borrow_mut().chars().last().unwrap_or_default();
            let is_number = last_char.to_string().parse::<u8>().is_ok();
            match keyval {
                '+'|'/'|'*'|'.'|'%' => {
                    if is_number {
                        calc.borrow_mut().push(keyval);
                    }
                },
                _ => {
                    if !key_state.intersects(gdk::ModifierType::SHIFT_MASK) {
                        match key_code {
                            36 => {
                                let result = meval::eval_str(&*calc.borrow_mut());
                                match result {
                                    Ok(number) => {
                                        text_buffer.set_text(&number.to_string());
                                        calc.replace(number.to_string());
                                    },
                                    Err(_) => {
                                        text_buffer.set_text("Error");
                                        calc.borrow_mut().clear();
                                    },
                                }
                            },
                            20 => {
                                if last_char == '\u{0}' || is_number {
                                    calc.borrow_mut().push(keyval);
                                }
                            },
                            22 => {
                                calc.borrow_mut().pop();
                            }
                            10..=19 =>{
                                calc.borrow_mut().push(keyval);
                            }
                            _ => {}
                        }
                    }
                }
            }
            
            text_buffer.set_text(&calc.borrow());
            Inhibit(false)
        },
    );
    window.set_application(Some(application));
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.altart.calculadora"),
        Default::default(),
    )
    .expect("Initialization failed.");
    application.connect_activate(|app| {
        build_ui(app)
    });
    application.run(&args().collect::<Vec<_>>());
}
