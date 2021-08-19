use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;
use super::*;
use crate::color::*;
use rocket::serde::json::from_str;
use std::error::Error;

fn setup()->Client{
    let lights = vec!["light1","light2","light3","light4"];
    let manager = Manager::mock(lights.clone());
    Client::tracked(rocket(manager)).unwrap()
}

fn verify(client:&Client,colors:Vec<Color>){
    let response = client.get("/status").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let expected : Vec<(String,Color)> =
        vec!["light1","light2","light3","light4"]
        .into_iter()
        .map(|x| String::from(x))
        .zip(colors)
        .collect();


    let response : Vec<(String,Color)> =
        from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(response, expected);
}

#[test]
fn starting_statuse_should_be_black()->Result<(),Box<dyn Error>>{
    let client = setup();
    verify(&client,vec![BLACK,BLACK,BLACK,BLACK]);
    Ok(())
}

#[test]
fn should_be_able_to_set_one_light()->Result<(),Box<dyn Error>>{
    let client = setup();
    let response = client
        .post("/set/light1")
        .json(&RED)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    verify(&client,vec![RED,BLACK,BLACK,BLACK]);

    Ok(())
}
#[test]
fn should_regect_unknown_names(){
    todo!();
}
