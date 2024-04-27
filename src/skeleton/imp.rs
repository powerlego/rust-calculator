use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/nc/calculator/skeleton.ui")]
pub struct Skeleton{

}

#[glib::object_subclass]
impl ObjectSubclass for Skeleton {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "Skeleton";
    type Type = super::Skeleton;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Skeleton {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

impl WidgetImpl for Skeleton {}

impl BinImpl for Skeleton {}