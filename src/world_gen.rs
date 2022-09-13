mod terrain;

const DEFAULT_SEED: u32 = 0;

pub fn gen_noise_img() {
    terrain::generate_noisemap(256, 256, DEFAULT_SEED)
}
