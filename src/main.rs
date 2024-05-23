use rand::distributions::{Distribution, Uniform};
use image::{RgbImage, Rgb};
use std::path::Path;

fn main() {
    let pallet_count:u32 = 5;
    let pallet_size:u32 = 128;

    let height = &pallet_size;

    let mut img = RgbImage::new(pallet_count * *height, *height);

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..=225);

    for pallet_number in 0..pallet_count{
        let red = die.sample(&mut rng);
        let green = die.sample(&mut rng);
        let blue = die.sample(&mut rng);
        for x in 0..pallet_size{
            for y in 0..pallet_size{
                img.put_pixel(x+(pallet_number*pallet_size), y, Rgb([red, blue, green]));
            }   
        }        
    }
    let path = Path::new("./test.png");
    img.save(path).unwrap();

}