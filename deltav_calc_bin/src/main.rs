use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Expander, Widget};
use gtk::Orientation::Vertical;
use deltav_calc;
use deltav_calc::MenuTree;


const APP_ID: &str = "vck.zll.deltav_calc";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Deltav Calculator")
        .build();

    let map = deltav_calc::DeltavMap::get_stock();
    let tree = map.get_menu_tree();
    window.set_child(Some(&build_tree(tree)));

    window.present();
}

fn build_tree(tree: &MenuTree) -> Widget {
    return match tree {
        MenuTree::MiddleNode { children, .. } => {
            let vbox = gtk::Box::builder()
                .orientation(Vertical)
                .margin_start(20)
                .build();

            for child in children {
                BoxExt::append(&vbox, &build_tree(child));
            }

            Widget::from(Expander::builder()
                .label(tree.get_name())
                .child(&vbox)
                .build())
        }
        MenuTree::EndNode { .. } => {
            let button = Button::builder()
                .label(tree.get_name())
                .width_request(100)
                .build();
            button.connect_clicked(tree_clicked);

            Widget::from(button)
        }
    }
}

fn tree_clicked(button: &Button) {
    println!("{}", button.label().unwrap());
}