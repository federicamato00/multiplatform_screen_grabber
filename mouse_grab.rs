use rdev::{listen, Event, EventType, Button};
use std::sync::{Arc, Mutex};
use druid::{EventCtx};


pub fn mouse_grab(current_position_clone:Arc<Mutex<Option<(f64, f64)>>>,is_dragging_clone:Arc<Mutex<Option<bool>>>,start_position_clone:Arc<Mutex<Option<(f64, f64)>>>,end_position_clone:Arc<Mutex<Option<(f64, f64)>>>,ctx: &mut EventCtx,data : &AppData){
    if let Err(error) = listen(move |event| {
        match event.event_type {
            EventType::MouseMove { x, y } => {
                // Aggiorna la posizione corrente del mouse
                *current_position_clone.lock().unwrap() = Some((x, y));
                // Richiedi re-draw
                data.end_position = Some((x,y));
                data.is_selecting = true;
                ctx.request_paint();
            },
            rdev::EventType::ButtonPress(button)=> {
                if button == rdev::Button::Left{
                    if let Some(position) = *current_position_clone.lock().unwrap(){
                        println!("Mouse position when left button pressed: x: {}, y: {}", position.0, position.1);
                        *is_dragging_clone.lock().unwrap() = Some(true);
                        *start_position_clone.lock().unwrap() = Some(position);
                    }
                }
            }
            rdev::EventType::ButtonRelease(button) => {
                if button == rdev::Button::Left{
                    if let Some(position) = *current_position_clone.lock().unwrap(){
                        
                        println!("Mouse position when left button released: x: {}, y: {}", position.0, position.1);
                        *is_dragging_clone.lock().unwrap() = Some(false);
                        *end_position_clone.lock().unwrap() = Some(position);
                    }
                }
            }
            _ => (),
        }
    }) {
        println!("Error: {:?}", error)
    }
}