use std::sync::{Arc, Mutex};
use gtk::{Application, ApplicationWindow, Box, Button, Expander, Inhibit, Label, Orientation, Widget, Window};
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
    let map = Arc::new(DeltavMap::get_stock());

    // Defines if the origin or the target should be selected
    let sel = Arc::new(Mutex::new(Selection::ORIGIN));

    // The window for the node selection
    let select_window = Arc::new(Window::builder()
        .title("Select a node")
        .build());

    // THe button to click when you want to set te start node
    let origin_button = Arc::new(Button::builder()
        .label("Click here to select the start")
        .build());

    // THe button to click when you want to set te end node
    let target_button = Arc::new(Button::builder()
        .label("Click here to select the end")
        .build());
    // When clicked open the selection window
    let sel_clone = sel.clone();
    let select_window_clone = select_window.clone();
    let origin_button_clone = origin_button.clone();
    let target_button_clone = target_button.clone();
    target_button.connect_clicked(move |_| {
        let mut sel = sel_clone.lock().unwrap();
        *sel = Selection::TARGET;
        drop(sel);
        show_selection(&select_window_clone, &origin_button_clone, &target_button_clone);
    });
    // When origin button is clicked open the selection window
    let sel_clone = sel.clone();
    let select_window_clone = select_window.clone();
    let origin_button_clone = origin_button.clone();
    let target_button_clone = target_button.clone();
    origin_button.connect_clicked(move |_| {
        let mut sel = sel_clone.lock().unwrap();
        *sel = Selection::ORIGIN;
        drop(sel);
        show_selection(&select_window_clone, &origin_button_clone, &target_button_clone);
    });

    let result_label = Label::new(None);
    set_result(&result_label, &map,origin_button.label().unwrap().as_str(), target_button.label().unwrap().as_str());

    // Build the layout everything is put in
    let layout = Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    layout.append(&*origin_button);
    layout.append(&result_label);
    layout.append(&*target_button);

    let sel_clone = sel.clone();
    let map_clone = map.clone();
    let select_window_clone = select_window.clone();
    let origin_button_clone = origin_button.clone();
    let target_button_clone = target_button.clone();
    let selection_tree = build_tree(
        map.get_menu_tree(),
        Arc::new(move |button: &Button| {
            selected(button.label().unwrap().as_str(),
                     &sel_clone,
                     &*origin_button_clone,
                     &*target_button_clone,
                     &result_label,
                     &map_clone,
                     &select_window_clone);
        }));
    select_window.set_child(Some(&selection_tree));

    let origin_button_clone = origin_button.clone();
    let target_button_clone = target_button.clone();
    select_window.connect_close_request(move |window| {
        close_selection(window, &origin_button_clone, &target_button_clone);
        Inhibit(true)
    });

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
fn show_selection(select_window: &Arc<Window>, start_button: &Arc<Button>, end_button: &Arc<Button>) {
    start_button.set_sensitive(false);
    end_button.set_sensitive(false);
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
fn build_tree(tree: &MenuTree, click_callback: Arc<impl Fn(&Button) + 'static>) -> Widget{
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
                let cloned_callback = click_callback.clone();
                layout.append(&build_tree(child, cloned_callback));
            }

            Widget::from(expander)
        }

        MenuTree::EndNode { name, .. } => {
            let button = Button::builder()
                .label(&name)
                .build();
            button.connect_clicked(move |button| {
                click_callback(button);
            });

            Widget::from(button)
        }
    }
}

// Updates the selected button and the result label
fn selected(selection: &str, to_change: &Arc<Mutex<Selection>>, start: &Button, end: &Button, result: &Label, map: &DeltavMap, select_window: &Arc<Window>) {
    let to_change = to_change.lock().unwrap();
    match *to_change {
        Selection::ORIGIN => {
            start.set_label(selection);
        }
        Selection::TARGET => {
            end.set_label(selection);
        }
    }
    set_result(result, map, start.label().unwrap().as_str(), end.label().unwrap().as_str());
    close_selection(&*select_window, start, end);
}

// Closes the selection window and activates the buttons
fn close_selection(select_window: &Window, start_button: &Button, end_button: &Button) {
    select_window.hide();
    start_button.set_sensitive(true);
    end_button.set_sensitive(true);
}