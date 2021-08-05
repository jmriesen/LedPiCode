use std::time::Instant;
use std::sync::{Arc, Mutex};

#[cfg(test)]
pub mod mock;

pub mod real;
pub trait Updatable:Send{
    fn update(&mut self,time:Instant);
}
impl <U:Updatable> Updatable for Arc<Mutex<U>>{
    fn update(&mut self,time:Instant){
        self.lock().unwrap().update(time);
    }
}
pub trait TimeSourceHandle: 'static + Clone +Send{
    fn now(&self)->Instant;
    fn spawn_update_loop<U:'static + Send +Updatable>(&self,updatable:U);
}

