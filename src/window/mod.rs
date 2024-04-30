mod imp;

use std::fs::File;
use std::io::Read;

use adw::subclass::prelude::*;
use glib::{clone, Object};
use gtk::glib::object::Cast;
use gtk::prelude::*;
use gtk::{gio, glib, NoSelection};
use toml_edit::DocumentMut;

use crate::integer_object::IntegerObject;
use crate::skeleton::Skeleton;
use crate::utils::settings_path;

// use crate::APP_ID;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn load_settings(&self) {
        if let Ok(mut path) = File::open(settings_path()) {
            let mut contents = String::new();
            path.read_to_string(&mut contents)
                .expect("Failed to read settings file");
            let doc = contents.parse::<DocumentMut>().expect("Failed to parse settings file");

            let settings = doc.get("settings").expect("Failed to get settings table");
            let window_settings = doc.get("window").expect("Failed to get window table");
            let persistent_keypad = settings
                .get("persistent_keypad")
                .expect("Failed to get persistent_keypad value")
                .as_bool()
                .expect("Failed to get persistent_keypad as bool");
            let window_width = i32::try_from(
                window_settings
                    .get("width")
                    .expect("Failed to get width value")
                    .as_integer()
                    .expect("Failed to get width as integer"),
            )
            .expect("Failed to convert width to i32");
            let window_height = i32::try_from(
                window_settings
                    .get("height")
                    .expect("Failed to get height value")
                    .as_integer()
                    .expect("Failed to get height as integer"),
            )
            .expect("Failed to convert height to i32");
            let is_maximized = window_settings
                .get("is_maximized")
                .expect("Failed to get maximized value")
                .as_bool()
                .expect("Failed to get maximized as bool");
            self.imp().persistent_keypad.set(persistent_keypad);
            self.set_default_size(window_width, window_height);
            if is_maximized {
                self.maximize();
            }
        }
    }

    fn mem_hist(&self) -> gio::ListStore {
        self.imp()
            .mem_hist
            .borrow()
            .clone()
            .expect("Could not get current mem_hist")
    }

    fn setup_mem_hist(&self) {
        let model = gio::ListStore::new::<IntegerObject>();
        self.imp().mem_hist.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.mem_hist()));
        self.imp().mem_hist_list.bind_model(
            Some(&selection_model),
            clone!(@weak self as window => @default-panic, move |_|{
                let row = window.create_integer_row();
                row.upcast()
            }),
        );
    }

    fn create_integer_row(&self) -> Skeleton {
        let row = Skeleton::new();
        row
    }

    fn create_rows(&self) {
        let vector: Vec<IntegerObject> = (0..=10).map(IntegerObject::new).collect();
        self.mem_hist().extend_from_slice(&vector);
    }
}
