#![feature(div_duration,proc_macro_hygiene, decl_macro,iter_intersperse)]
#[macro_use] extern crate rocket;
use std::error::Error;

mod color;
pub mod time;
mod hardware;
mod web;

use std::collections::HashMap;
use time::TimeHandle;
use lights::manager::Manager;
use lights::Light;

mod lights;
#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /*
    let manager:Arc<Mutex<LightManager<RocketManager>>> = LightManager::new();
    {
        let mut  manager = manager.lock().unwrap();

        let led_jacob = manager.create_strip("Jacob".to_string(),(27,22,17))?;
        let led_ben   = manager.create_strip("Ben".to_string(),(25,24,23))?;
        let led_night = manager.create_strip("night_light".to_string(),(13,6,5))?;
     */

    let mut lights = HashMap::default();
    lights.insert("light1".into(),Light::new((27,22,17).into()));
    lights.insert("light2".into(),Light::new((25,24,23).into()));
    lights.insert("light3".into(),Light::new((13,6,5).into()));
    let time_handle = TimeHandle::new();
    let manager = Manager::new(lights,time_handle);
    web::rocket(manager).launch().await;
    Ok(())
}


