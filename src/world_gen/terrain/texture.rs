/// This is a temp file, just to see if this works

use image::{
    Rgb, ImageBuffer, 
};
use crate::world_gen::{noise::noise_map::NoiseMap, terrain::terrain_type::TerrainType};

use super::height_map::HeightMap;

/// Not very happy with this, but it'll do for now.
/// 
/// Saves an image of the given HeightMap to
/// `demos/terrain_demo/` + `filename`
pub fn texture_from_noise_map(
    height_map: HeightMap,
    filename: &str,
) {
    let img 
        = ImageBuffer::from_fn(
            height_map.get_width() as u32,
            height_map.get_height() as u32,
            |x, y| {    // Describes the value of every pixel
                let terrain = 
                    TerrainType::ident(
                        &height_map.get(x as usize, y as usize)
                    );
                terrain.colour()
        });
    let path = String::from("demos/terrain_demo/") + filename;
    println!("\nSaving image to path:\n\t{}\n\t...", path);

    // Save the ImageBuffer
    let res = img.save(path);

    // If any error occured while saving the image, we print error
    match res {
        Ok(_) => println!("\tno errors saving image, we good 🚜\n"),
        Err(e) => println!("Oh no\n{}", e),
    }
}
