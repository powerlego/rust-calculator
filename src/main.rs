mod window;
mod skeleton;
mod integer_object;
use gtk::prelude::*;
use gtk::{gio, glib};

use window::Window;
const APP_ID: &str = "com.nc.Calculator";

fn main() -> glib::ExitCode{
    gio::resources_register_include!("resource.gresource").expect("Failed to include our compiled resources");

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &adw::Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}