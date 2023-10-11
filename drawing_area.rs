use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use druid::Color;
use druid::Data;
use druid::Env;
use druid::Event;
use druid::EventCtx;
use druid::PaintCtx;
use druid::Point;
use druid::Rect;
use druid::RenderContext;
use druid::Size;
use druid::Widget;
use druid::WindowId;
use druid::widget::Flex;

use druid_shell::MouseButton;
use scrap::Capturer;
use scrap::Display;
use druid::Lens;
use druid_shell::HotKey;
use crate::screenshot;
use crate::window_format;

#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub(crate) is_selecting: bool,
    pub(crate) start_position: Option<Point>,
    pub(crate) end_position: Option<Point>,
    pub(crate) start_position_to_display: Option<Point>,
    pub(crate) end_position_to_display: Option<Point>,
    pub(crate) modify:bool,
    pub(crate) is_dragging:bool,
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
    pub(crate) cancel_image_modifier: String,
    pub(crate) cancel_image_key: String,
    pub(crate) restart_app_modifier: String,
    pub(crate) restart_app_key: String,
    pub(crate) restart_format_app_modifier: String,
    pub(crate) restart_format_app_key: String,
    
    #[data(ignore)]
    pub(crate) format_window_id: WindowId,
    #[data(ignore)]
    pub(crate)shortkeys_window_id: WindowId,
    #[data(ignore)]
    pub(crate) main_window_id: WindowId,
    #[data(ignore)]
    pub(crate) hotkeys: Vec<HotKey>,
    
}


#[derive(Clone,Data,PartialEq)]
pub enum DragHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct DrawingArea;

