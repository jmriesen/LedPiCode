pub use super::strip::Strip;

//possibly change to enum.
const RED_INDEX   :usize = 0;
const GREEN_INDEX :usize = 1;
const BLUE_INDEX  :usize = 2;
const ALFA_INDEX  :usize = 3;
#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Color([u8;4]);


const COLOR_MAX:f64 =  255.;
pub const BLACK:Color = Color([255,0,0,0]);

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
        let mut arr = [0 as u8;4];
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

mod serialization{
    use super::*;
    use serde::{Serialize,Serializer,Deserialize,Deserializer};
    const RED_SHIFT:usize = 8*2;
    const GREEN_SHIFT:usize = 8*1;
    const BLUE_SHIFT:usize = 8*0;

    impl Color{
        fn to_int(&self)->u32{
            let mut val = 0;
            for (index,shift) in [(RED_INDEX,RED_SHIFT),(GREEN_INDEX,GREEN_SHIFT),(BLUE_INDEX,BLUE_SHIFT)].iter() {
                val = val | (self.0[*index] as u32) << (shift+8*1);
            }
            //val = val | 0xff000000; // the alpha value android use it.
            val = val | 0x000000ff; // the alpha value android use it.
            val //as i32
        }
    }
    impl Serialize for Color {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            //self.to_int().serialize(serializer)
            //u32::from_be_bytes(self.0).serialize(serializer)
            let hex = format!("#{}",hex::encode(self.0));
            hex.serialize(serializer)
        }
    }
    /*
    impl From<i32> for Color{
        fn from(i:i32)->Self{
            let mut arr = [0 as u8;3];
            for (index,shift) in [(RED_INDEX,RED_SHIFT),(GREEN_INDEX,GREEN_SHIFT),(BLUE_INDEX,BLUE_SHIFT)].iter(){
                arr[*index] = (0x000000ff & (i as u32 >> shift)) as u8;
            }
            Color(arr)
        }
    }
    */

    impl<'de> Deserialize<'de> for Color {
        fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
        where
            D: Deserializer<'de>,
        {
            //Ok(Color::from(i32::deserialize(deserializer)?))
            let mut bytes = [0u8; 4];
            let hex = String::deserialize(deserializer)?;
            println!("{}",hex);
            hex::decode_to_slice(&hex[1..], &mut bytes as &mut [u8]).unwrap();
            println!("{:?}",bytes);
            Ok(Color(bytes))
            //Ok(Color(u32::deserialize(deserializer)?.to_be_bytes()))
        }
    }

    #[cfg(test)]
    mod test{
        use super::*;
        use serde::de::{IntoDeserializer,value};
        #[test]
        fn deserialize_black() {
            let i = 0;
            let black:Result<Color,value::Error> = Color::deserialize(i.into_deserializer());
            assert_eq!(Ok(Color([0,0,0])),black);
        }

        #[test]
        fn deserialize_red() {
            let i = 0x00ff0000;
            let color:Result<Color,value::Error> = Color::deserialize(i.into_deserializer());
            assert_eq!(Ok(Color([COLOR_MAX as u8,0,0])),color);
        }
        #[test]
        fn deserialize_green() {
            let i = 0x0000ff00;
            let color:Result<Color,value::Error> = Color::deserialize(i.into_deserializer());
            assert_eq!(Ok(Color([0,COLOR_MAX as u8,0])),color);
        }
        #[test]
        fn deserialize_blue() {
            let i = 0x000000ff;
            let color:Result<Color,value::Error> = Color::deserialize(i.into_deserializer());
            assert_eq!(Ok(Color([0,0,COLOR_MAX as u8])),color);
        }
        #[test]
        fn deserialize_with_alpha() {
            //let i = 0xff0f0f0f as i32;
            let i = -15790321i32;
            let color:Result<Color,value::Error> = Color::deserialize(i.into_deserializer());
            assert_eq!(Ok(Color([15,15,15])),color);
        }

        #[test]
        fn serialize_black() {
            assert_eq!(serde_json::to_string(&Color([0,0,0,255])).unwrap(),"0".to_string());
        }
        #[test]
        fn serialize_red() {
            assert_eq!(serde_json::to_string(&Color([15,0,0,255])).unwrap(),"983040".to_string());
        }
        #[test]
        fn serialize_green() {
            assert_eq!(serde_json::to_string(&Color([0,15,0,255])).unwrap(),"3840".to_string());
        }
        #[test]
        fn serialize_blue() {
            assert_eq!(serde_json::to_string(&Color([0,0,15,255])).unwrap(),"15".to_string());
        }
    }
}
