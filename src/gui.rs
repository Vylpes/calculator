use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Box, Button, CssProvider, Entry, IconTheme, Label,
    Orientation, Window,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use crate::calculator::{Calculator, Operation};

const APP_ID: &str = "com.vylpes.vyxcalc";
const ICON_NAME: &str = "com.vylpes.vyxcalc";
const ICON_PNG: &[u8] = include_bytes!("../assets/icon.png");

const STYLE: &str = r#"
window {
    background-color: #F5F5F5;
}

.calc-root {
    background-color: #F5F5F5;
}

.calc-display {
    background-color: #FFFFFF;
    color: #444444;
    border: none;
    border-radius: 12px;
    padding: 18px 16px;
    font-size: 28px;
    font-weight: 500;
    box-shadow: inset 0 2px 5px rgba(0, 0, 0, 0.08);
    min-height: 28px;
}

.calc-display:focus {
    box-shadow: inset 0 2px 5px rgba(0, 0, 0, 0.08);
    outline: none;
    border: none;
}

.calc-btn {
    background: #FFFFFF;
    color: #444444;
    border: none;
    border-radius: 12px;
    font-size: 20px;
    font-weight: 500;
    min-width: 56px;
    min-height: 52px;
    padding: 8px;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.10);
    transition: 100ms ease-in-out;
}

.calc-btn:hover {
    background: #FAFAFA;
}

.calc-btn:active {
    box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.10);
}

.calc-btn-clear {
    color: #FF9800;
    border: 1px solid rgba(255, 152, 0, 0.35);
}

.calc-btn-blank,
.calc-btn-blank:hover,
.calc-btn-blank:active,
.calc-btn-blank:disabled {
    background: #FFFFFF;
    color: transparent;
    opacity: 1;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.10);
}

.calc-btn-equals-primary {
    background-image: linear-gradient(to bottom, #FFC133, #FF9800);
    color: #FFFFFF;
    font-weight: 600;
    box-shadow: 0 2px 6px rgba(255, 152, 0, 0.35);
}

.calc-btn-equals-primary:hover {
    background-image: linear-gradient(to bottom, #FFCA4D, #FFA31A);
}

.calc-btn-equals-primary:active {
    background-image: linear-gradient(to bottom, #FF9800, #F57C00);
    box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.15);
}
"#;

pub fn run_gui_mode() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    // Clap already consumed argv (e.g. "gui"). Passing those through to
    // GApplication makes it treat leftover args as files to open and fail with
    // "This application can not open files". Only forward the program name.
    let program = std::env::args().next().unwrap_or_else(|| "calculator-app".to_string());
    app.run_with_args(&[program])
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(STYLE);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn cache_dir() -> PathBuf {
    if let Some(xdg) = std::env::var_os("XDG_CACHE_HOME") {
        return PathBuf::from(xdg);
    }
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home).join(".cache");
    }
    std::env::temp_dir()
}

/// GTK4 windows use icon-name (looked up in the icon theme). Embed the PNG in
/// the binary and expose it via a local hicolor search path.
fn setup_window_icon(window: &ApplicationWindow) {
    let Some(display) = gtk4::gdk::Display::default() else {
        return;
    };

    let icon_root = cache_dir().join("vyxcalc/icons");
    let apps_dir = icon_root.join("hicolor/128x128/apps");
    if std::fs::create_dir_all(&apps_dir).is_err() {
        return;
    }

    let icon_path = apps_dir.join(format!("{ICON_NAME}.png"));
    if std::fs::write(&icon_path, ICON_PNG).is_err() {
        return;
    }

    let theme = IconTheme::for_display(&display);
    theme.add_search_path(&icon_root);
    Window::set_default_icon_name(ICON_NAME);
    window.set_icon_name(Some(ICON_NAME));
}

fn styled_button(label: &str, classes: &[&str]) -> Button {
    let button = Button::with_label(label);
    button.add_css_class("calc-btn");
    for class in classes {
        button.add_css_class(class);
    }
    button.set_hexpand(true);
    button.set_vexpand(true);
    button
}

