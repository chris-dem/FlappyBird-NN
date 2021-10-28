
export const WIDTH_SIZE = 100;
export const HEIGHT_SIZE = 220;

export const WIDTH = 60;
export const HEIGHT = 35;
const ALIVE_COL = '#fae';
const DEAD_COL = '#0'

export default class SimRenderer {
    constructor(sim) {
        this.sim = sim
        this.stats = undefined
        this.current_gen;
    }
    show(p) {
        const world = this.sim.world();
        let max = 0;
        for(const bird of world.birds.filter((bird) => bird.state)) {
            this.showBird(p,bird.x,bird.y,bird.state);
            max = bird.counter;
        }
        
        for(const pipe of world.pipes) {
            this.showPipe(p,pipe.x,pipe.y)
        }
        p.push()
        p.fill('#0');
        p.textSize(30);
        p.text(max - 1,p.width /2 ,p.height / 8)
        p.pop()
        this.showPopNum(p)
        this.showLastStat(p)
    }


    showBird(p,x_bird,y_bird,state) {
        p.push()
        p.rectMode(p.CENTER); 
        p.translate(x_bird - WIDTH / 2 ,y_bird- HEIGHT / 2)
        p.fill(ALIVE_COL)
        if(!state) {
            p.rotate(p.PI / 2);
        }
        p.rect(WIDTH / 2,HEIGHT / 2,WIDTH, HEIGHT);
        
        p.pop()
    }


    showPipe(p,pipe_x,pipe_y) {
        p.push()
        p.fill('#1fd8a2')
        p.rect(pipe_x, 0, WIDTH_SIZE ,pipe_y)
        p.rect(pipe_x, pipe_y + HEIGHT_SIZE, WIDTH_SIZE , p.height - (pipe_y + HEIGHT_SIZE));
        p.pop()
    }


    showLastStat(p) {
        if(this.stat) {
            p.push()
            p.fill('#0');

            p.textSize(30);
            p.text(`Current Statistics\nPrevious max :${this.stat.max().toFixed(2)}\nPrevious avg:${this.stat.avg().toFixed(2)}\nPop number ${this.stat.pop_num()}`,p.width / 50  ,p.height / 8 + 50)
            p.pop()
        }
    }

    showPopNum(p) {
        p.push()
        p.fill('#0');

        p.textSize(30);
        p.text("Current Gen : " + (this.sim.current_gen() + 1),p.width / 50  ,p.height / 8)
        p.pop()
    }
    

    addStatistic(stat) {
        this.stat = stat
    }

}