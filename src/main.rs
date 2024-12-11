pub(crate) mod honeywell_tz4;

use honeywell_tz4::{Celcius, Fan, HvacMymodel, PlantTemps, Zone, Zones};

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
