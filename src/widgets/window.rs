//! This module contains the implementation of the [`Window`] object. The [`Window`] object is a subclass of
//! [`adw::ApplicationWindow`] and is the main window of the application.
use std::fs::File;
use std::io::Read;

use adw::subclass::prelude::*;
use gdk::Key;
use gio::ActionEntry;
use glib::{clone, Object};
use gtk::glib::object::Cast;
use gtk::prelude::*;
use gtk::{gdk, gio, glib, EventControllerKey, NoSelection};
use toml_edit::DocumentMut;

use crate::integer_object::IntegerObject;
use crate::utils::{display_thousands_separator, settings_path};
use crate::widgets::Skeleton;

mod imp {
    //! The private implementation details of the [`Window`] object.

    use std::cell::{Cell, RefCell};
    use std::fs::File;
    use std::io::Write;

    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use glib::SignalHandlerId;
    use gtk::prelude::*;
    use gtk::{gio, glib, Box, Button, CompositeTemplate, Expander, ListBox, Notebook, Text};
    use toml_edit::{table, value, DocumentMut};

    use crate::utils::settings_path;
    use crate::widgets::{BasicNumpad, Skeleton};

    /// The `Window` widget. It is the main window of the application.
    /// 
    /// # Actions
    /// 
    /// The `Window` implements the following actions:
    /// * `num-insert` - Inserts a number into the display.
    /// * `op-insert` - Inserts an operator into the display.
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/nc/calculator/window.ui")]
    pub struct Window {
        #[template_child]
        pub mem_hist_list:                TemplateChild<ListBox>,
        #[template_child]
        pub tabs:                         TemplateChild<Notebook>,
        #[template_child]
        pub expander_keypad:              TemplateChild<Expander>,
        #[template_child]
        pub expander_history:             TemplateChild<Expander>,
        #[template_child]
        pub expander_convert:             TemplateChild<Expander>,
        #[template_child]
        pub keypad_buttons:               TemplateChild<Box>,
        #[template_child]
        pub keypad_lock:                  TemplateChild<Button>,
        #[template_child]
        pub input_display:                TemplateChild<Text>,
        #[template_child]
        pub basic_numpad:                 TemplateChild<BasicNumpad>,
        pub input_display_changed_signal: RefCell<Option<SignalHandlerId>>,
        pub persistent_keypad:            Cell<bool>,
        pub history:                      RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        type ParentType = adw::ApplicationWindow;
        type Type = super::Window;

        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "MainWindow";

