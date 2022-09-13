use noise::{Perlin, Seedable, NoiseFn, utils::NoiseMap};

const SCALE: f64 = 100.0;

// Should also require scale as input. Do later :)
pub fn generate_noisemap(width: usize, height: usize, seed: u32) {
    let perlin = Perlin::new();
    perlin.set_seed(seed);

    let mut noise_map = NoiseMap::new(width, height);   // For generating image example of noisemap

    for x in 0..width {
        let scaled_x = (x as f64) / SCALE;
        for y in 0..height {
            let scaled_y = (y as f64) / SCALE;
            noise_map.set_value(x, y, perlin.get([scaled_x, scaled_y]));
        }
    }

    noise_map.write_to_file("noisemap.png");
}
