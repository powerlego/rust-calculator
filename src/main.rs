mod basic_numpad;
mod integer_object;
mod skeleton;
mod utils;
mod window;
use gdk::Display;
use gtk::prelude::*;
use gtk::{gdk, gio, glib, CssProvider};
use window::Window;
const APP_ID: &str = "com.nc.Calculator";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("resource.gresource").expect("Failed to include our compiled resources");

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        load_css();
    });

    app.connect_activate(build_ui);

    app.run()
}

/// Loads the CSS from the resource file
fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/com/nc/calculator/style.css");
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Builds the UI
/// # Arguments
/// * `app` - The application
/// # Returns
/// None
fn build_ui(app: &adw::Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
