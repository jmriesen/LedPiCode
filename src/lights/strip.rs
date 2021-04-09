use std::marker::PhantomData;
use std::error::Error;
use rppal::gpio::OutputPin;
use rppal::gpio::Gpio;
use super::color::{Color,BLACK};
pub type PinConfig = (u8,u8,u8);
const FREQUENCY: f64 = 100.0;

pub struct Strip<Manager> {
    red_pin: OutputPin,
    green_pin: OutputPin,
    blue_pin: OutputPin,
    color:Color,
    phantom:PhantomData<Manager>,
}
pub fn new<Manager>((red,green,blue):PinConfig)->Result<Strip<Manager>,Box<dyn Error>>{
    let mut strip = Strip{
        red_pin:Gpio::new()?.get(red)?.into_output(),
        green_pin:Gpio::new()?.get(green)?.into_output(),
        blue_pin:Gpio::new()?.get(blue)?.into_output(),
        color:BLACK,
        phantom:PhantomData,

    };
    strip.set(BLACK);
    strip.refresh()?;
    Ok(strip)
}

impl <Manager>Drop for Strip<Manager> {
    fn drop(&mut self) {
        let _  = self.set(BLACK);
    }
}

pub trait Controls{

    fn set(&mut self,color:Color);
    fn refresh(&mut self)->Result<(),Box<dyn Error>>;
}

impl <Manager>Controls for Strip<Manager>{
    fn set(&mut self,color:Color){
        self.color = color;
    }
    fn refresh(&mut self)->Result<(),Box<dyn Error>>{
        self.red_pin.set_pwm_frequency(FREQUENCY,self.color.red())?;
        self.green_pin.set_pwm_frequency(FREQUENCY,self.color.green())?;
        self.blue_pin.set_pwm_frequency(FREQUENCY,self.color.blue())?;
        Ok(())
    }
}
