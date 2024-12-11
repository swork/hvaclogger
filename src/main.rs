use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
struct Celcius(f32);

/*
impl<'a,
     M: FloatMargin,
     F: ApproxEq<Margin=M>
    > ApproxEq for &'a Celcius {
    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.0.approx_eq(other.0, margin)
    }
}
    */

#[derive(Clone, Copy, Debug, Serialize)]
struct PlantTemps {
    iat: Celcius,
    dat: Celcius,
}

#[derive(Clone, Copy, Debug, Serialize)]
enum Fan {
    _On { temps: PlantTemps },
    _Purge { temps: PlantTemps },
    Off,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
enum Zone {
    _Active,
    _Inactive,
}

#[derive(Clone, Copy, Debug, Serialize)]
struct Zones([Zone; 4]);

#[derive(Clone, Copy, Debug, Serialize)]
struct HvacMymodel {
    outside_at: Option<Celcius>,
    ambient_at: Option<Celcius>,
    indoor_at: Option<Celcius>,
    fan: Option<Fan>,
    emergency: Option<bool>,
    zones: Option<Zones>,
}

fn main() {
    let m = HvacMymodel {
        outside_at: None,
        ambient_at: Some(Celcius(15.)),
        indoor_at: None,
        fan: Some(Fan::Off),
        emergency: Some(false),
        zones: None,
    };
    match serde_json::to_string(&m) {
        Ok(j) => println!("Hello, {j}!"),
        Err(e) => println!("Whoops: {e}"),
    }
    println!("Done.");
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
}
