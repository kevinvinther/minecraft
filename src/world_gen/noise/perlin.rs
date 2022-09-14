use rand::{
    SeedableRng, Rng,
    distributions::{Distribution, Standard},
    seq::SliceRandom,
};
use rand_xorshift::XorShiftRng;

const TABLE_SIZE: usize = 256;

pub fn perlin_noise(x: f64, y: f64, p: PermutationTable) -> f64{
    let q_point = Vector2::new(x, y);
    let unit_point = unit_square(&q_point);   // Find unit square containing query point (x, y)

    let x = x - x.floor();  // Relative coords of the point in the unit square
    let y = y - y.floor();

    let a = Vector2::new(x, y - 1.0);
    let b = Vector2::new(x - 1.0, y - 1.0);
    let c = Vector2::new(x, y);
    let d = Vector2::new(x - 1.0, y);

    let val_a = p.values[unit_point.x as usize + 1] + unit_point.y as u8;     // Hash the coordinates of the corners of the square
    let val_b = p.values[unit_point.x as usize    ] + unit_point.y as u8;
    let val_c = p.values[unit_point.x as usize + 1] + unit_point.y as u8 + 1;
    let val_d = p.values[unit_point.x as usize    ] + unit_point.y as u8 + 1;

    let dot_a = a.dot(get_gradient(val_a));
    let dot_b = b.dot(get_gradient(val_b));
    let dot_c = c.dot(get_gradient(val_c));
    let dot_d = d.dot(get_gradient(val_d));

    let u = fade_curve(x);
    let v = fade_curve(y);

    lerp(u,
        lerp(v, dot_c, dot_a),
        lerp(v, dot_d, dot_b),
    )
}

fn unit_square(point: &Vector2) -> Vector2 {
    let (x, y) = point.floor();
    Vector2 {
        x: (x & 255) as f64,    // Bit-wise AND makes values "wrap" ie.
        y: (y & 255) as f64,    //      [1 - 255] & 255 = [1 - 255] (no change)
                                //      [256..] & 255 = [1 - 255] (value wraps around)
    }
}

/// Uses the fifth degree interpolator
///     6t^5 - 15t^4 + 10t^3
/// as described by the improvements made to the perlin noise algorithm in 2002
fn fade_curve(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Bilinear interpolation gives us the final value for our query point (x, y)
fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

fn get_gradient(perm: u8) -> Vector2{
    match perm & 0b11 {
        0 =>  Vector2::new( 1.0,  1.0),    // ( 1,  1)
        1 =>  Vector2::new( 1.0, -1.0),    // ( 1, -1)
        2 =>  Vector2::new(-1.0,  1.0),    // (-1,  1)
        3 =>  Vector2::new(-1.0, -1.0),    // (-1, -1)
        _ => unreachable!()
    }
}

struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    fn new(x: f64, y: f64) -> Self {
        Vector2{x, y}
    }

    fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn floor(&self) -> (i32, i32) {
        (self.x.floor() as i32, self.y.floor() as i32)
    }
}

/// Almost everything about the PermutationTable is from the noise crate
#[derive(Debug)]
pub struct PermutationTable {
    values: [u8; TABLE_SIZE],
}

impl Distribution<PermutationTable> for Standard {
    // Defines how a PermutationTable can be randomly generated
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PermutationTable {
        let mut values: Vec<u8> = (0..TABLE_SIZE).map(|x| x as u8).collect();
        values.shuffle(rng);

        let mut perm_table = PermutationTable {
            values: [0; TABLE_SIZE],
        };

        perm_table.values
            .iter_mut()
            .zip(values)
            .for_each(|(x, y)| {
                *x = y
            });
        
        perm_table
    }
}

impl PermutationTable {
    pub fn new(seed: u32) -> Self {
        let mut real = [0; 16];
        real[0] = 1;
        for i in 1..4 {
            real[i * 4] = seed as u8;
            real[(i * 4) + 1] = (seed >> 8) as u8;
            real[(i * 4) + 2] = (seed >> 16) as u8;
            real[(i * 4) + 3] = (seed >> 24) as u8;
        }
        let mut rng: XorShiftRng = SeedableRng::from_seed(real);
        rng.gen()
    }
}
