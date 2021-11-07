
use crate::*;
use nalgebra as na;
use rand::{Rng, prelude::ThreadRng, thread_rng};

pub const PIPE_SPEED  : na::Vector2<f32> = na::Vector2::new(-5.,0.);
pub const WIDTH_SIZE :f32  = 100.;
pub const HEIGHT_SIZE : f32 = 200.;
pub static PIPE_COLOR  : &str = "1fd8a2";

#[derive(Clone,Debug)]
pub struct Pipe {
    crate position : na::Point2<f32>,
    crate rng : ThreadRng,
    crate bound :  (f32,f32)
}

impl Pipe {
    pub fn update(&mut self, pos_prev : f32,speed_inc : na::Vector2<f32>) {
        if self.position.x > -WIDTH_SIZE  {
            self.position += PIPE_SPEED + speed_inc;
        } else {
            self.position.x = pos_prev;
            self.position.y = (self.rng.gen::<f32>() * self.bound.1).clamp(HEIGHT_SIZE + 10.,self.bound.1 - HEIGHT_SIZE - 50. );
        }
    }

    pub fn new(x : f32, y : f32 ,bound : (f32,f32) ) -> Self {
        Self {
            position : na::Point2::new(x,y),
            bound,
            rng : thread_rng()

        }
    }

    pub fn position(&self) -> &na::Point2<f32> {
        &self.position
    }

    pub fn init(&mut self,pos_x : f32) {
        self.position.x = pos_x;
        self.position.y = (self.rng.gen::<f32>() * self.bound.1).clamp(HEIGHT_SIZE + 10.,self.bound.1 - HEIGHT_SIZE - 50. );
    }
}
