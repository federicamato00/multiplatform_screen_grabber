use image::{DynamicImage, ImageBuffer, Rgba};
use scrap::Capturer;
use screenshots::{self, Screen};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::MyRadio;

pub fn screen(
    format: MyRadio,
    mut capturer: Capturer,
    width: u32,
    height: u32,
    start_position: Arc<Mutex<Option<(f64, f64)>>>,
    end_position: Arc<Mutex<Option<(f64, f64)>>>,
    name: String,
) {
    let new_format = format;
    let name_capture = name;

    // altri sistemi operativi è l'unico che non da problemi di distorsione, controllare se su windows funziona ugualmente e in tal caso sostituire
    if let (Some((x1, y1)), Some((x2, y2))) = (
        *start_position.lock().unwrap(),
        *end_position.lock().unwrap(),
    ) {
        let form = match new_format {
            MyRadio::Png => "png",
            MyRadio::Jpeg => "jpeg",
            MyRadio::Bmp => "bmp",
        };
        // let screens = Screen::all().unwrap();

        // for screen in screens {
        //     let image = screen.capture().unwrap();
        //     image
        //         .save("original.".to_owned()+form)
        //         .unwrap();

        // }

        let screen = Screen::from_point(0, 0).unwrap();
        // let original=screen.capture().unwrap();
        // original.save("original.".to_owned()+form.clone()).unwrap();
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = screen
            .capture_area(
                x1 as i32 + 1,
                y1 as i32 + 1,
                ((x2 - x1) - 1.5) as u32,
                ((y2 - y1) - 1.5) as u32,
            )
            .unwrap();
        image
            .save(name_capture.to_owned() + "." + form.clone())
            .unwrap();
    }
}

// // Versione che fa post processing in un thread apparte. Al momento non funzionante
// pub fn screen_thread( _is_dragging: Arc<Mutex<Option<bool>>>,mut capturer:Capturer,width: u32,height: u32,start_position:Arc<Mutex<Option<(f64, f64)>>>,end_position:Arc<Mutex<Option<(f64, f64)>>>)
// {

//     // Aspetta finché l'utente non ha finito di tracciare l'area dello schermo.

//     let frame: Option<Vec<u8>> = loop {
//         match capturer.frame() {

//             /* DOPO L'AGGIUNTA DELLO STRIDE GLI SCREEN SONO SPOSTATI LEGGERMENTE IN ALTO, VEDI DI AGGIUSTARE*/
//             Ok(frame) => {
//                 // println!("Demtro frame");
//                 // Calcola lo stride dell'immagine
//                 let stride = frame.len() / height as usize;

//                 // Crea una nuova immagine con le dimensioni corrette
//                 let mut image = vec![0; stride * height as usize];

//                 // Copia i dati dell'immagine, tenendo conto dello stride
//                 for y in 0..height {
//                     let src_off = (y * stride as u32) as usize;
//                     let dst_off = (y * width * 4) as usize;
//                     let src_slice = &frame[src_off..src_off + width as usize * 4];
//                     let dst_slice = &mut image[dst_off..dst_off + width as usize * 4];
//                     dst_slice.copy_from_slice(src_slice);
//                 }
//                     // Ritorna l'immagine modificata
//                     break Some(image);
//                 }
//                     ,
//             Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
//                 // Aspetta un po' per il prossimo frame.

//                 thread::sleep(Duration::from_millis(100));
//                 break None;
//             }
//             Err(e) => panic!("frame error: {}", e),
//         }
//     };

//     if frame.is_none(){
//         return;
//     }

//     let frame = frame.unwrap();

//     thread::spawn(move ||{
//                     // Converte il frame in un'immagine.
//                 let mut image: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
//                     width as u32,
//                     height as u32,
//                     frame.to_vec(),
//                 ).unwrap();

//                 for pixel in image.pixels_mut() {
//                     let b = pixel[0];
//                     let g = pixel[1];
//                     let r = pixel[2];
//                     let a = pixel[3];
//                     *pixel = Rgba([r, g, b, a]);
//                 }

//             // println!("height: {:?}",height);
//             // // Converte il frame in un'immagine.
//             // let mut image: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
//             //     width as u32,
//             //     height as u32,
//             //     frame.to_vec(),
//             // ).unwrap();