impl Widget<AppData> for DrawingArea {
    fn event(&mut self, ctx: &mut EventCtx, event: &druid::Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::WindowConnected => {
                // Richiedi il focus quando la finestra è connessa.
                data.main_window_id=ctx.window_id();
                if data.modify==false
                {
                    ctx.request_focus();
                }
                
                            // Imposta la dimensione della finestra
                            let display_primary = Display::primary().expect("couldn't find primary display");
                            let size = Size::new(display_primary.width() as f64, display_primary.height() as f64); // Imposta le dimensioni desiderate qui
                            ctx.window().set_size(size);
                            //println!("size window {:?}",size);
                       
                
            }
            
               Event::KeyUp(key_event)=> {
                
                
                    if data.hotkeys.get(2).unwrap().matches(key_event){
                        // Chiudi la finestra
                        
                        ctx.submit_command(druid::commands::QUIT_APP);
                    }
                    if data.hotkeys.get(3).unwrap().matches(key_event) && data.modify==true {
                        
                                if let (Some(_start), Some(_end)) = (data.start_position, data.end_position) {
                                    // Calculate the selected rectangle
                                    data.is_dragging=true;
                                }
                            
                        
                        
                    }
                    if data.hotkeys.get(5).unwrap().matches(key_event){
                        data.start_position=None;
                        data.end_position=None;
                        data.start_position_to_display=None;
                        data.end_position_to_display=None;
                        data.is_dragging=false;
                        data.is_selecting=false;
                        data.modify=false;
                        data.rect=Rect::new(0.0, 0.0, 0.0, 0.0);
                        ctx.submit_command(druid::commands::HIDE_WINDOW.to(data.main_window_id));
                        ctx.submit_command(druid::commands::SHOW_WINDOW.to(data.format_window_id));
                    
                
                
                    }
                    if data.hotkeys.get(4).unwrap().matches(key_event) {
                        data.start_position=None;
                        data.end_position=None;
                        data.start_position_to_display=None;
                        data.end_position_to_display=None;
                        data.is_dragging=false;
                        data.is_selecting=false;
                        data.modify=false;
                        data.rect=Rect::new(0.0, 0.0, 0.0, 0.0);
                        ctx.submit_command(druid::commands::HIDE_WINDOW.to(data.main_window_id));
                        ctx.submit_command(druid::commands::SHOW_WINDOW.to(data.shortkeys_window_id));
                    
                
                
                    }
                    if data.hotkeys.get(1).unwrap().matches(key_event){
                        // Chiudi la finestra
                        data.start_position=None;
                        data.end_position=None;
                        data.start_position_to_display=None;
                        data.end_position_to_display=None;
                        data.is_dragging=false;
                        data.is_selecting=false;
                        data.modify=false;
                        data.rect=Rect::new(0.0, 0.0, 0.0, 0.0);
                        ctx.request_paint();
                    }
                    
                    if data.hotkeys.get(0).unwrap().matches(key_event) {
                        
                        if let (Some(start), Some(end)) = (data.start_position, data.end_position) {
                            data.is_dragging=false;
                            
                           
                            let name= data.label.clone();
                            let format= data.radio_group;
                            let size_clone= Arc::new(Mutex::new(None));
                        
                             // Calcola il rettangolo selezionato
                            let rect = druid::Rect::from_points(start, end);
                            
                            // Chiama la funzione per catturare lo screenshot
                            
                            let start_position = Arc::new(Mutex::new(None));
                            let end_position = Arc::new(Mutex::new(None));
                            //println!("Selected area: {:?}, {:?}", (rect.x0, rect.y0), (rect.x1, rect.y1));
                            // Crea un thread separato per catturare lo screenshot
                            let size= ctx.window().get_size();
                            
                            let screenshot_thread = thread::spawn(move || {
    
                                
                                // Imposta i dati di trascinamento per iniziare la cattura
                                let end_position_clone_2 = Arc::clone(&end_position);
                                let start_position_clone_3 = Arc::clone(&start_position);
                                
                                // Cattura uno screenshot.
                                let display = Display::primary().expect("couldn't find primary display");
                                let (width, height) = (display.width(), display.height());
                                //println!("Larghezza display: {:?}, altezza display: {:?}",display.width(),display.height());
                                let capturer: Capturer = Capturer::new(display).expect("couldn't begin capture");
                                
                                let size_clone2= Arc::clone(&size_clone);
                                *size_clone2.lock().unwrap()=Some(size);
                                
                                *start_position_clone_3.lock().unwrap() = Some((rect.x0, rect.y0));
                                *end_position_clone_2.lock().unwrap() = Some((rect.x1, rect.y1));
                                //println!("Selected area: {:?}, {:?}", (*start_position_clone_3.lock().unwrap()), *end_position_clone_2.lock().unwrap() );
                                // Chiama la funzione di cattura screenshot
                                //println!("wid: {:?}, {:?}",size.width,size.height);
                                
                                
                                screenshot::screen(format, capturer, width as u32, height as u32, start_position_clone_3, end_position_clone_2,name);
                                
                    });
                    // Attendi la fine del thread di cattura screenshot
                    screenshot_thread.join().unwrap();
                    ctx.request_paint();
                    
                    data.is_dragging=false;
                    data.is_selecting=false;
                    data.modify=false;
                    
                
                    
                    
                }
                    }
                }
               Event::WindowCloseRequested => {
                    // Qui puoi gestire l'evento di chiusura della finestra.
                    // Ad esempio, potresti voler salvare i dati dell'applicazione o mostrare un messaggio all'utente.
                    ctx.submit_command(druid::commands::QUIT_APP);
                    
                    
        
                }
            
                
            druid::Event::MouseDown(mouse_event) => {
                if data.modify==true && data.is_dragging==false{
                    data.start_position = None;
                    data.end_position = None; 
                    data.start_position_to_display=None;
                    data.end_position_to_display=None;
                    data.is_selecting=false;
                    data.is_dragging=false;
                    data.modify=false;

                }
                if data.modify==false && data.is_dragging==false
                   { if mouse_event.button == MouseButton::Left {
                        data.start_position = None;
                        data.end_position = None; 
                    let os = env::consts::OS;
                    match os {
                        "windows" => {
                            let scale_factor_x = ctx.window().get_scale().unwrap().x();
                            let scale_factor_y = ctx.window().get_scale().unwrap().y();
                            let coord = druid::Point{ x:mouse_event.pos.x * scale_factor_x ,y:mouse_event.pos.y*scale_factor_y};
                            data.start_position = Some(coord);
                        }
                        "macos" => {
                            
                            
                            let pos= ctx.to_screen(druid::Point::new(mouse_event.pos
                                .x, mouse_event.pos.y));
                            //let size=ctx.window().get_size();
                            //println!("size: {:?}",size);
                            let coord = druid::Point{ x: pos.x,y:pos.y};
                            data.start_position_to_display=Some(druid::Point{ x: mouse_event.pos.x,y:mouse_event.pos.y});
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
                if data.is_dragging==true {
                    //println!("{:?}",(mouse_event.pos - data.rect.origin()).hypot());
                    if (mouse_event.pos - data.rect.origin()).hypot() < 70.0 {
                        data.where_dragging = Some(DragHandle::TopLeft);
                        ctx.set_active(true);
                    } else if (mouse_event.pos - Point::new(data.rect.x1, data.rect.y1)).hypot() < 70.0 {
                        data.where_dragging = Some(DragHandle::BottomRight);
                        ctx.set_active(true);
                    }
                    else if (mouse_event.pos - Point::new(data.rect.x0, data.rect.y1)).hypot() < 70.0 {
                        data.where_dragging = Some(DragHandle::BottomLeft);
                        ctx.set_active(true);
                    }
                    else if (mouse_event.pos - Point::new(data.rect.x1, data.rect.y0)).hypot() < 70.0 {
                        data.where_dragging = Some(DragHandle::TopRight);
                        ctx.set_active(true);
                    }
                    data.is_selecting=true;
                }
            }

            druid::Event::MouseMove(mouse_event) => {
                // Aggiorna la posizione finale del rettangolo qui
                
                
                let os = env::consts::OS;
                    match os {
                        "windows" =>{
                            let scale_factor_x = ctx.window().get_scale().unwrap().x();
                            let scale_factor_y = ctx.window().get_scale().unwrap().y();
                            let coord = druid::Point{ x:mouse_event.pos.x * scale_factor_x ,y:mouse_event.pos.y*scale_factor_y};
                            data.end_position = Some(coord);
                        }
                        "macos" => {
                            if ctx.is_active()==false && data.is_dragging==false
                            {
                                
                                let pos= ctx.to_screen(druid::Point::new(mouse_event.pos
                                .x, mouse_event.pos.y));
                           
                            let coord = druid::Point{ x: pos.x,y:pos.y};
                            data.end_position_to_display=Some(druid::Point{ x: mouse_event.pos.x,y:mouse_event.pos.y});
                            data.end_position= Some(coord);
                            }
                            if ctx.is_active() {
                    
                                if let Some(handle) = &data.where_dragging {
                                    match handle {
                                        DragHandle::TopLeft => {
                                            
                                            data.rect.x0 = mouse_event.pos.x;
                                            data.rect.y0 = mouse_event.pos.y;
                                            let pos= ctx.to_screen(druid::Point::new(mouse_event.pos
                                                .x, mouse_event.pos.y));
                                           
                                            let coord = druid::Point{ x: pos.x,y:pos.y};
                                            data.start_position_to_display=Some(druid::Point{ x: mouse_event.pos.x,y:mouse_event.pos.y});
                                            data.start_position= Some(coord);
                                            data.is_selecting=true;
                                            
                                            //println!("{:?}, {:?}",data.start_position,data.end_position);
                                            
                                        }
                                        DragHandle::BottomRight => {
                                            data.rect.x1 = mouse_event.pos.x;
                                            data.rect.y1 = mouse_event.pos.y;
                                            let pos= ctx.to_screen(druid::Point::new(mouse_event.pos
                                                .x, mouse_event.pos.y));
                                           
                                            let coord = druid::Point{ x: pos.x,y:pos.y};
                                            data.end_position_to_display=Some(druid::Point{ x: mouse_event.pos.x,y:mouse_event.pos.y});
                                            data.end_position= Some(coord);
                                            data.is_selecting=true;
                                            
                                            
                                        }
                                        DragHandle::BottomLeft => {
                                            data.rect.x0 = mouse_event.pos.x;
                                            data.rect.y1 = mouse_event.pos.y;
                                            let pos= ctx.to_screen(druid::Point::new(mouse_event.pos
                                                .x, mouse_event.pos.y));
                                           
                                            let coord = druid::Point{ x: data.end_position.unwrap().x,y:pos.y};
                                            data.end_position_to_display=Some(druid::Point{ x: data.end_position_to_display.unwrap().x,y:mouse_event.pos.y});
                                            data.end_position= Some(coord);
                                            let coord= druid::Point {x: pos.x, y: data.start_position.unwrap().y};
                                            data.start_position_to_display= Some(druid::Point {x:data.rect.x0, y:data.start_position_to_display.unwrap().y});
                                            data.start_position= Some(coord);
                                            data.is_selecting=true;
                                            
                                            
                                            
                                        }
                                        DragHandle::TopRight => {
                                            data.rect.x1 = mouse_event.pos.x;
                                            data.rect.y0 = mouse_event.pos.y;
                                            let pos= ctx.to_screen(druid::Point::new(mouse_event.pos
                                                .x, mouse_event.pos.y));
                                           
                                            let coord = druid::Point{ x: pos.x,y:data.end_position.unwrap().y};
                                            data.end_position_to_display=Some(druid::Point{ x: mouse_event.pos.x,y:data.end_position_to_display.unwrap().y});
                                            data.end_position= Some(coord);
                                            let coord= druid::Point {x:data.start_position.unwrap().x, y: pos.y};

                                            data.start_position_to_display= Some(druid::Point {x:data.start_position_to_display.unwrap().x, y:data.rect.y0});
                                            data.start_position= Some(coord);
                                            data.is_selecting=true;
                                           
                                            
                                        }
                                    }
                                    
                                    ctx.request_paint();
                                }
                            }
                            
                        }
                        _ => {
                            // Gestisci altri sistemi operativi se necessario
                        }
                        
                    }
                
                
                
                // Richiedi un aggiornamento del widget per ridisegnare il rettangolo
               
                if data.modify==false 
                {
                    ctx.request_paint();
                }
                
                
            }
            
            druid::Event::MouseUp(mouse_event) => {
                if data.is_dragging==true {
                    data.where_dragging=None;
                    ctx.set_active(false);
                    data.is_selecting=true;

                }
                if data.modify==false && data.is_dragging==false
                {
                    if mouse_event.button == MouseButton::Left {
                    
                    data.is_selecting = false;
                    data.modify=true;
                    //ctx.request_paint();
                    //println!("Click end: {:?}", mouse_event.pos);
                    //thread::sleep(Duration::from_millis(1000));
                let os = env::consts::OS;
                    match os {
                        "windows" =>{
                            let scale_factor_x = ctx.window().get_scale().unwrap().x();
                            let scale_factor_y = ctx.window().get_scale().unwrap().y();
                            let coord = druid::Point{ x:mouse_event.pos.x * scale_factor_x ,y:mouse_event.pos.y*scale_factor_y};
                            data.end_position = Some(coord);
                        }
                        _ => {
                            
                            let pos= ctx.to_screen(druid::Point::new(mouse_event.pos
                                .x, mouse_event.pos.y));
                            
                            
                            let coord = druid::Point{ x: pos.x,y:pos.y};
                            
                            data.end_position_to_display=Some(druid::Point{ x: mouse_event.pos.x,y:mouse_event.pos.y});
                            data.end_position = Some(coord);
                            data.rect = druid::Rect::from_points(data.start_position.unwrap(), data.end_position.unwrap());
                            
                        }
                        
                    }
           
            

            
                    
                }}
            
            }
            
            _ => {}
            
        }
        
        
        
    }
    



    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &AppData, _env: &Env) {
        
    }
   
    
    
   
    

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {
        
        
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, _data: &AppData, _env: &Env) -> druid::Size {
        let os= std::env::consts::OS;
        match os 
        {
            "windows" => {// Set the size of the drawing area.
            let display_primary = Display::primary().expect("couldn't find primary display");
            //println!("Altezza layout{:?}",display_primary.height());
            let width = display_primary.width();
            let height = display_primary.height();
        
            ctx.set_paint_insets(druid::Insets::uniform_xy(width as f64, height as f64));
            let size = Size::new(width as f64, height as f64);
            size
            }
            "macos" => {
                    
                    let pos=ctx.window().get_position();
                    
                    let display_primary = Display::primary().expect("couldn't find primary display");
                    //println!("Altezza layout{:?}",display_primary.height());
                    let width = display_primary.width();
                    let height = display_primary.height();
                    ctx.window().set_position(druid::Point::new(0.-pos.x, 0.-pos.y));
                    let size = Size::new(width as f64, height as f64);
            size
                        
                
            }
            _ =>{
                Size::ZERO
            }
        }



    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &AppData, _env: &Env)  { 
        

        let scale_factor_x = paint_ctx.window().get_scale().unwrap().x();
        let scale_factor_y = paint_ctx.window().get_scale().unwrap().y();
        
        if let Some(start) = data.start_position {
        
            if let Some(end) = data.end_position {
                
                if data.is_selecting == true{
                    
                    let os = env::consts::OS;
                    match os {
                        "windows" =>{
                            let start_descaled = druid::Point{x:start.x/scale_factor_x,y:start.y/scale_factor_y};
                            let end_descaled = druid::Point{x:end.x/scale_factor_x,y:end.y/scale_factor_y};
                            let rect = druid::Rect::from_points(start_descaled, end_descaled);
                            //paint_ctx.fill(rect, &Color::rgba(0.0, 0.0, 1.0, 0.3));
                            paint_ctx.stroke(rect, &Color::WHITE, 1.0);
                        }
                        "macos" => {
                           
                                
                                let start_points= data.start_position_to_display.unwrap();
                                let end_points= data.end_position_to_display.unwrap();
                                
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


pub(crate) fn build_ui() -> impl Widget<AppData> {
 
     Flex::column()
     .with_child(DrawingArea)
      
     
  }