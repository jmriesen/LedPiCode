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
use crate::color::{Color,BLACK,WHITE};
use rocket::serde::{json::Json};

#[get("/status")]
fn status(manager:&State<Arc<Mutex<Manager>>>) ->Json<Vec<(String,Color)>>{
    Json(manager.lock().unwrap().status())
}

#[post("/set/<name>", data = "<color>")]
fn set(name:String,color:Json<Color>,manager:&State<Arc<Mutex<Manager>>>)->Json<(String,Color)>{
    let _ = manager.lock().unwrap().command(&name,Pattern::Constent(*color));
    Json((name,*color))
}
//TODO This is just temporary HAS NOT BEEN TESTED
#[post("/off/<name>")]
fn off(name:String,manager:&State<Arc<Mutex<Manager>>>)->Json<(String,Color)>{
    let _ = manager.lock().unwrap().command(&name,Pattern::Constent(BLACK));
    Json((name,BLACK))
}
//TODO This is just temporary HAS NOT BEEN TESTED
#[post("/on/<name>")]
fn on(name:String,manager:&State<Arc<Mutex<Manager>>>)->Json<(String,Color)>{
    let _ = manager.lock().unwrap().command(&name,Pattern::Constent(WHITE));
    Json((name,WHITE))
}
pub fn rocket(light_manager:Arc<Mutex<Manager>>) -> Rocket<Build> {
    rocket::build()
        .manage(light_manager)
        .mount("/", routes![status,set,off,on])
}
