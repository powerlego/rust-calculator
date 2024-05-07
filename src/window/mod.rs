//! This module contains the implementation of the [`Window`] object. The [`Window`] object is a subclass of
//! [`adw::ApplicationWindow`] and is the main window of the application.

mod imp;

use std::fs::File;
use std::io::Read;

use adw::subclass::prelude::*;
use gio::ActionEntry;
use glib::{clone, Object};
use gtk::glib::object::Cast;
use gtk::prelude::*;
use gtk::{gio, glib, NoSelection};
use toml_edit::DocumentMut;

use crate::integer_object::IntegerObject;
use crate::skeleton::Skeleton;
use crate::utils::{display_thousands_separator, settings_path};

// use crate::APP_ID;

glib::wrapper! {
    /// The Main [`Window`] of the application. It is a subclass of [`adw::ApplicationWindow`].
    /// # Actions
    /// The [`Window`] implements the following actions:
    /// * `num-insert` - Inserts a number into the display.
    /// * `op-insert` - Inserts an operator into the display.
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    /// Creates a new [`Window`].
    pub fn new(app: &adw::Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    /// Load the settings from the settings file and apply them to the window.
    /// If the settings file does not exist, it will set the default settings.
    ///
    /// # Arguments
    /// * `self` - The [`Window`] object.
    fn load_settings(&self) {
        let imp = self.imp();
        imp.tabs.set_visible(false);
        imp.keypad_buttons.set_visible(false);
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
            let window_height = 76;
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

    /// The [`gio::ListStore`] representing the calculation history.
    fn history(&self) -> gio::ListStore {
        self.imp()
            .history
            .borrow()
            .clone()
            .expect("Could not get current mem_hist")
    }

    /// Sets up the history list on first creation.
    fn setup_history(&self) {
        let model = gio::ListStore::new::<IntegerObject>();
        self.imp().history.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.history()));
        self.imp().mem_hist_list.bind_model(
            Some(&selection_model),
            clone!(@weak self as window => @default-panic, move |_|{
                let row = window.create_integer_row();
                row.upcast()
            }),
        );
    }

    /// Creates a new row widget for the history list.
    fn create_integer_row(&self) -> Skeleton {
        let row = Skeleton::new();
        row
    }

    /// Sets up the callbacks utilized by the child widgets.
    fn setup_callbacks(&self) {
        self.imp()
            .keypad_lock
            .connect_clicked(clone!(@weak self as window => move |_|{
                window.imp().persistent_keypad.set(!window.imp().persistent_keypad.get());
                window.imp().update_persistent_keypad(false);
            }));

        self.imp().input_display.connect_paste_clipboard(clone!(@weak self as window => move |input_display| {
            input_display.block_signal(&window.imp().input_display_changed_signal.borrow().as_ref().expect("Could not get input_display_changed_signal"));
            input_display.set_text("");
            input_display.unblock_signal(&window.imp().input_display_changed_signal.borrow().as_ref().expect("Could not get input_display_changed_signal"));
        }));
        self.imp()
            .input_display_changed_signal
            .replace(Some(self.imp().input_display.connect_changed(
                clone!(@weak self as window => move |disp| {
                    let binding = disp.text();
                    let text = binding.trim();
                    println!("Text: {}", text);
                    if text.is_empty(){
                        window.imp().input_display.block_signal(&window.imp().input_display_changed_signal.borrow().as_ref().expect("Could not get input_display_changed_signal"));
                        disp.set_text("0");
                        window.imp().input_display.unblock_signal(&window.imp().input_display_changed_signal.borrow().as_ref().expect("Could not get input_display_changed_signal"));
                    }
                    else {
                        let text = display_thousands_separator(text);
                        window.imp().input_display.block_signal(&window.imp().input_display_changed_signal.borrow().as_ref().expect("Could not get input_display_changed_signal"));
                        disp.set_text(&text);
                        window.imp().input_display.unblock_signal(&window.imp().input_display_changed_signal.borrow().as_ref().expect("Could not get input_display_changed_signal"));
                    }
                }),
            )));
    }

    /// Creates the rows for the history list.
    fn create_rows(&self) {
        let vector: Vec<IntegerObject> = (0..=10).map(IntegerObject::new).collect();
        self.history().extend_from_slice(&vector);
    }

    /// Sets up the actions for the [`Window`].
    fn setup_actions(&self) {
        let action_num_insert = ActionEntry::builder("num-insert")
            .parameter_type(Some(&i32::static_variant_type()))
            .activate(move |_: &Self, _action, parameter| {
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<i32>()
                    .expect("The variant needs to be of type `i32`.");

                println!("Num insert: {}", parameter);
            })
            .build();
        let action_op_insert = ActionEntry::builder("op-insert")
            .parameter_type(Some(&String::static_variant_type()))
            .activate(move |_: &Self, _action, parameter| {
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<String>()
                    .expect("The variant needs to be of type `String`.");

                println!("Op insert: {}", parameter);
            })
            .build();

        self.add_action_entries([action_num_insert]);
        self.add_action_entries([action_op_insert]);
    }
}
