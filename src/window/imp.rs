// Module: window
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::{
    gio,
    glib::{self, types::StaticTypeExt},
    CompositeTemplate, Expander, ListBox, Notebook,
};
use std::cell::{Cell, RefCell};

use crate::skeleton::Skeleton;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/nc/calculator/window.ui")]
pub struct Window {
    #[template_child]
    pub mem_hist_list: TemplateChild<ListBox>,
    #[template_child]
    pub tabs: TemplateChild<Notebook>,
    #[template_child]
    pub expander_keypad: TemplateChild<Expander>,
    #[template_child]
    pub expander_history: TemplateChild<Expander>,
    #[template_child]
    pub expander_convert: TemplateChild<Expander>,
    #[template_child]
    pub keypad_buttons: TemplateChild<Skeleton>,
    pub persistent_keypad: Cell<bool>,
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
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn on_expander_keypad_expanded(&self, _p: glib::ParamSpec) {
        if self.expander_keypad.is_expanded() {
            self.show_keypad_widget(true);
            if !self.persistent_keypad.get() {
                if self.expander_history.is_expanded() {
                    self.expander_history.set_expanded(false);
                }
                if self.expander_convert.is_expanded() {
                    self.expander_convert.set_expanded(false);
                }
            }
        } else {
            self.show_keypad_widget(false);
        }
    }

    #[template_callback]
    fn on_expander_history_expanded(&self, _p: glib::ParamSpec) {
        if self.expander_history.is_expanded() {
            self.tabs.set_current_page(Some(0));
            self.show_tabs(true);
            if !self.persistent_keypad.get() && self.expander_keypad.is_expanded() {
                self.expander_keypad.set_expanded(false);
            }
            if self.expander_convert.is_expanded() {
                self.expander_convert.set_expanded(false);
            }
        }
        else if !self.expander_convert.is_expanded() {
            self.show_tabs(false);
        }
    }

    #[template_callback]
    fn on_expander_convert_expanded(&self, _p: glib::ParamSpec) {
        if self.expander_convert.is_expanded() {
            self.tabs.set_current_page(Some(1));
            self.show_tabs(true);
            if !self.persistent_keypad.get() && self.expander_keypad.is_expanded() {
                self.expander_keypad.set_expanded(false);
            }
            if self.expander_history.is_expanded() {
                self.expander_history.set_expanded(false);
            }
        }
        else if !self.expander_history.is_expanded() {
            self.show_tabs(false);
        }
    }

    fn show_keypad_widget(&self, do_show: bool) {
        if do_show == self.keypad_buttons.is_visible() {
            return;
        }
        let w: i32 = self.obj().width();
        let mut h: i32 = self.obj().height();

        if !self.persistent_keypad.get() && self.tabs.is_visible() {
            h -= self.tabs.height();
        }
        if self.persistent_keypad.get() && self.expander_convert.is_expanded() {
            if do_show {
                h += 6;
            } else {
                h -= 6;
            }
        }
        if do_show {
            self.keypad_buttons.set_visible(true);
            let kb_h = self.keypad_buttons.height();
            if kb_h > 10 {
                h += kb_h + 9;
            } else {
                h += 9;
            }
            if !self.persistent_keypad.get() {
                self.tabs.set_visible(false);
            }
            self.obj().set_default_size(w, h);
        } else {
            h -= self.keypad_buttons.height();
            self.keypad_buttons.set_visible(false);
            self.obj().set_default_size(w, h);
        }
        self.keypad_buttons
            .set_vexpand(!self.persistent_keypad.get() || !self.tabs.is_visible());
    }

    fn show_tabs(&self, do_show: bool) {
        if do_show == self.tabs.is_visible() {
            return;
        }
        let w: i32 = self.obj().width();
        let mut h: i32 = self.obj().height();

        if !self.persistent_keypad.get() && self.keypad_buttons.is_visible() {
            h -= self.keypad_buttons.height();
        }
        if do_show {
            self.tabs.set_visible(true);
            let t_h = self.tabs.height();
            if t_h > 10 {
                h += t_h + 9;
            } else {
                h += 9;
            }
            if !self.persistent_keypad.get() {
                self.keypad_buttons.set_visible(false);
            }
            self.obj().set_default_size(w, h);
        }
        else {
            h -= self.tabs.height();
            self.tabs.set_visible(false);
            self.obj().set_default_size(w, h);
        }
        self.keypad_buttons.set_vexpand(!self.persistent_keypad.get() || !self.tabs.is_visible());
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        self.keypad_buttons.set_visible(false);
        self.tabs.set_visible(false);

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
