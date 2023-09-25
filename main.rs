use druid::widget::{Button, Flex,Controller};
use druid::widget::prelude::*;
use druid::KbKey;
use druid::{AppLauncher, Color, Data, Env,Event, EventCtx, LocalizedString, MouseButton, PaintCtx, Point, RenderContext, Widget, WidgetExt, WindowDesc};
use std::sync::{Arc, Mutex};
use scrap::Display;
use scrap::Capturer;
use std::process::exit;
use std::thread;

//use crate::mouse_grab::mouse_grab;
mod screenshot;
//mod mouse_grab;


#[derive(Clone, Data, Default)]
struct AppData {
    is_selecting: bool,
    start_position: Option<Point>,
    end_position: Option<Point>,
    
}

struct EscapeController;

impl<T, W: Widget<T>> Controller<T, W> for EscapeController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env,
    ) {
        if let Event::KeyDown(key_event) = event {
            if key_event.key == KbKey::Escape {
                // Chiudi la finestra
                ctx.window().close();
                exit(0);
            }
        }
        if let Event::WindowCloseRequested = event {
            // Qui puoi gestire l'evento di chiusura della finestra.
            // Ad esempio, potresti voler salvare i dati dell'applicazione o mostrare un messaggio all'utente.
            ctx.submit_command(druid::commands::QUIT_APP);
            

        }
        child.event(ctx, event, data, env);
    }
}



struct DrawingArea;




impl Widget<AppData> for DrawingArea {
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut AppData, _env: &Env) {
        match event {
            druid::Event::MouseDown(mouse_event) => {
                if mouse_event.button == MouseButton::Left {
                    data.start_position = None;
                    data.end_position = None; 
                    data.start_position = Some(mouse_event.pos);
                    data.is_selecting = true;
                }
            }
            druid::Event::MouseUp(mouse_event) => {
                if mouse_event.button == MouseButton::Left {
                    data.end_position = Some(mouse_event.pos);
                    data.is_selecting = false;
            if let (Some(start), Some(end)) = (data.start_position, data.end_position) {
                // Calcola il rettangolo selezionato
                let rect = druid::Rect::from_points(start, end);
                // Chiama la funzione per catturare lo screenshot
                let is_dragging = Arc::new(Mutex::new(None));
                let start_position = Arc::new(Mutex::new(None));
                let end_position = Arc::new(Mutex::new(None));
                println!("Selected area: {:?}, {:?}", (rect.x0, rect.y0), (rect.x1, rect.y1));
                // Crea un thread separato per catturare lo screenshot
                let screenshot_thread = thread::spawn(move || {

                    
                        // Imposta i dati di trascinamento per iniziare la cattura
                        let end_position_clone_2 = Arc::clone(&end_position);
                        let start_position_clone_3 = Arc::clone(&start_position);
                        let is_dragging_clone_2 = Arc::clone(&is_dragging);
                        // Cattura uno screenshot.
                        let display = Display::primary().expect("couldn't find primary display");
                        let capturer: Capturer = Capturer::new(display).expect("couldn't begin capture");
                        let (width, height) = (capturer.width(), capturer.height());

                        *is_dragging_clone_2.lock().unwrap() = Some(true);
                        *start_position_clone_3.lock().unwrap() = Some((rect.x0, rect.y0));
                        *end_position_clone_2.lock().unwrap() = Some((rect.x1, rect.y1));
                        
                        // Chiama la funzione di cattura screenshot
                        screenshot::screen(is_dragging_clone_2, capturer, width as u32, height as u32, start_position_clone_3, end_position_clone_2);
                    
                });
                
                // Attendi la fine del thread di cattura screenshot
                screenshot_thread.join().unwrap();
                
            }

            ctx.request_paint();
                    
                }
            
            }
            _ => {}
            
        }
        
        ctx.request_paint();
        
    }
    // ...



    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, _data: &AppData, _env: &Env) {
        match event {
            druid::LifeCycle::WidgetAdded => {
                // Qui puoi gestire l'evento di disconnessione della finestra.

            }
            
            
            _ => {}
        }
    }
   
    
    
   
    

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, _data: &AppData, _env: &Env) -> druid::Size {
        // Set the size of the drawing area.
        let display_primary = Display::primary().expect("couldn't find primary display");
        let width = display_primary.width();
        let height = display_primary.height();
        ctx.set_paint_insets(druid::Insets::uniform_xy(width as f64, height as f64));
        let size = Size::new(width as f64, height as f64);
        size

    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &AppData, _env: &Env)  { 
        // Disegna il rettangolo selezionato sopra l'area di disegno.
        if let Some(start) = data.start_position {
            if let Some(end) = data.end_position {
                let rect = druid::Rect::from_points(start, end);
                paint_ctx.fill(rect, &Color::rgba(0.0, 0.0, 1.0, 0.3));
                paint_ctx.stroke(rect, &Color::BLACK, 1.0);
            }
        }
       
        
       
    }
    
}

fn main() {

    
    let display_primary = Display::primary().expect("couldn't find primary display");
    let width = display_primary.width();
    let height = display_primary.height();
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("Screen Capture Utility"))
        .transparent(true)
        .window_size((width as f64, height as f64));

    let initial_data = AppData::default();
    let launcher = AppLauncher::with_window(main_window);
    launcher
        .launch(initial_data)
        .expect("Failed to launch application");
    
   
}


fn build_ui() -> impl Widget<AppData> {
        
    let button = Button::new("Close Window").on_click(|ctx, _data: &mut AppData, _| {
        ctx.submit_command(druid::commands::QUIT_APP);
        
    });

    // Aggiungi un filtro degli eventi per chiudere la finestra quando viene premuto il tasto ESC
    button.controller(EscapeController);
   
    let button = Button::new("Close Window").on_click(|ctx, _, _| {
        ctx.submit_command(druid::commands::QUIT_APP);
        
    });

    Flex::column()
        .with_child(button)
        .with_child(DrawingArea) 
        
        
}