fn build_ui(app: &Application) {
    load_css();

    let calculator = Rc::new(Calculator::new());
    let current_input = Rc::new(RefCell::new(String::new()));
    let first_number = Rc::new(RefCell::new(0.0));
    let operation = Rc::new(RefCell::new(None::<Operation>));
    let waiting_for_operand = Rc::new(RefCell::new(true));

    // Size to content — avoid leftover empty space under the keypad
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Calculator")
        .default_width(300)
        .resizable(false)
        .build();
    setup_window_icon(&window);

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.add_css_class("calc-root");
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);

    let display = Entry::new();
    display.set_text("0");
    display.set_editable(false);
    display.set_can_focus(false);
    display.add_css_class("calc-display");
    gtk4::prelude::EditableExt::set_alignment(&display, 1.0);
    vbox.append(&display);

    let button_box = Box::new(Orientation::Vertical, 10);

    // Row 1: C / * -
    let row1 = Box::new(Orientation::Horizontal, 10);
    row1.set_homogeneous(true);

    let clear_btn = styled_button("C", &["calc-btn-clear"]);
    let divide_btn = styled_button("/", &[]);
    let multiply_btn = styled_button("*", &[]);
    let subtract_btn = styled_button("-", &[]);

    row1.append(&clear_btn);
    row1.append(&divide_btn);
    row1.append(&multiply_btn);
    row1.append(&subtract_btn);
    button_box.append(&row1);

    // Row 2: 7 8 9 +
    let row2 = Box::new(Orientation::Horizontal, 10);
    row2.set_homogeneous(true);

    let btn7 = styled_button("7", &[]);
    let btn8 = styled_button("8", &[]);
    let btn9 = styled_button("9", &[]);
    let add_btn = styled_button("+", &[]);

    row2.append(&btn7);
    row2.append(&btn8);
    row2.append(&btn9);
    row2.append(&add_btn);
    button_box.append(&row2);

    // Row 3: 4 5 6 [blank]
    let row3 = Box::new(Orientation::Horizontal, 10);
    row3.set_homogeneous(true);

    let btn4 = styled_button("4", &[]);
    let btn5 = styled_button("5", &[]);
    let btn6 = styled_button("6", &[]);
    let blank_btn = styled_button("", &["calc-btn-blank"]);
    blank_btn.set_sensitive(false);
    blank_btn.set_can_focus(false);

    row3.append(&btn4);
    row3.append(&btn5);
    row3.append(&btn6);
    row3.append(&blank_btn);
    button_box.append(&row3);

    // Row 4: 1 2 3 =
    let row4 = Box::new(Orientation::Horizontal, 10);
    row4.set_homogeneous(true);

    let btn1 = styled_button("1", &[]);
    let btn2 = styled_button("2", &[]);
    let btn3 = styled_button("3", &[]);
    let equals_btn = styled_button("=", &[]);

    row4.append(&btn1);
    row4.append(&btn2);
    row4.append(&btn3);
    row4.append(&equals_btn);
    button_box.append(&row4);

    // Row 5: 0 . [spacer] = (primary)
    let row5 = Box::new(Orientation::Horizontal, 10);
    row5.set_homogeneous(true);

    let btn0 = styled_button("0", &[]);
    let dot_btn = styled_button(".", &[]);
    let spacer = Label::new(None);
    let equals_primary_btn = styled_button("=", &["calc-btn-equals-primary"]);

    row5.append(&btn0);
    row5.append(&dot_btn);
    row5.append(&spacer);
    row5.append(&equals_primary_btn);
    button_box.append(&row5);

    vbox.append(&button_box);
    window.set_child(Some(&vbox));

    // Number button connections
    let number_buttons = vec![
        (&btn0, "0"),
        (&btn1, "1"),
        (&btn2, "2"),
        (&btn3, "3"),
        (&btn4, "4"),
        (&btn5, "5"),
        (&btn6, "6"),
        (&btn7, "7"),
        (&btn8, "8"),
        (&btn9, "9"),
    ];

    for (button, digit) in number_buttons {
        let display_clone = display.clone();
        let current_input_clone = current_input.clone();
        let waiting_for_operand_clone = waiting_for_operand.clone();
        let digit = digit.to_string();

        button.connect_clicked(move |_| {
            if *waiting_for_operand_clone.borrow() {
                current_input_clone.borrow_mut().clear();
                *waiting_for_operand_clone.borrow_mut() = false;
            }

            current_input_clone.borrow_mut().push_str(&digit);
            display_clone.set_text(&current_input_clone.borrow());
        });
    }

    // Decimal point button
    {
        let display_clone = display.clone();
        let current_input_clone = current_input.clone();
        let waiting_for_operand_clone = waiting_for_operand.clone();

        dot_btn.connect_clicked(move |_| {
            if *waiting_for_operand_clone.borrow() {
                current_input_clone.replace("0.".to_string());
                *waiting_for_operand_clone.borrow_mut() = false;
            } else if !current_input_clone.borrow().contains('.') {
                current_input_clone.borrow_mut().push('.');
            }
            display_clone.set_text(&current_input_clone.borrow());
        });
    }

    // Operation buttons
    let operation_buttons = vec![
        (&add_btn, Operation::Add),
        (&subtract_btn, Operation::Subtract),
        (&multiply_btn, Operation::Multiply),
        (&divide_btn, Operation::Divide),
    ];

    for (button, op) in operation_buttons {
        let current_input_clone = current_input.clone();
        let first_number_clone = first_number.clone();
        let operation_clone = operation.clone();
        let waiting_for_operand_clone = waiting_for_operand.clone();

        button.connect_clicked(move |_| {
            let input = current_input_clone.borrow().clone();
            if let Ok(number) = input.parse::<f64>() {
                *first_number_clone.borrow_mut() = number;
                *operation_clone.borrow_mut() = Some(op.clone());
                *waiting_for_operand_clone.borrow_mut() = true;
            }
        });
    }

    let evaluate = {
        let display = display.clone();
        let current_input = current_input.clone();
        let first_number = first_number.clone();
        let operation = operation.clone();
        let waiting_for_operand = waiting_for_operand.clone();
        let calculator = calculator.clone();

        Rc::new(move || {
            let op = operation.borrow().clone();
            if let Some(operation_value) = op {
                let input = current_input.borrow().clone();
                if let Ok(second_number) = input.parse::<f64>() {
                    let first = *first_number.borrow();
                    match calculator.calculate(first, second_number, operation_value) {
                        Ok(result) => {
                            let result_str = if result.fract() == 0.0 {
                                format!("{}", result as i64)
                            } else {
                                format!("{}", result)
                            };
                            display.set_text(&result_str);
                            current_input.replace(result_str);
                        }
                        Err(_e) => {
                            display.set_text("Error");
                            current_input.replace("0".to_string());
                        }
                    }
                    *operation.borrow_mut() = None;
                    *waiting_for_operand.borrow_mut() = true;
                }
            }
        })
    };

    {
        let evaluate = evaluate.clone();
        equals_btn.connect_clicked(move |_| evaluate());
    }
    {
        let evaluate = evaluate.clone();
        equals_primary_btn.connect_clicked(move |_| evaluate());
    }

    // Clear button
    {
        let display_clone = display.clone();
        let current_input_clone = current_input.clone();
        let first_number_clone = first_number.clone();
        let operation_clone = operation.clone();
        let waiting_for_operand_clone = waiting_for_operand.clone();

        clear_btn.connect_clicked(move |_| {
            display_clone.set_text("0");
            current_input_clone.replace("0".to_string());
            *first_number_clone.borrow_mut() = 0.0;
            *operation_clone.borrow_mut() = None;
            *waiting_for_operand_clone.borrow_mut() = true;
        });
    }

    // Keyboard input mirrors button behaviour
    {
        let display_clone = display.clone();
        let current_input_clone = current_input.clone();
        let first_number_clone = first_number.clone();
        let operation_clone = operation.clone();
        let waiting_for_operand_clone = waiting_for_operand.clone();
        let evaluate = evaluate.clone();

        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            use gtk4::gdk::Key;

            let handle_digit = |digit: &str| {
                if *waiting_for_operand_clone.borrow() {
                    current_input_clone.borrow_mut().clear();
                    *waiting_for_operand_clone.borrow_mut() = false;
                }
                current_input_clone.borrow_mut().push_str(digit);
                display_clone.set_text(&current_input_clone.borrow());
            };

            let handle_op = |op: Operation| {
                let input = current_input_clone.borrow().clone();
                if let Ok(number) = input.parse::<f64>() {
                    *first_number_clone.borrow_mut() = number;
                    *operation_clone.borrow_mut() = Some(op);
                    *waiting_for_operand_clone.borrow_mut() = true;
                }
            };

            match key {
                Key::_0 | Key::KP_0 => handle_digit("0"),
                Key::_1 | Key::KP_1 => handle_digit("1"),
                Key::_2 | Key::KP_2 => handle_digit("2"),
                Key::_3 | Key::KP_3 => handle_digit("3"),
                Key::_4 | Key::KP_4 => handle_digit("4"),
                Key::_5 | Key::KP_5 => handle_digit("5"),
                Key::_6 | Key::KP_6 => handle_digit("6"),
                Key::_7 | Key::KP_7 => handle_digit("7"),
                Key::_8 | Key::KP_8 => handle_digit("8"),
                Key::_9 | Key::KP_9 => handle_digit("9"),
                Key::period | Key::KP_Decimal => {
                    if *waiting_for_operand_clone.borrow() {
                        current_input_clone.replace("0.".to_string());
                        *waiting_for_operand_clone.borrow_mut() = false;
                    } else if !current_input_clone.borrow().contains('.') {
                        current_input_clone.borrow_mut().push('.');
                    }
                    display_clone.set_text(&current_input_clone.borrow());
                }
                Key::plus | Key::KP_Add => handle_op(Operation::Add),
                Key::minus | Key::KP_Subtract => handle_op(Operation::Subtract),
                Key::asterisk | Key::KP_Multiply => handle_op(Operation::Multiply),
                Key::slash | Key::KP_Divide => handle_op(Operation::Divide),
                Key::Return | Key::KP_Enter | Key::equal | Key::KP_Equal => evaluate(),
                Key::Escape | Key::Delete | Key::c | Key::C => {
                    display_clone.set_text("0");
                    current_input_clone.replace("0".to_string());
                    *first_number_clone.borrow_mut() = 0.0;
                    *operation_clone.borrow_mut() = None;
                    *waiting_for_operand_clone.borrow_mut() = true;
                }
                Key::BackSpace => {
                    let mut input = current_input_clone.borrow_mut();
                    input.pop();
                    if input.is_empty() {
                        *waiting_for_operand_clone.borrow_mut() = true;
                        display_clone.set_text("0");
                    } else {
                        display_clone.set_text(&input);
                    }
                }
                _ => return glib::Propagation::Proceed,
            }

            glib::Propagation::Stop
        });
        window.add_controller(key_controller);
    }

    window.present();
}
