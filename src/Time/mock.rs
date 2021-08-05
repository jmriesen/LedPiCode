use super::*;
use std::time::Duration;

#[derive(Clone)]
pub struct MockTimeHandle{
    time:Arc<Mutex<Instant>>,
    things_in_update_loop:Arc<Mutex<Vec<Box<dyn Updatable>>>>
}

impl MockTimeHandle{
    pub fn advance(&self,amount:Duration){
        *self.time.lock().unwrap()+=amount;
        let current_time = *self.time.lock().unwrap();

        let mut things_to_update = self.things_in_update_loop.lock().unwrap();
        for updatable in things_to_update.iter_mut(){
            updatable.update(current_time);
        }
    }
}
impl TimeSourceHandle for MockTimeHandle{
    fn now(&self)->Instant{
        *self.time.lock().unwrap()
    }
    fn spawn_update_loop<U:'static +Updatable>(&self,updatable:U){
        self.things_in_update_loop.lock().unwrap().push(Box::new(updatable));
    }
}
pub fn new_mock_time_source()->MockTimeHandle{
    MockTimeHandle{
        time:Arc::new(Mutex::new(Instant::now())),
        things_in_update_loop:Arc::new(Mutex::new(vec![])),
    }
}
