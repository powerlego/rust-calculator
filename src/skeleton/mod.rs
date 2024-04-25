mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct Skeleton(ObjectSubclass<imp::Skeleton>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Skeleton {
    pub fn new() -> Self {
        Object::builder().build()
    }
}