use adw::subclass::prelude::*;
use gtk::glib;

#[derive(Default)]
pub struct CalculatorButton {}

#[glib::object_subclass]
impl ObjectSubclass for CalculatorButton {
    type ParentType = gtk::Button;
    type Type = super::CalculatorButton;

    const NAME: &'static str = "CalculatorButton";
}

impl ObjectImpl for CalculatorButton {}

impl WidgetImpl for CalculatorButton {}

impl ButtonImpl for CalculatorButton {}
