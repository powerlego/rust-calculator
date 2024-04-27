use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{gio,glib::{self, types::StaticTypeExt}, CompositeTemplate, ListBox};
use std::cell::RefCell;

use crate::skeleton::Skeleton;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/nc/calculator/window.ui")]
pub struct Window {
    #[template_child]
    pub mem_hist_list: TemplateChild<ListBox>,
    pub mem_hist: RefCell<Option<gio::ListStore>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MainWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Skeleton::ensure_type();

        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_mem_hist();
        obj.create_rows();
        
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

// Trait shared by all adwaita application windows
impl AdwApplicationWindowImpl for Window {}
