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
                //1 : TopL dist
                //2 : BotL dist
                //3 : TopR dist
                //4 : BotR dist
                //5 : Ground dist
                //6 :  y vel
                nn::LayerTopology {
                    neurons : 6
                },
                //Hidden layer
                nn::LayerTopology {
                    neurons : 5
                },

                //Output Layer
                //Jump
                //Dont jump
                nn::LayerTopology { neurons : 2}

        ]
    }
}