// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_Planet {
    (for $t:ty, $n:expr) => {
        impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                (d.0 as f64 / 31557600.) * 1.0 / $n
            }
        }
    };
}

impl_Planet!(for Mercury, 0.2408467);
impl_Planet!(for Venus, 0.61519726);
impl_Planet!(for Earth, 1.0);
impl_Planet!(for Mars, 1.8808158);
impl_Planet!(for Jupiter, 11.862615);
impl_Planet!(for Saturn, 29.447498);
impl_Planet!(for Uranus, 84.016846);
impl_Planet!(for Neptune, 164.79132);
