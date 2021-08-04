use super::*;
use std::{thread, time};

#[derive(Clone)]
pub struct RealTimeHandle;

/*
impl MockTimeHandle{
    pub fn advance(&self,amount:Duration){
        let mut things_to_update = self.things_in_update_loop.lock().unwrap();
        for updatable in things_to_update.iter_mut(){
            updatable.update(current_time);
        }
    }
}
*/
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
