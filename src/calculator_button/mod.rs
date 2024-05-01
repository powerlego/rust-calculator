mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct CalculatorButton(ObjectSubclass<imp::CalculatorButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CalculatorButton {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for CalculatorButton {
    fn default() -> Self {
        Self::new()
    }
}
