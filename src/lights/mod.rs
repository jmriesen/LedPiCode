pub mod commands;
pub mod manager;
mod config;
#[cfg(test)]
mod tests;

use config::PinConfig;
use crate::color::*;
use crate::hardware::OutputPin;

pub struct Light{
    red_pin: OutputPin,
    green_pin: OutputPin,
    blue_pin: OutputPin,
    color:Color,
}

impl Light{
    pub fn new(pin_config:PinConfig)->Self{
        let mut light = Light{
            red_pin:pin_config.red,
            green_pin:pin_config.green,
            blue_pin:pin_config.blue,
            color: BLACK,
        };
        light.set(BLACK);
        light
    }
    fn refresh(&mut self){
        self.red_pin.pwm(self.color.red());
        self.green_pin.pwm(self.color.green());
        self.blue_pin.pwm(self.color.blue());
    }
    fn set(&mut self,color:Color){
        self.color = color;
        self.refresh();
    }
}
