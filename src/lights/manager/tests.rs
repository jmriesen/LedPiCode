use super::*;
use std::time::Duration;
use crate::time::mock::{MockTimeHandle,new_mock_time_source};
fn setup()->
    (MockTimeHandle, Light, Arc<Mutex<Manager<MockTimeHandle>>>){
        let light = Light::mock();
        let time_handle = new_mock_time_source();
        let mut lights = HashMap::default();
        lights.insert(String::from("name"),light.clone());
        let manager = Manager::new(lights,time_handle.clone());
        (time_handle, light, manager)
    }

#[test]
fn lights_should_change_to_initial_color(){
    let (time, light, manager) = setup();
    let pattern = Pattern::Constent(RED);
    assert_eq!(manager.lock().unwrap().command("name",pattern),Ok(()));
    time.advance(Duration::new(1, 0));
    assert_eq!(light.color(),RED);
}

#[test]
fn invalid_name_should_be_repored(){
    let (_, _, manager) = setup();
    let pattern = Pattern::Constent(RED);
    assert_eq!(manager.lock().unwrap().command("Invalid Name",pattern),Err("name not found"));
}

#[test]
fn when_time_advances_color_advances(){
    let (time, light, manager) = setup();

    let pattern = Pattern::EvenCycle(vec![RED,BLUE,GREEN],Duration::new(2, 0));
    assert_eq!(manager.lock().unwrap().command("name",pattern),Ok(()));
    assert_eq!(light.color(), RED);
    time.advance(Duration::new(2, 0));
    assert_eq!(light.color(), BLUE);
    time.advance(Duration::new(2, 0));
    assert_eq!(light.color(), GREEN);
}
