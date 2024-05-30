#[macro_use] extern crate rocket;

use rocket::get;
// use rocket::http::RawStr;
// use rocket::request;
use rocket::form::{self, FromForm, Error};
// use std::default::Default;
use rand::distributions::{Distribution, Uniform};
use image::{RgbImage, Rgb};
use std::path::Path;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;



#[derive(Debug, FromForm)]
struct Pallet {
    #[field(default = 5)]
    pallet_count: u32,
    #[field(default = 128)]
    pallet_size: u32,
    #[field(default = 255)]
    #[field(validate = validate_brightness(&self.min_brightness))]
    max_brightness: u8,
    #[field(default = 0)]
    min_brightness: u8,
}

fn validate_brightness<'v>(max_brightness: &u8, min_brightness: &u8) -> form::Result<'v, ()> {
    if max_brightness < min_brightness  {
        Err(Error::validation("Invalid brightness levels. Max must be higher than min"))?;
    }
    Ok(())
}


fn create_pallet(pallet_count: &u32, pallet_size: &u32, max_brightness: &u8, min_brightness: &u8, path: &Path) {
    let height = pallet_size;

    let mut img = RgbImage::new(pallet_count * *height, *height);

    let mut rng = rand::thread_rng();
    let die = Uniform::from(*min_brightness..=*max_brightness);

    for pallet_number in 0..*pallet_count{
        let red = die.sample(&mut rng);
        let green = die.sample(&mut rng);
        let blue = die.sample(&mut rng);
        for x in 0..*pallet_size{
            for y in 0..*pallet_size{
                img.put_pixel(x+(pallet_number*pallet_size), y, Rgb([red, blue, green]));
            }   
        }        
    }
    img.save(path).unwrap();
}

#[get("/palette?<pallet..>")]
async fn palette(pallet: Pallet) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("./palette.png");
    create_pallet(&pallet.pallet_count, &pallet.pallet_size, &pallet.max_brightness, &pallet.min_brightness, path);
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

#[get("/palette_chooser")]
async fn palette_chooser() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("palette_chooser.html").await.map_err(|e| NotFound(e.to_string()))
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![palette, palette_chooser])
}