use druid::widget::{Flex, Label, RadioGroup, TextBox, Button};
use druid::{Widget, WidgetExt, WindowDesc, Size, EventCtx, Event, Env, UpdateCtx, LifeCycleCtx, LifeCycle, BoxConstraints, LayoutCtx, PaintCtx};
use druid_shell::keyboard_types::Key;
use druid_shell::{RawMods, HotKey};

use crate::drawing_area;


pub struct AppDataHandler;
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
                }}
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
        .with_child(RadioGroup::row(vec![("Ctrl","Ctrl".to_string()), ("Shift","Shift".to_string()),("Escape","Escape".to_string()),("Enter","Enter".to_string()),("None","".to_string())]).lens(drawing_area::AppData::save_image_modifier))
        .with_child(Label::new("Save Image Key: "))
        .with_child(TextBox::new().lens(drawing_area::AppData::save_image_key));


    let quit_app = Flex::row()
        .with_child(Label::new("Quit modifier: "))
        .with_child(RadioGroup::row(vec![("Ctrl","Ctrl".to_string()), ("Shift","Shift".to_string()),("Escape","Escape".to_string()),("Enter","Enter".to_string()),("None","".to_string())]).lens(drawing_area::AppData::quit_app_modifier))
        .with_child(Label::new("Quit App Key: "))
        .with_child(TextBox::new().lens(drawing_area::AppData::quit_app_key));

    let edit_image = Flex::row()
        .with_child(Label::new("Edit modifier: "))
        .with_child(RadioGroup::row(vec![("Ctrl","Ctrl".to_string()), ("Shift","Shift".to_string()),("Escape","Escape".to_string()),("Enter","Enter".to_string()),("None","".to_string())]).lens(drawing_area::AppData::edit_image_modifier))
        .with_child(Label::new("Edit Image Key: "))
        .with_child(TextBox::new().lens(drawing_area::AppData::edit_image_key));
    let cancel_image = Flex::row()
        .with_child(Label::new("Cancel modifier: "))
        .with_child(RadioGroup::row(vec![("Ctrl","Ctrl".to_string()), ("Shift","Shift".to_string()),("Escape","Escape".to_string()),("Enter","Enter".to_string()),("None","".to_string())]).lens(drawing_area::AppData::cancel_image_modifier))
        .with_child(Label::new("Cancel Image Key: "))
        .with_child(TextBox::new().lens(drawing_area::AppData::cancel_image_key));

    let apply_button = Button::new("Apply").on_click(|ctx, data: &mut drawing_area::AppData, _env| {
        // Qui puoi definire le tue HotKey basate sui valori in data
        
        let save_image_modifier = match data.save_image_modifier.as_str() {
            "Ctrl" => Some(RawMods::Ctrl), 
            "Shift" => Some(RawMods::Shift),
            "Escape" => Some(RawMods::None),
            "Enter" => Some(RawMods::Meta),
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
        
        
        let format_window= WindowDesc::new(drawing_area::build_ui(true)).transparent(false)
                        .title("Choose the format. Default is .png").window_size(Size::new(200.0, 200.0))
                        .set_always_on_top(true);
                        
                    
       if are_all_fields_completed(data) 
            {
                let id= format_window.id;
            
                ctx.new_window(format_window);
                ctx.submit_command(druid::commands::SHOW_WINDOW.to(id));
                ctx.submit_command(druid::commands::HIDE_WINDOW.to(ctx.window_id()));
            }
        


    });
    let errore = Label::new(|data: &drawing_area::AppData, _env: &Env| {
        if are_all_fields_completed(data) {
            "".to_string()
        } else {
            "Per favore, compila tutti i campi.".to_string()
        }
    });
    Flex::column()
        .with_child(errore)
        .with_child(save_image)
        .with_child(quit_app)
        .with_child(edit_image)
        .with_child(cancel_image)
        .with_child(apply_button)
        .with_child(AppDataHandler)
        
}

fn are_all_fields_completed(data: &drawing_area::AppData) -> bool {
    
    if (data.save_image_modifier!="None".to_string() || data.save_image_key!="".to_string()) && (data.edit_image_modifier!="None".to_string() || data.edit_image_key!="".to_string()) 
    && (data.quit_app_key!="".to_string() || data.quit_app_modifier!="None".to_string()) && (data.cancel_image_key!="".to_string() || data.cancel_image_modifier!="None".to_string()) 
    {
        
        true
    }
    else 
    {
            false
    }
}
