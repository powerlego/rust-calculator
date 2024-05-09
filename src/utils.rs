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

pub fn display_thousands_separator(number: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    let split = number.split('.').collect::<Vec<&str>>();
    let mut num = split[0];
    if split.len() > 1 {
        result.push_str(&format!("{}.", split[1].chars().rev().collect::<String>()));
    }
    let is_negative = num.starts_with('-');
    if is_negative {
        num = &num[1..];
    }
    for c in num.chars().rev() {
        if count == 3 {
            result.push(',');
            count = 0;
        }
        result.push(c);
        count += 1;
    }
    result.push_str(&if is_negative { "-" } else { "" });
    result.chars().rev().collect()
}
