use crate::hardware::OutputPin;

#[cfg_attr(test,derive(Clone))]
pub struct PinConfig{
    pub red:OutputPin,
    pub green:OutputPin,
    pub blue:OutputPin,
}

#[cfg(test)]
pub use test::*;
#[cfg(test)]
mod test{
    use crate::color::Color;
    use super::*;
    impl PinConfig{
        pub fn mock()->Self{
            use crate::hardware::mock_pin;
            Self{
                red : mock_pin(),
                green : mock_pin(),
                blue : mock_pin(),
            }
        }
        pub fn verify(&self,color:Color){
            assert_eq!(self.red.intencity(),color.red());
            assert_eq!(self.green.intencity(),color.green());
            assert_eq!(self.blue.intencity(),color.blue());
        }

    }

}
