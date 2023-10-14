use druid::widget::{Flex, Label, RadioGroup, TextBox, Button, Controller};
use druid::{Widget, WidgetExt, WindowDesc, Size, EventCtx, Event, Env, UpdateCtx, LifeCycleCtx, LifeCycle, BoxConstraints, LayoutCtx, PaintCtx};
use druid_shell::keyboard_types::Key;
use druid_shell::RawMods;
use druid::HotKey;
use druid::Event::KeyDown;
use crate::drawing_area;
use crate::window_format;
pub struct AppDataHandler;



struct MyController;

impl Controller<String, TextBox<String>> for MyController {
    fn event(
        &mut self,
        child: &mut TextBox<String>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut String,
        env: &Env,
    ) {
        match event {

            
            KeyDown(_key_event) => {
                if data.len()>1{
                    
                    data.truncate(1);
                    
                }
            }
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}


impl Widget<drawing_area::AppData> for AppDataHandler {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut drawing_area::AppData, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_focus()
            },
            Event::WindowCloseRequested => {
                
                ctx.submit_command(druid::commands::QUIT_APP);
            }
            
            Event::KeyUp(key_event) => {
                
                if !data.hotkeys.is_empty()
                {
                if data.hotkeys.get(2).unwrap().matches(key_event){
                    // Chiudi la finestra
                    
                    ctx.submit_command(druid::commands::QUIT_APP);
                }
                
                if data.hotkeys.get(4).unwrap().matches(key_event){
                    
                    ctx.submit_command(druid::commands::HIDE_WINDOW.to(data.format_window_id));
                    ctx.submit_command(druid::commands::SHOW_WINDOW.to(data.shortkeys_window_id));
                
            
            
                }
                }
            },
            
            _=>{

            }
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx,_eventt: &LifeCycle, _data: &drawing_area::AppData, _env: &Env) {
       
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &drawing_area::AppData, _data: &drawing_area::AppData, _env: &Env) {
        
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, _bc: &BoxConstraints, _data: &drawing_area::AppData, _env: &Env) -> Size {
        Size::new(ctx.window().get_size().width, ctx.window().get_size().height)
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &drawing_area::AppData, _env: &Env) {
        
    }

    
}

