import Pipe from './pipe';
import { HEIGHT_SIZE,WIDTH_SIZE } from './pipe';
import { HEIGHT,WIDTH }  from './bird';
const ARR_SIZE = 4;
const MIN_PIPE_DIFF = 450;

export default class PipeHandler {
    constructor(p,b) {
        this.allPipes = new Array(ARR_SIZE);
        const colors = ['#fae','#afe','#eaf','#efa'];
        for(let i = 0 ;i < ARR_SIZE; i++){
            const y = p.constrain(Math.random() * p.height, HEIGHT_SIZE,p.height - HEIGHT_SIZE - 50);
            this.allPipes[i] = new Pipe(p.width + MIN_PIPE_DIFF * i,y, p.width,colors[i] );
        }
        this.p5 = p;
        this.idx = 0;
        this.bird =b; 
    }



    update() {

        this.allPipes.forEach((x,i,arr) => x.update(this.p5,arr[(i - 1 + ARR_SIZE) % ARR_SIZE].pos.x + MIN_PIPE_DIFF));
        if(this.allPipes[this.idx].pos.x  + WIDTH_SIZE + WIDTH / 2 < this.p5.width / 3) {
            if(this.bird.state)  this.bird.counter++;
            this.idx = (this.idx + 1) % ARR_SIZE
        }
    }
    
    show() {
        this.allPipes.forEach(x => x.show(this.p5));
    }

    checkCollision() {
        const min_pipe = this.allPipes[this.idx]; 
        const check_y = this.bird.pos.y  - HEIGHT / 2 < min_pipe.pos.y || this.bird.pos.y + HEIGHT / 2 > min_pipe.pos.y + HEIGHT_SIZE;
        const check_x =  (this.bird.pos.x  - WIDTH / 2 < min_pipe.pos.x + WIDTH_SIZE) &&  (this.bird.pos.x + WIDTH / 2 >  min_pipe.pos.x) ;

        if(check_x && check_y) {
            this.bird.state = false
        } 
    }

}