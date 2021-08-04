pub mod strip;
pub use strip::Strip;

pub mod light_group;
pub use light_group::LightGroup;

//pub use light_group::LightGroup;
pub mod light_manager;
pub use light_manager::LightManager;

mod commands;
mod manager;

/*
use serde::{Serialize, Deserialize};

use std::time::{Duration,Instant};
//I think it would be nice to preserve this as high level commands.
//That way I do not have to worry about updating the app yet.
//State information should be removed and they will just change into the lower level command.
//Possible name User commands.
#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum Command{
    Constant(Color),
    Dim,
    Jump {
        colors:Vec<Color>,
        stage_time:Duration,
        current_index:usize,
        #[serde(skip)]
        last_updated:Option<Instant>
    }
}
impl Command{
    fn evolve(&mut self,current:Color)->Color{
        use Command::*;
        match self{
            Constant(color) => *color,
            Dim => {
                let color = Color::from((current.red()/2.,current.green()/2.,current.blue()/2.));
                *self = Constant(color);
                color
            },
            Jump{colors,stage_time,current_index,last_updated} =>{
                let update = match last_updated{
                    Some(time)=> time.elapsed()>= *stage_time,
                    None => true,
                };
                if update {
                    *current_index = (*current_index+1)%colors.len();
                    *last_updated = Some(Instant::now());
                };
                colors[*current_index]
            }
        }
    }
}
*/
