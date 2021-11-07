
#![feature(crate_visibility_modifier)]
use std::usize;

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
mod world;
mod bird;
mod pipe;
mod piphandler;
mod bird_individual;
mod brain;
mod statistics;
// mod drawable;

const DEFAULT_POP : usize = 100usize;
pub use self::{world::*, bird::*,pipe::*,piphandler::*,bird_individual::*,brain::*,statistics::*};



use rand::{self, RngCore};

// #[derive(Debug)]

pub struct Simulation {
    width : f32,
    height : f32,
    pub world : World,
    ga : ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    birds_alive : usize,
    pub pop_no : usize
}


impl  Simulation {
    pub fn random(w : f32, h : f32,mut rng : impl rand::RngCore) -> Self {
        Self {
            world :  World::random(w,h,&mut rng,DEFAULT_POP),
            ga : ga::GeneticAlgorithm::new(
                ga::RouletteWheelSelection::default(),
                ga::UniformCrossover::default(),
                ga::GaussianMutation::new(0.07,0.8)//testing for now,
            ),
            width  : w,
            height : h,
            birds_alive : DEFAULT_POP,
            pop_no : 0
        }
    }
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self,rng : &mut dyn RngCore) -> Option<Statistics> {
        self.check_collision();
        self.process_brain();
        self.process_movement();
        
        if self.birds_alive == 0 {
            Some(self.evolve(rng))
        } else {
            None
        }
        // Some(self.birds_alive)
    }
    

    fn check_collision(&mut self){
        self.world.pipe_handler.check_collision(&mut self.birds_alive,&mut self.world.birds);
        
    }

    
    fn process_brain(&mut self) {
        for bird in  self.world.birds.iter_mut().filter(|p| p.alive){
            let vision = bird.look(&self.world.pipe_handler);
            let response = bird.brain.nn.propagate(vision);
            if response[0] > response[1] {
                bird.flap(-response[2])
            }
        }
    }

    
    fn process_movement(&mut self) {
        self.world.birds
            .iter_mut()
            .filter(|b| b.alive)
            .for_each(|x| x.process_movement() );
        self.world.pipe_handler.step(&mut self.world.birds);
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
        self.birds_alive = DEFAULT_POP;
        self.pop_no += 1;
        //Step 1: Prep population to evolve
        let current_population : Vec<AnimalIndividual> = self
            .world
            .birds
            .iter()
            .map(AnimalIndividual::from_animal).collect();
        //Step 2: Evolve
        let (evolved_pop,stats) = self.ga.evolve(rng, &current_population);

        //Step 3: Recover
        self.world.birds = evolved_pop
                .into_iter()
                .map(|ind| ind.to_bird(self.width,self.height))
                .collect();
        //Restart World
        self.world.pipe_handler.init_pipes(rng);
        Statistics{
            generation : self.pop_no,
            ga : stats
        }
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    mod test_sim {
        use super::*;
        use rand;

        #[test]
        fn test() {
            let rng = rand::thread_rng(); 
            let mut sim = Simulation::random(3072., 1452., rng);
            let mut rng = rand::thread_rng();
            loop {
                sim.step(&mut rng);
            }
        }
    }
}
