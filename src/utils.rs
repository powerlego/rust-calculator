use std::path::PathBuf;

use gtk::glib;

use crate::APP_ID;

/// Creates the directory to store the settings file and returns the path to the settings file.
/// If the directory already exists, it will just return the path to the settings file.
/// If the directory does not exist, it will create the directory and then return the path to the settings file.
/// # Returns
/// The path to the settings file.
pub fn settings_path() -> PathBuf {
    let mut path = glib::user_config_dir();
    path.push(APP_ID);
    std::fs::create_dir_all(&path).expect("Failed to create settings directory");
    path.push("settings.toml");
    path
}
