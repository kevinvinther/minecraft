mod noise;
mod terrain;

use ::noise::{
    Perlin, Seedable,
};

use self::noise::noise_map::NoiseMap;
use self::terrain::height_map::HeightMap;
use self::terrain::texture::texture_from_noise_map;
use noise_consts::*;

/// Constants relevant to generating noise
mod noise_consts{
    pub const DEFAULT_SEED: u32 = 0x5EED;
    
    pub const SCALE: usize = 100;
    pub const OCTAVES: usize = 1;
    pub const LACUNARITY: f64 = 2.0;
    pub const PERSISTANCE: f64 = 0.5;
}

/// Creates 4 images representing a NoiseMap
pub fn noisemap_demo(
    scale: usize,
    octaves: usize,
    lacunarity: f64,
    persistance: f64,
    version: &str,
) {
    let perlin = Perlin::new();
    perlin.set_seed(DEFAULT_SEED);

    let sizes = [(256, 256), (512, 1024), (1024, 512), (1024, 1024)];
    for (height, width) in sizes {
        println!("Imaging noisemap{}x{}", height, width);
        let n_map = NoiseMap::from_noisefn(
            height,
            width,
            scale,
            octaves,
            lacunarity,
            persistance,
            perlin,
            DEFAULT_SEED,
        );

        let lac_fmt = lacunarity.to_string().replace(".", "_");
        let per_fmt = persistance.to_string().replace(".", "_");
        
        let filename = 
            format!("{}/perlin{}x{}-{}-{}-{}.png",
            version,
            height,
            width,
            octaves,
            lac_fmt,
            per_fmt
        );

        let _ = n_map.save_as_img(&filename);
        println!();
    }
}

/// Saves a demo of the current terrain generation
pub fn texture_demo(
    height: usize,
    width: usize,
    scale: usize,
    octaves: usize,
    lacunarity: f64,
    persistance: f64,
    version: &str,
) -> () {
    // Create a perlin noise generator and set its seed
    let perlin = Perlin::new();
    perlin.set_seed(DEFAULT_SEED);

    // Create a NoiseMap
    let n_map = NoiseMap::from_noisefn(
        height,
        width,
        scale,
        octaves,
        lacunarity,
        persistance,
        perlin,
        DEFAULT_SEED
    );

    // Used when naming the image file
    let lac_fmt = lacunarity.to_string().replace(".", "_");
    let per_fmt = persistance.to_string().replace(".", "_");
    
    // The name for the image file
    let filename = 
        format!("{}/perlin{}x{}-{}-{}-{}.png",
        version,
        height,
        width,
        octaves,
        lac_fmt,
        per_fmt
    );

    // Noise values are mapped from [0.0; 1.0] to [0; 100]
    let height_mapper = | val: f64 | -> i32 {
        (val * 100.0).round() as i32
    };

    // Create HeightMap
    let h_map = HeightMap::from_noise_map(&n_map, height_mapper);
    
    texture_from_noise_map(h_map, &filename);
}
