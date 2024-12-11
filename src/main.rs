use serde::Serialize;

#[derive(Serialize)]
struct Celcius(f32);

#[derive(Serialize)]
struct InputDischargeTemps {
    iatt: Celcius,
    datt: Celcius
}

#[derive(Serialize)]
enum Fan {
    On { temps: InputDischargeTemps },
    Purge { temps: InputDischargeTemps },
    Off,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
enum Zone {
    Active,
    Inactive,
}

#[derive(Serialize)]
struct Zones([Zone; 4]);

impl Zones {
    fn different(&self, other: &Zones) -> bool {
        for (index, z) in self.0.iter().enumerate() {
            if *z != other.0[index] {
                return false
            }
        }
        true
    }
}

#[derive(Serialize)]
struct HvacMymodel {
    outside_att: Celcius,
    ambient_att: Celcius,
    indoor_att: Celcius,
    fan: Fan,
    emergency: bool,
    zones: Zones
}

impl HvacMymodel {
}


fn main() {
    println!("Hello, world!");
}
