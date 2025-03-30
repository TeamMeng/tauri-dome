// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri_dome_lib::run()
}

#[cfg(test)]
mod tests {
    #[test]
    fn eq() {
        assert_eq!(1, 1);
    }
}
