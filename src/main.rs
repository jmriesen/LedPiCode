#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use std::error::Error;
use std::sync::Mutex;
use std::collections::HashMap;
use rocket::State;
// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.

mod strip;
use strip::{
    Strip,
    Color,
};
type Lights<'a> = State<'a,Mutex<HashMap<String,Strip>>>;
fn main() -> Result<(), Box<dyn Error>> {
    let mut strips = HashMap::new();
    strips.insert(String::from("Jacob"), Strip::new((27,22,17))?);
    strips.insert(String::from("Ben"), Strip::new((25,24,23))?);
    strips.insert(String::from("Night_Light"), Strip::new((13,6,5))?);
    rocket::ignite()
        .manage(Mutex::new(strips))
        .mount("/", routes![on,off,set_int,example,get_lights])
        .launch();
    Ok(())

    // When the pin variable goes out of scope, software-based PWM is automatically disabled.
    // You can manually disable PWM by calling the clear_pwm() method.
}


#[get("/on/<strip>")]
fn on(strip:String,lights: Lights){
    apply_to_strip(strip,lights,|strip|{
        let _  = strip.on();
    })
}

#[get("/off/<strip>")]
fn off(strip: String,lights: Lights){
    apply_to_strip(strip,lights,|strip|{
        let _  = strip.off();
    })
}

#[post("/set/<strip>", data = "<color>")]
fn set(color:Json<Color>,strip: String,lights: Lights){
    apply_to_strip(strip,lights,|strip|{
        let _  = strip.set(*color);
    })
}

#[get("/set/<strip>/<color>")]
fn set_int(color:u32,strip: String,lights: Lights){
    apply_to_strip(strip,lights,|strip|{
        let _  = strip.set(Color::from(color));
    })
}

#[get("/example")]
fn example() -> Json<Color> {
    Json(Color::from((0.0,0.0,0.0)))
}

#[get("/lights")]
fn get_lights(lights: Lights) ->Json<Vec<String>>{
    let map = lights.lock().unwrap();
    let mut lights :Vec<String>= map
        .keys()
        .map(|light| light.clone())
        .collect();
    lights.push(String::from("all"));
    Json(lights)
}

//Possibly replace with a light set idea.
//That way this higher level would not have to handle individual lights.
//The leds could contain a stack with references back to the light sets that are on. 
//When one of the lights sets is turned off it goose to the next in the set.
//I will Still want a main off switch so that I can kill everything ie turn off all sets..
//I think this will work quite nicely.
fn apply_to_strip<F>(strip: String,lights: Lights, action:F) where
    F: Fn(&mut Strip){
    let mut map = lights.lock().unwrap();
    if strip == String::from("all"){
        for strip in map.values_mut() {
            let _ = action(strip);
        }
    }else{
        match map.get_mut(&strip){
            Some(strip) => {
                let _ = action(strip);
            },
            None => {},
        }
    }
}
