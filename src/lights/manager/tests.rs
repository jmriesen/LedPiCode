use super::*;
use std::time::Duration;
use crate::time::mock::{MockTimeHandle,new_mock_time_source};
use crate::lights::strip::MockPhantom;
fn setup()->
    (MockTimeHandle, Arc<Mutex<Color>>, Arc<Mutex<Manager<MockPhantom,MockTimeHandle>>>){
        let (light,internal) = Strip::mock();
        let time_handle = new_mock_time_source();
        let mut lights = HashMap::default();
        lights.insert(String::from("name"),light);
        let manager = Manager::new(lights,time_handle.clone());
        (time_handle, internal, manager)
    }

#[test]
fn lights_should_change_to_initial_color(){
    let (time, internal, manager) = setup();
    let pattern = Pattern::Constent(RED);
    assert_eq!(manager.lock().unwrap().command("name",pattern),Ok(()));
    time.advance(Duration::new(1, 0));
    assert_eq!(*internal.lock().unwrap(),RED);
}

#[test]
fn invalid_name_should_be_repored(){
    let (_, _, manager) = setup();
    let pattern = Pattern::Constent(RED);
    assert_eq!(manager.lock().unwrap().command("Invalid Name",pattern),Err("name not found"));
}

#[test]
fn when_time_advances_color_advances(){
    let (time, internal, manager) = setup();
    let check_color = || {*internal.lock().unwrap()};

    let pattern = Pattern::EvenCycle(vec![RED,BLUE,GREEN],Duration::new(2, 0));
    assert_eq!(manager.lock().unwrap().command("name",pattern),Ok(()));
    assert_eq!(check_color(),RED);
    time.advance(Duration::new(2, 0));
    assert_eq!(check_color(),BLUE);
    time.advance(Duration::new(2, 0));
    assert_eq!(check_color(),GREEN);
}
