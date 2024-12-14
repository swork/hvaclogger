use crate::honeywell_tz4::{Fan, HvacHoneywellTz4};

pub(crate) mod honeywell_tz4;
pub(crate) mod hvac;

fn main() {
    let m = HvacHoneywellTz4 {
        testing: Some(true),
        temps: None,
        fan: Some(Fan::Off),
        emergency: Some(false),
        cool: None,
        zones: None,
    };
    match serde_json::to_string(&m) {
        Ok(j) => println!("Hello, {j}!"),
        Err(e) => println!("Whoops: {e}"),
    }
    println!("Done.");
}
