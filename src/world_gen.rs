use self::noise::perlin::perlin_noise;

mod terrain;
mod noise;

const DEFAULT_SEED: u32 = 0x5EED;
const SCALE: f64 = 100.0;

pub fn gen_noise_img() {
    terrain::generate_noisemap(256, 256, DEFAULT_SEED)
}

pub fn print_perms() {
    let table = noise::perlin::PermutationTable::new(DEFAULT_SEED);
    println!("{:?}", table);
}

pub fn print_perlin() {
    let table = noise::perlin::PermutationTable::new(DEFAULT_SEED);
    let noise_res = perlin_noise(1.0 / SCALE, 1.0 / SCALE, table);
    println!("{}", noise_res);
}
