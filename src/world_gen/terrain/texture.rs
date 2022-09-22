/// This is a temp file, just to see if this works

use std::ops::{
    Range, RangeFrom, RangeTo,
    RangeFull, RangeInclusive, RangeToInclusive,
};
use std::path::Path;
use image::{
    Rgb, ImageBuffer, 
};
use crate::world_gen::noise::noise_map::NoiseMap;

pub fn texture_from_noise_map(
    noise_map: NoiseMap,
    filename: &str,
) {
    let img 
        = ImageBuffer::from_fn(
            noise_map.get_width() as u32,
            noise_map.get_height() as u32,
            |x, y| {
                match noise_map.get_value(x as usize, y as usize) {
                    c if c <= 0.4   => Rgb::<u8>([ 51, 134, 255]),
                    c if c > 0.4    => Rgb::<u8>([ 30, 176,  23]),
                    _ => {
                        println!("Invalid value!");
                        Rgb::<u8>([255, 0, 255])
                    }
                }
        });
    let path = String::from("demos/terrain_demo/") + filename;
    println!("{}", path);

    let res = img.save(path);

    match res {
        Ok(_) => println!("We good"),
        Err(e) => println!("Oh no\n{}", e),
    }
}
