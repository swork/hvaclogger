use blinkie::ExampleConcreteBlinker;
use honeywell_tz4::{Fan, HvacHoneywellTz4, Zone, Zones};
use hvac::{Celcius, EnvironmentTemps, PlantTemps};
use log::ObservationQueueFront;
use rand::prelude::*;
use std::{
    env,
    sync::{Arc, Mutex},
};

pub(crate) mod blinkie;
pub(crate) mod honeywell_tz4;
pub(crate) mod hvac;
pub(crate) mod log;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let cmd = &args[1][..];

        match cmd {
            "send" => send(),
            _ => hello(),
        }
    } else {
        hello()
    }
}

fn hello() {
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

fn send() {
    let cool = Some(rand::random());
    let cooling;
    match &cool {
        &Some(true) => cooling = 1.,
        _ => cooling = -1.,
    };

    let fan;
    let fan_select: f32 = rand::random();
    let fan_iat: f32 = rand::random::<f32>() * 17. + 10.;
    let fan_dat: f32 = fan_iat + (cooling * (rand::random::<f32>() * 10.));
    let fan_temps = PlantTemps {
        iat: Some(Celcius(fan_iat)),
        dat: Some(Celcius(fan_dat)),
    };
    let fan_off = Fan::Off;
    let fan_on: Fan = Fan::On {
        temps: Some(fan_temps),
    };
    let fan_purge: Fan = Fan::Purge {
        temps: Some(fan_temps),
    };

    if fan_select < 0.333 {
        fan = Some(fan_off)
    } else if fan_select < 0.666 {
        fan = Some(fan_on)
    } else {
        fan = Some(fan_purge)
    }

    let emergency = Some(rand::random());

    let mut zs1 = Zone::Active;
    let mut zs2 = Zone::Active;
    let mut zs3 = Zone::Active;
    let mut zs4 = Zone::Active;
    if random() {
        zs1 = Zone::Inactive
    }
    if random() {
        zs2 = Zone::Inactive
    }
    if random() {
        zs3 = Zone::Inactive;
    }
    if random() {
        zs4 = Zone::Inactive
    }
    let zones = Some(Zones([zs1, zs2, zs3, zs4]));

    let m = HvacHoneywellTz4 {
        testing: Some(true),
        temps: Some(EnvironmentTemps {
            outside_at: Some(Celcius(rand::random::<f32>() * 40. - 10.)),
            plant_at: Some(Celcius(rand::random::<f32>() * 20. + 5.)),
            indoor_at: Some(Celcius(rand::random::<f32>() * 17. + 10.)),
        }),
        cool,
        fan,
        emergency,
        zones,
    };

    println!("send: {m:?}");

    let cb = Arc::new(Mutex::new(ExampleConcreteBlinker::new()));
    let mut q = ObservationQueueFront::new(cb);
    q.submit(m);
    // q.end_when_idle();
}
