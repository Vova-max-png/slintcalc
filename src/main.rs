use std::{cell::RefCell, rc::Rc};

slint::include_modules!();

#[derive(Default)]
struct CalcState {
    prevValue: f32,
    currentValue: f32,
    operator: slint::SharedString,
}

fn main() {
    let mut dot = false;
    let app = MainWindow::new().unwrap();
    let weak = app.as_weak();
    let state = Rc::new(RefCell::new(CalcState::default()));

    app.global::<CalcLogic>().on_button_pressed(move |value| {
        let app = weak.unwrap();
        let mut state = state.borrow_mut();
        if let Ok(val) = value.parse::<f32>() {
            if dot == true {
                state.currentValue = (state.currentValue.to_string() + "." + &val.to_string()).parse().unwrap();    
                app.set_value(state.currentValue);
                dot = false;
                return;
            }
            state.currentValue = (state.currentValue.to_string() + &val.to_string()).parse().unwrap();
            app.set_value(state.currentValue);
            return;
        }

        if value.as_str() == "=" {
            if (state.operator == "") {
                return;
            }
            let result = match state.operator.as_str() {
                "+" => state.prevValue + state.currentValue,
                "-" => state.prevValue - state.currentValue,
                "*" => state.prevValue * state.currentValue,
                "/" => state.prevValue / state.currentValue,
                "%" => state.prevValue / state.currentValue * 100.0,
                _ => 0.0
            };
            app.set_value(result);
            state.currentValue = result;
            state.prevValue = 0.0;
            state.operator = "".into();
        } else {
            if value.as_str() == "CE" {
                state.prevValue = 0.0;
                state.currentValue = 0.0;
                state.operator = "".into();
                app.set_value(0.0);
            }

            else if value.as_str() == "C" {
                state.currentValue = 0.0;
                app.set_value(state.currentValue);
            }

            else if value.as_str() == "." {
                dot = true;
            }

            else if value.as_str() == "Back" {
                let strCurrentValue = state.currentValue.to_string();
                let newStrCurrentValue = strCurrentValue[0..strCurrentValue.len() - 1].parse::<f32>();
                state.currentValue = match newStrCurrentValue {
                    Err(_) => {0.0},
                    _ => {newStrCurrentValue.unwrap()}
                };
                app.set_value(state.currentValue);
            }

            else {
                if state.operator != "" {
                    state.operator = value.clone();
                    return;
                }
                state.operator = value.clone();
                state.prevValue = state.currentValue;
                state.currentValue = 0.0;
            }
        }
    });
    
    app.run().unwrap();
}