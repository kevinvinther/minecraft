mod noise;
use image::ImageFormat;
use ::noise::{
    Perlin, Seedable,
};

use self::noise::noise_map::{self, NoiseMap};

const DEFAULT_SEED: u32 = 0x5EED;
const SCALE: usize = 100;

pub fn save_img() {
    let perlin = Perlin::new();
    perlin.set_seed(DEFAULT_SEED);

    let n_map = NoiseMap::from_noisefn(
        1024,
        1024,
        SCALE,
        perlin,
    );

    let _ = n_map.save_as_img(
        "perlin1024x1024.png",
    );
}
