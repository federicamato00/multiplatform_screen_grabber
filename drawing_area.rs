use std::env;

use druid::widget::BackgroundBrush;
use druid::widget::Button;
use druid::widget::Controller;
use druid::widget::Flex;
use druid::Color;
use druid::Command;
use druid::Data;
use druid::Env;
use druid::Event;
use druid::EventCtx;
use druid::Insets;
use druid::PaintCtx;
use druid::Point;
use druid::Rect;
use druid::RenderContext;
use druid::Selector;
use druid::Size;
use druid::Target;
use druid::Widget;
use druid::WidgetExt;
use druid::WindowDesc;
use druid::WindowId;

use druid::widget::Label;
use druid::widget::Padding;

use druid::widget::ViewSwitcher;
use druid_shell::keyboard_types::Key;
use druid_shell::KeyEvent;
use druid_shell::Modifiers as Mod;

use crate::function;
use crate::shortkeys_window;
use crate::window_format;
use druid::Lens;
use druid_shell::keyboard_types::Modifiers;
use druid_shell::MouseButton;
use scrap::Display;

#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub(crate) is_selecting: bool,
    pub(crate) start_position: Option<Point>,
    pub(crate) end_position: Option<Point>,
    pub(crate) start_position_to_display: Option<Point>,
    pub(crate) end_position_to_display: Option<Point>,
    pub(crate) modify: bool,
    pub(crate) is_dragging: bool,
    pub(crate) rect: Rect,
    pub(crate) where_dragging: Option<DragHandle>,
    pub(crate) radio_group: window_format::MyRadio,
    pub(crate) label: String,
    pub(crate) save_image_modifier: String,
    pub(crate) save_image_key: String,
    pub(crate) quit_app_modifier: String,
    pub(crate) quit_app_key: String,
    pub(crate) edit_image_modifier: String,
    pub(crate) edit_image_key: String,
    pub(crate) start_image_modifier: String,
    pub(crate) start_image_key: String,
    pub(crate) restart_app_modifier: String,
    pub(crate) restart_app_key: String,
    pub(crate) restart_format_app_modifier: String,
    pub(crate) restart_format_app_key: String,
    pub(crate) is_pressed: bool,
    pub(crate) hide_buttons: bool,
    pub(crate) save: bool,
    #[data(ignore)]
    pub(crate) format_window_id: WindowId,
    #[data(ignore)]
    pub(crate) shortkeys_window_id: WindowId,
    #[data(ignore)]
    pub(crate) main_window_id: WindowId,
    #[data(ignore)]
    pub(crate) hotkeys: Vec<MyHotkey>,
    #[data(ignore)]
    pub(crate) last_key_event: Option<KeyEvent>,
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum Action {
    Quit,
    Edit,
    Save,
    RestartFormat,
    RestartApp,
    Cancel,
    // Aggiungi qui altre azioni...
}

// Definisci la struttura della tua hotkey
pub const SAVE: Selector<&'static str> = Selector::new("SAVE");
#[derive(Clone, PartialEq, Debug)]
pub struct MyHotkey {
    pub(crate) keys: druid::keyboard_types::Key,
    pub(crate) action: Action,
    pub(crate) mods: Modifiers,
}

#[derive(Clone, Data, PartialEq)]
pub enum DragHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl MyHotkey {
    pub fn matches(&self, event_mods: Mod, event_key: Key) -> bool {
        // Should be a const but const bit_or doesn't work here.
        let mods = match self.mods {
            Modifiers::SHIFT => Mod::SHIFT,
            Modifiers::META => Mod::META,
            Modifiers::CONTROL => Mod::CONTROL,
            Modifiers::CAPS_LOCK => Mod::CAPS_LOCK,
            _ => Mod::empty(),
        };
        // let event = event.borrow();

        // mods == event.mods  && self.keys == event.key
        //println!("hotkey {:?},event: {:?}, key event: {:?}",mods,event_mods,event_key);
        let all_mod_pressed = mods == event_mods;

        // Controlla se ci sono altri tasti premuti
        let all_keys_pressed = self.keys == event_key;

        all_keys_pressed && all_mod_pressed
    }
}

