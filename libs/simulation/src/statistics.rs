use crate::*;
use std::fmt;

#[derive(Clone,Debug)]
pub struct Statistics {
    pub generation : usize,
    pub ga : ga::Statistics
}

impl fmt::Display for Statistics {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"generation {}",self.generation)?;
        write!(
            f,
            "min[{:.2}] max[{:.2}] avg[{:.2}] median[{:.2}]",
            self.ga.min_fitness() - 1.,
            self.ga.max_fitness() - 1., //due to the addition of 1 in the beginning
            self.ga.avg_fitness() -1.,
            self.ga.median_fitness() -1.,
        )

    }
}