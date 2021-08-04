use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::marker::PhantomData;

use std::{thread, time};
use crate::color::BLACK;
use super::{
    light_group::{self,LightGroup},
    Strip,
    strip::{self,PinConfig,Controls},
};



pub struct LightManager<ID>{
    pub lights : Vec<LightGroup<ID>>,
    strips: Vec<Arc<Mutex<Strip<ID>>>>,
    phantom:PhantomData<ID>,
}

impl <ID:'static + Send>LightManager<ID>{
    pub fn new()->Arc<Mutex<Self>>{
        let manager = Arc::new(Mutex::new(LightManager{
            //List of all light groups in the order the commands should be applied.
            lights:vec![],
            strips:vec![],
            phantom: PhantomData,
        }));
        LightManager::spawn_update_thread(manager.clone());
        manager
    }

    fn spawn_update_thread(manager:Arc<Mutex<Self>>){
        thread::spawn(move || {
            let delta = time::Duration::from_millis(10);
            loop{
                thread::sleep(delta);
                {
                    manager.lock().unwrap().refresh()
                }
            }
        });
    }

    fn refresh(&mut self){
        for strip in &self.strips{
            strip.lock().unwrap().set(BLACK);
        }
        for group in &mut  self.lights{
            group.refresh();
        }
        for strip in &self.strips{
            let _ = strip.lock().unwrap().refresh();
        }
    }
    pub fn get_group(&mut self,name:&str)->Option<&mut LightGroup<ID>>{
        self.lights.iter_mut()
            .filter(|x| x.name.eq(name))
            .next()
    }
    pub fn get_group_change_pos(&mut self,name:&str)->Option<&mut LightGroup<ID>>{
        let pos = self.lights.iter().position(|x| x.name.eq(name));
        let strip = self.lights.remove(pos?);
        self.lights.push(strip);
        self.get_group(name)
    }
    pub fn create_group(&mut self,name:String,leds:Vec<Arc<Mutex<Strip<ID>>>>)->&mut LightGroup<ID>{
        let mut group = light_group::new(name.clone());
        for led in leds{
            group.add(led)
        }
        self.lights.push(group);
        self.get_group(&name).unwrap()
    }
    pub fn create_strip(&mut self,name:String,pins:PinConfig)->Result<Arc<Mutex<Strip<ID>>>,Box<dyn Error>>{
        let strip = Arc::new(Mutex::new(strip::new(pins)?));
        self.strips.push(strip.clone());
        let _ = self.create_group(name,vec![strip.clone()]);
        Ok(strip)
    }
}

