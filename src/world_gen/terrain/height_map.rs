use crate::world_gen::noise::noise_map::NoiseMap;

pub type Height = i32;

/// Bandaid fix, but some form of this should propably be used.
/// 
/// Need to implement a better version of this, but i like the idea of having this 'higher level' version of [`NoiseMap`] 
/// making the noise values make sense as height measurements.
/// 
/// I also disslike the way i have been handling the ranges of [`TerrainType`](super::terrain_type::TerrainType)'s when they are floats, 
/// so mapping them to integers also solves that.
pub struct HeightMap {
    height: usize,
    width: usize,
    values: Vec<Height>,
}

/// Methods for accessing data in the HeightMap.
impl HeightMap {
    fn index(&self, row: usize, column: usize) -> usize {
        assert!(row < self.height);
        assert!(column < self.width);
    
        row * self.width + column
    }
    
    /// Returns the height value at the given position
    pub fn get(&self, row: usize, column: usize) -> Height {
        self.values[ self.index(row, column) ]
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
}

/// Methods for creating a HeightMap.
impl HeightMap {

    /// Returns an empty HeightMap with the given size.
    fn from_size(height: usize, width: usize) -> HeightMap {
        HeightMap {
            height,
            width,
            values: Vec::with_capacity(height * width),
        }
    }

    /// There is realy no reason to create a HeightMap in any other way.
    /// 
    /// ## Returns
    /// Returns a HeightMap where every value is the result of parsing every, 
    /// value from a NoiseMap through the given mapping function.
    pub fn from_noise_map<F>(noise_map: &NoiseMap, mapper: F) -> Self 
    where
        F: Fn(f64) -> Height
    {   
        let mut height_map =
            HeightMap::from_size(noise_map.get_height(), noise_map.get_width());
        
        for column in 0..height_map.width {
            for row in 0..height_map.height {
                let noise_val = noise_map.get_value(row, column);
                let height = mapper(noise_val);

                height_map
                    .values
                    .push(height);
            }
        }
        height_map
    }
}
