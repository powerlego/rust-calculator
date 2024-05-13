use glib::Object;
use gtk::glib;

mod imp {
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::{glib, Button, CompositeTemplate, Grid};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/nc/calculator/basic_numpad.ui")]
    pub struct BasicNumpad {
        #[template_child]
        pub button_percent:     TemplateChild<Button>,
        #[template_child]
        pub button_clear_entry: TemplateChild<Button>,
        #[template_child]
        pub button_clear:       TemplateChild<Button>,
        #[template_child]
        pub button_backspace:   TemplateChild<Button>,
        #[template_child]
        pub button_one_over:    TemplateChild<Button>,
        #[template_child]
        pub button_square:      TemplateChild<Button>,
        #[template_child]
        pub button_square_root: TemplateChild<Button>,
        #[template_child]
        pub button_divide:      TemplateChild<Button>,
        #[template_child]
        pub button_seven:       TemplateChild<Button>,
        #[template_child]
        pub button_eight:       TemplateChild<Button>,
        #[template_child]
        pub button_nine:        TemplateChild<Button>,
        #[template_child]
        pub button_multiply:    TemplateChild<Button>,
        #[template_child]
        pub button_four:        TemplateChild<Button>,
        #[template_child]
        pub button_five:        TemplateChild<Button>,
        #[template_child]
        pub button_six:         TemplateChild<Button>,
        #[template_child]
        pub button_subtract:    TemplateChild<Button>,
        #[template_child]
        pub button_one:         TemplateChild<Button>,
        #[template_child]
        pub button_two:         TemplateChild<Button>,
        #[template_child]
        pub button_three:       TemplateChild<Button>,
        #[template_child]
        pub button_add:         TemplateChild<Button>,
        #[template_child]
        pub button_plus_minus:  TemplateChild<Button>,
        #[template_child]
        pub button_zero:        TemplateChild<Button>,
        #[template_child]
        pub button_decimal:     TemplateChild<Button>,
        #[template_child]
        pub button_equals:      TemplateChild<Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BasicNumpad {
        type ParentType = Grid;
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
}

glib::wrapper! {
    pub struct BasicNumpad(ObjectSubclass<imp::BasicNumpad>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable,gtk::ConstraintTarget, gtk::Orientable;
}

impl BasicNumpad {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for BasicNumpad {
    fn default() -> Self {
        Self::new()
    }
}
