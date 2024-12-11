// Honeywell TotalZone 4 HVAC controller

use crate::hvac::{EnvironmentTemps, PlantTemps};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Fan {
    _On { temps: PlantTemps },
    _Purge { temps: PlantTemps },
    Off,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Zone {
    _Active,
    _Inactive,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Zones(pub [Zone; 4]);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct HvacHoneywellTz4 {
    pub temps: Option<EnvironmentTemps>,
    pub fan: Option<Fan>,
    pub emergency: Option<bool>,
    pub zones: Option<Zones>,
}

#[cfg(test)]
mod tests {
    use super::*;

    static EMPTY_MODEL: HvacHoneywellTz4 = HvacHoneywellTz4 {
        temps: None,
        fan: None,
        emergency: None,
        zones: None,
    };

    #[test]
    #[should_panic]
    fn oat_null() {
        let me = HvacHoneywellTz4 { ..EMPTY_MODEL };
        me.temps.unwrap().outside_at.unwrap();
    }

    #[test]
    #[should_panic]
    fn aat_null() {
        let me = HvacHoneywellTz4 { ..EMPTY_MODEL };
        me.temps.unwrap().plant_at.unwrap();
    }

    #[test]
    #[should_panic]
    fn fan_null() {
        let me = HvacHoneywellTz4 { ..EMPTY_MODEL };
        me.fan.unwrap();
    }

    #[test]
    fn fan_not_null() {
        let me: HvacHoneywellTz4 = HvacHoneywellTz4 {
            fan: Some(Fan::Off),
            ..EMPTY_MODEL
        };
        assert_eq!(me.fan.unwrap(), Fan::Off);
    }
}
