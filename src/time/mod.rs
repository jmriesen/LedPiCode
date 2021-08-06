use std::time::Instant;
use std::sync::{Arc, Mutex};

#[cfg(test)]
pub mod mock;

#[cfg(test)]
pub type TimeHandle = mock::MockTimeHandle;

#[cfg(not(test))]
pub mod real;


#[cfg(not(test))]
pub type TimeHandle = real::RealTimeHandle;

impl TimeHandle {
    pub fn new()->Self{
        #[cfg(not(test))]
        {
            real::RealTimeHandle
        }
        #[cfg(test)]
        {
            mock::new_mock_time_source()
        }
    }
}

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

