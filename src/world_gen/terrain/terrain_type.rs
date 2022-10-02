/// Not sure if we will actually end up using this (maybe), but it should help
/// make working with the 2D texture things easier.

use std::ops::{Range, RangeBounds, RangeInclusive};
use image::Rgb;

use terrain_data::meta_data as meta_data;

/// The type of the height value
type Height = f64;

/// All different types of terrain.
/// 
/// The 'None' variant is not a valid TerrainType. Any method called on it will result in panic.
/// The intention of [`TerrainType::None`] is for potentially recoverable errors.
#[derive(Debug, Copy, Clone)]
pub enum TerrainType {
    Ocean,
    Land,

    #[allow(dead_code)]
    None,
}

/// Identify terrain
impl TerrainType {
    /// Identifies the TerrainType whose range contains the given height
    /// 
    /// # Panics
    /// 
    /// Height values not contained in any TerrainType range are invalid.
    pub fn ident(height: &Height) -> TerrainType {
        // Assert that height is within valid bounds according to MAX_VALUE 
        assert!(
            height <= &meta_data::MAX_VALUE,
            "\n
            INVALID HEIGHT VALUE:\n
            Parsed height = {:?}\tLargest valid height value = {}\n
            \t  Cannot identify TerrainType based on invalid value! ({:?} > {})\n
            ", height, meta_data::MAX_VALUE, height, meta_data::MAX_VALUE,
        );

        // Assert that height is within valid bounds according to MIN_VALUE
        assert!(
            height >= &meta_data::MIN_VALUE,
            "\n
            INVALID HEIGHT VALUE:\n
            Parsed height = {:?}\tSmallest valid height value = {}\n
            \t  Cannot identify TerrainType based on invalid value! ({:?} <) {}\n
            ", height, meta_data::MIN_VALUE, height, meta_data::MIN_VALUE,
        );

        // Not sure if there is a better way of doing this, but this is what i could come up with
        match height {
            _ if terrain_data::OCEAN.contains(height)   => return Self::Ocean,
            _ if terrain_data::LAND.contains(height)    => return Self::Land,

            _ => {  // I can't imagine recovery from here ever being a good idea, panic seems appropriate
                panic!("\n
                INVALID HEIGHT VALUE:\n
                Could not identify a valid TerrainType given value {:?}.\n
                \t  It is possible a TerrainType has been excluded from the `crate::world_gen::terrain::terrain_type::TerrainType::ident()` method\n
                ",
                height
                )
            }
        }
    }
}

/// Data-relevant methods for TerrainType's
impl TerrainType {
    /// Returns the assosiated colour of the TerrainType variant.
    /// 
    /// # Panics
    /// 
    /// Parsing the [`None`](TerrainType::None) variant will panic.
    pub fn colour(&self) -> Rgb<u8> {
        match self {
            Self::Ocean => terrain_data::OCEAN.colour,
            Self::Land  => terrain_data::LAND.colour,

            Self::None  => {
                panic!("\n{}{}{}{}",
                    "INVALID METHOD CALL:\n",
                    "Attempted to get colour of TerrainType::None\n",
                    "\tThe None variant represents the lack of a type, as such,",
                    "\tit is invalid for methods accessing data related to a TerrainType.\n",
                );
            }
        }
    }

    /// Returns true if the value is within the scope of a TerrainType variant.
    /// 
    /// It's just nicer to write it like this, don't actually know if we need this method.
    pub fn contains(&self, value: &Height) -> bool {
        match self {
            Self::Ocean 
                => return terrain_data::OCEAN.range.contains(value),
            Self::Land
                => return terrain_data::LAND.range.contains(value),
            
            Self::None => {
                panic!("\n{}{}{:?}{}{}{}",
                    "INVALID METHOD CALL:\n",
                    "Attempted to range match value ", value, " on TerrainType::None\n",
                        "\tThe None variant represents the lack of a type, as such,",
                        "\tit is invalid for methods accessing data related to a TerrainType.\n",
                );   
            }
        }        
    }
}

/// Generic type to hold relevant data to define different types of terrain.
pub struct TypeData<R>
where
    R: RangeBounds<Height>
{
    range: R,
    colour: Rgb<u8>,
}

impl TypeData<Range<Height>> {
    fn contains(&self, value: &Height) -> bool {
        self.range.contains(value)
    }
}

impl TypeData<RangeInclusive<Height>> {
    fn contains(&self, value: &Height) -> bool {
        self.range.contains(value)
    }
}

/// Data for the different terrain types.
/// 
/// Note, that i plan on expanding the range of values in the future, to make it more intuitive,
/// but for now, this is made to work on the pure output data from a NoiseMap.
mod terrain_data {
    use image::Rgb;
    use std::ops::{Range, RangeInclusive};

    use super::{TypeData, Height};
    use meta_data::*;

    /// Data to define valid values for TypeData constants
    pub(super) mod meta_data {
        use super::super::{Height};
        
        /// The smallest, valid, value for the range of a terrain type
        pub const MIN_VALUE: Height = -1.0;

        /// The largest, valid, value for the range of a terrain type
        pub const MAX_VALUE: Height =  1.0;
    }


    /// Women when they see me
    pub const OCEAN: TypeData<Range<Height>> = TypeData {
        range:  MIN_VALUE..0.4,
        colour: Rgb([ 50,  99, 195]),
    };

    /// Women when i talk to them :(
    pub const LAND: TypeData<RangeInclusive<Height>> = TypeData {
        range:  0.4..=MAX_VALUE,
        colour: Rgb([ 69, 120,  20]),
    };
}

/// # TESTS TO ADD:
/// 
/// Are all range consts valid? (are they all within the accepted range? do any of them overlap?)
/// 
/// `from_range()` method output is valid and as expected for ALL ranges?
/// 
/// `ident()` method returns the expected TerrainType (also test the asserts)
#[cfg(test)]
mod tests {
    #[test]
    fn terrain_type_range_validation() {
        todo!();
    }
}
