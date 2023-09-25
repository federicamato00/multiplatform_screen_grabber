use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use image::{ImageBuffer, Rgba,DynamicImage};
use scrap::{Capturer};




pub fn screen( _is_dragging: Arc<Mutex<Option<bool>>>,mut capturer:Capturer,width: u32,height: u32,start_position:Arc<Mutex<Option<(f64, f64)>>>,end_position:Arc<Mutex<Option<(f64, f64)>>>)
{
    
 loop{
    
    // Aspetta finché l'utente non ha finito di tracciare l'area dello schermo.
    
    let frame = loop {
        match capturer.frame() {
            
            Ok(frame) => break frame,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Aspetta un po' per il prossimo frame.
             
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => panic!("frame error: {}", e),
        }
    };

    // Converte il frame in un'immagine.
    let mut image: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
        width as u32,
        height as u32,
        frame.to_vec(),
    ).unwrap();
    
    for pixel in image.pixels_mut() {
        let b = pixel[0];
        let g = pixel[1];
        let r = pixel[2];
        let a = pixel[3];
        *pixel = Rgba([r, g, b, a]);
    }  
    
    let im=DynamicImage::from(image.clone());
    let _=DynamicImage::from(im).save("original.png");
    //println!("Image size: {:?}, {:?}", im.width(), im.height());
    // Ritaglia l'immagine all'area specificata.
    if let (Some((x1, y1)), Some((x2, y2))) = (*start_position.lock().unwrap(), *end_position.lock().unwrap()) {
        if (x2-x1) as u32 <= 0 || (y2-y1) as u32 <= 0{
            continue;
        }
        let sub_image = (DynamicImage::from(image)).crop(x1 as u32, y1 as u32, (x2-x1) as u32, (y2-y1) as u32);
        println!("{:?}, {:?} x1: {:?}, y1: {:?}", (x2-x1), (y2-y1),x1,y1);
        match sub_image.save("screenshot_grabbed.png") {
            Ok(_) => break,
            Err(e) if e.to_string().contains("Zero width not allowed") => continue,
            Err(_) => panic!("Unexpected error!"),
        }
        
    }
}
}
