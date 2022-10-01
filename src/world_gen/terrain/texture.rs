/// This is a temp file, just to see if this works

use image::{
    Rgb, ImageBuffer, 
};
use crate::world_gen::{noise::noise_map::NoiseMap, terrain::terrain_type::TerrainType};

pub fn texture_from_noise_map(
    noise_map: NoiseMap,
    filename: &str,
) {
    let img 
        = ImageBuffer::from_fn(
            noise_map.get_width() as u32,
            noise_map.get_height() as u32,
            |x, y| {
                let terrain = 
                    TerrainType::from_height(
                        noise_map.get_value(x as usize, y as usize)
                    );
                terrain.colour()
        });
    let path = String::from("demos/terrain_demo/") + filename;
    println!("\nSaving image to path:\n\t{}\n\t...", path);

    let res = img.save(path);

    match res {
        Ok(_) => println!("\tno errors saving image, we good 🚜\n"),
        Err(e) => println!("Oh no\n{}", e),
    }
}
