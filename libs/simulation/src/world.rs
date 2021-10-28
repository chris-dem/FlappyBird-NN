use crate::*;
use rand::RngCore;
#[derive(Debug)]
pub struct World {
    pub birds : Vec<Bird>,
    pub pipe_handler : PipeHandler,
    pub counter : usize
}


impl World {
    crate fn random(w: f32, h : f32,  rng :&mut impl RngCore,pop : usize) -> Self {
        let mut  birds = Vec::with_capacity(pop);
        for _ in 0..pop {
            birds.push(Bird::new(w, h  ,rng));
        }

        let pipe_h = PipeHandler::new(rng,(w,h));
        
        Self {
            birds,
            pipe_handler : pipe_h,
            counter : 0
        }

    }
    pub fn pipes(&self) -> &[Pipe] {
        &self.pipe_handler.pipes
    }
}