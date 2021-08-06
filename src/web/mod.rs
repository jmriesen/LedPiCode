#[cfg(test)]
mod tests;

use std::sync::{Arc, Mutex};
use crate::rocket;
use rocket::{Rocket, Build,State};
//use rocket_contrib::json::Json;
use crate::lights::{
    manager::Manager,
    commands::Pattern,
};
use crate::color::Color;
use rocket::serde::{json::Json};

#[get("/status")]
fn status(manager:&State<Arc<Mutex<Manager>>>) ->Json<Vec<(String,Color)>>{
    Json(manager.lock().unwrap().status())
}

#[post("/set/<name>", data = "<color>")]
fn set(name:String,color:Json<Color>,manager:&State<Arc<Mutex<Manager>>>){
    let _ = manager.lock().unwrap().command(&name,Pattern::Constent(*color));
}

pub fn rocket(light_manager:Arc<Mutex<Manager>>) -> Rocket<Build> {
    rocket::build()
        .manage(light_manager)
        .mount("/", routes![status,set])
}
