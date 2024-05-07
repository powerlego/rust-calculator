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

    use std::cell::{Cell, RefCell};
    use std::fs::File;
    use std::io::Write;

    // use std::sync::OnceLock;
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use glib::SignalHandlerId;
    use gtk::prelude::*;
    use gtk::{gio, glib, Box, Button, CompositeTemplate, Expander, ListBox, Notebook, Text};
    use toml_edit::{table, value, DocumentMut};

    use crate::utils::settings_path;
    use crate::widgets::{BasicNumpad, Skeleton};

    /// The `Window` widget.
    /// # Actions
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

    fn setup_event_controllers(&self) {
        let controller = EventControllerKey::builder()
            .name("keypad-controller")
            .propagation_phase(gtk::PropagationPhase::Target)
            .propagation_limit(gtk::PropagationLimit::SameNative)
            .build();
        controller.connect_key_released(
            clone!(@weak self as window => move |_controller, key, _keyval, state| {
                match key {
                    Key::BackSpace => {
                        if window.imp().input_display.text().as_str() == "0" {
                            return;
                        }
                        window
                            .imp()
                            .input_display
                            .delete_text((window.imp().input_display.text().len() - 1) as i32, -1);
                    }
                    Key::Return | Key::KP_Enter => {
                        // Calculate the result
                    }
                    Key::KP_Add => {
                        // Insert the addition operator
                    }
                    Key::KP_Subtract | Key::minus => {
                        // Insert the subtraction operator
                    }
                    Key::KP_Multiply => {
                        // Insert the multiplication operator
                    }
                    Key::KP_Divide => {
                        // Insert the division operator
                    }
                    Key::KP_Decimal => {
                        // Insert the decimal point
                    }
                    Key::_8 => {
                        if state.contains(gdk::ModifierType::SHIFT_MASK){
                            // Insert the multiplication operator
                        }
                        else {
                            window.insert_display_text("8");
                        }
                    }
                    Key::_0
                    | Key::_1
                    | Key::_2
                    | Key::_3
                    | Key::_4
                    | Key::_5
                    | Key::_6
                    | Key::_7
                    | Key::_9
                    | Key::KP_0
                    | Key::KP_1
                    | Key::KP_2
                    | Key::KP_3
                    | Key::KP_4
                    | Key::KP_5
                    | Key::KP_6
                    | Key::KP_7
                    | Key::KP_8
                    | Key::KP_9 => {
                        window.insert_display_text(&key.to_unicode().expect("Could not get unicode value").to_string().as_str());
                    }
                    _ => {}
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

        self.imp().input_display.connect_paste_clipboard(clone!(@weak self as window => move |input_display| {
            input_display.block_signal(&window.imp().input_display_changed_signal.borrow().as_ref().expect("Could not get input_display_changed_signal"));
            input_display.set_text("");
            input_display.unblock_signal(&window.imp().input_display_changed_signal.borrow().as_ref().expect("Could not get input_display_changed_signal"));
        }));
        self.imp()
            .input_display_changed_signal
            .replace(Some(self.imp().input_display.connect_changed(
                clone!(@weak self as window => move |disp| {
                    let binding = disp.text().replace(",", "");
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
