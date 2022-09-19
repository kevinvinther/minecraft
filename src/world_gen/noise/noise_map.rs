use image::{ImageFormat, ImageResult};
use noise::NoiseFn;
use rand::prelude::*;
use rand_seeder::{Seeder, SipHasher};   // Seeder is not cryptographically safe, but that does not matter for us
use rand_pcg::Pcg64;

pub const DEFAULT_SCALE: usize = 100;
pub const DEFAULT_OCTAVES: usize = 1;
pub const DEFAULT_LACUNARITY: f64 = 1.0;

/// A struct holding noise data for a section of the given noise function
/// TODO:
///     Height and width should be replaced with two points
///     Find an apropriate default scale (this can't be done yet)
pub struct NoiseMap {
    height: usize,
    width: usize,
    values: Vec<f64>,
    
// Noise modyfiers:
    scale: usize,
    octaves: usize,     // A NoiseMap contains noise built from several noise maps aka. octaves
    lacunarity: f64,    // Lacunarity defines the frequency of octaves
    persistance: f64,   // Persistance defines the amplitude of octaves
}

impl NoiseMap {
    /// Creates a new and empty NoiseMap
    /// 
    /// The [`fill`](`Self::fill`) method should be used to fill the map with values
    /// ## Note
    /// Note that all values, except for `height` and `width`, should be set with 
    /// their assosiated methods, before any further use of NoiseMaps created from this method
    pub fn new(height: usize, width: usize) -> Self {
        NoiseMap {
            height,
            width,
            values: Vec::with_capacity(height * width),
            ..Default::default()
        }
    }

    /// Creates and fills a NoiseMap with values from the given noise function.
    /// 
    /// # Panics
    /// Panics if `lacunarity` or `persistance` are parsed as 
    ///     [`NAN`](`std::primitive::f64::NAN`), 
    ///     [`INFINITY`](`std::primitive::f64::INFINITY`), 
    ///     or 
    ///     [`NEG_INFINITY`](`std::primitive::f64::NEG_INFINITY`).
    pub fn from_noisefn(
        height: usize,
        width: usize,
        scale: usize,
        octaves: usize,
        lacunarity: f64,
        persistance: f64,
        noise_fn: impl NoiseFn<[f64; 2]>,
        seed: u32,
    ) -> Self {
        let mut map = NoiseMap::new(
            height,
            width,
        );

        map.set_scale(scale);
        map.set_octaves(octaves);
        map.set_lacunarity(lacunarity);
        map.set_persistance(persistance);
        
        map.fill(noise_fn, seed);
        map
    }

    /// Sets the scale of the NoiseMap.
    /// 
    /// # Note
    /// Note that the scale cannot be 0. If 0 is parsed into this method, 
    /// the scale will be set to [DEFAULT_SCALE]
    pub fn set_scale(&mut self, scale: usize) {
        match scale {
            0 => self.scale = DEFAULT_SCALE,
            _ => self.scale = scale,
        }
    }

    /// Sets the octaves of the NoiseMap.
    /// 
    /// # Note
    /// Note that the octaves cannot be 0. If 0 is parsed into this method, 
    /// the octaves will be set to [DEFAULT_OCTAVES]
    pub fn set_octaves(&mut self, octaves: usize) {
        match octaves {
            0 => self.octaves = DEFAULT_OCTAVES,
            _ => self.octaves = octaves,
        }
    }

    /// Sets the lacunarity of the NoiseMap
    /// 
    /// # Panics
    /// Panics if the parsed lacunarity is 
    ///     [`NAN`](`std::primitive::f64::NAN`), 
    ///     [`INFINITY`](`std::primitive::f64::INFINITY`), 
    ///     or 
    ///     [`NEG_INFINITY`](`std::primitive::f64::NEG_INFINITY`).
    /// 
    /// ## Note
    /// Note that the lacunarity can't be 0. If 0 is parsed into this method, 
    /// the lacunarity will be set to [DEFAULT_LACUNARITY]
    pub fn set_lacunarity(&mut self, lacunarity: f64) {
        match lacunarity {
            _x if _x == 0.0 => self.lacunarity = DEFAULT_LACUNARITY,
            _x if _x.is_finite() => self.lacunarity = lacunarity,
            _x if _x.is_nan() => panic!("NaN lacunarity"),
            _x if _x.is_infinite() => panic!("Infinite lacunarity"),
            _ => unreachable!(),
        }
    }

    /// Sets the persistance of the NoiseMap.
    /// 
    /// # Panics
    /// Panics if the parsed persistance is 
    ///     [`NAN`](`std::primitive::f64::NAN`), 
    ///     [`INFINITY`](`std::primitive::f64::INFINITY`), 
    ///     or 
    ///     [`NEG_INFINITY`](`std::primitive::f64::NEG_INFINITY`).
    /// 
    /// ## Note
    /// Note that although the persistance can be set to 0, 
    /// doing this would be equivilant to only having 1 octave
    pub fn set_persistance(&mut self, persistance: f64) {
        match persistance {
            _x if _x.is_finite() => self.persistance = persistance,
            _x if _x.is_nan() => panic!("NaN lacunarity"),
            _x if _x.is_infinite() => panic!("Infinite lacunarity"),
            _ => unreachable!(),
        }
    }