pub(crate) fn ui_builder() -> impl Widget<drawing_area::AppData> {
    
    
    let save_image = Flex::row()
        .with_child(Label::new("Save modifier: "))
        .with_child(RadioGroup::row(vec![("Ctrl","Ctrl".to_string()), ("Shift","Shift".to_string()),("Escape","Escape".to_string()),("Enter","Enter".to_string()), ("Alt", "Alt".to_string())]).lens(drawing_area::AppData::save_image_modifier))
        .with_child(Label::new("Save Image Key: "))
        .with_child(TextBox::new().controller(MyController).lens(drawing_area::AppData::save_image_key));
    

    let quit_app = Flex::row()
        .with_child(Label::new("Quit modifier: "))
        .with_child(RadioGroup::row(vec![("Ctrl","Ctrl".to_string()), ("Shift","Shift".to_string()),("Escape","Escape".to_string()),("Enter","Enter".to_string()), ("Alt", "Alt".to_string())]).lens(drawing_area::AppData::quit_app_modifier))
        .with_child(Label::new("Quit App Key: "))
        .with_child(TextBox::new().controller(MyController).lens(drawing_area::AppData::quit_app_key));

    let edit_image = Flex::row()
        .with_child(Label::new("Edit modifier: "))
        .with_child(RadioGroup::row(vec![("Ctrl","Ctrl".to_string()), ("Shift","Shift".to_string()),("Escape","Escape".to_string()),("Enter","Enter".to_string()), ("Alt", "Alt".to_string())]).lens(drawing_area::AppData::edit_image_modifier))
        .with_child(Label::new("Edit Image Key: "))
        .with_child(TextBox::new().controller(MyController).lens(drawing_area::AppData::edit_image_key));
    let cancel_image = Flex::row()
        .with_child(Label::new("Cancel modifier: "))
        .with_child(RadioGroup::row(vec![("Ctrl","Ctrl".to_string()), ("Shift","Shift".to_string()),("Escape","Escape".to_string()),("Enter","Enter".to_string()), ("Alt", "Alt".to_string())]).lens(drawing_area::AppData::cancel_image_modifier))
        .with_child(Label::new("Cancel Image Key: "))
        .with_child(TextBox::new().controller(MyController).lens(drawing_area::AppData::cancel_image_key));

    let restart = Flex::row()
        .with_child(Label::new("Restart modifier: "))
        .with_child(RadioGroup::row(vec![("Ctrl","Ctrl".to_string()), ("Shift","Shift".to_string()),("Escape","Escape".to_string()),("Enter","Enter".to_string()), ("Alt", "Alt".to_string())]).lens(drawing_area::AppData::restart_app_modifier))
        .with_child(Label::new("Restar Image Key: "))
        .with_child(TextBox::new().controller(MyController).lens(drawing_area::AppData::restart_app_key));
    let choose_format: Flex<drawing_area::AppData> = Flex::row()
        .with_child(Label::new("Rechoose format modifier: "))
        .with_child(RadioGroup::row(vec![("Ctrl","Ctrl".to_string()), ("Shift","Shift".to_string()),("Escape","Escape".to_string()),("Enter","Enter".to_string()), ("Alt", "Alt".to_string())]).lens(drawing_area::AppData::restart_format_app_modifier))
        .with_child(Label::new("Rechoose format Image Key: "))
        .with_child(TextBox::new().with_placeholder("Inserisci un solo carattere").controller(MyController).lens(drawing_area::AppData::restart_format_app_key));


    let apply_button = Button::new("Apply").on_click(|ctx, data: &mut drawing_area::AppData, _env| {
        // Qui puoi definire le tue HotKey basate sui valori in data
        
        let save_image_modifier = match data.save_image_modifier.as_str() {
            "Ctrl" => Some(RawMods::Ctrl), 
            "Shift" => Some(RawMods::Shift),
            "Escape" => Some(RawMods::None),
            "Enter" => Some(RawMods::Meta),
            "Alt" => Some(RawMods::Alt),
            _ => None,
        };
        let mut key=data.save_image_key.clone();
       
        //println!("{:?}",data.save_image_key);
       if (save_image_modifier.eq(&Some(RawMods::Ctrl)) && data.save_image_key=="".to_string()) || (save_image_modifier.eq(&Some(RawMods::Shift)) && data.save_image_key=="".to_string()){
            if save_image_modifier.eq(&Some(RawMods::Ctrl))
            {
                let save_image_hotkey = HotKey::new(None, Key::Control);
                data.hotkeys.push(save_image_hotkey);
            }
            else {
                let save_image_hotkey = HotKey::new(None, Key::Shift);
                data.hotkeys.push(save_image_hotkey);
            }
           }
       else if save_image_modifier.eq(&Some(RawMods::Shift)){
            key.make_ascii_uppercase();
            let save_image_hotkey = HotKey::new(save_image_modifier, Key::Character(key));
            data.hotkeys.push(save_image_hotkey);

       }
       
       else if save_image_modifier.eq(&Some(RawMods::None))
        {
        let save_image_hotkey = HotKey::new(None, Key::Escape);
        data.hotkeys.push(save_image_hotkey);

        }
        else if save_image_modifier.eq(&Some(RawMods::Meta)) {
            let save_image_hotkey = HotKey::new(None, Key::Enter);
            data.hotkeys.push(save_image_hotkey);
        }
        else 
        {
            
        let save_image_hotkey = HotKey::new(save_image_modifier, Key::Character(key));
        data.hotkeys.push(save_image_hotkey);
        }
        let cancel_image_modifier = match data.cancel_image_modifier.as_str() {
            "Ctrl" => Some(RawMods::Ctrl),
            "Shift" => Some(RawMods::Shift),
            "Escape" => Some(RawMods::None),
            "Enter" => Some(RawMods::Meta),
            "Alt" => Some(RawMods::Alt),
            _ => None,
        };
        let mut key=data.cancel_image_key.clone();
        //println!("{:?}",data.save_image_key);
       if (cancel_image_modifier.eq(&Some(RawMods::Ctrl)) && data.cancel_image_key=="".to_string()) || ((cancel_image_modifier.eq(&Some(RawMods::Shift)) && data.cancel_image_key=="".to_string()) ){
            if cancel_image_modifier.eq(&Some(RawMods::Ctrl)){
                let cancel_image_hotkey = HotKey::new(None, Key::Control);
                data.hotkeys.push(cancel_image_hotkey);
            }
            else {
                let cancel_image_hotkey = HotKey::new(None, Key::Shift);
                data.hotkeys.push(cancel_image_hotkey);
            }
           }
       else if cancel_image_modifier.eq(&Some(RawMods::Shift)){
            key.make_ascii_uppercase();
            let cancel_image_hotkey = HotKey::new(cancel_image_modifier, Key::Character(key));
            data.hotkeys.push(cancel_image_hotkey);

       }
       
       else if cancel_image_modifier.eq(&Some(RawMods::None))
        {
        let cancel_image_hotkey = HotKey::new(None, Key::Escape);
        data.hotkeys.push(cancel_image_hotkey);

        }
        else if cancel_image_modifier.eq(&Some(RawMods::Meta)) {
            let cancel_image_hotkey = HotKey::new(None, Key::Enter);
            data.hotkeys.push(cancel_image_hotkey);
        }
        else 
        {
        let cancel_image_hotkey = HotKey::new(cancel_image_modifier, Key::Character(key));
        data.hotkeys.push(cancel_image_hotkey);
        }
        //println!("key: {:?}", key);
        //let _save_image_hotkey = HotKey::new(save_image_modifier, key);
        //let key = Code::from_str(&data.quit_app_key).unwrap();
        
        let quit_app_modifier = match data.quit_app_modifier.as_str() {
            "Ctrl" => Some(RawMods::Ctrl),
            "Shift" => Some(RawMods::Shift),
            "Escape" => Some(RawMods::None),
            "Enter" => Some(RawMods::Meta),
            "Alt" => Some(RawMods::Alt),
            _ => None,
        };
        //let _quit_app_hotkey = HotKey::new(quit_app_modifier, key);
        let mut key=data.quit_app_key.clone();
        //println!("{:?}",data.save_image_key);
       if( quit_app_modifier.eq(&Some(RawMods::Ctrl)) && data.quit_app_key=="".to_string()) || (quit_app_modifier.eq(&Some(RawMods::Shift)) && data.quit_app_key=="".to_string()){
            if quit_app_modifier.eq(&Some(RawMods::Ctrl))
            {
                let quit_image_hotkey = HotKey::new(None, Key::Control);
                data.hotkeys.push(quit_image_hotkey);
            }
            else {
                let quit_image_hotkey = HotKey::new(None, Key::Shift);
                data.hotkeys.push(quit_image_hotkey);
            }
           }
       else if quit_app_modifier.eq(&Some(RawMods::Shift)){
            
            key.make_ascii_uppercase();
            let quit_image_hotkey = HotKey::new(quit_app_modifier, Key::Character(key));
            data.hotkeys.push(quit_image_hotkey);

       }
       
       else if quit_app_modifier.eq(&Some(RawMods::None))
        {
        let quit_image_hotkey = HotKey::new(None, Key::Escape);
        data.hotkeys.push(quit_image_hotkey);

        }
        else if quit_app_modifier.eq(&Some(RawMods::Meta)) {
            let quit_image_hotkey = HotKey::new(None, Key::Enter);
            data.hotkeys.push(quit_image_hotkey);
        }
        else 
        {
            
        let quit_image_hotkey = HotKey::new(quit_app_modifier, Key::Character(key));
        data.hotkeys.push(quit_image_hotkey);
        }
        
        let edit_image_modifier = match data.edit_image_modifier.as_str() {
            "Ctrl" => Some(RawMods::Ctrl),
            "Shift" => Some(RawMods::Shift),
            "Escape" => Some(RawMods::None),
            "Enter" => Some(RawMods::Meta),
            "Alt" => Some(RawMods::Alt),
            _ => None,
        };
        
        let mut key=data.edit_image_key.clone();
        
      if (edit_image_modifier.eq(&Some(RawMods::Ctrl)) && data.edit_image_key=="".to_string()) || (edit_image_modifier.eq(&Some(RawMods::Shift)) && data.edit_image_key=="".to_string()) {
            if edit_image_modifier.eq(&Some(RawMods::Ctrl)){
                let edit_image_hotkey = HotKey::new(None, Key::Control);
                data.hotkeys.push(edit_image_hotkey);
            }
            else {
                let edit_image_hotkey = HotKey::new(None, Key::Shift);
                data.hotkeys.push(edit_image_hotkey);
            }
           }
       else if edit_image_modifier.eq(&Some(RawMods::Shift)){
            key.make_ascii_uppercase();
            let edit_image_hotkey = HotKey::new(edit_image_modifier, Key::Character(key));
            data.hotkeys.push(edit_image_hotkey);

       }
      
       else if edit_image_modifier.eq(&Some(RawMods::Meta)) {
            let edit_image_hotkey = HotKey::new(None, Key::Enter);
            data.hotkeys.push(edit_image_hotkey);
        }
       else if edit_image_modifier.eq(&Some(RawMods::None))
        {
            let edit_image_hotkey = HotKey::new(None, Key::Escape);
            data.hotkeys.push(edit_image_hotkey);

        }
        else 
        {
            let edit_image_hotkey = HotKey::new(edit_image_modifier, Key::Character(key));
            data.hotkeys.push(edit_image_hotkey);
        }
        let restart_app_modifier = match data.restart_app_modifier.as_str() {
            "Ctrl" => Some(RawMods::Ctrl), 
            "Shift" => Some(RawMods::Shift),
            "Escape" => Some(RawMods::None),
            "Enter" => Some(RawMods::Meta),
            "Alt" => Some(RawMods::Alt),
            _ => None,
        };
        let mut key=data.restart_app_key.clone();
        //println!("{:?}",data.save_image_key);
       if (restart_app_modifier.eq(&Some(RawMods::Ctrl)) && data.restart_app_key=="".to_string()) || (restart_app_modifier.eq(&Some(RawMods::Shift)) && data.restart_app_key=="".to_string()){
            if restart_app_modifier.eq(&Some(RawMods::Ctrl))
            {
                let restart_app_hotkey = HotKey::new(None, Key::Control);
                data.hotkeys.push(restart_app_hotkey);
            }
            else {
                let restart_app_hotkey = HotKey::new(None, Key::Shift);
                data.hotkeys.push(restart_app_hotkey);
            }
           }
       else if restart_app_modifier.eq(&Some(RawMods::Shift)){
            key.make_ascii_uppercase();
            let restart_app_hotkey = HotKey::new(restart_app_modifier, Key::Character(key));
            data.hotkeys.push(restart_app_hotkey);

       }
       
       else if restart_app_modifier.eq(&Some(RawMods::None))
        {
        let restart_app_hotkey = HotKey::new(None, Key::Escape);
        data.hotkeys.push(restart_app_hotkey);

        }
        else if restart_app_modifier.eq(&Some(RawMods::Meta)) {
            let restart_app_hotkey = HotKey::new(None, Key::Enter);
            data.hotkeys.push(restart_app_hotkey);
        }
        else 
        {
            
        let restart_app_hotkey = HotKey::new(restart_app_modifier, Key::Character(key));
        data.hotkeys.push(restart_app_hotkey);
        }
        
        let restart_format_app_modifier = match data.restart_format_app_modifier.as_str() {
            "Ctrl" => Some(RawMods::Ctrl), 
            "Shift" => Some(RawMods::Shift),
            "Escape" => Some(RawMods::None),
            "Enter" => Some(RawMods::Meta),
            "Alt" => Some(RawMods::Alt),
            _ => None,
        };
        let mut key=data.restart_format_app_key.clone();
        //println!("{:?}",data.save_image_key);
       if (restart_format_app_modifier.eq(&Some(RawMods::Ctrl)) && data.restart_format_app_key=="".to_string()) || (restart_format_app_modifier.eq(&Some(RawMods::Shift)) && data.restart_format_app_key=="".to_string()){
            if restart_format_app_modifier.eq(&Some(RawMods::Ctrl))
            {
                let restart_format_app_hotkey = HotKey::new(None, Key::Control);
                data.hotkeys.push(restart_format_app_hotkey);
            }
            else {
                let restart_format_app_hotkey = HotKey::new(None, Key::Shift);
                data.hotkeys.push(restart_format_app_hotkey);
            }
           }
       else if restart_format_app_modifier.eq(&Some(RawMods::Shift)){
            key.make_ascii_uppercase();
            let restart_format_app_hotkey = HotKey::new(restart_format_app_modifier, Key::Character(key));
            data.hotkeys.push(restart_format_app_hotkey);

       }
       
       else if restart_format_app_modifier.eq(&Some(RawMods::None))
        {
        let restart_format_app_hotkey = HotKey::new(None, Key::Escape);
        data.hotkeys.push(restart_format_app_hotkey);

        }
        else if restart_format_app_modifier.eq(&Some(RawMods::Meta)) {
            let restart_format_app_hotkey = HotKey::new(None, Key::Enter);
            data.hotkeys.push(restart_format_app_hotkey);
        }
        else 
        {
            
        let restart_format_app_hotkey = HotKey::new(restart_format_app_modifier, Key::Character(key));
        data.hotkeys.push(restart_format_app_hotkey);
        }
        


        let format_window= WindowDesc::new(window_format::build_ui()).transparent(false)
                        .title("Choose the format. Default is .png").window_size(Size::new(200.0, 200.0))
                        .set_always_on_top(true);
                        
                    
       if are_all_fields_completed(data) && !some_fields_are_equal(data)
            {
                data.format_window_id= format_window.id;
                data.shortkeys_window_id= ctx.window_id();
                let id= format_window.id;
            
                ctx.new_window(format_window);
                ctx.submit_command(druid::commands::HIDE_WINDOW.to(ctx.window_id()));
                ctx.submit_command(druid::commands::SHOW_WINDOW.to(id));
                
            }
        


    });
    let errore = Label::new(|data: &drawing_area::AppData, _env: &Env| {
        if are_all_fields_completed(data) {
            "".to_string()
        } else {
            "Per favore, compila tutti i campi.".to_string()
        }
        
    });
    let errore_field = Label::new(|data: &drawing_area::AppData,_env: &Env| {
        if some_fields_are_equal(data) {
            
            "Stesse shortkeys non sono ammesse".to_string()
        }
        else  {
            "".to_string()
        }
    });
    
    Flex::column()
        .with_child(errore)
        .with_child(errore_field)
        .with_child(save_image)
        .with_child(quit_app)
        .with_child(edit_image)
        .with_child(cancel_image)
        .with_child(restart)
        .with_child(choose_format)
        .with_child(apply_button)
        .with_child(AppDataHandler)
        
}

