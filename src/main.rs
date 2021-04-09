#![feature(div_duration,proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use std::sync::Mutex;
use std::error::Error;
use std::sync::Arc;
// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.

mod lights;
use lights::{
    light_manager::LightManager,
    RocketManager,
};
fn main() -> Result<(), Box<dyn Error>> {
    let manager:Arc<Mutex<LightManager<RocketManager>>> = LightManager::new();
    {
        let mut  manager = manager.lock().unwrap();

        let led_jacob = manager.create_strip("Jacob".to_string(),(27,22,17))?;
        let led_ben   = manager.create_strip("Ben".to_string(),(25,24,23))?;
        let led_night = manager.create_strip("night_light".to_string(),(13,6,5))?;

        let _ = manager.create_group(String::from("All"),vec![led_jacob,led_ben,led_night]);
    }
    rocket::ignite()
        .manage(manager)
        .mount("/led/", lights::controler::endpoints())
        .mount("/led/examples/", lights::controler::examples())
        .launch();
    Ok(())
}


