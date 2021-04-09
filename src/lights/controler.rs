use rocket_contrib::json::Json;
use rocket::Route;
use rocket::State;
use std::sync::Arc;

use std::sync::Mutex;
use super::{
    light_group::Status,
    light_group::Pattern,
    LightManager
};
pub struct RocketManager;
type RocketLightManager<'a> = State<'a, Arc<Mutex<LightManager<RocketManager>>>>;

#[get("/group")]
fn status(manager: RocketLightManager)->Json<Vec<Status>>{
    let state = manager.lock().unwrap().lights.iter()
        .map(|group| group.status())
        .collect();
    Json(state)
}

#[put("/group/<group>", data = "<pattern>")]
fn https_group_update(group:String,pattern:Json<Pattern>,manager:RocketLightManager){
    let mut manager = manager.lock().unwrap();
    match manager.get_group(&group){
        Some(group) => group.set(pattern.into_inner()),
        None => {},
    }
}

#[put("/off")]
fn master_off(manager:RocketLightManager){
    let mut manager = manager.lock().unwrap();
    for group in manager.lights.iter_mut(){
        group.power(false);
    }
}
//TODO on off should be thought about.
#[put("/on/<group>")]
fn on(group:String,manager:RocketLightManager){
    let mut manager = manager.lock().unwrap();
    match manager.get_group_change_pos(&group){
        Some(group) => group.power(true),
        None => {},
    }
}
#[put("/off/<group>")]
fn off(group:String,manager:RocketLightManager){
    let mut manager = manager.lock().unwrap();
    match manager.get_group(&group){
        Some(group) => group.power(false),
        None => {},
    }
}

pub fn endpoints()->Vec<Route>{
    routes![on,off,https_group_update,master_off,status]
}
pub fn examples()->Vec<Route>{
    //routes![on,off,set,set_int,example,get_groups]
    routes![patterns]
}

#[put("/patterns")]
pub fn patterns()->Json<Vec<Pattern>>{
    Json(vec![
        Pattern::Constent(Default::default()),
        Pattern::Loop(0,vec![(Default::default(),Default::default())])
    ])
}
