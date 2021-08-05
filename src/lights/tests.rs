use super::*;
pub fn setup_light_for_testing()->(PinConfig,Light){
    let pin_confg = PinConfig::mock();
    let light = Light::new(pin_confg.clone());
    (pin_confg,light)
}

#[test]
fn should_start_as_black(){
    let (pins,mut light) = setup_light_for_testing();
    let _ = light.set(BLACK);
    pins.verify(BLACK);
}

#[test]
fn pin_intensity_should_be_0_on_black(){
    let (pins,mut light) = setup_light_for_testing();
    let _ = light.set(BLACK);
    assert_eq!(pins.red.intencity(),0.);
    assert_eq!(pins.green.intencity(),0.);
    assert_eq!(pins.blue.intencity(),0.);
}
#[test]
fn pin_intensity_should_be_1_on_white(){
    let (pins,mut light) = setup_light_for_testing();
    let _ = light.set(WHITE);
    assert_eq!(pins.red.intencity(),1.);
    assert_eq!(pins.green.intencity(),1.);
    assert_eq!(pins.blue.intencity(),1.);
}

#[test]
fn should_aquamarine_match(){
    let (pins,mut light) = setup_light_for_testing();
    let _ = light.set(AQUAMARINE);
    pins.verify(AQUAMARINE);
}
