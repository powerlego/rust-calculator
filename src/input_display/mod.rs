mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::{gdk, glib};

glib::wrapper! {
    pub struct InputDisplay(ObjectSubclass<imp::InputDisplay>)
        @extends gtk::TextView, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Scrollable;
}

impl InputDisplay {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setup_callbacks(&self) {
        self.connect_paste_clipboard(|input_display| {
            let buffer = input_display.buffer();
            let clipboard = gdk::Display::default()
                .expect("Unable to get default display")
                .clipboard();
            buffer.delete(&mut buffer.start_iter(), &mut buffer.end_iter());
            buffer.paste_clipboard(&clipboard, None, false);
        });
        self.buffer().connect_changed(|buffer| {
            println!(
                "Buffer changed: {}",
                buffer.text(&buffer.start_iter(), &buffer.end_iter(), false)
            );
        });
    }
}

impl Default for InputDisplay {
    fn default() -> Self {
        Self::new()
    }
}
