mod noise;
use image::ImageFormat;
use ::noise::{
    Perlin, Seedable,
};
use rand_seeder::{Seeder, SipHasher};   // Seeder is not cryptographically safe, but that does not matter for us
use rand_pcg::Pcg64;

use self::noise::noise_map::{self, NoiseMap};

const DEFAULT_SEED: u32 = 0x5EED;

const SCALE: usize = 100;
const OCTAVES: usize = 1;
const LACUNARITY: f64 = 2.0;
const PERSISTANCE: f64 = 0.5;

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
            format!("/{}/perlin{}x{}-{}-{}-{}.png",
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
