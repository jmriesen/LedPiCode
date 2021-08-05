#[cfg(test)]
mod tests;
use super::{Light};
use super::commands::{Pattern,Command};
use std::time::Instant;
use crate::color::*;

use std::collections::HashMap;
use crate::time::{Updatable,TimeSourceHandle};
use std::sync::{Arc, Mutex};

struct Manager<TIMEHANDLE:TimeSourceHandle>{
    lights:HashMap<String,(Light,Option<Command>)>,
    time_source:TIMEHANDLE,

}

impl<TIMEHANDLE:TimeSourceHandle> Manager<TIMEHANDLE>{
    fn new(lights:HashMap<String,Light>,time_source:TIMEHANDLE)->Arc<Mutex<Self>>{
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
            *command = Some(pattern.start(current_time.clone()));
        self.update(current_time);
        Ok(())
    }
}
impl<TIMEHANDLE:TimeSourceHandle> Updatable for Manager<TIMEHANDLE>{
    fn update(&mut self,time:Instant){
        for (light,command) in self.lights.values_mut(){
            if let Some(command) = command{
                let _ = light.set(command.get(time).unwrap_or(BLACK));
            }
        }
    }
}
