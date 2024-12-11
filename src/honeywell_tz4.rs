// Honeywell TotalZone 4 HVAC controller

use float_cmp::approx_eq;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Celcius(pub f32);

impl PartialEq for Celcius {
    fn eq(&self, other: &Self) -> bool {
        let close_enough = approx_eq!(f32, self.0, other.0, ulps = 1);
        close_enough
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct PlantTemps {
    pub iat: Celcius,
    pub dat: Celcius,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Fan {
    _On { temps: PlantTemps },
    _Purge { temps: PlantTemps },
    Off,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum Zone {
    _Active,
    _Inactive,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Zones(pub [Zone; 4]);

#[derive(Clone, Copy, Debug, Serialize)]
pub struct HvacMymodel {
    pub outside_at: Option<Celcius>,
    pub ambient_at: Option<Celcius>,
    pub indoor_at: Option<Celcius>,
    pub fan: Option<Fan>,
    pub emergency: Option<bool>,
    pub zones: Option<Zones>,
}

#[cfg(test)]
mod tests {
    use super::*;

    static EMPTY_MODEL: HvacMymodel = HvacMymodel {
        outside_at: None,
        ambient_at: None,
        indoor_at: None,
        fan: None,
        emergency: None,
        zones: None,
    };

    #[test]
    #[should_panic]
    fn oat_null() {
        let me = HvacMymodel { ..EMPTY_MODEL };
        me.outside_at.unwrap();
    }

    #[test]
    #[should_panic]
    fn aat_null() {
        let me = HvacMymodel { ..EMPTY_MODEL };
        me.ambient_at.unwrap();
    }

    #[test]
    #[should_panic]
    fn fan_null() {
        let me = HvacMymodel { ..EMPTY_MODEL };
        me.fan.unwrap();
    }

    #[test]
    fn fan_not_null() {
        let me: HvacMymodel = HvacMymodel {
            fan: Some(Fan::Off),
            ..EMPTY_MODEL
        };
        assert_eq!(me.fan.unwrap(), Fan::Off);
    }
}
