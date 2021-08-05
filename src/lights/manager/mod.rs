#[cfg(test)]
mod tests;
use super::{Light};
use super::commands::{Pattern,Command};
use std::time::Instant;
use crate::color::*;

use std::collections::HashMap;
use crate::time::{Updatable,TimeHandle,TimeSourceHandle};
use std::sync::{Arc, Mutex};

struct Manager{
    lights:HashMap<String,(Light,Option<Command>)>,
    time_source:TimeHandle,

}

impl Manager{
    fn new(lights:HashMap<String,Light>,time_source:TimeHandle)->Arc<Mutex<Self>>{
        let lights  = lights
            .into_iter()
            .map(|(_name, light)|(_name,(light,None)))
            .collect();

        let manager = Arc::new(Mutex::new(
            Manager{lights,time_source:time_source.clone()}));
        time_source.spawn_update_loop(manager.clone());
        manager
    }
    fn command(&mut self,name:&str,pattern:Pattern)->Result<(),&'static str>{
        let (_light,command) = self.lights.get_mut(name).ok_or("name not found")?;
        let current_time = self.time_source.now();
            *command = Some(pattern.start(current_time));
        self.update(current_time);
        Ok(())
    }
}
impl Updatable for Manager{
    fn update(&mut self,time:Instant){
        for (light,command) in self.lights.values_mut(){
            if let Some(command) = command{
                let _ = light.set(command.get(time).unwrap_or(BLACK));
            }
        }
    }
}
