use std::path::PathBuf;

use gtk::glib;

use crate::APP_ID;

pub fn settings_path() -> PathBuf {
    let mut path = glib::user_config_dir();
    path.push(APP_ID);
    std::fs::create_dir_all(&path).expect("Failed to create settings directory");
    path.push("settings.toml");
    path
}
