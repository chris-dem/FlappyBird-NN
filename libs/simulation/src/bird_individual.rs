use ga;

use crate::*;

#[derive(Debug)]
pub struct AnimalIndividual {
    fitness : f32,
    chromosome : ga::Chromosome
}


impl ga::Individual for AnimalIndividual {
    fn create(chromosome : ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }
    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }
    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal : &Bird ) -> Self {
        Self {
            fitness : animal.counter as f32,
            chromosome : animal.as_chromosome()
        }   
    }

    pub fn to_bird(self, x : f32 , y : f32) -> Bird {
        Bird::from_chromosome(self.chromosome,x,y)
    }
}