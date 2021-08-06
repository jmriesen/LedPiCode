#[cfg(test)]
mod tests;
use super::{Light};
use super::commands::{Pattern,Command};
use std::time::Instant;
use crate::color::*;

use std::collections::HashMap;
use crate::time::{Updatable,TimeHandle,TimeSourceHandle};
use std::sync::{Arc, Mutex};

/*
The manager is responsible interacting with outside systems that do not have access to the light object directly.

Currently it is also responsible for updating the lights according to the time and their pattern.
I may consider splitting that up in the future but not right now.
*/
pub struct Manager{
    lights:HashMap<String,(Light,Option<Command>)>,
    time_source:TimeHandle,
}

impl Manager{
    pub fn new(lights:HashMap<String,Light>,time_source:TimeHandle)->Arc<Mutex<Self>>{
        let lights  = lights
            .into_iter()
            .map(|(_name, light)|(_name,(light,None)))
            .collect();

        let manager = Arc::new(Mutex::new(
            Manager{lights,time_source:time_source.clone()}));
        time_source.spawn_update_loop(manager.clone());
        manager
    }
    pub fn command(&mut self,name:&str,pattern:Pattern)->Result<(),&'static str>{
        let (_light,command) = self.lights.get_mut(name).ok_or("name not found")?;
        let current_time = self.time_source.now();
        *command = Some(pattern.start(current_time));
        self.update(current_time);
        Ok(())
    }
    #[cfg(test)]
    pub fn mock(names:Vec<&str>)->Arc<Mutex<Self>>{
        use crate::time::mock::{new_mock_time_source};
        use super::config::PinConfig;
        let mut lights = HashMap::default();
        let time_handle = new_mock_time_source();
        for name in names{
            lights.insert(name.into(), Light::new(PinConfig::mock()));
        }
        Manager::new(lights,time_handle.clone())
    }
    pub fn status(&self)->Vec<(String,Color)>{
        let mut lights :Vec<_>= self.lights.iter()
            .map(|(name,(light, _))| (name.clone(),light.color))
            .collect();
        lights.sort_by_key(|x| x.0.clone());
        lights
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
