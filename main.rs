use std::collections::HashMap;

use druid::LocalizedString;
use image::ImageBuffer;
use window_format::MyRadio;

use druid::{AppLauncher, Rect, WindowDesc};

use druid_shell::keyboard_types::Key;

mod drawing_area;
mod function;
mod screenshot;
mod shortkeys_window;
mod window_format;

fn main() {
    let main_window = WindowDesc::new(shortkeys_window::ui_builder())
        .title(LocalizedString::new("Keyboard Shortcut Settings"))
        .window_size((1000.0, 1000.0));

    let initial_state = drawing_area::AppData {
        save_image_modifier: "Enter".into(),
        save_image_key: (Key::Character("".to_string())).to_string(),
        quit_app_modifier: "Escape".into(),
        quit_app_key: (Key::Character("".to_string())).to_string(),
        edit_image_modifier: "None".into(),
        edit_image_key: (Key::Character("m".to_string())).to_string(),
        start_image_modifier: "None".into(),
        start_image_key: (Key::Character("s".to_string())).to_string(),
        restart_app_modifier: "Shift".into(),
        restart_app_key: (Key::Character("".to_string())).to_string(),
        restart_format_app_modifier: "Ctrl".into(),
        restart_format_app_key: (Key::Character("".to_string())).to_string(),
        hotkeys: Vec::new(),
        is_selecting: false,
        start_position: None,
        end_position: None,
        start_position_to_display: None,
        end_position_to_display: None,
        modify: false,
        is_dragging: false,
        rect: Rect::new(0.0, 0.0, 0.0, 0.0),
        where_dragging: None,
        radio_group: MyRadio::Png,
        label: "screenshot_grabbed".to_string(),
        switch_window: false,
        is_found: false,
        last_key_event: None,
        hide_buttons: false,
        save: false,
        tasti: HashMap::new(),
        attivazione: HashMap::new(),
        count: 0,
        myimage: ImageBuffer::new(0, 0),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
