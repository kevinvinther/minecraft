/// Not sure if we will actually end up using this (maybe), but it should help
/// make working with the 2D texture things easier.
/// 
/// The error handling here isn't perfect, but it should be plenty for this.

use std::ops::Range;
use image::Rgb;

use terrain_data::meta_data as meta_data;

/// The type of the height value
use super::height_map::Height;

/// All different types of terrain.
/// 
/// The 'None' variant is not a valid TerrainType. Any method called on it will result in panic.
/// The intention of [`TerrainType::None`] is for potentially recoverable errors.
/// 
/// Maybe use a HashMap to point from variant to constant?
#[derive(Debug, Copy, Clone)]
pub enum TerrainType {
    DeepOcean,
    Ocean,
    Beach,
    LowLand,
    HighLand,

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

        // Not sure if there is a better way of doing this, but this is what i came up with
        match height {
            _ if terrain_data::DEEP_OCEAN.contains(height)  => Self::DeepOcean,
            _ if terrain_data::OCEAN.contains(height)       => Self::Ocean,
            _ if terrain_data::BEACH.contains(height)       => Self::Beach,
            _ if terrain_data::LOW_LAND.contains(height)    => Self::LowLand,
            _ if terrain_data::HIGH_LAND.contains(height)   => Self::HighLand,

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
            Self::DeepOcean => terrain_data::DEEP_OCEAN.colour,
            Self::Ocean     => terrain_data::OCEAN.colour,
            Self::Beach     => terrain_data::BEACH.colour,
            Self::LowLand   => terrain_data::LOW_LAND.colour,
            Self::HighLand  => terrain_data::HIGH_LAND.colour,

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
    pub fn _contains(&self, value: &Height) -> bool {
        match self {
            Self::DeepOcean
                => terrain_data::DEEP_OCEAN.range.contains(value),
            Self::Ocean 
                => terrain_data::OCEAN.range.contains(value),
            Self::Beach
                => terrain_data::BEACH.range.contains(value),
            Self::LowLand
                => terrain_data::LOW_LAND.range.contains(value),
            Self::HighLand
                => terrain_data::HIGH_LAND.range.contains(value),
            
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
pub struct TypeData {
    range: Range<Height>,
    colour: Rgb<u8>,
}

impl TypeData {
    fn contains(&self, value: &Height) -> bool {
        self.range.contains(value)
    }
}

/// Data defining the different terrain types.
/// 
/// I did try making this work with [`Range`] and [`RangeInclusive`]
/// as to not exclude [`MAX_VALUE`], which is a valid value, but this ended up causing a couple problems i was unable to solve in a nice way. 
/// However, with the addition of [`HeightMap`], simply sticking to [`Range`] is possible as it lets us handle the noise values as integers. 
/// That still isn't perfect, but it's a lot better than before.
/// 
/// [`Range`]: std::ops::Range
/// [`RangeInclusive`]: std::ops::RangeInclusive
/// [`HeightMap`]: crate::world_gen::terrain::height_map::HeightMap
/// [`MAX_VALUE`]: meta_data::MAX_VALUE
mod terrain_data {
    use image::Rgb;

    use super::TypeData;
    use super::Height;
    use meta_data::*;

    /// Data to define valid values for TypeData constants
    pub(super) mod meta_data {
        use super::Height;
        
        /// The smallest, valid, value for the range of a terrain type
        pub const MIN_VALUE: Height = 0;

        /// The largest, valid, value for the range of a terrain type.
        /// 
        /// Range is not inclusive on the maximum, so we +1 to allow all values. 
        /// This addition is written excplicitly to make this clearer.
        pub const MAX_VALUE: Height =  100 + 1;
    }

    pub const DEEP_OCEAN: TypeData = TypeData {
        range:  MIN_VALUE..35,
        colour: Rgb([ 15,  82, 186]),
    };

    pub const OCEAN: TypeData = TypeData {
        range:  35..45,
        colour: Rgb([ 65, 105, 225]),
    };

    pub const BEACH: TypeData = TypeData {
        range:  45..50,
        colour: Rgb([194, 178, 128]),
    };

    pub const LOW_LAND: TypeData = TypeData {
        range:  50..55,
        colour: Rgb([ 19, 133,  16]),
    };

    pub const HIGH_LAND: TypeData = TypeData {
        range:  55..MAX_VALUE,
        colour: Rgb([ 19, 109,  21]),
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
        unimplemented!();
    }
}
