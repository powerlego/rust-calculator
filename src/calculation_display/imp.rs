use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate};


#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/nc/calculator/calculation_display.ui")]
pub struct CalculationDisplay {}

#[glib::object_subclass]
impl ObjectSubclass for CalculationDisplay {
    type ParentType = gtk::Entry;
    type Type = super::CalculationDisplay;

    const NAME: &'static str = "CalculationDisplay";

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CalculationDisplay {
}

impl WidgetImpl for CalculationDisplay {}

impl EntryImpl for CalculationDisplay {}