    /// Changes the size of the NoiseMap.
    /// 
    /// This will empty the map
    pub fn resize(&mut self, height: usize, width: usize) {
        self.height = height;
        self.width = width;

        self.values = Vec::with_capacity(height * width);
    }

    /// Returns the actual index of the given row, column.
    /// 
    /// Uses a single vec as it is faster and easier to create a buffer from.
    /// 
    /// # Panics
    /// Panics if the NoiseMap is empty
    fn index(&self, row: usize, column: usize) -> usize {
        assert!(!self.values.is_empty());
        row * self.width + column
    }

    /// Returns the value at the given index (the map is 0-index)
    /// 
    /// # Panics
    /// Panics if the NoiseMap is empty
    pub fn get(&self, row: usize, column: usize) -> f64 {
        assert!(!self.values.is_empty());
        self.values[row * self.width + column]
    }

    /// Set the value of a single element in the map.
    /// 
    /// # Panics
    /// Panics if the NoiseMap is empty
    /// 
    /// (I don't know if we need this, so maybe delete later)
    pub fn set(&mut self, row: usize, column: usize, value: f64) {
        assert!(!self.values.is_empty());
        self.values[row * self.width + column] = value;
    }

    /// Pushes a new element into the NoiseMap.
    /// 
    /// Should only be used with empty NoiseMaps
    fn push(&mut self, value: f64) {
        self.values.push(value);
    }

    /// Normalizes the values of the NoiseMap between 0 and 1
    /// 
    /// Maybe find math crate to make this prettier :\
    fn normalize(&mut self) {
        let max = self.values   // Largest value
            .iter()
            .fold(
                f64::NEG_INFINITY,
                |x, &y| x.max(y)
        );

        let min = self.values   // Smallest value
            .iter()
                .fold(
                    f64::INFINITY,
                    |x, &y| x.min(y)
                );

        self.values = self.values
            .iter_mut()
            .map(|&mut x| {
                (x - min) / (max - min) // Calculates the normalized x
            })
            .collect();
    }

    /// Returns the scaled index.
    /// 
    /// This is used for a noise functions get method
    fn noise_point(
        &self,
        row: usize,
        column: usize,
        frequency: f64,
        offset: (i32, i32),
    ) -> [f64; 2] {
        let mut x = (row as f64 / self.scale as f64) * frequency;
        let mut y = (column as f64 / self.scale as f64) * frequency;

        x += offset.0 as f64;
        y += offset.1 as f64;

        [x, y]
    }

    /// Fills the NoiseMap with values from the given noise function
    pub fn fill(
        &mut self,
        noise_fn: impl NoiseFn<[f64; 2]>,
        seed: u32,
    ) {
        let mut prng: Pcg64 = Seeder::from(seed).make_rng();

        let mut octave_offsets = Vec::with_capacity(self.octaves);
        for _ in 0..self.octaves {
            let x = prng.gen_range(-1_000_000..1_000_000);
            let y = prng.gen_range(-1_000_000..1_000_000);
            octave_offsets.push((x, y));
        }

        for column in 0..self.height {
            for row in 0..self.width {
                let mut noise_height = 0.0;
                let mut frequency = 1.0;
                let mut amplitude = 1.0;
                
                for i in 0..self.octaves {
                    
                    let q_point = 
                        self.noise_point(
                            row,
                            column,
                            frequency,
                            octave_offsets[i]
                        );
                    
                    let value = noise_fn.get(q_point);
                    noise_height += value * amplitude;
                    
                    frequency *= self.lacunarity;   // Scale frequency with lacunarity
                    amplitude *= self.persistance;  // Scale amplitude with percistance
                }
                self.push(noise_height);


            }
        }

        self.normalize();
    }

    /// Maps values into 8bit values. 
    /// Used as the buffer when imaging the map
    /// 
    /// # Panics
    /// Panics if the NoiseMap is empty
    fn as_u8(&self) -> Vec<u8> {
        assert!(!self.values.is_empty());
        self
            .values
            .iter()
            .map(|&v| ((v * 256.0) as u8))
            .collect()
    }

    /// Creates an achromatic image in the 'noisemap_demo' folder.
    /// 
    /// This is simply to show the NoiseMap
    /// 
    /// # Panics
    /// Panics if the NoiseMap is empty
    pub fn save_as_img(&self, filename: &str) -> ImageResult<()>{
        assert!(!self.values.is_empty());
        let path = String::from("noisemap_demo/") + filename;
        
        image::save_buffer(
            path,
            &*self.as_u8(),
            self.width as u32,
            self.height as u32,
            image::ColorType::L8,
        )
    }
}

impl Default for NoiseMap {
    fn default() -> Self {
        NoiseMap {
            height: 0,
            width: 0,
            values: Vec::with_capacity(0),
            scale: DEFAULT_SCALE,
            octaves: 1,
            lacunarity: 1.0,
            persistance: 1.0,
        }
    }
}
