//! Windows registry read helpers for Toontrack Ezbass.

pub const APP_KEY: &str = "Software\\Toontrack\\Toontrack Ezbass";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Toontrack Ezbass";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Toontrack Ezbass") },
        RegValue { name: "Version",     value: String::from("v3.4.3") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Toontrack Ezbass") },
        RegValue { name: "Publisher",       value: String::from("Toontrack") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Toontrack Ezbass\\uninstall.exe\"") },
    ]
}
