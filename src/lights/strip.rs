use std::marker::PhantomData;
use std::error::Error;
use super::color::{Color};
pub type PinConfig = (u8,u8,u8);

pub trait Controls{
    fn set(&mut self,color:Color);
    fn refresh(&mut self)->Result<(),Box<dyn Error>>;
}

#[cfg(test)]
pub use mock::*;

#[cfg(not(test))]
pub use hardware::*;

#[cfg(test)]
mod mock{
    use super::*;
    pub struct Strip<Manager> {
        phantom:PhantomData<Manager>,
    }

    pub fn new<Manager>((_red,_green,_blue):PinConfig)->Result<Strip<Manager>,Box<dyn Error>>{
        Ok(Strip{
            phantom:PhantomData,
        })
    }

    impl <Manager>Controls for Strip<Manager>{
        fn set(&mut self,_color:Color){
        }

        fn refresh(&mut self)->Result<(),Box<dyn Error>>{
            Ok(())
        }
    }
}

#[cfg(not(test))]
mod hardware{
    use rppal::gpio::OutputPin;
    use rppal::gpio::Gpio;
    use super::*;
    use color::BLACK;
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

}
