mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct CalculationDisplay(ObjectSubclass<imp::CalculationDisplay>)
        @extends gtk::Entry, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Editable;
}

impl CalculationDisplay {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for CalculationDisplay {
    fn default() -> Self {
        Self::new()
    }
}