use crate::*;
use rand::{Rng, RngCore};

const ARR_SIZE : usize= 4;
const PIPE_DIFF : f32 = 450.;

type Bound = (f32,f32);
#[derive(Debug)]
pub struct PipeHandler {
    pub pipes : Vec<Pipe>,
    idx : usize,
    bound : Bound
}


impl PipeHandler {
    pub fn new(rng : &mut dyn RngCore, bound : Bound) -> Self {
        let mut arr : Vec<Pipe> = Vec::with_capacity(ARR_SIZE);
        for i in 0..ARR_SIZE {
            let val : f32 = rng.gen::<f32>();
            let y : f32 = ( val * bound.1).clamp(HEIGHT_SIZE,bound.1 - HEIGHT_SIZE - 50.);
            arr.push(Pipe::new(bound.0 + PIPE_DIFF * i as f32 ,y,bound.clone()));
        }
        Self {
            pipes : arr,
            idx : 0,
            bound
        }
    }

    pub fn init_pipes(&mut self, rng: &mut dyn RngCore) {
        for i in 0..ARR_SIZE {
            self.pipes[i].position.x = self.bound.0 + PIPE_DIFF * i as f32;
            self.pipes[i].position.y = (rng.gen::<f32>() * self.bound.1).clamp(HEIGHT_SIZE - HEIGHT / 2.,self.bound.1 - HEIGHT_SIZE - 50.);
        }
        self.idx = 0;
    }

    pub fn step(&mut self, birds : &mut Vec<Bird>) {
        let x : f32 = self.pipes[ARR_SIZE - 1].position.x;
        self.pipes[0].update(x + PIPE_DIFF);

        for (i,j) in (0..ARR_SIZE).zip(1..ARR_SIZE){
            let x = self.pipes[i].position.x + PIPE_DIFF;
            self.pipes[j].update(x);
        };
        if self.pipes[self.idx].position.x  + WIDTH_SIZE + WIDTH / 2. < self.bound.0 / 3. {
            self.idx = (self.idx + 1) % ARR_SIZE;
            birds.iter_mut()
                .filter(|b| b.state())
                .for_each(|b| b.counter += 1);
        }
    }
    pub fn check_collision(&mut self, bird_counter : &mut usize, birds: &mut Vec<Bird>) {
        let min_pipe = &self.pipes[self.idx];
        let mut counter : usize = 0;
        birds.iter_mut()
            .filter(|b| b.alive)
            .for_each(|bird|  {
                let check_y : bool  = bird.position.y - HEIGHT / 2. < min_pipe.position.y || bird.position.y + HEIGHT / 2. > min_pipe.position.y + HEIGHT_SIZE;
                let check_x  : bool = (bird.position.x - WIDTH  /2f32 < min_pipe.position.x + WIDTH_SIZE) && (bird.position.x + WIDTH / 2. > min_pipe.position.x );
                if check_x && check_y {
                    bird.alive = false;
                }else {
                    counter+= 1;
                }
            });
            *bird_counter = counter;
    }


    pub fn pipes_ptr(&self)  -> *const Pipe{
        self.pipes.as_ptr()
    }


    pub fn ret_min_pipe(&self) -> &Pipe {
        &self.pipes[self.idx]
    }
}