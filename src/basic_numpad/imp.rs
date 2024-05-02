use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/nc/calculator/basic_numpad.ui")]
pub struct BasicNumpad {}

#[glib::object_subclass]
impl ObjectSubclass for BasicNumpad {
    type ParentType = gtk::Grid;
    type Type = super::BasicNumpad;

    const NAME: &'static str = "BasicNumpad";

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BasicNumpad {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

impl WidgetImpl for BasicNumpad {}

impl GridImpl for BasicNumpad {}
