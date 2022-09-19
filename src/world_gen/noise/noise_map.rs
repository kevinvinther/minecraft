use image::{ImageFormat, ImageResult};
use noise::NoiseFn;

pub const DEFAULT_SCALE: usize = 100;

/// A struct holding noise data for a section of the given noise function
/// TODO:
///     Height and width should be replaced with two points
///     Find an apropriate default scale (this can't be done yet)
pub struct NoiseMap {
    height: usize,
    width: usize,
    values: Vec<f64>,
    scale: usize,
}

impl NoiseMap {
    /// Creates a new and empty NoiseMap
    /// 
    /// The [`fill`](`Self::fill`) method should be used to fill the map with values
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
    /// # Note
    /// Note that the scale cannot be 0. If 0 is parsed into this method,
    /// the scale will be set to [DEFAULT_SCALE]
    pub fn from_noisefn(
        height: usize,
        width: usize,
        scale: usize,
        noise_fn: impl NoiseFn<[f64; 2]>
    ) -> Self {
        let mut map = NoiseMap::new(
            height,
            width,
        );
        map.set_scale(scale);
        map.fill(noise_fn);
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

    /// Returns the scaled index.
    /// 
    /// This is used for a noise functions get method
    fn noise_point(&self, row: usize, column: usize) -> [f64; 2] {
        let r = (row as f64) / self.scale as f64;   // If scale is 0, we get `row / 0` (not good)
        let c = (column as f64) / self.scale as f64;
        [r, c]
    }

    /// Fills the NoiseMap with values from the given noise function
    pub fn fill(&mut self, noise_fn: impl NoiseFn<[f64; 2]>) {
        for column in 0..self.height {
            for row in 0..self.width {
                self.push(
                    noise_fn.get(self.noise_point(row, column))
                );
            }
        }
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
            .map(|v| ((*v + 1.0) / 2.0 * 256.0) as u8)
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
        }
    }
}
