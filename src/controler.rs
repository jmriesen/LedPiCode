
use rocket_contrib::json::Json;
use rocket::Route;
use rocket::State;
use std::sync::Arc;

use std::sync::Mutex;
use crate::lights::{
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
fn https_group_update(group:String,pattern:Json<Pattern>,manager:RocketLightManager)->Option<Json<Status>>{
    let mut manager = manager.lock().unwrap();
    let group = manager.get_group(&group)?;
    group.set(pattern.into_inner());
    Some(Json(group.status()))
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
fn on(group:String,manager:RocketLightManager)->Option<Json<Status>>{
    let mut manager = manager.lock().unwrap();
    let group = manager.get_group(&group)?;
    group.power(true);
    Some(Json(group.status()))
}
#[put("/off/<group>")]
fn off(group:String,manager:RocketLightManager)->Option<Json<Status>>{
    let mut manager = manager.lock().unwrap();
    let group = manager.get_group(&group)?;
    group.power(false);
    Some(Json(group.status()))

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
