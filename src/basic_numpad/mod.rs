mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct BasicNumpad(ObjectSubclass<imp::BasicNumpad>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable,gtk::ConstraintTarget, gtk::Orientable;
}

impl BasicNumpad {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for BasicNumpad {
    fn default() -> Self {
        Self::new()
    }
}