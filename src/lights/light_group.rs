use std::sync::Arc;
use std::sync::Mutex;

use std::time::Duration;
use std::marker::PhantomData;
use serde::{Serialize, Deserialize};
use std::time::Instant;
use super::{
    Color,
    strip::Strip,
    strip::Controls,
};
//TODO remove this at some point.
#[derive(Debug, Serialize, Deserialize)]
pub struct Status{
    name: String,
    pattern:Pattern,
    on: bool
}

#[derive(Default,Debug, Serialize, Deserialize,Clone)]
pub struct Instruction{
    target:Color,
    fade_time:Duration,
}

impl Instruction{
    fn get(&self,current:Color,elapsed:Duration,delta:Duration)->Color{
        if elapsed>= self.fade_time{
            self.target
        }else {
            let time_left  = self.fade_time - elapsed;
            current.move_toward(self.target,delta.div_duration_f32(time_left))
        }
    }
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum Pattern{
    Constent(Instruction),
    Loop(usize,Vec<(Duration,Instruction)>),
    //Possible addition RunOnce(Vec<CommandAtoms>),
}


pub struct LightGroup<Manager>{
    pub name: String,
    lights:Vec<Arc<Mutex<Strip<Manager>>>>,
    color: Color,
    pattern:Pattern,
    on:bool,
    instruction_start_time:Instant,
    last_updated:Instant,
    phantom:PhantomData<Manager>,
}
pub fn new<Manager>(name:String)->LightGroup<Manager>{
    LightGroup{
        name: name,
        lights:vec![],
        color:Default::default(),
        pattern: Pattern::Constent(Default::default()),
        on:false,
        instruction_start_time:Instant::now(),
        last_updated:Instant::now(),
        phantom:PhantomData,
    }
}

impl <Manager>LightGroup<Manager>{
    pub fn add(&mut self,light:Arc<Mutex<Strip<Manager>>>){
        self.lights.push(light);
    }
    pub fn refresh(&mut self){
        if self.on {
            let instruction = match &mut self.pattern{
                Pattern::Constent(instruction) =>instruction,
                Pattern::Loop(index,sequence) =>{
                    let (duration,_) = &sequence[*index];
                    if duration < &self.instruction_start_time.elapsed(){
                        *index+=1;
                        *index%=sequence.len();
                        self.instruction_start_time = Instant::now();
                    }
                    let (_,instruction) = &sequence[*index];
                    instruction
                }
            };
            self.color = instruction.get(
                self.color,
                self.instruction_start_time.elapsed(),
                self.last_updated.elapsed()
            );

            for led in &mut self.lights{
                let mut led= led.lock().unwrap();
                let _ = led.set(self.color);
            }
        }
        self.last_updated = Instant::now();
    }
    pub fn set(&mut self,pattern:Pattern){
        self.pattern = pattern;
        self.instruction_start_time=  Instant::now();
    }
    pub fn power(&mut self,on:bool){
        self.on = on;
    }
    pub fn status(&self)->Status{
        Status{
            name :self.name.clone(),
            pattern:self.pattern.clone(),
            on   :self.on
        }
    }
}
