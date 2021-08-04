use super::*;
#[test]
fn white_should_give_float_1s(){
    assert_eq!(WHITE.red(),1.0);
    assert_eq!(WHITE.green(),1.0);
    assert_eq!(WHITE.blue(),1.0);
}

#[test]
fn black_should_give_float_0s(){
    assert_eq!(BLACK.red(),0.0);
    assert_eq!(BLACK.green(),0.0);
    assert_eq!(BLACK.blue(),0.0);
}

#[test]
fn red_should_have_1_in_only_red(){
    assert_eq!(RED.red(),1.0);
    assert_eq!(RED.green(),0.0);
    assert_eq!(RED.blue(),0.0);
}

#[test]
fn green_should_have_1_in_only_red(){
    assert_eq!(GREEN.red(),0.0);
    assert_eq!(GREEN.green(),1.0);
    assert_eq!(GREEN.blue(),0.0);
}

#[test]
fn blue_should_have_1_in_only_red(){
    assert_eq!(BLUE.red(),0.0);
    assert_eq!(BLUE.green(),0.0);
    assert_eq!(BLUE.blue(),1.0);
}

#[test]
fn custome_color(){
    let custome_color = Color([0xff,0x88,0x0,0xff]);
    assert_eq!(custome_color.red(),1.0);
    assert!((custome_color.green()-0.5).abs()<0.1);
    assert_eq!(custome_color.blue(),0.0);
}

#[test]
fn aquamarine_should_serialize(){
    assert_eq!(serde_json::to_string(&AQUAMARINE).unwrap(),String::from("\"#7fffd4ff\""));
}

#[test]
fn aquamarine_should_deserialize(){
    assert_eq!(serde_json::from_str::<Color>("\"#7fffd4ff\"").unwrap(), AQUAMARINE);
}
