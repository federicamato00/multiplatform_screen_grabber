use std::{
    sync::{Arc, Mutex},
    thread,
};

use druid::{MouseEvent, Point, Size};

use scrap::{Capturer, Display};

use crate::{
    drawing_area::{self, AppData, DragHandle},
    screenshot,
};

pub(crate) fn some_fields_are_equal(data: &drawing_area::AppData) -> bool {
    if (data.start_image_modifier == data.save_image_modifier
        && data.start_image_key == data.save_image_key)
        || (data.start_image_modifier == data.restart_app_modifier
            && data.start_image_key == data.restart_app_key)
        || (data.start_image_modifier == data.restart_format_app_modifier
            && data.start_image_key == data.restart_format_app_key)
        || (data.start_image_modifier == data.edit_image_modifier
            && data.start_image_key == data.edit_image_key)
        || (data.start_image_modifier == data.quit_app_modifier
            && data.start_image_key == data.quit_app_modifier)
        || (data.save_image_modifier == data.quit_app_modifier
            && data.save_image_key == data.quit_app_modifier)
        || (data.save_image_modifier == data.restart_format_app_modifier
            && data.save_image_key == data.restart_format_app_key)
        || (data.save_image_modifier == data.restart_app_modifier
            && data.save_image_key == data.restart_app_key)
        || (data.save_image_modifier == data.edit_image_modifier
            && data.save_image_key == data.edit_image_key)
        || (data.quit_app_modifier == data.edit_image_modifier
            && data.quit_app_key == data.edit_image_key)
        || (data.quit_app_modifier == data.restart_app_modifier
            && data.quit_app_key == data.restart_app_key)
        || (data.quit_app_modifier == data.restart_format_app_modifier
            && data.quit_app_key == data.restart_format_app_key)
        || (data.edit_image_modifier == data.restart_app_modifier
            && data.edit_image_key == data.restart_app_key)
        || (data.restart_app_modifier == data.restart_format_app_modifier
            && data.restart_app_key == data.restart_format_app_key)
        || (data.edit_image_modifier == data.restart_format_app_modifier
            && data.edit_image_key == data.restart_format_app_key)
    {
        true
    } else {
        false
    }
}

pub(crate) fn are_all_fields_completed(data: &drawing_area::AppData) -> bool {
    if (data.save_image_modifier != "None".to_string() || data.save_image_key != "".to_string())
        && (data.edit_image_modifier != "None".to_string() || data.edit_image_key != "".to_string())
        && (data.quit_app_key != "".to_string() || data.quit_app_modifier != "None".to_string())
        && (data.start_image_key != "".to_string()
            || data.start_image_modifier != "None".to_string())
        && (data.restart_app_modifier != "None".to_string()
            || data.restart_app_key != "".to_string())
        && (data.restart_format_app_modifier != "None".to_string()
            || data.restart_format_app_key != "".to_string())
    {
        true
    } else {
        false
    }
}

pub(crate) fn save_screen(data: &mut drawing_area::AppData, size: Size) {
    if let (Some(start), Some(end)) = (data.start_position, data.end_position) {
        data.is_dragging = false;

        let name = data.label.clone();
        let format = data.radio_group;
        let size_clone = Arc::new(Mutex::new(None));

        // Calcola il rettangolo selezionato
        let rect = druid::Rect::from_points(start, end);

        // Chiama la funzione per catturare lo screenshot

        let start_position = Arc::new(Mutex::new(None));
        let end_position = Arc::new(Mutex::new(None));
        //println!("Selected area: {:?}, {:?}", (rect.x0, rect.y0), (rect.x1, rect.y1));
        // Crea un thread separato per catturare lo screenshot

        let screenshot_thread = thread::spawn(move || {
            // Imposta i dati di trascinamento per iniziare la cattura
            let end_position_clone_2 = Arc::clone(&end_position);
            let start_position_clone_3 = Arc::clone(&start_position);

            // Cattura uno screenshot.
            let display = Display::primary().expect("couldn't find primary display");
            let (width, height) = (display.width(), display.height());
            //println!("Larghezza display: {:?}, altezza display: {:?}",display.width(),display.height());
            let capturer: Capturer = Capturer::new(display).expect("couldn't begin capture");

            let size_clone2 = Arc::clone(&size_clone);
            *size_clone2.lock().unwrap() = Some(size);

            *start_position_clone_3.lock().unwrap() = Some((rect.x0, rect.y0));
            *end_position_clone_2.lock().unwrap() = Some((rect.x1, rect.y1));
            //println!("Selected area: {:?}, {:?}", (*start_position_clone_3.lock().unwrap()), *end_position_clone_2.lock().unwrap() );
            // Chiama la funzione di cattura screenshot
            //println!("wid: {:?}, {:?}",size.width,size.height);

            screenshot::screen(
                format,
                capturer,
                width as u32,
                height as u32,
                start_position_clone_3,
                end_position_clone_2,
                name,
            );
        });
        // Attendi la fine del thread di cattura screenshot
        screenshot_thread.join().unwrap();

        data.is_dragging = false;
        data.is_selecting = false;
        data.modify = false;
        data.is_found = false;
        data.hide_buttons = false;
        data.save = false;
        //println!("{:?}",data.last_key_event);
    }
}

