use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate};


#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/nc/calculator/input_display.ui")]
pub struct InputDisplay {}

#[glib::object_subclass]
impl ObjectSubclass for InputDisplay {
    type ParentType = gtk::Entry;
    type Type = super::InputDisplay;

    const NAME: &'static str = "InputDisplay";

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for InputDisplay {
}

impl WidgetImpl for InputDisplay {}

impl EntryImpl for InputDisplay {}