use float_cmp::approx_eq;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Celcius(pub f32);

impl PartialEq for Celcius {
    fn eq(&self, other: &Self) -> bool {
        println!("eq({self:?}, {other:?})");
        // ulps=10_000 gives 0.01 tolerance but not 0.02 at 20.x, empirically.
        let close_enough = approx_eq!(f32, self.0, other.0, epsilon = 0.01);
        close_enough
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct PlantTemps {
    pub iat: Celcius,
    pub dat: Celcius,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_temps_equal() {
        assert_eq!(Celcius(-0.), Celcius(0.));
    }

    #[test]
    fn close_temps_equal() {
        assert_eq!(Celcius(2.001), Celcius(2.002));
        assert_eq!(Celcius(20.001), Celcius(20.002));
        assert_eq!(Celcius(200.001), Celcius(200.002));
        assert_eq!(Celcius(-2.001), Celcius(-2.002));
        assert_eq!(Celcius(-20.001), Celcius(-20.002));
        assert_eq!(Celcius(-200.001), Celcius(-200.002));
    }

    #[test]
    fn not_close_temps_not_equal() {
        assert_ne!(Celcius(20.), Celcius(30.));
    }
}
