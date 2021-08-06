use super::*;
use crate::color::*;

#[test]
fn constent_should_return_the_same_thing_regardless_of_time(){
    let command = Constent(RED).start(Instant::now());
    assert_eq!(command.get(Instant::now()),Some(RED));
    assert_eq!(command.get(Instant::now()+Duration::new(5, 0)),Some(RED));

    let command = Constent(BLUE).start(Instant::now());
    assert_eq!(command.get(Instant::now()+Duration::new(5, 0)),Some(BLUE));
}

#[test]
fn even_cycle_should_change_colors(){
    let command = EvenCycle(vec![RED,BLUE,GREEN],Duration::new(2, 0)).start(Instant::now());
    let start = Instant::now();
    let expected_results = vec![RED,RED,BLUE,BLUE,GREEN,GREEN];
    for (i, result) in expected_results.into_iter().enumerate(){
        assert_eq!(command.get(start + Duration::new(i as u64, 0)),Some(result));
    }
}
#[test]
fn empty_cycle_should_returen_none(){
    let command = EvenCycle(vec![],Duration::new(2, 0)).start(Instant::now());
    let start = Instant::now();
    assert_eq!(command.get(start + Duration::new(1, 0)),None);
}
#[test]
fn times_before_start_should_return_none(){
    let time = Instant::now();
    let command = Constent(RED).start(Instant::now());
    assert_eq!(command.get(time),None);
}
