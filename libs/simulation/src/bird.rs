use std::{ops::AddAssign};
use crate::*;
use rand::{ RngCore};
use nalgebra as na;
use na::Vector2;
pub const ACC  : f32 = 0.7;
pub const WIDTH :f32 = 60.;
pub const HEIGHT : f32 = 35.;
const LIFT_MAX : f32= -20.;
const MIN_VY : f32 = -10.0 ;
const MAX_VY  : f32 =  30.0;
// static ALIVE_COL  : &str = "#fae";
// static  DEAD_COL  : &str = "#afe";

#[derive(Debug)]
pub struct Bird {
    boundary  : (f32,f32),
    crate position : na::Point2<f32>,
    velocity : na::Vector2<f32>,
    acc : na::Vector2<f32>,
    crate alive : bool,
    crate counter : usize,
    pub(crate) brain : Brain
}


impl Bird {
    // crate fn random(rng : &mut dyn RngCore,) -> Self {
    //     Self::new(0.,0.,(0.,0.))
    // }

    crate fn new(x :  f32, y : f32,rng : &mut dyn RngCore ) -> Self {
        let brain =  Brain::random(rng);
        Self {
            boundary : (x,y),
            position : na::Point2::new(x / 3. ,y / 2.),
            velocity : na::Vector2::new(0.,0.),
            acc : na::Vector2::new(0.,ACC),
            alive : true,
            counter : 1,
            brain
        }
    }

    fn new_with_brain(x :  f32, y : f32, brain : Brain) -> Self {
        Self {
            boundary : (x,y),
            position : na::Point2::new(x / 3.,y / 2.),
            velocity : na::Vector2::new(0.,0.),
            acc : na::Vector2::new(0.,ACC),
            alive : true,
            counter : 1,
            brain
        }
    }

    crate fn from_chromosome(
        chromosome : ga::Chromosome,
        x : f32, y : f32
    ) -> Self {
        Self::new_with_brain(x,y,Brain::from_chromosome(chromosome))
    }
    crate fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    
    pub fn rotation(&self) -> na::Vector2<f32> {
        self.velocity
    }

    
    pub fn acceleration(&self) -> na::Vector2<f32> {
        self.acc
    }

    pub fn state(&self) -> bool {
        self.alive
    }

    pub fn counter(&self) -> usize {
        self.counter
    }
}

impl  Bird {
    pub fn process_movement(&mut self) {
        if !self.alive {
            self.velocity = PIPE_SPEED;
        }
        if self.position.y + HEIGHT / 2. + 10. > self.boundary.1 {
            self.velocity.y = 0.;
            self.alive = false;
        } else {
            if self.position.y< 0. {  self.velocity.set_magnitude(0.) };
            self.velocity.add_assign(self.acc);
        }
        self.velocity.y = self.velocity.y.clamp(MIN_VY,MAX_VY);
        self.position.add_assign(self.velocity);
    }

    pub fn flap(&mut self,y : f32) {
        self.velocity.y += y.clamp(LIFT_MAX, 0.1);
    }

    fn dist_pipe(x : &na::Point2<f32>, y : &na::Point2<f32>) -> [f32;4] {
        [
            na::distance(x,y),
            na::distance(x, &(y + Vector2::new(0.,HEIGHT_SIZE))),
            na::distance(x, &(y + Vector2::new(WIDTH_SIZE,0.))),
            na::distance(x, &(y + Vector2::new(WIDTH_SIZE,HEIGHT_SIZE))),
        ]
    }
    pub fn look(&self, pipe_h : &PipeHandler) -> Vec<f32> {
        let mut vec = Vec::with_capacity(4usize);
        let min_pipe = pipe_h.ret_min_pipe();
        let smin_pipe = pipe_h.ret_smin_pipe();
        vec.extend(Self::dist_pipe(&self.position,min_pipe.position()).iter());
        vec.extend(Self::dist_pipe(&self.position,smin_pipe.position()).iter());
        vec.push(self.boundary.1 - self.position.y);
        vec.push(self.velocity.y);
        vec.push((pipe_h.speed_inc.x + PIPE_SPEED.x).abs());
        vec
    }

    
}

/* 
#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra as na;
    struct TestCase {
        position : na::Point2<f32>,
        pipe_pos : na::Point2<f32>,
        bird_vel : f32,
        expected_res : Vec<f32>
    }


    impl TestCase {
        fn run(self) {
            let mut vec = Vec::with_capacity(5usize);
            let min_pipe = self.pipe_pos;
            let top_dist : f32 = (self.position.y - HEIGHT / 2. -  min_pipe.y).abs();
            let bot_dist : f32 = (self.position.y + HEIGHT / 2. - (min_pipe.y + HEIGHT_SIZE)).abs();
            let hor_dist : f32 = (self.position.x - min_pipe.x).abs();
            vec.push(top_dist);
            vec.push(bot_dist);
            vec.push(hor_dist);
            // vec.push(self.position.y)
            vec.push(self.bird_vel);
            assert_eq!(vec,self.expected_res)
        }
    }
    mod pipe_dist {
        use super::*;
        use test_case::test_case;

        // #[test_case(250.,750.,300.,500.,0.,vec![1.,,,0.])]
        // // #[test_case(250.,0.,300.,0.,0.,vec![1.])]
        // // #[test_case(250.,0.,300.,0.,0.,vec![1.])]
        // // #[test_case(250.,0.,300.,0.,0.,vec![1.])]
        // // #[test_case(250.,0.,300.,0.,0.,vec![1.])]
        // fn test(xb : f32, yb : f32, xp: f32,yp : f32, y_vel : f32,expected : Vec<f32>) {
        //     TestCase {
        //         position : na::Point2::new(xb,yb),
        //         pipe_pos : na::Point2::new(xp,yp),
        //         bird_vel : y_vel,
        //         expected_res : expected
        //     }.run();
        // }
    }
}*/