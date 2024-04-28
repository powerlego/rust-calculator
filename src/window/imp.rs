use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{gio,glib::{self, types::StaticTypeExt}, CompositeTemplate, ListBox, Notebook, Expander};
use std::cell::RefCell;

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
impl Window{
    #[template_callback]
    fn on_expander_keypad_expanded(&self, _p: glib::ParamSpec){
        if self.expander_keypad.is_expanded(){
            if self.expander_history.is_expanded(){
                self.expander_history.set_expanded(false);
            }
            if self.expander_convert.is_expanded(){
                self.expander_convert.set_expanded(false);
            }
        }
    }

    #[template_callback]
    fn on_expander_history_expanded(&self, _p: glib::ParamSpec){
        if self.expander_history.is_expanded(){
            self.tabs.set_current_page(Some(0));
            if self.expander_keypad.is_bound(){
                self.expander_keypad.set_expanded(false);
            }
            if self.expander_convert.is_expanded() {
                self.expander_convert.set_expanded(false);
            }
        }
    }

    #[template_callback]
    fn on_expander_convert_expanded(&self, _p: glib::ParamSpec){
        if self.expander_convert.is_expanded(){
            self.tabs.set_current_page(Some(1));
            if self.expander_keypad.is_expanded(){
                self.expander_keypad.set_expanded(false);
            }
            if self.expander_history.is_expanded(){
                self.expander_history.set_expanded(false);
            }
        }
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
