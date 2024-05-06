use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/nc/calculator/input_display.ui")]
pub struct InputDisplay {
}

#[glib::object_subclass]
impl ObjectSubclass for InputDisplay {
    type ParentType = gtk::TextView;
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
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.remove_css_class("view");
        obj.buffer().set_text("0");
        obj.setup_callbacks();
    }
}

impl WidgetImpl for InputDisplay {}

impl TextViewImpl for InputDisplay {}
