use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Box, Button, Entry, Label, Orientation};
use std::rc::Rc;
use std::cell::RefCell;
use crate::calculator::{Calculator, Operation};

const APP_ID: &str = "org.example.CalculatorApp";

pub fn run_gui_mode() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let calculator = Rc::new(Calculator::new());
    let current_input = Rc::new(RefCell::new(String::new()));
    let first_number = Rc::new(RefCell::new(0.0));
    let operation = Rc::new(RefCell::new(None::<Operation>));
    let waiting_for_operand = Rc::new(RefCell::new(true));

    // Create the main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Calculator")
        .default_width(300)
        .default_height(400)
        .build();

    // Create the main container
    let vbox = Box::new(Orientation::Vertical, 5);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);

    // Display entry
    let display = Entry::new();
    display.set_text("0");
    display.set_editable(false);
    gtk4::prelude::EditableExt::set_alignment(&display, 1.0); // Right align
    vbox.append(&display);

    // Create button grid
    let button_box = Box::new(Orientation::Vertical, 5);

    // Row 1: Clear and operations
    let row1 = Box::new(Orientation::Horizontal, 5);
    row1.set_homogeneous(true);
    
    let clear_btn = Button::with_label("C");
    let divide_btn = Button::with_label("/");
    let multiply_btn = Button::with_label("*");
    let subtract_btn = Button::with_label("-");
    
    row1.append(&clear_btn);
    row1.append(&divide_btn);
    row1.append(&multiply_btn);
    row1.append(&subtract_btn);
    button_box.append(&row1);

    // Row 2: 7, 8, 9
    let row2 = Box::new(Orientation::Horizontal, 5);
    row2.set_homogeneous(true);
    
    let btn7 = Button::with_label("7");
    let btn8 = Button::with_label("8");
    let btn9 = Button::with_label("9");
    let add_btn = Button::with_label("+");
    
    row2.append(&btn7);
    row2.append(&btn8);
    row2.append(&btn9);
    row2.append(&add_btn);
    button_box.append(&row2);

    // Row 3: 4, 5, 6
    let row3 = Box::new(Orientation::Horizontal, 5);
    row3.set_homogeneous(true);
    
    let btn4 = Button::with_label("4");
    let btn5 = Button::with_label("5");
    let btn6 = Button::with_label("6");
    let placeholder1 = Label::new(Some(""));
    
    row3.append(&btn4);
    row3.append(&btn5);
    row3.append(&btn6);
    row3.append(&placeholder1);
    button_box.append(&row3);

    // Row 4: 1, 2, 3
    let row4 = Box::new(Orientation::Horizontal, 5);
    row4.set_homogeneous(true);
    
    let btn1 = Button::with_label("1");
    let btn2 = Button::with_label("2");
    let btn3 = Button::with_label("3");
    let equals_btn = Button::with_label("=");
    
    row4.append(&btn1);
    row4.append(&btn2);
    row4.append(&btn3);
    row4.append(&equals_btn);
    button_box.append(&row4);

    // Row 5: 0, .
    let row5 = Box::new(Orientation::Horizontal, 5);
    row5.set_homogeneous(true);
    
    let btn0 = Button::with_label("0");
    let dot_btn = Button::with_label(".");
    let placeholder2 = Label::new(Some(""));
    let placeholder3 = Label::new(Some(""));
    
    row5.append(&btn0);
    row5.append(&dot_btn);
    row5.append(&placeholder2);
    row5.append(&placeholder3);
    button_box.append(&row5);

    vbox.append(&button_box);
    window.set_child(Some(&vbox));

    // Number button connections
    let number_buttons = vec![
        (&btn0, "0"), (&btn1, "1"), (&btn2, "2"), (&btn3, "3"),
        (&btn4, "4"), (&btn5, "5"), (&btn6, "6"), (&btn7, "7"),
        (&btn8, "8"), (&btn9, "9")
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

    // Equals button
    {
        let display_clone = display.clone();
        let current_input_clone = current_input.clone();
        let first_number_clone = first_number.clone();
        let operation_clone = operation.clone();
        let waiting_for_operand_clone = waiting_for_operand.clone();
        let calculator_clone = calculator.clone();

        equals_btn.connect_clicked(move |_| {
            // Clone the operation to avoid borrowing conflicts
            let op = operation_clone.borrow().clone();
            if let Some(operation) = op {
                let input = current_input_clone.borrow().clone();
                if let Ok(second_number) = input.parse::<f64>() {
                    let first = *first_number_clone.borrow();
                    match calculator_clone.calculate(first, second_number, operation) {
                        Ok(result) => {
                            let result_str = if result.fract() == 0.0 {
                                format!("{}", result as i64)
                            } else {
                                format!("{}", result)
                            };
                            display_clone.set_text(&result_str);
                            current_input_clone.replace(result_str);
                        }
                        Err(_e) => {
                            display_clone.set_text("Error");
                            current_input_clone.replace("0".to_string());
                        }
                    }
                    *operation_clone.borrow_mut() = None;
                    *waiting_for_operand_clone.borrow_mut() = true;
                }
            }
        });
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

    window.present();
}