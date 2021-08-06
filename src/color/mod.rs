#[cfg(test)]
mod tests;

mod constants;
pub use constants::*;
//possibly change to enum.
const RED_INDEX   :usize = 0;
const GREEN_INDEX :usize = 1;
const BLUE_INDEX  :usize = 2;
const _ALFA_INDEX  :usize = 3;
#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Color([u8;4]);


const COLOR_MAX:f64 =  255.;

impl Color{
    pub fn red(&self)->f64{
        self.0[RED_INDEX] as f64 /COLOR_MAX
    }
    pub fn green(&self)->f64{
        self.0[GREEN_INDEX] as f64 /COLOR_MAX
    }
    pub fn blue(&self)->f64{
        self.0[BLUE_INDEX] as f64 /COLOR_MAX
    }

    pub fn move_toward(mut self,target:Color, amount:f32)->Color{
        for i in 0..self.0.len(){
            let diff = target.0[i] as f32 - self.0[i] as f32;
            let new_value = self.0[i] as f32 + amount*diff;
            self.0[i] = new_value.clamp(0.0,COLOR_MAX as f32) as u8;
        }
        self
    }
}

impl From<[f64;4]> for Color{
    fn from(color:[f64;4])->Self{
        let mut arr = [0_u8;4];
        for i in 0..arr.len(){
            arr[i] = (color[i].clamp(0.,1.)*COLOR_MAX) as u8;
        }
        Color(arr)
    }
}
impl Default for Color{
    fn default() ->Self{
        Color::from([0.5,0.5,0.5,1.0])
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn move_to(){
        let current = Color([0,255,0,255]);
        let target  = Color([0,0,255,255]);
        assert_eq!(target,current.move_toward(target,1.0));
    }
    #[test]
    fn dim(){
        let current = Color([0,255,0,255]);
        let target  = Color([0,0,0,255]);
        assert_eq!(Color([0,127,0,255]),current.move_toward(target,0.5));
    }
    #[test]
    fn move_past(){
        let current = Color([0,0,0,255]);
        let target  = Color([1,2,3,255]);
        assert_eq!(Color([2,4,6,255]),current.move_toward(target,2.0));
    }
}
use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"#{}",hex::encode(self.0))
    }
}

mod serialization{
    use super::*;
    use serde::{Serialize,Serializer,Deserialize,Deserializer};

    impl Serialize for Color {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            format!("{}",self).serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Color {
        fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
        where
            D: Deserializer<'de>,
        {
            let mut bytes = [0u8; 4];
            let hex = String::deserialize(deserializer)?;
            hex::decode_to_slice(&hex[1..], &mut bytes as &mut [u8]).unwrap();
            Ok(Color(bytes))
        }
    }
}
