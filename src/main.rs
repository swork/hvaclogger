pub(crate) mod honeywell_tz4;
pub(crate) mod hvac;

use crate::hvac::Celcius;
use honeywell_tz4::{Fan, HvacHoneywellTz4};

fn main() {
    let m = HvacHoneywellTz4 {
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
