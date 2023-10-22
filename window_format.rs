use druid::{
    widget::{Button, Flex, RadioGroup, TextBox},
    Data, Size, Widget, WidgetExt, WindowDesc,
};
use scrap::Display;

use crate::drawing_area::{self, AppData};

#[derive(Clone, Data, PartialEq, Copy, Debug)]
pub enum MyRadio {
    Png,
    Jpeg,
    Bmp,
}
pub(crate) fn build_ui() -> impl Widget<AppData> {
    let button = Button::new("Save").on_click(move |ctx, data: &mut AppData, _| {
        if data.label == "".to_string() {
            data.label = "screenshot_grabbed".to_string();
        }
        let display_primary = Display::primary().expect("couldn't find primary display");

        let main_window = WindowDesc::new(drawing_area::build_ui())
            //.title(LocalizedString::new("Screen Capture Utility"))
            //.show_titlebar(false)
            //.set_level(druid::WindowLevel::AppWindow)
            .with_min_size(Size::new(
                display_primary.width() as f64,
                display_primary.height() as f64,
            ))
            .show_titlebar(false)
            .set_position(druid::Point::new(0., 0.))
            .window_size(Size::new(
                display_primary.width() as f64,
                display_primary.height() as f64,
            ))
            .resizable(true)
            //.show_titlebar(false)
            .set_always_on_top(true)
            .transparent(true)
            .set_window_state(druid_shell::WindowState::Maximized);

        let id = main_window.id.clone();
        data.main_window_id = main_window.id.clone();
        ctx.new_window(main_window);
        ctx.submit_command(druid::commands::SHOW_WINDOW.to(id));
        ctx.submit_command(druid::commands::CLOSE_WINDOW.to(ctx.window_id()));
    });

    let textbox = TextBox::new()
        .with_placeholder("choose the name of the screen (default screenshot_grabbed)")
        .lens(AppData::label)
        .padding(3.0);

    Flex::column()
        .with_child(textbox)
        .with_child(
            RadioGroup::column(vec![
                ("Png", MyRadio::Png),
                ("Jpeg", MyRadio::Jpeg),
                ("Bmp", MyRadio::Bmp),
            ])
            .lens(AppData::radio_group),
        )
        .with_child(button)
}