pub(crate) fn edit_rect(
    handle: &DragHandle,
    pos: Point,
    data: &mut AppData,
    mouse_event: &MouseEvent,
) {
    match handle {
        DragHandle::TopLeft => {
            data.rect.x0 = mouse_event.pos.x;
            data.rect.y0 = mouse_event.pos.y;
            // let pos = ctx.to_screen(druid::Point::new(mouse_event.pos.x, mouse_event.pos.y));

            let coord = druid::Point { x: pos.x, y: pos.y };
            data.start_position_to_display = Some(druid::Point {
                x: mouse_event.pos.x,
                y: mouse_event.pos.y,
            });
            data.start_position = Some(coord);
            data.is_selecting = true;

            //println!("{:?}, {:?}",data.start_position,data.end_position);
        }
        DragHandle::BottomRight => {
            data.rect.x1 = mouse_event.pos.x;
            data.rect.y1 = mouse_event.pos.y;
            // let pos = ctx.to_screen(druid::Point::new(mouse_event.pos.x, mouse_event.pos.y));

            let coord = druid::Point { x: pos.x, y: pos.y };
            data.end_position_to_display = Some(druid::Point {
                x: mouse_event.pos.x,
                y: mouse_event.pos.y,
            });
            data.end_position = Some(coord);
            data.is_selecting = true;
        }
        DragHandle::BottomLeft => {
            data.rect.x0 = mouse_event.pos.x;
            data.rect.y1 = mouse_event.pos.y;
            // let pos = ctx.to_screen(druid::Point::new(mouse_event.pos.x, mouse_event.pos.y));

            let coord = druid::Point {
                x: data.end_position.unwrap().x,
                y: pos.y,
            };
            data.end_position_to_display = Some(druid::Point {
                x: data.end_position_to_display.unwrap().x,
                y: mouse_event.pos.y,
            });
            data.end_position = Some(coord);
            let coord = druid::Point {
                x: pos.x,
                y: data.start_position.unwrap().y,
            };
            data.start_position_to_display = Some(druid::Point {
                x: data.rect.x0,
                y: data.start_position_to_display.unwrap().y,
            });
            data.start_position = Some(coord);
            data.is_selecting = true;
        }
        DragHandle::TopRight => {
            data.rect.x1 = mouse_event.pos.x;
            data.rect.y0 = mouse_event.pos.y;
            // let pos = ctx.to_screen(druid::Point::new(mouse_event.pos.x, mouse_event.pos.y));

            let coord = druid::Point {
                x: pos.x,
                y: data.end_position.unwrap().y,
            };
            data.end_position_to_display = Some(druid::Point {
                x: mouse_event.pos.x,
                y: data.end_position_to_display.unwrap().y,
            });
            data.end_position = Some(coord);
            let coord = druid::Point {
                x: data.start_position.unwrap().x,
                y: pos.y,
            };

            data.start_position_to_display = Some(druid::Point {
                x: data.start_position_to_display.unwrap().x,
                y: data.rect.y0,
            });
            data.start_position = Some(coord);
            data.is_selecting = true;
        }
    }
}
