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
        let imp = self.imp();
        if let Ok(mut path) = File::open(settings_path()) {
            let mut contents = String::new();
            path.read_to_string(&mut contents)
                .expect("Failed to read settings file");
            let doc = contents.parse::<DocumentMut>().expect("Failed to parse settings file");

            // Get settings
            let settings = doc.get("settings").expect("Failed to get settings table");

            let persistent_keypad = settings
                .get("persistent_keypad")
                .expect("Failed to get persistent_keypad value")
                .as_bool()
                .expect("Failed to get persistent_keypad as bool");
            let keypad_expanded = settings
                .get("keypad_expanded")
                .expect("Failed to get keypad_expanded value")
                .as_bool()
                .expect("Failed to get keypad_expanded as bool");
            let history_expanded = settings
                .get("history_expanded")
                .expect("Failed to get history_expanded value")
                .as_bool()
                .expect("Failed to get history_expanded as bool");
            let convert_expanded = settings
                .get("convert_expanded")
                .expect("Failed to get convert_expanded value")
                .as_bool()
                .expect("Failed to get convert_expanded as bool");

            // Get window settings
            let window_settings = doc.get("window").expect("Failed to get window table");

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

            // Set settings
            imp.persistent_keypad.set(persistent_keypad);
            imp.keypad_lock.set_icon_name(
                if persistent_keypad {
                    "changes-prevent-symbolic"
                }
                else {
                    "changes-allow-symbolic"
                },
            );
            imp.expander_keypad.set_expanded(
                (persistent_keypad && keypad_expanded)
                    || (!persistent_keypad && keypad_expanded && !history_expanded && !convert_expanded),
            );
            imp.expander_history.set_expanded(
                (persistent_keypad && history_expanded)
                    || (!persistent_keypad && !keypad_expanded && history_expanded && !convert_expanded),
            );
            imp.expander_convert.set_expanded(
                (persistent_keypad && convert_expanded)
                    || (!persistent_keypad && !keypad_expanded && !history_expanded && convert_expanded),
            );

            // Set window settings
            if !keypad_expanded && !history_expanded && !convert_expanded {
                self.set_default_size(window_width, 76);
            }
            else {
                self.set_default_size(window_width, window_height);
            }
            if !((persistent_keypad && keypad_expanded)
                || (!persistent_keypad && keypad_expanded && !history_expanded && !convert_expanded))
                && !((persistent_keypad && (history_expanded || convert_expanded))
                    || (!persistent_keypad && !keypad_expanded && (history_expanded || convert_expanded)))
            {
                imp.keypad_buttons.set_visible(false);
                imp.tabs.set_visible(false);
            }
            else {
                imp.show_keypad_widget(
                    (persistent_keypad && keypad_expanded)
                        || (!persistent_keypad && keypad_expanded && !history_expanded && !convert_expanded),
                );
                imp.show_tabs(
                    (persistent_keypad && (history_expanded || convert_expanded))
                        || (!persistent_keypad && !keypad_expanded && (history_expanded || convert_expanded)),
                );
            }
            if is_maximized {
                self.maximize();
            }
        }
        else {
            // Set default settings
            imp.persistent_keypad.set(false);
            imp.keypad_lock.set_icon_name("changes-allow-symbolic");
            imp.expander_keypad.set_expanded(true);
            imp.expander_keypad.set_expanded(false);
            imp.expander_keypad.set_expanded(true);
            imp.show_keypad_widget(true);
            imp.show_tabs(false);

            // Set default window settings
            self.set_default_size(675, 76);
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

    fn setup_callbacks(&self) {
        self.imp()
            .keypad_lock
            .connect_clicked(clone!(@weak self as window => move |_|{
                window.imp().persistent_keypad.set(!window.imp().persistent_keypad.get());
                window.imp().update_persistent_keypad(false);
            }));
    }

    fn create_rows(&self) {
        let vector: Vec<IntegerObject> = (0..=10).map(IntegerObject::new).collect();
        self.mem_hist().extend_from_slice(&vector);
    }
}
