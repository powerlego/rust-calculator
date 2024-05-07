use glib::Object;
use gtk::glib;

mod imp {
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::{glib, CompositeTemplate};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/nc/calculator/skeleton.ui")]
    pub struct Skeleton {}

    #[glib::object_subclass]
    impl ObjectSubclass for Skeleton {
        type ParentType = adw::Bin;
        type Type = super::Skeleton;

        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "Skeleton";

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
}

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

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}
