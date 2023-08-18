use std::io::{SeekFrom, Cursor, Seek, Read};

use image::{ImageFormat, DynamicImage};
use base64::{engine::general_purpose, Engine as _};

fn buffer_to_image(buf: &[u8], _format: ImageFormat) -> DynamicImage {
    println!("@@@@");
    match image::load_from_memory(buf) {
       Ok(img) => {
        println!("convert succeed!");
        img
       },
       Err(error) => {
        panic!("can not convert the picture {:?}", error)
       } 
    }
}

fn image_to_base64 (img: DynamicImage, format: ImageFormat) -> String {
    println!("!!!!! hi");
    let mut c = Cursor::new(Vec::new());
    match img.write_to(&mut c, format) {
        Ok(c) => c,
        Err(error) => {
            panic!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            )
        }
    };

    c.seek(SeekFrom::Start(0)).unwrap();
    let mut out =  Vec::new();

    c.read_to_end(&mut out).unwrap();

    let stt = general_purpose::STANDARD.encode(&mut out);

    format!("data:{};base64,{}", format.to_mime_type(), stt)
}

fn main() {
    let from_suffix = "png";
    let to_suffix = "jpeg";

    let img = match image::open("test/roe_deer.png") {
       Ok(file) => {
            println!("load succeed!");
            file
       },
       Err(error) => {
            panic!("load failed! {:?}", error)
       }
    };

    let origin_format = match ImageFormat::from_extension(from_suffix) {
        Some(format) => format,
        None => {
            panic!("error suffix!")
        }
    };
    // println!("{:?}", img.as_bytes());
    let dyn_image = buffer_to_image(img.as_bytes(), origin_format);

    let target_format = match ImageFormat::from_extension(to_suffix) {
        Some(format) => format,
        None => {
            panic!("error suffix!")
        }
    };

    let base64_str = image_to_base64(dyn_image, target_format);

    println!("{}", base64_str);
    // println!("dimensions {:?}", img.dimensions());
    
    
    // println!("{:?}", img.color());

    // img.save("test/test.jpeg").unwrap();
}