        fn class_init(klass: &mut Self::Class) {
            Skeleton::ensure_type();
            BasicNumpad::ensure_type();

            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl Window {

        /// Callback for the `on_expander_keypad_expanded` signal.
        /// If the keypad expander is expanded, it will show the keypad buttons and hide the tabs if persistent keypad
        /// is `false`. If the keypad expander is collapsed, it will hide the keypad buttons. If the keypad expander is
        /// expanded and the history expander or the convert expander was expanded, it will collapse the history or the
        /// convert expander respectively.
        /// 
        /// # Arguments
        /// 
        /// * `_p` - The parameter spec. (Unused)
        #[template_callback]
        fn on_expander_keypad_expanded(&self, _p: glib::ParamSpec) {
            if self.expander_keypad.is_expanded() {
                self.show_keypad_widget(true);
                if !self.persistent_keypad.get() {
                    if self.expander_history.is_expanded() {
                        self.expander_history.set_expanded(false);
                    }
                    if self.expander_convert.is_expanded() {
                        self.expander_convert.set_expanded(false);
                    }
                }
            }
            else {
                self.show_keypad_widget(false);
            }
        }

        /// Callback for the `on_expander_history_expanded` signal.
        /// 
        /// If the history expander is expanded, it will show the tabs and hide the keypad buttons if persistent keypad
        /// is `false`. If the history expander is collapsed and the convert expander is not expanded, it will
        /// hide the tabs. If the history expander is expanded and the convert expander was expanded, it will
        /// collapse the convert expander.
        /// 
        /// # Arguments
        /// 
        /// * `_p` - The parameter spec. (Unused)
        #[template_callback]
        fn on_expander_history_expanded(&self, _p: glib::ParamSpec) {
            if self.expander_history.is_expanded() {
                self.tabs.set_current_page(Some(0));
                self.show_tabs(true);
                if !self.persistent_keypad.get() && self.expander_keypad.is_expanded() {
                    self.expander_keypad.set_expanded(false);
                }
                if self.expander_convert.is_expanded() {
                    self.expander_convert.set_expanded(false);
                }
            }
            else if !self.expander_convert.is_expanded() {
                self.show_tabs(false);
            }
        }

        /// Callback for the `on_expander_convert_expanded` signal.
        /// If the convert expander is expanded, it will show the tabs and hide the keypad buttons if persistent keypad
        /// is `false`. If the convert expander is collapsed and the history expander is not expanded, it will
        /// hide the tabs. If the convert expander is expanded and the history expander was expanded, it will
        /// collapse the history expander.
        /// 
        /// # Arguments
        /// 
        /// * `_p` - The parameter spec. (Unused)
        #[template_callback]
        fn on_expander_convert_expanded(&self, _p: glib::ParamSpec) {
            if self.expander_convert.is_expanded() {
                self.tabs.set_current_page(Some(1));
                self.show_tabs(true);
                if !self.persistent_keypad.get() && self.expander_keypad.is_expanded() {
                    self.expander_keypad.set_expanded(false);
                }
                if self.expander_history.is_expanded() {
                    self.expander_history.set_expanded(false);
                }
            }
            else if !self.expander_history.is_expanded() {
                self.show_tabs(false);
            }
        }

        /// Shows or hides the keypad buttons based on the value of `do_show`.
        /// If `do_show` is `true`, it will show the keypad buttons. If `do_show` is `false`, it will hide the keypad
        /// buttons. If the keypad buttons are already visible and `do_show` is `true`, it will do nothing.
        /// If persistent keypad is `true` and the keypad buttons are not visible, it will show the keypad buttons and
        /// keep the tabs visible. If persistent keypad is `false` and the tabs are visible, it will hide the
        /// tabs.
        ///
        /// # Arguments
        ///
        /// * `do_show` - A boolean indicating whether to show or hide the keypad buttons.
        pub fn show_keypad_widget(&self, do_show: bool) {
            if do_show == self.keypad_buttons.is_visible() {
                return;
            }
            let (w, mut h) = self.obj().default_size();
            let persistent_keypad = self.persistent_keypad.get();

            if !persistent_keypad && self.tabs.is_visible() {
                h -= self.tabs.height() + 9;
            }
            if persistent_keypad && self.expander_convert.is_expanded() {
                if do_show {
                    h += 6;
                }
                else {
                    h -= 6;
                }
            }
            if do_show {
                self.keypad_buttons.set_visible(true);
                let kb_h = self.keypad_buttons.height();
                if kb_h > 10 {
                    h += kb_h + 9;
                }
                else {
                    h += 9;
                }
                if !persistent_keypad {
                    self.tabs.set_visible(false);
                }
                self.obj().set_default_size(w, h);
            }
            else {
                h -= self.keypad_buttons.height() + 9;
                self.keypad_buttons.set_visible(false);
                self.obj().set_default_size(w, h);
            }
            self.keypad_buttons
                .set_vexpand(!persistent_keypad || !self.tabs.is_visible());
        }

        /// Shows or hides the tabs based on the value of `do_show`.
        /// If `do_show` is `true`, it will show the tabs. If `do_show` is `false`, it will hide the tabs.
        /// If the tabs are already visible and `do_show` is `true`, it will do nothing.
        /// If persistent keypad is `false` and the keypad buttons are visible, it will hide the keypad buttons.
        ///
        /// # Arguments
        ///
        /// * `do_show` - A boolean indicating whether to show or hide the tabs.
        pub fn show_tabs(&self, do_show: bool) {
            if do_show == self.tabs.is_visible() {
                return;
            }
            let (w, mut h) = self.obj().default_size();
            let persistent_keypad = self.persistent_keypad.get();

            if !persistent_keypad && self.keypad_buttons.is_visible() {
                h -= self.keypad_buttons.height() + 9;
            }
            if do_show {
                self.tabs.set_visible(true);
                let t_h = self.tabs.height();
                if t_h > 10 {
                    h += t_h + 9;
                }
                else {
                    h += 9;
                }
                if !persistent_keypad {
                    self.keypad_buttons.set_visible(false);
                }
                self.obj().set_default_size(w, h);
            }
            else {
                h -= self.tabs.height() + 9;
                self.tabs.set_visible(false);
                self.obj().set_default_size(w, h);
            }
            self.keypad_buttons
                .set_vexpand(!persistent_keypad || !self.tabs.is_visible());
        }

        /// Updates the persistent keypad state. If `show_hide_buttons` is `true`, it will show or hide the keypad
        /// buttons. If the keypad is persistent, it will always show the keypad buttons.
        /// If the keypad is not persistent, it will show the keypad buttons if the tabs are not visible.
        ///
        /// # Arguments
        ///
        /// * `show_hide_buttons` - A boolean indicating whether to show or hide the keypad buttons.
        pub fn update_persistent_keypad(&self, mut show_hide_buttons: bool) {
            let persistent_keypad = self.persistent_keypad.get();
            if !persistent_keypad && self.tabs.is_visible() {
                show_hide_buttons = true;
            }
            self.keypad_buttons
                .set_vexpand(!persistent_keypad || !self.tabs.is_visible());
            if show_hide_buttons && (persistent_keypad || self.tabs.is_visible()) {
                self.expander_keypad.set_expanded(persistent_keypad);
                if persistent_keypad {
                    self.keypad_buttons.set_visible(true);
                }
                else {
                    self.show_keypad_widget(false);
                }
            }
            self.keypad_lock.set_icon_name(
                if persistent_keypad {
                    "changes-prevent-symbolic"
                }
                else {
                    "changes-allow-symbolic"
                },
            );
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for Window {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            self.input_display.set_text("0");

            let obj = self.obj();
            obj.load_settings();
            obj.setup_callbacks();
            obj.setup_actions();
            obj.setup_event_controllers();
            obj.setup_history();
            obj.create_rows();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for Window {}

    // Trait shared by all windows
    impl WindowImpl for Window {
        fn close_request(&self) -> glib::Propagation {
            // Save settings
            let mut settings_table = table();
            settings_table["persistent_keypad"] = value(self.persistent_keypad.get());
            settings_table["keypad_expanded"] = value(self.expander_keypad.is_expanded());
            settings_table["history_expanded"] = value(self.expander_history.is_expanded());
            settings_table["convert_expanded"] = value(self.expander_convert.is_expanded());

            // Window Settings
            let mut window_settings = table();
            let (w, _h) = self.obj().default_size();
            window_settings["width"] = value(i64::try_from(w).expect("Cannot convert width to i64"));
            window_settings["is_maximized"] = value(self.obj().is_maximized());

            let mut doc = DocumentMut::new();
            doc.insert("settings", settings_table);
            doc.insert("window", window_settings);

            let mut file = File::create(settings_path()).expect("Failed to create settings file");
            file.write(doc.to_string().as_bytes())
                .expect("Failed to write settings file");

            // Pass close request on to the parent
            self.parent_close_request()
        }
    }

    // Trait shared by all application windows
    impl ApplicationWindowImpl for Window {}

    // Trait shared by all adwaita application windows
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    /// The Main [`Window`] of the application. It is a subclass of [`adw::ApplicationWindow`].
    /// 
    /// # Actions
    /// 
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
    ///
    /// # Panics
    ///
    /// * Panics if the settings file cannot be read or parsed.
    /// * Panics if the settings table cannot be retrieved.
    /// * Panics if the settings values cannot be retrieved.
    /// * Panics if the window settings table cannot be retrieved.
    /// * Panics if the width value cannot be retrieved.
    /// * Panics if the width value cannot be converted to an `i32`.
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
    ///
    /// # Returns
    ///
    /// The calculation history list store.
    ///
    /// # Panics
    ///
    /// * Panics if the current mem_hist is `None`.
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
    ///
    /// # Returns
    ///
    /// The new row widget.
    fn create_integer_row(&self) -> Skeleton {
        let row = Skeleton::new();
        row
    }

    /// Inserts the given text into the input display.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to be inserted.
    ///
    /// # Panics
    ///
    /// * Panics if the input display changed signal cannot be retrieved.
    fn insert_display_text(&self, text: &str) {
        if self.imp().input_display.text().as_str() == "0" {
            self.imp().input_display.block_signal(
                &self
                    .imp()
                    .input_display_changed_signal
                    .borrow()
                    .as_ref()
                    .expect("Could not get input_display_changed_signal"),
            );
            self.imp().input_display.set_text("");
            self.imp().input_display.unblock_signal(
                &self
                    .imp()
                    .input_display_changed_signal
                    .borrow()
                    .as_ref()
                    .expect("Could not get input_display_changed_signal"),
            );
        }
        let mut pos = -1;
        self.imp().input_display.insert_text(text, &mut pos);
    }

    /// Sets the text of the input display.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to set the input display to.
    fn set_display_text(&self, text: &str) {
        self.imp().input_display.block_signal(
            &self
                .imp()
                .input_display_changed_signal
                .borrow()
                .as_ref()
                .expect("Could not get input_display_changed_signal"),
        );
        self.imp().input_display.set_text(text);
        self.imp().input_display.unblock_signal(
            &self
                .imp()
                .input_display_changed_signal
                .borrow()
                .as_ref()
                .expect("Could not get input_display_changed_signal"),
        );
    }

    /// Sets up the event controllers for the [`Window`].
    /// The event controllers are used to handle key presses and releases.
    /// The key presses and releases are used to interact with the calculator.
    fn setup_event_controllers(&self) {
        let controller = EventControllerKey::builder()
            .name("keypad-controller")
            .propagation_phase(gtk::PropagationPhase::Target)
            .propagation_limit(gtk::PropagationLimit::SameNative)
            .build();
        controller.connect_key_pressed(
            clone!(@weak self as window => @default-return glib::Propagation::Stop, move |_controller, key, _keyval, _state| {
                match key {
                    Key::BackSpace => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_backspace
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        if window.imp().input_display.text().as_str() != "0" {
                            window
                                .imp()
                                .input_display
                                .delete_text((window.imp().input_display.text().len() - 1) as i32, -1);
                        }
                    }
                    Key::_0 | Key::KP_0 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_zero
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text("0");
                    }
                    Key::_1 | Key::KP_1 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_one
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text("1");
                    }
                    Key::_2 | Key::KP_2 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_two
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text("2");
                    }
                    Key::_3 | Key::KP_3 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_three
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text("3");
                    }
                    Key::_4 | Key::KP_4 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_four
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text("4");
                    }
                    Key::_5 | Key::KP_5 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_five
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text("5");
                    }
                    Key::_6 | Key::KP_6 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_six
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text("6");
                    }
                    Key::_7 | Key::KP_7 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_seven
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text("7");
                    }
                    Key::_8 | Key::KP_8 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_eight
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text("8");
                    }
                    Key::_9 | Key::KP_9 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_nine
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text("9");
                    }
                    Key::Delete | Key::KP_Delete => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_clear_entry
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.set_display_text("0");
                        window.imp().input_display.set_max_length(21);
                    }
                    Key::period | Key::KP_Decimal => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_decimal
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.insert_display_text(".");
                    }
                    Key::exclam => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_plus_minus
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        let text = window.imp().input_display.text();
                        if text != "0" {
                            if text.starts_with('-') {
                                window.imp().input_display.delete_text(0, 1);
                                window.imp().input_display.set_max_length(21);
                            }
                            else {
                                let mut pos = 0;
                                window.imp().input_display.set_max_length(22);
                                window.imp().input_display.insert_text("-", &mut pos);
                            }
                        }
                    }
                    Key::Escape => {
                        // TODO: Clear calculation buffer
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_clear
                            .set_state_flags(gtk::StateFlags::ACTIVE, false);
                        window.set_display_text("0");
                    }
                    _ => {}
                }
                glib::Propagation::Proceed
            })
        );

        controller.connect_key_released(
            clone!(@weak self as window => move |_controller, key, _keyval, _state| match key {
                    Key::BackSpace => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_backspace
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::_0 | Key::KP_0 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_zero
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::_1 | Key::KP_1 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_one
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::_2 | Key::KP_2 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_two
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::_3 | Key::KP_3 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_three
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::_4 | Key::KP_4 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_four
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::_5 | Key::KP_5 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_five
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::_6 | Key::KP_6 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_six
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::_7 | Key::KP_7 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_seven
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::_8 | Key::KP_8 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_eight
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::_9 | Key::KP_9 => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_nine
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::Delete | Key::KP_Delete => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_clear_entry
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::period | Key::KP_Decimal => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_decimal
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::exclam => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_plus_minus
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    Key::Escape => {
                        window
                            .imp()
                            .basic_numpad
                            .imp()
                            .button_clear
                            .unset_state_flags(gtk::StateFlags::ACTIVE);
                    }
                    _ => {
                        println!("Key released: {:?}, Key Name: {:?}", key, key.name());
                    }
            }),
        );

        self.add_controller(controller);
    }

    /// Sets up the callbacks utilized by the child widgets.
    fn setup_callbacks(&self) {
        self.imp()
            .keypad_lock
            .connect_clicked(clone!(@weak self as window => move |_|{
                window.imp().persistent_keypad.set(!window.imp().persistent_keypad.get());
                window.imp().update_persistent_keypad(false);
            }));

        self.imp()
            .input_display
            .connect_paste_clipboard(clone!(@weak self as window => move |input_display| {
                input_display.block_signal(
                    &window
                        .imp()
                        .input_display_changed_signal
                        .borrow()
                        .as_ref()
                        .expect("Could not get input_display_changed_signal"),
                );
                input_display.set_text("");
                input_display.unblock_signal(
                    &window
                        .imp()
                        .input_display_changed_signal
                        .borrow()
                        .as_ref()
                        .expect("Could not get input_display_changed_signal"),
                );
            }));
        self.imp()
            .input_display_changed_signal
            .replace(Some(self.imp().input_display.connect_changed(
                clone!(@weak self as window => move |disp| {
                    let binding = disp.text().replace(",", "");
                    let text = binding.trim();
                    if text.is_empty() {
                        window.imp().input_display.block_signal(
                            &window
                                .imp()
                                .input_display_changed_signal
                                .borrow()
                                .as_ref()
                                .expect("Could not get input_display_changed_signal"),
                        );
                        disp.set_text("0");
                        window.imp().input_display.set_max_length(21);
                        window.imp().input_display.unblock_signal(
                            &window
                                .imp()
                                .input_display_changed_signal
                                .borrow()
                                .as_ref()
                                .expect("Could not get input_display_changed_signal"),
                        );
                    }
                    else {
                        let text = display_thousands_separator(text);
                        window.imp().input_display.block_signal(
                            &window
                                .imp()
                                .input_display_changed_signal
                                .borrow()
                                .as_ref()
                                .expect("Could not get input_display_changed_signal"),
                        );
                        disp.set_text(&text);
                        window.imp().input_display.unblock_signal(
                            &window
                                .imp()
                                .input_display_changed_signal
                                .borrow()
                                .as_ref()
                                .expect("Could not get input_display_changed_signal"),
                        );
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
    /// The actions are used to interact with the calculator.
    ///
    /// # Panics
    ///
    /// * Panics if the action `num-insert` parameter cannot be retrieved.
    /// * Panics if the action `num-insert` parameter type is not `i32`.
    /// * Panics if the action `op-insert` parameter cannot be retrieved.
    /// * Panics if the action `op-insert` parameter type is not `String`.
    fn setup_actions(&self) {
        let action_num_insert = ActionEntry::builder("num-insert")
            .parameter_type(Some(&i32::static_variant_type()))
            .activate(move |window: &Self, _action, parameter| {
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<i32>()
                    .expect("The variant needs to be of type `i32`.");

                window.insert_display_text(parameter.to_string().as_str());
            })
            .build();
        let action_op_insert = ActionEntry::builder("op-insert")
            .parameter_type(Some(&String::static_variant_type()))
            .activate(move |window: &Self, _action, parameter| {
                let parameter = parameter
                    .expect("Could not get parameter.")
                    .get::<String>()
                    .expect("The variant needs to be of type `String`.");

                if parameter == "backspace" {
                    if window.imp().input_display.text().as_str() != "0" {
                        window
                            .imp()
                            .input_display
                            .delete_text((window.imp().input_display.text().len() - 1) as i32, -1);
                    }
                    // window.imp().basic_numpad.imp().button_backspace.emit_activate();
                }
                else if parameter == "decimal" {
                    window.insert_display_text(".");
                }
                else if parameter == "plus_minus" {
                    let text = window.imp().input_display.text();
                    if text != "0" {
                        if text.starts_with('-') {
                            window.imp().input_display.delete_text(0, 1);
                            window.imp().input_display.set_max_length(21);
                        }
                        else {
                            let mut pos = 0;
                            window.imp().input_display.set_max_length(22);
                            window.imp().input_display.insert_text("-", &mut pos);
                        }
                    }
                }
                else if parameter == "clear-entry" {
                    window.set_display_text("0");
                    window.imp().input_display.set_max_length(21);
                }
                else if parameter == "clear" {
                    // TODO: Clear calculation buffer
                    window.set_display_text("0");
                }
                else {
                    println!("Op insert: {}", parameter);
                }
            })
            .build();

        self.add_action_entries([action_num_insert]);
        self.add_action_entries([action_op_insert]);
    }
}
