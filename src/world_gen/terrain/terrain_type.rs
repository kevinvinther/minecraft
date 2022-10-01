/// Not sure if we will actually end up using this (maybe), but it should help
/// make working with the 2D texture things easier.
/// 
/// Consider finding a better way of handling TerrainType's and their assosiated constants.
/// Maybe a handler type?
/// 
/// I would like it, if we only needed to change one thing to add/remove TerrainType's, will look at tomorrow i think

use std::ops::Range;
use image::Rgb;

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum TerrainType {
    Ocean,
    Land,

    /// The 'None' variant 
    None,
}

/// Identify terrain
impl TerrainType {
    /// Returns the apropriate TerrainType according to the parsed height.
    /// 
    /// # Panics
    /// 
    /// Heights not found in any TerrainType range are invalid, causing a panic.
    pub fn from_height(height: f64) -> TerrainType {
        match height {
            h if ranges::OCEAN.contains(&h) => TerrainType::Ocean,
            h if ranges::LAND.contains(&h)  => TerrainType::Land,

            // I don't like using a catch-all here, but don't know how you would not
            // If a new range is added, the compiler will NOT emit an error, can this be fixed?
            h => panic!("Invalid height value! height{{ {} }}", h)
        }
    }
}

/// Terrain ranges
impl TerrainType {
    /// Returns the height range for the provided TerrainType variant.
    /// 
    /// # Panics
    /// 
    /// Parsing the [`None`](TerrainType::None) variant will panic, as this type is invalid.
    pub fn range(&self) -> Range<f64>{        
        match self {
            Self::Ocean => ranges::OCEAN,
            Self::Land  => ranges::LAND,
            
            // In case of something invalid happening
            // Consider returning an empty range, i'm not sure
            Self::None  => panic!(
                "TerrainType::None is not a valid terrain type and has no assosiated range!"
            ),
        }
    }
}

/// Terrain colour values
impl TerrainType {
    /// Returns the assosiated colour of the TerrainType variant.
    /// 
    /// # Panics
    /// 
    /// Parsing the [`None`](TerrainType::None) variant will panic, as this type is invalid.
    pub fn colour(&self) -> Rgb<u8> {
        match self {
            Self::Ocean => colours::OCEAN,
            Self::Land  => colours::LAND,

            Self::None  => panic!(
                "TerrainType::None is not a valid terrain type and has no assosiated colour!"
            ),
        }
    }
}

/// Ranges for the different terrain types
pub(super) mod ranges {
    use std::ops::{Range, RangeInclusive};

    /// The smallest, valid, value for a TerrainType's range
    const MIN_VALUE: f64 = -1.0;

    /// The largest, valid, value for a TerrainType's range.
    /// 
    /// I kinda forgot about how 1.0 ends up excluded, so now it's working.
    /// 
    /// i plan on translating the noise map to a height map containing integer values, but for now... this.
    const MAX_VALUE: f64 = 1.5;

    pub const OCEAN: Range<f64> = MIN_VALUE..0.4;
    pub const LAND: Range<f64>  = 0.4..MAX_VALUE;
}

pub(super) mod colours {
    use image::Rgb;

    pub const OCEAN: Rgb<u8>    = Rgb([ 50,  99, 195]);
    pub const LAND: Rgb<u8>     = Rgb([ 69, 120,  20]);
}

/// # TESTS TO ADD:
/// 
/// Are all range consts valid? (are they all within the accepted range? do any of them overlap?)
/// 
/// `from_range()` method output is valid and as expected for ALL ranges?
/// 
mod tests {

}
