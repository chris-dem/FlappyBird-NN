use crate::*;


#[derive(Debug)]
pub struct Brain {
    crate nn : nn::Network

}

impl Brain {
    pub fn random(rng : &mut dyn RngCore) -> Self {
        Self {
            nn : nn::Network::random(rng,&Self::topology())
        }
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    crate fn from_chromosome(chrom : ga::Chromosome) -> Self {
        Self {
            nn : nn::Network::from_weights(&Self::topology(), chrom)
        }
    }


    fn topology() -> [nn::LayerTopology ; 3] {
        [
                //Input nodes
                //01 : TopL dist
                //02 : BotL dist
                //03 : TopR dist
                //04 : BotR dist
                //05 : Next Pipe TopL dist
                //06 : Next Pipe BotL dist
                //07 : Next Pipe TopR dist
                //08 : Next Pipe BotR dist
                //09 : Ground dist
                //10 : y vel
                //11: x speed
                nn::LayerTopology {
                    neurons : 11
                },
                //Hidden layer
                nn::LayerTopology {
                    neurons : 5
                },
                



                //Output Layer
                //Jump
                //Dont jump
                //How much to jump
                nn::LayerTopology { neurons : 3}

        ]
    }
}