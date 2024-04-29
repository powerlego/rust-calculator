mod imp;

// use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{clone, Object};
use gtk::glib::object::Cast;
use gtk::{
    gio::{self, Settings},
    glib, NoSelection,
};

use crate::integer_object::IntegerObject;
use crate::skeleton::Skeleton;

use crate::APP_ID;

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

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp().settings.get().expect("`settings` should be set in `setup_settings`.")
    }

    fn setup_actions(&self) {}

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
