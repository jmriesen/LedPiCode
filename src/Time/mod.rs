use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(test)]
pub mod mock;

pub trait Updatable{
    fn update(&mut self,time:Instant);
}
impl <U:Updatable> Updatable for Arc<Mutex<U>>{
    fn update(&mut self,time:Instant){
        self.lock().unwrap().update(time);
    }
}
pub trait TimeSourceHandle: 'static + Clone{
    fn now(&self)->Instant;
    fn spawn_update_loop<U:'static +Updatable>(&self,updatable:U);
}

