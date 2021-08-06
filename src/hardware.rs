use std::sync::{Arc,Mutex};
//use rppal::gpio::{OutputPin,Gpio};

#[cfg(not(any(test,target_os = "macos")))]
pub struct RealPin(rppal::gpio::OutputPin);
#[cfg(not(any(test,target_os = "macos")))]
pub type OutputPin = RealPin;
#[cfg(any(test,target_os = "macos"))]
pub type OutputPin = MockOutputPin;

#[derive(Clone)]
pub struct MockOutputPin{
    intencity:Arc<Mutex<f64>>
}
impl MockOutputPin{
    #[cfg(test)]
    pub fn intencity(&self)->f64{
        *self.intencity.lock().unwrap()
    }
    pub fn new()->Self{
        Self{
            //I don't think rppal zeros pins so I just gave it some random value.
            //The starting value was reached by keyboard mashing
            intencity:Arc::new(Mutex::new(0.623732983))
        }
    }
}

#[cfg(test)]
pub fn mock_pin()->OutputPin{
    MockOutputPin::new()
}

#[allow(unused_variables)]
pub fn new_output_pin(numb:u8)->OutputPin{
    #[cfg(any(test,target_os = "macos"))]
    {
        MockOutputPin::new()
    }
    #[cfg(not(any(test,target_os = "macos")))]
    {
        use::rppal::gpio::Gpio;
        RealPin(Gpio::new().unwrap().get(numb).unwrap().into_output())
    }
}

impl OutputPin{
    pub fn pwm(&mut self, duty_cycle: f64){
        #[cfg(not(any(test,target_os = "macos")))]
        {
            let frequency =  100.0;
            let _ = self.0.set_pwm_frequency(frequency,duty_cycle);
        }
        #[cfg(any(test,target_os = "macos"))]
        {
            *self.intencity.lock().unwrap() = duty_cycle;
        }
    }
}