fn are_all_fields_completed(data: &drawing_area::AppData) -> bool {
    
    if (data.save_image_modifier!="None".to_string() || data.save_image_key!="".to_string()) && (data.edit_image_modifier!="None".to_string() || data.edit_image_key!="".to_string()) 
    && (data.quit_app_key!="".to_string() || data.quit_app_modifier!="None".to_string()) && (data.cancel_image_key!="".to_string() || data.cancel_image_modifier!="None".to_string()) 
    && (data.restart_app_modifier!="None".to_string() || data.restart_app_key!="".to_string()) && (data.restart_format_app_modifier!="None".to_string() || data.restart_format_app_key!="".to_string()) 
    {
        
        true
    }
    else 
    {
            false
    }
}

fn some_fields_are_equal(data: &drawing_area::AppData) -> bool {
    if (data.cancel_image_modifier == data.save_image_modifier && data.cancel_image_key == data.save_image_key ) 
    || (data.cancel_image_modifier == data.restart_app_modifier && data.cancel_image_key == data.restart_app_key ) ||
    (data.cancel_image_modifier == data.restart_format_app_modifier && data.cancel_image_key == data.restart_format_app_key ) ||
    (data.cancel_image_modifier == data.edit_image_modifier && data.cancel_image_key == data.edit_image_key) || 
    (data.cancel_image_modifier == data.quit_app_modifier && data.cancel_image_key == data.quit_app_modifier ) ||
    (data.save_image_modifier == data.quit_app_modifier && data.save_image_key == data.quit_app_modifier ) ||
    (data.save_image_modifier == data.restart_format_app_modifier && data.save_image_key == data.restart_format_app_key ) ||
    (data.save_image_modifier == data.restart_app_modifier && data.save_image_key == data.restart_app_key ) ||
    (data.save_image_modifier == data.edit_image_modifier && data.save_image_key == data.edit_image_key ) ||
    (data.quit_app_modifier == data.edit_image_modifier && data.quit_app_key == data.edit_image_key ) || 
    (data.quit_app_modifier == data.restart_app_modifier && data.quit_app_key == data.restart_app_key ) ||
    (data.quit_app_modifier == data.restart_format_app_modifier && data.quit_app_key == data.restart_format_app_key ) ||
    (data.edit_image_modifier == data.restart_app_modifier && data.edit_image_key == data.restart_app_key ) ||
    (data.restart_app_modifier == data.restart_format_app_modifier && data.restart_app_key == data.restart_format_app_key ) ||
    (data.edit_image_modifier == data.restart_format_app_modifier && data.edit_image_key == data.restart_format_app_key )
    {
        true
    }
    else {
        false
    }
}