struct DrawingArea;
impl Widget<AppData> for DrawingArea {
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut AppData, _env: &Env) {
        ctx.request_update();
        //println!("{:?}", event);
        match event {
            Event::WindowConnected => {
                println!("ciao2");
                // Richiedi il focus quando la finestra è connessa.
                data.main_window_id = ctx.window_id();
                if data.modify == false {}
                // Imposta la dimensione della finestra
                let display_primary = Display::primary().expect("couldn't find primary display");
                let size = Size::new(
                    display_primary.width() as f64,
                    display_primary.height() as f64,
                ); // Imposta le dimensioni desiderate qui
                ctx.window().set_size(size);
                //println!("size window {:?}",size);
                data.is_pressed = false;
            }
            // Event::KeyDown(key_event) => {
            //     if data.hide_buttons {
            //         if let Some(last_event) = data.last_key_event.clone() {
            //             let key = match last_event.mods {
            //                 Mod::CONTROL => Key::Control,
            //                 Mod::SHIFT => Key::Shift,
            //                 Mod::META => Key::Meta,
            //                 Mod::CAPS_LOCK => Key::CapsLock,
            //                 _ => Key::Fn,
            //             };

            //             if key == (key_event.key) && key_event.key != last_event.key {
            //                 data.is_pressed = true;
            //             } else {
            //                 data.is_pressed = false;
            //             }
            //         }

            //         if data.is_pressed != true {
            //             if data
            //                 .hotkeys
            //                 .get(2)
            //                 .unwrap()
            //                 .matches(key_event.mods, key_event.key.clone())
            //                 && !data.is_pressed
            //             {
            //                 // Chiudi la finestra
            //                 ctx.submit_command(druid::commands::QUIT_APP);
            //             } else if data
            //                 .hotkeys
            //                 .get(4)
            //                 .unwrap()
            //                 .matches(key_event.mods, key_event.key.clone())
            //                 && !data.is_pressed
            //             {
            //                 data.start_position = None;
            //                 data.end_position = None;
            //                 data.start_position_to_display = None;
            //                 data.end_position_to_display = None;
            //                 data.is_dragging = false;
            //                 data.is_selecting = false;
            //                 data.modify = false;
            //                 data.hotkeys = Vec::new();
            //                 data.is_pressed = true;
            //                 data.hide_buttons = false;
            //                 data.last_key_event = Some(key_event.clone());
            //                 data.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
            //                 ctx.submit_command(
            //                     druid::commands::HIDE_WINDOW.to(data.main_window_id),
            //                 );
            //                 ctx.submit_command(
            //                     druid::commands::SHOW_WINDOW.to(data.shortkeys_window_id),
            //                 );
            //             } else if data
            //                 .hotkeys
            //                 .get(5)
            //                 .unwrap()
            //                 .matches(key_event.mods, key_event.key.clone())
            //                 && data.is_pressed != true
            //             {
            //                 data.start_position = None;
            //                 data.end_position = None;
            //                 data.start_position_to_display = None;
            //                 data.end_position_to_display = None;
            //                 data.is_dragging = false;
            //                 data.is_selecting = false;
            //                 data.modify = false;
            //                 data.is_pressed = false;
            //                 data.hide_buttons = false;
            //                 data.last_key_event = Some(key_event.clone());
            //                 data.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
            //                 ctx.submit_command(
            //                     druid::commands::HIDE_WINDOW.to(data.main_window_id),
            //                 );
            //                 ctx.submit_command(
            //                     druid::commands::SHOW_WINDOW.to(data.format_window_id),
            //                 );
            //             } else if data
            //                 .hotkeys
            //                 .get(0)
            //                 .unwrap()
            //                 .matches(key_event.mods, key_event.key.clone())
            //                 && !data.is_pressed
            //             {
            //                 println!("ciao stai per salvare");
            //                 data.hide_buttons = true;
            //                 let size = ctx.window().get_size();
            //                 function::save_screen(data, size);
            //                 data.last_key_event = Some(key_event.clone());
            //                 //ctx.request_paint();
            //             } else {
            //                 data.is_pressed = true;
            //                 data.hide_buttons = false;
            //                 data.last_key_event = Some(key_event.clone());
            //             }
            //         } else {
            //             data.is_pressed = false;
            //             data.hide_buttons = false;
            //             data.last_key_event = None;
            //         }
            //     }
            // }
            Event::WindowCloseRequested => {
                // Qui puoi gestire l'evento di chiusura della finestra.
                // Ad esempio, potresti voler salvare i dati dell'applicazione o mostrare un messaggio all'utente.
                ctx.submit_command(druid::commands::QUIT_APP);
            }

            druid::Event::MouseDown(mouse_event) => {
                if data.modify == true && data.is_dragging == false {
                    data.start_position = None;
                    data.end_position = None;
                    data.start_position_to_display = None;
                    data.end_position_to_display = None;
                    data.is_selecting = false;
                    data.is_dragging = false;
                    data.modify = false;
                }
                if data.modify == false && data.is_dragging == false {
                    if mouse_event.button == MouseButton::Left {
                        data.start_position = None;
                        data.end_position = None;
                        let os = env::consts::OS;
                        match os {
                            "windows" => {
                                let scale_factor_x = ctx.window().get_scale().unwrap().x();
                                let scale_factor_y = ctx.window().get_scale().unwrap().y();
                                let coord = druid::Point {
                                    x: mouse_event.pos.x * scale_factor_x,
                                    y: mouse_event.pos.y * scale_factor_y,
                                };
                                data.start_position_to_display = Some(druid::Point {
                                    x: mouse_event.pos.x,
                                    y: mouse_event.pos.y,
                                });
                                data.start_position = Some(coord);
                            }
                            "macos" => {
                                let pos = ctx.to_screen(druid::Point::new(
                                    mouse_event.pos.x,
                                    mouse_event.pos.y,
                                ));
                                //let size=ctx.window().get_size();
                                //println!("size: {:?}",size);
                                let coord = druid::Point { x: pos.x, y: pos.y };
                                data.start_position_to_display = Some(druid::Point {
                                    x: mouse_event.pos.x,
                                    y: mouse_event.pos.y,
                                });
                                data.start_position = Some(coord);
                            }
                            _ => {
                                // Gestisci altri sistemi operativi se necessario
                            }
                        }

                        //println!("Click su pos: {:?}",mouse_event.pos);
                        // println!("Click su window_pos: {:?}",mouse_event.window_pos);

                        data.is_selecting = true;
                    }
                }
                if data.is_dragging == true {
                    //println!("{:?}",(mouse_event.pos - data.rect.origin()).hypot());
                    if (mouse_event.pos - data.rect.origin()).hypot() < 70.0 {
                        ctx.set_cursor(&druid::Cursor::ResizeUpDown);
                        data.where_dragging = Some(DragHandle::TopLeft);
                        ctx.set_active(true);
                    } else if (mouse_event.pos - Point::new(data.rect.x1, data.rect.y1)).hypot()
                        < 70.0
                    {
                        ctx.set_cursor(&druid::Cursor::ResizeUpDown);
                        data.where_dragging = Some(DragHandle::BottomRight);
                        ctx.set_active(true);
                    } else if (mouse_event.pos - Point::new(data.rect.x0, data.rect.y1)).hypot()
                        < 70.0
                    {
                        ctx.set_cursor(&druid::Cursor::ResizeUpDown);
                        data.where_dragging = Some(DragHandle::BottomLeft);
                        ctx.set_active(true);
                    } else if (mouse_event.pos - Point::new(data.rect.x1, data.rect.y0)).hypot()
                        < 70.0
                    {
                        ctx.set_cursor(&druid::Cursor::ResizeUpDown);
                        data.where_dragging = Some(DragHandle::TopRight);
                        ctx.set_active(true);
                    } else {
                        data.hide_buttons = false;
                        ctx.set_cursor(&druid::Cursor::Arrow);
                    }
                    data.is_selecting = true;
                }
            }

            druid::Event::MouseMove(mouse_event) => {
                // Aggiorna la posizione finale del rettangolo qui

                let os = env::consts::OS;
                match os {
                    "windows" => {
                        if ctx.is_active() == false
                            && data.is_dragging == false
                            && data.save != true
                        {
                            let scale_factor_x = ctx.window().get_scale().unwrap().x();
                            let scale_factor_y = ctx.window().get_scale().unwrap().y();
                            let coord = druid::Point {
                                x: mouse_event.pos.x * scale_factor_x,
                                y: mouse_event.pos.y * scale_factor_y,
                            };
                            data.end_position_to_display = Some(druid::Point {
                                x: mouse_event.pos.x,
                                y: mouse_event.pos.y,
                            });
                            data.end_position = Some(coord);
                        }
                        if ctx.is_active() {
                            let scale_factor_x = ctx.window().get_scale().unwrap().x();
                            let scale_factor_y = ctx.window().get_scale().unwrap().y();
                            if let Some(handle) = &data.where_dragging.clone() {
                                // let scale_factor_x = ctx.window().get_scale().unwrap().x();
                                // let scale_factor_y = ctx.window().get_scale().unwrap().y();
                                let coord = druid::Point {
                                    x: mouse_event.pos.x * scale_factor_x,
                                    y: mouse_event.pos.y * scale_factor_y,
                                };

                                function::edit_rect(handle, coord, data, mouse_event);
                                ctx.request_paint();
                            }
                        }
                    }
                    "macos" => {
                        if ctx.is_active() == false
                            && data.is_dragging == false
                            && data.save != true
                        {
                            let pos = ctx
                                .to_screen(druid::Point::new(mouse_event.pos.x, mouse_event.pos.y));

                            let coord = druid::Point { x: pos.x, y: pos.y };
                            data.end_position_to_display = Some(druid::Point {
                                x: mouse_event.pos.x,
                                y: mouse_event.pos.y,
                            });
                            data.end_position = Some(coord);
                        }

                        // questa parte qua bisogna implementarla anche per il caso os="windows" tenendo conto che per quel caso
                        // data.start_position_to_display corrispondono ai dati non fattorizzati e così anche per end_position
                        // da considerare anche che pos per macos corrisponde alla fattorizzazione che con windows si fa con il fattore scala
                        // per cui coord bisogna adeguarlo con i dati fattorizzati con scale_factor_x
                        if ctx.is_active() {
                            if let Some(handle) = &data.where_dragging.clone() {
                                let pos = ctx.to_screen(druid::Point::new(
                                    mouse_event.pos.x,
                                    mouse_event.pos.y,
                                ));
                                function::edit_rect(handle, pos, data, mouse_event);

                                ctx.request_paint();
                            }
                        }
                    }
                    _ => {
                        // Gestisci altri sistemi operativi se necessario
                    }
                }

                // Richiedi un aggiornamento del widget per ridisegnare il rettangolo

                if data.modify == false {
                    ctx.request_paint();
                }
            }

            druid::Event::Command(cmd) if cmd.is(SAVE) && data.save == true => {
                function::save_screen(data, ctx.size());
            }
            druid::Event::MouseUp(mouse_event) => {
                if data.is_dragging == true {
                    data.where_dragging = None;
                    ctx.set_active(false);
                    data.is_selecting = true;
                }
                if data.modify == false && data.is_dragging == false {
                    if mouse_event.button == MouseButton::Left {
                        data.is_selecting = false;
                        data.modify = true;
                        //ctx.request_paint();
                        //println!("Click end: {:?}", mouse_event.pos);
                        //thread::sleep(Duration::from_millis(1000));
                        let os = env::consts::OS;
                        match os {
                            "windows" => {
                                let scale_factor_x = ctx.window().get_scale().unwrap().x();
                                let scale_factor_y = ctx.window().get_scale().unwrap().y();
                                let coord = druid::Point {
                                    x: mouse_event.pos.x * scale_factor_x,
                                    y: mouse_event.pos.y * scale_factor_y,
                                };
                                data.end_position_to_display = Some(druid::Point {
                                    x: mouse_event.pos.x,
                                    y: mouse_event.pos.y,
                                });
                                if coord.x < data.start_position.unwrap().x
                                    && coord.y < data.start_position.unwrap().y
                                {
                                    let prov = data.start_position;
                                    data.start_position = Some(coord);
                                    data.end_position = prov;
                                    let prov_display = data.start_position_to_display;
                                    data.start_position_to_display = data.end_position_to_display;
                                    data.end_position_to_display = prov_display;
                                } else {
                                    data.end_position = Some(coord);
                                }

                                data.rect = druid::Rect::from_points(
                                    data.start_position_to_display.unwrap(),
                                    data.end_position_to_display.unwrap(),
                                );
                            }
                            _ => {
                                let pos = ctx.to_screen(druid::Point::new(
                                    mouse_event.pos.x,
                                    mouse_event.pos.y,
                                ));

                                let coord = druid::Point { x: pos.x, y: pos.y };

                                data.end_position_to_display = Some(druid::Point {
                                    x: mouse_event.pos.x,
                                    y: mouse_event.pos.y,
                                });
                                if coord.x < data.start_position.unwrap().x
                                    && coord.y < data.start_position.unwrap().y
                                {
                                    let prov = data.start_position;
                                    data.start_position = Some(coord);
                                    data.end_position = prov;
                                    let prov_display = data.start_position_to_display;
                                    data.start_position_to_display = data.end_position_to_display;
                                    data.end_position_to_display = prov_display;
                                } else {
                                    data.end_position = Some(coord);
                                }
                                data.rect = druid::Rect::from_points(
                                    data.start_position.unwrap(),
                                    data.end_position.unwrap(),
                                );
                            }
                        }
                    }
                    data.hide_buttons = false;
                }
            }

            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &AppData,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &AppData,
        data: &AppData,
        _env: &Env,
    ) {
        if data.is_dragging == true && data.is_selecting == true && data.save != true {
            ctx.request_paint();
        }
        if data.save == true {
            ctx.submit_command(Command::new(SAVE, "", Target::Global));
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &AppData,
        _env: &Env,
    ) -> druid::Size {
        let os = std::env::consts::OS;
        match os {
            "windows" => {
                // Set the size of the drawing area.
                let display_primary = Display::primary().expect("couldn't find primary display");
                //println!("Altezza layout{:?}",display_primary.height());
                let width = display_primary.width();
                let height = display_primary.height();

                ctx.set_paint_insets(druid::Insets::uniform_xy(width as f64, height as f64));
                let size = Size::new(width as f64, height as f64);
                size
            }
            "macos" => {
                let display_primary = Display::primary().expect("couldn't find primary display");
                //println!("Altezza layout{:?}",display_primary.height());
                let width = display_primary.width();
                let height = display_primary.height();
                ctx.window().set_position(druid::Point::new(0., 0.));
                let size = Size::new(width as f64, height as f64);

                size
            }
            _ => Size::ZERO,
        }
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        if let Some(_start) = data.start_position {
            if let Some(_end) = data.end_position {
                if data.is_selecting == true {
                    let os = env::consts::OS;
                    match os {
                        "windows" => {
                            let start_points = data.start_position_to_display.unwrap();
                            let end_points = data.end_position_to_display.unwrap();
                            let rect = druid::Rect::from_points(start_points, end_points);
                            //paint_ctx.fill(rect, &Color::rgba(0.0, 0.0, 1.0, 0.3));
                            paint_ctx.stroke(rect, &Color::WHITE, 1.0);
                        }
                        "macos" => {
                            let start_points = data.start_position_to_display.unwrap();
                            let end_points = data.end_position_to_display.unwrap();

                            let rect = druid::Rect::from_points(start_points, end_points);

                            //paint_ctx.fill(rect, &Color::rgba(0.0, 0.0, 1.0, 0.3));
                            paint_ctx.stroke(rect, &Color::BLACK, 1.0);
                        }
                        _ => {
                            // Gestisci altri sistemi operativi se necessario
                        }
                    }
                }
            }
        }
    }
}
struct MyViewHandler;

impl<W: Widget<AppData>> Controller<AppData, W> for MyViewHandler {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        ctx.request_focus();

        match event {
            Event::KeyUp(key_event) => {
                //println!("{:?}", key_event);

                if let Some(last_event) = data.last_key_event.clone() {
                    let key = match last_event.mods {
                        Mod::CONTROL => Key::Control,
                        Mod::SHIFT => Key::Shift,
                        Mod::META => Key::Meta,
                        Mod::CAPS_LOCK => Key::CapsLock,
                        _ => Key::Fn,
                    };

                    if key == (key_event.key) && key_event.key != last_event.key {
                        data.is_pressed = true;
                    } else {
                        data.is_pressed = false;
                    }
                }

                if data.is_pressed != true {
                    if data
                        .hotkeys
                        .get(2)
                        .unwrap()
                        .matches(key_event.mods, key_event.key.clone())
                        && !data.is_pressed
                    {
                        // Chiudi la finestra
                        ctx.submit_command(druid::commands::QUIT_APP);
                    } else if data
                        .hotkeys
                        .get(4)
                        .unwrap()
                        .matches(key_event.mods, key_event.key.clone())
                        && !data.is_pressed
                    {
                        data.start_position = None;
                        data.end_position = None;
                        data.start_position_to_display = None;
                        data.end_position_to_display = None;
                        data.is_dragging = false;
                        data.is_selecting = false;
                        data.modify = false;
                        data.hotkeys = Vec::new();
                        data.is_pressed = true;
                        data.hide_buttons = false;
                        data.last_key_event = Some(key_event.clone());
                        data.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
                        ctx.submit_command(druid::commands::CLOSE_WINDOW.to(ctx.window_id()));
                        let format_window = WindowDesc::new(window_format::build_ui())
                            .transparent(false)
                            .title("Choose the format. Default is .png")
                            .window_size(Size::new(200.0, 200.0))
                            .set_always_on_top(true)
                            .show_titlebar(false);
                        ctx.new_window(format_window);
                    } else if data
                        .hotkeys
                        .get(3)
                        .unwrap()
                        .matches(key_event.mods, key_event.key.clone())
                        && !data.is_pressed
                    {
                        //sto modificando
                        if data.start_position != Some(Point { x: 0., y: 0. })
                            && data.end_position != Some(Point { x: 0., y: 0. })
                        {
                            if let (Some(_start), Some(_end)) =
                                (data.start_position, data.end_position)
                            {
                                // Calculate the selected rectangle
                                data.is_dragging = true;
                                data.is_selecting = true;
                            }
                            data.is_pressed = true;
                            data.hide_buttons = true;
                            data.last_key_event = Some(key_event.clone());
                        }
                    } else if data
                        .hotkeys
                        .get(5)
                        .unwrap()
                        .matches(key_event.mods, key_event.key.clone())
                        && data.is_pressed != true
                    {
                        data.start_position = None;
                        data.end_position = None;
                        data.start_position_to_display = None;
                        data.end_position_to_display = None;
                        data.is_dragging = false;
                        data.is_selecting = false;
                        data.modify = false;
                        data.is_pressed = false;

                        data.hide_buttons = false;
                        data.last_key_event = Some(key_event.clone());
                        data.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
                        ctx.submit_command(druid::commands::CLOSE_WINDOW.to(ctx.window_id()));
                        let format_window = WindowDesc::new(window_format::build_ui())
                            .transparent(false)
                            .title("Choose the format. Default is .png")
                            .window_size(Size::new(200.0, 200.0))
                            .set_always_on_top(true)
                            .show_titlebar(false);
                        ctx.new_window(format_window);
                    } else if data
                        .hotkeys
                        .get(1)
                        .unwrap()
                        .matches(key_event.mods, key_event.key.clone())
                        && !data.is_pressed
                    {
                        data.start_position = None;
                        data.end_position = None;
                        data.start_position_to_display = None;
                        data.end_position_to_display = None;
                        data.is_dragging = false;
                        data.is_selecting = false;
                        data.modify = false;
                        data.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
                        // ctx.request_paint();
                        data.is_pressed = true;
                        data.hide_buttons = true;
                        data.last_key_event = Some(key_event.clone());

                        //println!("sei in cancel {:?}", data.last_key_event);
                    } else if data
                        .hotkeys
                        .get(0)
                        .unwrap()
                        .matches(key_event.mods, key_event.key.clone())
                        && !data.is_pressed
                    {
                        if data.start_position != None
                            && data.end_position != None
                            && data.start_position != Some(Point::new(0., 0.))
                            && data.end_position != Some(Point { x: 0., y: 0. })
                        {
                            data.hide_buttons = true;
                            data.save = true;
                            //function::save_screen(data, ctx.size());
                            //ctx.submit_command(Command::new(SAVE, "", Target::Global));
                            data.last_key_event = Some(key_event.clone());
                        }
                        //ctx.request_paint();
                    } else {
                        data.is_pressed = true;
                        data.hide_buttons = false;
                        data.last_key_event = Some(key_event.clone());
                    }
                } else {
                    data.is_pressed = false;
                    data.hide_buttons = false;
                    data.last_key_event = None;
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
    // fn lifecycle(
    //     &mut self,
    //     _child: &mut W,
    //     ctx: &mut druid::LifeCycleCtx,
    //     event: &druid::LifeCycle,
    //     data: &AppData,
    //     env: &Env,
    // ) {
    //     // match event {
    //     //     druid::LifeCycle::ViewContextChanged(_) => {
    //     //         if()
    //     //     }
    //     //     _ => {}
    //     // }
    // }
}
pub(crate) fn build_ui() -> impl Widget<AppData> {
    let skip_panel = ViewSwitcher::new(
        |data: &AppData, _env| data.hide_buttons,
        move |selector, _data, _env| {
            match selector {
            false => Box::new(
                Box::new(
                    Flex::column()
                        .with_child(
                            Flex::row()
                                .with_child(Padding::new(
                                    Insets::new(40., 40., 1., 40.),
                                    Button::new("Start").on_click(
                                        |_: &mut EventCtx, data: &mut AppData, _: &Env| {
                                            data.hide_buttons = true;
                                            data.end_position = None;
                                            data.end_position_to_display = None;
                                            data.start_position_to_display = None;
                                            data.start_position = None;
                                            data.is_dragging = false;
                                            data.is_selecting = false;
                                        },
                                    ),
                                ))
                                .with_child(Button::new("Save Screen").on_click(
                                    |_ctx: &mut EventCtx, data: &mut AppData, _: &Env| {
                                        data.hide_buttons = true;
                                        data.save = true;
                                        // function::save_screen(data, ctx.size());
                                    },
                                ))
                                .with_child(Button::new("Close").on_click(
                                    |ctx: &mut EventCtx, _data: &mut AppData, _: &Env| {
                                        ctx.submit_command(druid::commands::QUIT_APP);
                                    },
                                ))
                                .with_child(Button::new("Edit").on_click(
                                    |_ctx: &mut EventCtx, data: &mut AppData, _: &Env| {
                                        if data.start_position != None && data.end_position != None
                                        {
                                            data.hide_buttons = true;
                                            data.is_dragging = true;
                                            data.is_selecting = true;
                                        }
                                    },
                                ))
                                .with_child(Button::new("Choose your shortkeys").on_click(
                                    |ctx: &mut EventCtx, data: &mut AppData, _: &Env| {
                                        data.start_position = None;
                                        data.end_position = None;
                                        data.start_position_to_display = None;
                                        data.end_position_to_display = None;
                                        data.is_dragging = false;
                                        data.is_selecting = false;
                                        data.modify = false;
                                        data.hotkeys = Vec::new();
                                        data.is_pressed = true;
                                        data.last_key_event = None;
                                        data.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
                                        ctx.submit_command(
                                            druid::commands::CLOSE_WINDOW.to(ctx.window_id()),
                                        );
                                        let shortkeys_window = WindowDesc::new(shortkeys_window::ui_builder())    
                                        .transparent(false)
                                        .title("Choose your personal shorkeys configuration. Selecting same combinations for different commands isn't allowed")    
                                        .window_size(Size::new(1000., 1000.0))
                                        .set_always_on_top(true)    .show_titlebar(false);
                                         ctx.new_window(shortkeys_window);
                                    },
                                ))
                                .with_child(Button::new("Choose image format").on_click(
                                    |ctx: &mut EventCtx, data: &mut AppData, _: &Env| {
                                        data.start_position = None;
                                        data.end_position = None;
                                        data.start_position_to_display = None;
                                        data.end_position_to_display = None;
                                        data.is_dragging = false;
                                        data.is_selecting = false;
                                        data.modify = false;
                                        data.is_pressed = false;
                                        data.last_key_event = None;
                                        data.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
                                        ctx.submit_command(
                                            druid::commands::CLOSE_WINDOW.to(ctx.window_id()),
                                        );
                                        let format_window = WindowDesc::new(window_format::build_ui())    
                                        .transparent(false)
                                        .title("Choose the format. Default is .png")    
                                        .window_size(Size::new(200.0, 200.0))
                                        .set_always_on_top(true)    .show_titlebar(false);
                                        ctx.new_window(format_window);
                                    },
                                )),
                        )
                        .with_child(Label::new(
                            "Per uscire dalla modalità edit, premi fuori dall'area disegnata",
                        )),
                )
                .background(BackgroundBrush::Color(Color::BLACK))
                .fix_size(
                    Display::primary().expect("erro").width() as f64,
                    Display::primary().expect("erro").height() as f64,
                ),
            ),
            true => Box::new(Flex::column().with_child(DrawingArea)),
        }
        },
    );

    Flex::column()
        .with_child(skip_panel)
        .controller(MyViewHandler)
}
