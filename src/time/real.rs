use super::*;
use std::{thread, time};

#[derive(Clone)]
pub struct RealTimeHandle;

impl TimeSourceHandle for RealTimeHandle{
    fn now(&self)->Instant{
        Instant::now()
    }
    fn spawn_update_loop<U:'static + Send +Updatable>(&self,mut updatable:U){
        thread::spawn(move || {
            let delta = time::Duration::from_millis(10);
            loop{
                thread::sleep(delta);
                {
                    updatable.update(Instant::now())
                }
            }
        });
    }
}
