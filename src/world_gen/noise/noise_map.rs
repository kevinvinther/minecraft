use image::{ImageFormat, ImageResult};
use noise::NoiseFn;

const DEFAULT_SCALE: usize = 100;

pub struct NoiseMap {
    height: usize,
    width: usize,
    values: Vec<f64>,
    scale: usize,
}

impl NoiseMap {
    /// Creates a new, empty, NoiseMap
    pub fn new(height: usize, width: usize) -> Self {
        NoiseMap {
            height,
            width,
            values: Vec::with_capacity(height * width),
            ..Default::default()
        }
    }

    /// Creates and fills a NoiseMap with values from the give noise function
    pub fn from_noisefn(
        height: usize,
        width: usize,
        scale: usize,
        noise_fn: impl NoiseFn<[f64; 2]>
    ) -> Self {
        let mut map = NoiseMap {
            height,
            width,
            values: Vec::with_capacity(height * width),
            scale
        };
        map.fill(noise_fn);
        map
    }

    /// Returns the actual index of the given row, column
    fn index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    /// Returns the value at the given index (the map is 0-index)
    pub fn get(&self, row: usize, column: usize) -> f64 {
        assert!(!self.values.is_empty());
        self.values[row * self.width + column]
    }

    /// Set the value of a single element in the map
    pub fn set(&mut self, row: usize, column: usize, value: f64) {
        self.values[row * self.width + column] = value;
    }

    /// Pushes a new element into the NoiseMap.
    /// Should only be used with empty NoiseMaps
    fn push(&mut self, value: f64) {
        self.values.push(value);
    }

    /// Returns the scaled index.
    /// This is used for a noise functions get method
    fn noise_point(&self, row: usize, column: usize) -> [f64; 2] {
        let x = (row as f64) / self.scale as f64;
        let y = (column as f64) / self.scale as f64;
        [x, y]
    }

    /// fills the NoiseMap with values from the given noise function
    pub fn fill(&mut self, noise_fn: impl NoiseFn<[f64; 2]>) {
        for row in 0..self.width {
            for column in 0..self.height {
                self.push(
                    noise_fn.get(self.noise_point(row, column))
                );
            }
        }
    }

    /// maps values into 8bit values.
    /// used for images
    fn as_u8(&self) -> Vec<u8> {
        self
            .values
            .iter()
            .map(|v| ((*v + 1.0) * (256.0 / 2.0)) as u8)
            .collect()
    }

    /// Creates an image in the 'noisemap_demo' folder.
    /// This is simply to show the NoiseMap so is simply grey-scale
    pub fn save_as_img(&self, filename: &str) -> ImageResult<()>{
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
        }
    }
}
