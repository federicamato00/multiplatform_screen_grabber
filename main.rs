use druid::{LocalizedString, WindowId};
use window_format::MyRadio;

use druid::{AppLauncher, Point, Rect, WindowDesc};

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
        save_image_modifier: "None".into(),
        save_image_key: (Key::Character("s".to_string())).to_string(),
        quit_app_modifier: "None".into(),
        quit_app_key: (Key::Character("q".to_string())).to_string(),
        edit_image_modifier: "None".into(),
        edit_image_key: (Key::Character("m".to_string())).to_string(),
        start_image_modifier: "Escape".into(),
        start_image_key: (Key::Character("".to_string())).to_string(),
        restart_app_modifier: "Enter".into(),
        restart_app_key: (Key::Character("".to_string())).to_string(),
        restart_format_app_modifier: "Ctrl".into(),
        restart_format_app_key: (Key::Character("".to_string())).to_string(),
        hotkeys: Vec::new(),
        is_selecting: false,
        start_position: Some(Point::ZERO),
        end_position: Some(Point::ZERO),
        start_position_to_display: Some(Point::ZERO),
        end_position_to_display: Some(Point::ZERO),
        modify: false,
        is_dragging: false,
        rect: Rect::new(0.0, 0.0, 0.0, 0.0),
        where_dragging: None,
        radio_group: MyRadio::Png,
        label: "screenshot_grabbed".to_string(),
        format_window_id: WindowId::next(),
        shortkeys_window_id: WindowId::next(),
        main_window_id: main_window.id,
        is_pressed: false,
        last_key_event: None,
        hide_buttons: false,
        save: false,
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