//             // for pixel in image.pixels_mut() {
//             //     let b = pixel[0];
//             //     let g = pixel[1];
//             //     let r = pixel[2];
//             //     let a = pixel[3];
//             //     *pixel = Rgba([r, g, b, a]);
//             // }

//             let im=DynamicImage::from(image.clone());
//             let _=DynamicImage::from(im).save("original.png");
//             //println!("Image size: {:?}, {:?}", im.width(), im.height());
//             // Ritaglia l'immagine all'area specificata.
//             if let (Some((x1, y1)), Some((x2, y2))) = (*start_position.lock().unwrap(), *end_position.lock().unwrap()) {

//                 let sub_image = (DynamicImage::from(image)).crop((x1+1.) as u32, (y1+1.) as u32, (x2-x1-1.5) as u32, (y2-y1-1.5) as u32);
//                 //println!("{:?}, {:?} x1: {:?}, y1: {:?}", (x2-x1), (y2-y1),x1,y1);
//                 match sub_image.save("screenshot_grabbed.png") {
//                     Ok(_) => {println!("Successo!");return},
//                     Err(e) if e.to_string().contains("Zero width not allowed") => {println!("Errore");return},
//                     Err(_) => panic!("Unexpected error!"),
//                 }

//             }
//     });

// }

// // Versione senza mutex, potrebbe essere rischiosa da usare
// pub fn screen_no_mutex( _is_dragging:Option<bool>,mut capturer:Capturer,width: u32,height: u32,start_position:Option<(f64, f64)>,end_position:Option<(f64, f64)>)
// {

//  loop{

//     // Aspetta finché l'utente non ha finito di tracciare l'area dello schermo.

//     let frame: Option<Vec<u8>> = loop {
//         match capturer.frame() {

//             /* DOPO L'AGGIUNTA DELLO STRIDE GLI SCREEN SONO SPOSTATI LEGGERMENTE IN ALTO, VEDI DI AGGIUSTARE*/
//             Ok(frame) => {
//                 // println!("Demtro frame");
//                 // Calcola lo stride dell'immagine
//                 let stride = frame.len() / height as usize;

//                 // Crea una nuova immagine con le dimensioni corrette
//                 let mut image = vec![0; stride * height as usize];

//                 // Copia i dati dell'immagine, tenendo conto dello stride
//                 for y in 0..height {
//                     let src_off = (y * stride as u32) as usize;
//                     let dst_off = (y * width * 4) as usize;
//                     let src_slice = &frame[src_off..src_off + width as usize * 4];
//                     let dst_slice = &mut image[dst_off..dst_off + width as usize * 4];
//                     dst_slice.copy_from_slice(src_slice);
//                 }
//                     // Ritorna l'immagine modificata
//                     break Some(image);
//                 }
//                     ,
//             Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
//                 // Aspetta un po' per il prossimo frame.

//                 thread::sleep(Duration::from_millis(100));
//                 break None;
//             }
//             Err(e) => panic!("frame error: {}", e),
//         }
//     };

//     if frame.is_none(){
//         continue;
//     }

//     let frame = frame.unwrap();

//      // Converte il frame in un'immagine.
//      let mut image: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
//          width as u32,
//          height as u32,
//          frame.to_vec(),
//      ).unwrap();

//      for pixel in image.pixels_mut() {
//          let b = pixel[0];
//          let g = pixel[1];
//          let r = pixel[2];
//          let a = pixel[3];
//          *pixel = Rgba([r, g, b, a]);
//      }

//     let im=DynamicImage::from(image.clone());
//     let _=DynamicImage::from(im).save("original.png");
//     //println!("Image size: {:?}, {:?}", im.width(), im.height());
//     // Ritaglia l'immagine all'area specificata.
//     if let (Some((x1, y1)), Some((x2, y2))) = (start_position, end_position) {
//         let sub_image = (DynamicImage::from(image)).crop((x1) as u32, (y1) as u32, (x2-x1) as u32, (y2-y1) as u32);
//         //println!("{:?}, {:?} x1: {:?}, y1: {:?}", (x2-x1), (y2-y1),x1,y1);
//         match sub_image.save("screenshot_grabbed.png") {
//             Ok(_) => break,
//             Err(e) if e.to_string().contains("Zero width not allowed") => continue,
//             Err(_) => panic!("Unexpected error!"),
//         }

//     }
// }
// }
