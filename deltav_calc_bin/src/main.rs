use std::sync::{Arc, Mutex};
use gtk::{Application, ApplicationWindow, Box, Button, Expander, Label, Orientation, Widget, Window};
use gtk::prelude::*;
use deltav_calc::{DeltavMap, MenuTree};

const APP_ID: &str = "vck.zll.deltav_calc";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

// Defines if the origin or the target should be selected
enum Selection {
    ORIGIN,
    TARGET
}

// Builds the ui
fn build_ui(app: &Application) {
    // The deltav map to use
    let map = DeltavMap::get_stock();

    // Defines if the origin or the target should be selected
    let sel = Arc::new(Mutex::new(Selection::ORIGIN));

    // The tree to select a node
    let selection_tree = build_tree(
        map.get_menu_tree(),
        |button: &Button| {println!("{}", button.label().unwrap().as_str())});

    // The window for the node selection
    let select_window = Arc::new(Window::builder()
        .title("Select a node")
        .child(&selection_tree)
        .build());

    // THe button to click when you want to set te start node
    let origin_button = Button::builder()
        .label("Click here to select the start")
        .build();
    // When clicked open the selection window
    let sel_clone = sel.clone();
    let select_window_clone = select_window.clone();
    origin_button.connect_clicked(move |_| {
        let mut sel = sel_clone.lock().unwrap();
        *sel = Selection::ORIGIN;
        drop(sel);
        show_selection(&select_window_clone);
    });

    // THe button to click when you want to set te end node
    let target_button = Button::builder()
        .label("Click here to select the end")
        .build();
    // When clicked open the selection window
    let sel_clone = sel.clone();
    let select_window_clone = select_window.clone();
    target_button.connect_clicked(move |_| {
        let mut sel = sel_clone.lock().unwrap();
        *sel = Selection::TARGET;
        drop(sel);
        show_selection(&select_window_clone);
    });

    let result_label = Label::new(None);
    set_result(&result_label, &map,origin_button.label().unwrap().as_str(), target_button.label().unwrap().as_str());

    // Build the layout everything is put in
    let layout = Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    BoxExt::append(&layout, &origin_button);
    BoxExt::append(&layout, &result_label);
    BoxExt::append(&layout, &target_button);

    // Build the final window
    let window = ApplicationWindow::builder()
        .title("Deltav Calculator")
        .application(app)
        .child(&layout)
        .hide_on_close(false)
        .build();
    window.show();
}

// Gets called when a node should be selected
fn show_selection(select_window: &Arc<Window>) {
    select_window.show();
}

// Uses the map to calculate the delta v needed to get from start to end and puts it into the result label
fn set_result(result_label: &Label, map: &DeltavMap, start: &str, end: &str) {
    match map.calculate_delta_v(start, end) {
        Err(e) => {
            if e.get_cause_name() == start {
                result_label.set_label("The start node hasn't been selected yet");
            } else {
                result_label.set_label("The end node hasn't been selected yet");
            }
        }

        Ok(result) => {
            match result {
                None => {
                    result_label.set_label("There seems to be no connection between the nodes")
                }

                Some(result) => {
                    result_label.set_label(&result.to_string());
                }
            }
        }
    }
}

// Builds the node selection tree
fn build_tree(tree: &MenuTree, click_callback: fn(&Button)) -> Widget{
    return match tree {
        MenuTree::MiddleNode { name, children } => {
            let layout = Box::builder()
                .orientation(Orientation::Vertical)
                .margin_start(10)
                .build();

            let expander = Expander::builder()
                .label(&name)
                .child(&layout)
                .build();

            for child in children {
                BoxExt::append(&layout, &build_tree(child, click_callback))
            }

            Widget::from(expander)
        }

        MenuTree::EndNode { name, .. } => {
            let button = Button::builder()
                .label(&name)
                .build();
            let click_callback_clone = click_callback.clone();
            button.connect_clicked(move |button| {
                click_callback_clone(button);
            });

            Widget::from(button)
        }
    }
}