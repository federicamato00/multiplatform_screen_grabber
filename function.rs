use crate::drawing_area;



pub(crate) fn some_fields_are_equal(data: &drawing_area::AppData) -> bool {
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

pub(crate) fn are_all_fields_completed(data: &drawing_area::AppData) -> bool {
    
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
