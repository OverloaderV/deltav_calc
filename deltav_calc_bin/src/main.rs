use gtk::{Application, ApplicationWindow};
use gtk::prelude::*;


const APP_ID: &str = "vck.zll.deltav_calc";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

// Builds the ui
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .title("Deltav Calculator")
        .application(app)
        .build();
    window.show();
}
