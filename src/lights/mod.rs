pub mod strip;
pub use strip::Strip;

//pub mod light_group;
//pub use light_group::LightGroup;

//pub use light_group::LightGroup;
//pub mod light_manager;
//pub use light_manager::LightManager;

mod commands;
mod manager;

/*
use serde::{Serialize, Deserialize};

use std::time::{Duration,Instant};
//I think it would be nice to preserve this as high level commands.
//That way I do not have to worry about updating the app yet.
//State information should be removed and they will just change into the lower level command.
//Possible name User commands.
#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum Command{
    Constant(Color),
    Dim,
    Jump {
        colors:Vec<Color>,
        stage_time:Duration,
        current_index:usize,
        #[serde(skip)]
        last_updated:Option<Instant>
    }
}
impl Command{
    fn evolve(&mut self,current:Color)->Color{
        use Command::*;
        match self{
            Constant(color) => *color,
            Dim => {
                let color = Color::from((current.red()/2.,current.green()/2.,current.blue()/2.));
                *self = Constant(color);
                color
            },
            Jump{colors,stage_time,current_index,last_updated} =>{
                let update = match last_updated{
                    Some(time)=> time.elapsed()>= *stage_time,
                    None => true,
                };
                if update {
                    *current_index = (*current_index+1)%colors.len();
                    *last_updated = Some(Instant::now());
                };
                colors[*current_index]
            }
        }
    }
}
*/

use crate::color::*;
pub type PinConfig = (u8,u8,u8);
#[cfg(test)]
use std::sync::{Arc, Mutex};

#[cfg(not(test))]
use rppal::gpio::OutputPin;
#[cfg_attr(test,derive(Clone))]
struct Light{
    #[cfg(not(test))]
    red_pin: OutputPin,
    #[cfg(not(test))]
    green_pin: OutputPin,
    #[cfg(not(test))]
    blue_pin: OutputPin,

    #[cfg(test)]
    color:Arc<Mutex<Color>>
}

impl Light{
    #[cfg(not(test))]
    pub fn new(config:PinConfig)->Result<Self,Box<dyn Error>>{
        let mut light = Light{
            red_pin:Gpio::new()?.get(red)?.into_output(),
            green_pin:Gpio::new()?.get(green)?.into_output(),
            blue_pin:Gpio::new()?.get(blue)?.into_output(),
        };
        light.set(BLACK);
        light
    }
    #[cfg(test)]
    pub fn mock()->Self{
        Light{
            color:Arc::new(Mutex::new(BLACK)),
        }
    }
    fn set(&mut self,color:Color){
        #[cfg(test)]{
            *self.color.lock().unwrap() = color;
        }
        #[cfg(not(test))]
        {
            let set = |pin,intencity| pin.set_pwm_frequency(FREQUENCY,intencity)?;
            set(self.red_pin, color.red());
            set(self.blue_pin, color.blue());
            set(self.green_pin, color.green());
        }
    }
    #[cfg(test)]
    fn color(&self)->Color{
        *self.color.lock().unwrap()
    }
}
