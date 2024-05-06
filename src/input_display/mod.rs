mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct InputDisplay(ObjectSubclass<imp::InputDisplay>)
        @extends gtk::Entry, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Editable;
}

impl InputDisplay {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for InputDisplay {
    fn default() -> Self {
        Self::new()
    }
}