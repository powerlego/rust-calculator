//! This module contains the implementation of the [`Skeleton`] widget. It provides a placeholder widget used for ui
//! design purposes. It is a subclass of [`adw::Bin`] allowing for easy layout of the widget.

use glib::Object;
use gtk::glib;

mod imp {
    //! Private implementation details of the [`Skeleton`] widget.

    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::{glib, CompositeTemplate};

    /// The `Skeleton` widget. It provides a placeholder widget used for ui design purposes. It is a subclass of
    /// [`adw::Bin`] allowing for easy layout of the widget.
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
    /// A [`Skeleton`] widget. It provides a placeholder widget used for ui design purposes. It is a subclass of [`adw::Bin`] allowing for easy layout of the widget.
    pub struct Skeleton(ObjectSubclass<imp::Skeleton>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Skeleton {
    /// Creates a new [`Skeleton`] widget.
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for Skeleton {
    /// The default implementation of the [`Skeleton`] widget. It creates a new [`Skeleton`] widget.
    fn default() -> Self {
        Self::new()
    }
}
