#[cfg(test)]
mod tests;

use std::time::{
    Instant,
    Duration,
};
use crate::color::Color;
pub enum Pattern{
    Constent(Color),
    EvenCycle(Vec<Color>,Duration),
}
use Pattern::*;

impl Pattern{
    pub fn start(self,time:Instant)->Command{
        //TODO I think I need to introduce a time source.
        Command{start:time,pattern:self}
    }
}


pub struct Command{
    start:Instant,
    pattern:Pattern,
}

impl Command{
    pub fn get(&self,time:Instant)->Option<Color>{
        if time >= self.start{
            match &self.pattern{
                Constent(color) => Some(*color),
                EvenCycle(cycle,duration) => {
                    if cycle.len() ==  0{
                        None
                    }else{
                        let elaps = time-self.start;
                        Some(
                            cycle[div(elaps,duration)%cycle.len()]
                        )

                    }
                }
            }
        }else{
            None
        }
    }
}
    //TODO this is not efficient and could be optimized later
    fn div(mut numerator:Duration,divisor:&Duration)-> usize{
        println!("devision num:{:?} divisor:{:?}:",numerator,divisor);
        let mut iteration = 0;
        while numerator>= *divisor{
            iteration +=1;
            numerator -= *divisor;
        }
        println!("result:{}",iteration);
        iteration
    }
