use std::error::Error;
use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use serde::{Serialize, Deserialize};

pub type PinConfig = (u8,u8,u8);
//pub type Color = (f64,f64,f64);
#[derive(Debug, Serialize, Deserialize,Copy,Clone)]
pub struct Color {
    red:f64,
    green:f64,
    blue:f64,
}

impl From<(f64,f64,f64)> for Color{
    fn from ((red,green,blue):(f64,f64,f64))->Self{
        Color{
            red:red,
            green:green,
            blue:blue,
        }
    }
}

// Ok I think I will redo this bit now to take in mind that i can use rgb values.
impl From<u32> for Color{
    fn from (int : u32)->Self{
        let red   = ((int & 0x00ff0000u32) >> 8*2) as u8;
        let green = ((int & 0x0000ff00u32) >> 8*1) as u8;
        let blue  = ((int & 0x000000ffu32) >> 8*0) as u8;
        Color{
            red:red as f64 /256.,
            green:green as f64/256.,
            blue:blue as f64/256.,
        }
    }
}
const FREQUENCY: f64 = 100.0;

pub struct Strip {
    red_pin: OutputPin,
    green_pin: OutputPin,
    blue_pin: OutputPin,
    color: Color,
    on :bool
}

impl Strip {
    pub fn new((red,green,blue):PinConfig)->Result<Strip,Box<dyn Error>>{
        let mut strip = Strip{
            red_pin:Gpio::new()?.get(red)?.into_output(),
            green_pin:Gpio::new()?.get(green)?.into_output(),
            blue_pin:Gpio::new()?.get(blue)?.into_output(),
            color:Color::from((0.5,0.5,0.5)),
            on:false,
        };
        let _ = strip.update_pwm();
        Ok(strip)
    }
    fn update_pwm(&mut self)->Result<(),Box<dyn Error>>{
        let color = match self.on{
            true => self.color,
            false => Color::from((0.0,0.0,0.0)),
        };
        self.red_pin.set_pwm_frequency(FREQUENCY,color.red)?;
        self.green_pin.set_pwm_frequency(FREQUENCY,color.green)?;
        self.blue_pin.set_pwm_frequency(FREQUENCY,color.blue)?;
        Ok(())
    }
    pub fn on(&mut self)->Result<(),Box<dyn Error>>{
        self.on = true;
        self.update_pwm()?;
        Ok(())
    }
    pub fn off(&mut self)->Result<(),Box<dyn Error>>{
        self.on = false;
        self.update_pwm()?;
        Ok(())
    }
    pub fn set(&mut self,color:Color)->Result<(),Box<dyn Error>>{
        self.color = color;
        self.update_pwm()?;
        Ok(())
    }
    /*
    pub fn fade(&mut self,target:Color)->Result<(),Box<dyn Error>>{
    let delta = delta;
    Loop;
    time stop.
}
     */
}
