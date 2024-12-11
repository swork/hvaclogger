pub(crate) mod honeywell_tz4;
pub(crate) mod hvac;

use honeywell_tz4::{Fan, HvacHoneywellTz4};

fn main() {
    let m = HvacHoneywellTz4 {
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
